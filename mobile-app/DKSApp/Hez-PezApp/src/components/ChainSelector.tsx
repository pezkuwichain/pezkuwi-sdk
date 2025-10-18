import React, { useState, useEffect } from 'react';
import { ChevronDown, Globe, Check, Loader2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Badge } from '@/components/ui/badge';
import { BlockchainAPI } from '@/services/blockchain/api';
import { ChainConfig, SUPPORTED_CHAINS } from '@/services/blockchain/config';
import { chainWebSocketManager } from '@/services/blockchain/websocket';
import { useToast } from '@/hooks/use-toast';

interface ChainSelectorProps {
  onChainChange?: (chain: ChainConfig) => void;
  className?: string;
}

export function ChainSelector({ onChainChange, className }: ChainSelectorProps) {
  const [selectedChain, setSelectedChain] = useState<ChainConfig>(SUPPORTED_CHAINS[0]);
  const [chains, setChains] = useState<ChainConfig[]>(SUPPORTED_CHAINS);
  const [connectionStatus, setConnectionStatus] = useState<Record<string, boolean>>({});
  const [loading, setLoading] = useState(false);
  const { toast } = useToast();

  useEffect(() => {
    loadChains();
    setupWebSocketConnections();

    return () => {
      chainWebSocketManager.disconnectAll();
    };
  }, []);

  const loadChains = async () => {
    try {
      const dbChains = await BlockchainAPI.getChainConfigs();
      if (dbChains.length > 0) {
        setChains(dbChains as ChainConfig[]);
      }
    } catch (error) {
      console.error('Error loading chains:', error);
    }
  };

  const setupWebSocketConnections = () => {
    chains.forEach(chain => {
      chainWebSocketManager.connect(chain);
      
      chainWebSocketManager.subscribe(chain.chainId, (data) => {
        if (data.type === 'connected') {
          setConnectionStatus(prev => ({ ...prev, [chain.chainId]: true }));
        } else if (data.type === 'error') {
          setConnectionStatus(prev => ({ ...prev, [chain.chainId]: false }));
        }
      });
    });
  };

  const handleChainSelect = async (chain: ChainConfig) => {
    setLoading(true);
    try {
      setSelectedChain(chain);
      
      // Save connection to database
      if (window.ethereum) {
        const accounts = await window.ethereum.request({ method: 'eth_accounts' });
        if (accounts.length > 0) {
          await BlockchainAPI.saveWalletConnection(
            chain.chainId,
            accounts[0],
            'metamask'
          );
        }
      }

      onChainChange?.(chain);
      
      toast({
        title: 'Chain Switched',
        description: `Connected to ${chain.name}`,
      });
    } catch (error) {
      toast({
        title: 'Switch Failed',
        description: 'Failed to switch chain',
        variant: 'destructive',
      });
    } finally {
      setLoading(false);
    }
  };

  const getChainIcon = (chain: ChainConfig) => {
    const icons: Record<string, string> = {
      ethereum: '⟠',
      polygon: '🟣',
      arbitrum: '🔵',
      optimism: '🔴',
      avalanche: '🔺',
      binance: '🟡',
    };
    return icons[chain.chainId] || '🌐';
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" className={className} disabled={loading}>
          {loading ? (
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          ) : (
            <>
              <span className="mr-2 text-lg">{getChainIcon(selectedChain)}</span>
              <span className="mr-2">{selectedChain.name}</span>
              <Badge variant={connectionStatus[selectedChain.chainId] ? 'default' : 'secondary'} className="mr-2">
                {connectionStatus[selectedChain.chainId] ? 'Connected' : 'Offline'}
              </Badge>
            </>
          )}
          <ChevronDown className="h-4 w-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-64">
        {chains.map((chain) => (
          <DropdownMenuItem
            key={chain.chainId}
            onClick={() => handleChainSelect(chain)}
            className="flex items-center justify-between"
          >
            <div className="flex items-center">
              <span className="mr-2 text-lg">{getChainIcon(chain)}</span>
              <div>
                <div className="font-medium">{chain.name}</div>
                <div className="text-xs text-muted-foreground">
                  {chain.nativeToken} • Block time: {chain.blockTime}s
                </div>
              </div>
            </div>
            <div className="flex items-center gap-2">
              {connectionStatus[chain.chainId] && (
                <div className="h-2 w-2 rounded-full bg-green-500" />
              )}
              {selectedChain.chainId === chain.chainId && (
                <Check className="h-4 w-4" />
              )}
            </div>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}