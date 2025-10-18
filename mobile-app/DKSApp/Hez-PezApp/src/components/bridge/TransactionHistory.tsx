import React, { useState, useEffect } from 'react';
import { ArrowRight, Clock, CheckCircle, XCircle, Loader2, ExternalLink } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { BridgeService } from '@/services/bridge/bridgeService';
import { BridgeTransaction } from '@/services/bridge/types';
import { SUPPORTED_CHAINS } from '@/services/blockchain/config';
import { formatDistanceToNow } from 'date-fns';

export function TransactionHistory() {
  const [transactions, setTransactions] = useState<BridgeTransaction[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<'all' | 'pending' | 'completed' | 'failed'>('all');

  useEffect(() => {
    loadTransactions();
    const interval = setInterval(loadTransactions, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  const loadTransactions = async () => {
    try {
      const history = await BridgeService.getBridgeHistory();
      setTransactions(history);
    } catch (error) {
      console.error('Error loading transaction history:', error);
    } finally {
      setLoading(false);
    }
  };

  const getChainName = (chainId: string) => {
    return SUPPORTED_CHAINS.find(c => c.chainId === chainId)?.name || chainId;
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'failed':
        return <XCircle className="h-4 w-4 text-red-500" />;
      case 'processing':
        return <Loader2 className="h-4 w-4 animate-spin text-blue-500" />;
      default:
        return <Clock className="h-4 w-4 text-yellow-500" />;
    }
  };

  const getStatusBadge = (status: string) => {
    const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
      completed: 'default',
      pending: 'secondary',
      processing: 'outline',
      failed: 'destructive'
    };
    
    return (
      <Badge variant={variants[status] || 'outline'} className="capitalize">
        {status}
      </Badge>
    );
  };

  const filteredTransactions = transactions.filter(tx => {
    if (filter === 'all') return true;
    return tx.status === filter;
  });

  const openExplorer = (chainId: string, txHash?: string) => {
    if (!txHash) return;
    const chain = SUPPORTED_CHAINS.find(c => c.chainId === chainId);
    if (chain?.explorerUrl) {
      window.open(`${chain.explorerUrl}/tx/${txHash}`, '_blank');
    }
  };

  if (loading) {
    return (
      <Card>
        <CardContent className="flex items-center justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Bridge History</CardTitle>
          <div className="flex gap-2">
            {(['all', 'pending', 'completed', 'failed'] as const).map(status => (
              <Button
                key={status}
                variant={filter === status ? 'default' : 'outline'}
                size="sm"
                onClick={() => setFilter(status)}
                className="capitalize"
              >
                {status}
              </Button>
            ))}
          </div>
        </div>
      </CardHeader>
      <CardContent>
        <ScrollArea className="h-[400px] pr-4">
          {filteredTransactions.length === 0 ? (
            <div className="text-center py-8 text-muted-foreground">
              No transactions found
            </div>
          ) : (
            <div className="space-y-4">
              {filteredTransactions.map((tx) => (
                <div
                  key={tx.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-muted/50 transition-colors"
                >
                  <div className="flex items-center gap-4">
                    {getStatusIcon(tx.status)}
                    <div>
                      <div className="flex items-center gap-2 font-medium">
                        <span>{getChainName(tx.fromChain)}</span>
                        <ArrowRight className="h-3 w-3" />
                        <span>{getChainName(tx.toChain)}</span>
                      </div>
                      <div className="text-sm text-muted-foreground mt-1">
                        {tx.amount} {tx.token}
                        {tx.fee && (
                          <span className="ml-2">
                            (Fee: {tx.fee} {tx.token})
                          </span>
                        )}
                      </div>
                      <div className="text-xs text-muted-foreground mt-1">
                        {formatDistanceToNow(new Date(tx.createdAt), { addSuffix: true })}
                      </div>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    {getStatusBadge(tx.status)}
                    {tx.txHash && (
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => openExplorer(tx.fromChain, tx.txHash)}
                      >
                        <ExternalLink className="h-4 w-4" />
                      </Button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </ScrollArea>
      </CardContent>
    </Card>
  );
}