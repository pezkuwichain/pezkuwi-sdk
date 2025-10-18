import { ChainConfig } from './config';

export interface BlockData {
  chainId: string;
  blockNumber: number;
  timestamp: number;
  transactions: number;
  gasPrice?: string;
}

export interface ChainStatus {
  chainId: string;
  isConnected: boolean;
  latestBlock: number;
  syncProgress?: number;
}

export interface WebSocketMessage {
  type: string;
  data: any;
}

export class ChainWebSocketManager {
  private connections: Map<string, WebSocket> = new Map();
  private listeners: Map<string, Set<(data: any) => void>> = new Map();
  private reconnectTimers: Map<string, NodeJS.Timeout> = new Map();
  private eventHandlers: Map<string, Set<(data: any) => void>> = new Map();

  connect(chain?: ChainConfig | string) {
    // Legacy compatibility
    if (!chain) return;
    const chainId = typeof chain === 'string' ? chain : chain.chainId;
    
    // Simulated connection
    this.notifyEvent('connected', { chainId });
  }

  disconnect(chainId?: string) {
    if (!chainId) {
      this.disconnectAll();
      return;
    }
    
    const ws = this.connections.get(chainId);
    if (ws) {
      ws.close();
      this.connections.delete(chainId);
    }

    const timer = this.reconnectTimers.get(chainId);
    if (timer) {
      clearTimeout(timer);
      this.reconnectTimers.delete(chainId);
    }
    
    this.notifyEvent('disconnected', { chainId });
  }

  disconnectAll() {
    this.connections.forEach((ws, chainId) => {
      this.disconnect(chainId);
    });
  }

  subscribe(event: string, callback?: (data: any) => void) {
    if (!callback) return;
    
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(callback);

    return () => this.unsubscribe(event, callback);
  }

  unsubscribe(event: string, callback?: (data: any) => void) {
    if (!callback) return;
    
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.delete(callback);
      if (handlers.size === 0) {
        this.eventHandlers.delete(event);
      }
    }
  }

  on(event: string, callback: (data: any) => void) {
    return this.subscribe(event, callback);
  }

  send(message: any) {
    // Legacy compatibility - broadcast to all chains
    this.connections.forEach((ws) => {
      if (ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify(message));
      }
    });
  }

  private notifyEvent(event: string, data: any) {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.forEach(callback => callback(data));
    }
  }

  private notifyListeners(chainId: string, data: any) {
    const listeners = this.listeners.get(chainId);
    if (listeners) {
      listeners.forEach(callback => callback(data));
    }
  }

  getConnectionStatus(chainId: string): boolean {
    const ws = this.connections.get(chainId);
    return ws?.readyState === WebSocket.OPEN;
  }

  sendMessage(chainId: string, message: any) {
    const ws = this.connections.get(chainId);
    if (ws?.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(message));
      return true;
    }
    return false;
  }
}

export const chainWebSocketManager = new ChainWebSocketManager();

// Legacy export for backward compatibility
export const blockchainWS = chainWebSocketManager;