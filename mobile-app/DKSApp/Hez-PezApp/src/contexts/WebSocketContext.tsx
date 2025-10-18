import React, { createContext, useContext, useEffect, useState, useCallback, useRef } from 'react';
import { useToast } from '@/hooks/use-toast';
import { blockchainWS, WebSocketMessage } from '@/services/blockchain/websocket';
import { dataSyncService, SyncState } from '@/services/blockchain/dataSync';

interface WebSocketContextType {
  isConnected: boolean;
  syncState: SyncState;
  subscribe: (event: string, callback: (data: any) => void) => void;
  unsubscribe: (event: string, callback: (data: any) => void) => void;
  sendMessage: (message: any) => void;
  reconnect: () => void;
  refreshProposals: () => Promise<void>;
  refreshTreasury: () => Promise<void>;
}

const WebSocketContext = createContext<WebSocketContextType | null>(null);

export const useWebSocket = () => {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error('useWebSocket must be used within WebSocketProvider');
  }
  return context;
};

export const WebSocketProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isConnected, setIsConnected] = useState(false);
  const [syncState, setSyncState] = useState<SyncState>({
    proposals: [],
    votes: new Map(),
    treasury: null,
    latestBlock: null,
    isLoading: false,
    error: null,
    lastSync: 0,
  });
  
  const eventListeners = useRef<Map<string, Set<(data: any) => void>>>(new Map());
  const { toast } = useToast();

  useEffect(() => {
    // Subscribe to blockchain WebSocket events
    blockchainWS.on('connected', () => {
      setIsConnected(true);
      toast({
        title: "Blockchain Connected",
        description: "Real-time blockchain updates enabled",
      });
    });

    blockchainWS.on('disconnected', () => {
      setIsConnected(false);
      toast({
        title: "Disconnected",
        description: "Attempting to reconnect...",
        variant: "destructive",
      });
    });

    blockchainWS.on('message', (message: WebSocketMessage) => {
      const listeners = eventListeners.current.get(message.type);
      if (listeners) {
        listeners.forEach(callback => callback(message.data));
      }
    });

    blockchainWS.on('error', (error) => {
      console.error('Blockchain WebSocket error:', error);
      toast({
        title: "Connection Error",
        description: "Failed to maintain blockchain connection",
        variant: "destructive",
      });
    });

    // Subscribe to data sync updates
    const unsubscribeSync = dataSyncService.subscribe((state) => {
      setSyncState(state);
      
      // Show sync status
      if (state.error) {
        toast({
          title: "Sync Error",
          description: state.error,
          variant: "destructive",
        });
      }
    });

    return () => {
      unsubscribeSync();
      blockchainWS.disconnect();
      dataSyncService.stopSync();
    };
  }, [toast]);

  const subscribe = useCallback((event: string, callback: (data: any) => void) => {
    if (!eventListeners.current.has(event)) {
      eventListeners.current.set(event, new Set());
      blockchainWS.subscribe(event);
    }
    eventListeners.current.get(event)?.add(callback);
  }, []);

  const unsubscribe = useCallback((event: string, callback: (data: any) => void) => {
    const listeners = eventListeners.current.get(event);
    if (listeners) {
      listeners.delete(callback);
      if (listeners.size === 0) {
        eventListeners.current.delete(event);
        blockchainWS.unsubscribe(event);
      }
    }
  }, []);

  const sendMessage = useCallback((message: any) => {
    blockchainWS.send(message);
  }, []);

  const reconnect = useCallback(() => {
    blockchainWS.disconnect();
    blockchainWS.connect();
  }, []);

  const refreshProposals = useCallback(async () => {
    await dataSyncService.refreshProposals();
  }, []);

  const refreshTreasury = useCallback(async () => {
    await dataSyncService.refreshTreasury();
  }, []);

  return (
    <WebSocketContext.Provider value={{ 
      isConnected, 
      syncState,
      subscribe, 
      unsubscribe, 
      sendMessage, 
      reconnect,
      refreshProposals,
      refreshTreasury
    }}>
      {children}
    </WebSocketContext.Provider>
  );
};