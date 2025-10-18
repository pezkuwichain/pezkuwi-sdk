import React, { useState, useEffect } from 'react';
import { Globe, Users, Clock, TrendingUp, AlertCircle, Loader2 } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { BlockchainAPI, CrossChainProposal } from '@/services/blockchain/api';
import { SUPPORTED_CHAINS } from '@/services/blockchain/config';
import { crossChainDataSync } from '@/services/blockchain/dataSync';
import { formatDistanceToNow } from 'date-fns';

export function ProposalsListLive() {
  const [proposals, setProposals] = useState<CrossChainProposal[]>([]);
  const [selectedChain, setSelectedChain] = useState('all');
  const [loading, setLoading] = useState(true);
  const [syncStatus, setSyncStatus] = useState<Record<string, any>>({});

  useEffect(() => {
    loadProposals();
    startDataSync();

    const interval = setInterval(() => {
      loadProposals();
      updateSyncStatus();
    }, 15000);

    return () => {
      clearInterval(interval);
      crossChainDataSync.stopAllSync();
    };
  }, [selectedChain]);

  const loadProposals = async () => {
    try {
      const chainFilter = selectedChain === 'all' ? undefined : selectedChain;
      const data = await BlockchainAPI.getCrossChainProposals(chainFilter);
      setProposals(data as CrossChainProposal[]);
    } catch (error) {
      console.error('Error loading proposals:', error);
    } finally {
      setLoading(false);
    }
  };

  const startDataSync = async () => {
    const chains = SUPPORTED_CHAINS.filter(c => c.isActive);
    await crossChainDataSync.startSync(chains);
  };

  const updateSyncStatus = () => {
    const statuses = crossChainDataSync.getSyncStatus();
    const statusMap: Record<string, any> = {};
    
    if (Array.isArray(statuses)) {
      statuses.forEach(status => {
        statusMap[status.chainId] = status;
      });
    }
    
    setSyncStatus(statusMap);
  };

  const getChainName = (chainId: string) => {
    return SUPPORTED_CHAINS.find(c => c.chainId === chainId)?.name || chainId;
  };

  const getChainIcon = (chainId: string) => {
    const icons: Record<string, string> = {
      ethereum: '⟠',
      polygon: '🟣',
      arbitrum: '🔵',
      optimism: '🔴',
      avalanche: '🔺',
      binance: '🟡',
    };
    return icons[chainId] || '🌐';
  };

  const getSyncStatusBadge = (chainId: string) => {
    const status = syncStatus[chainId];
    if (!status) return null;

    const variant = status.status === 'syncing' ? 'secondary' : 
                    status.status === 'error' ? 'destructive' : 'default';
    
    return (
      <Badge variant={variant} className="text-xs">
        {status.status === 'syncing' && <Loader2 className="h-3 w-3 mr-1 animate-spin" />}
        {status.status}
      </Badge>
    );
  };

  const activeProposals = proposals.filter(p => p.status === 'active');
  const pendingProposals = proposals.filter(p => p.status === 'pending');
  const completedProposals = proposals.filter(p => p.status === 'completed');

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
    <div className="space-y-6">
      {/* Chain Filter */}
      <div className="flex gap-2 overflow-x-auto pb-2">
        <Button
          variant={selectedChain === 'all' ? 'default' : 'outline'}
          size="sm"
          onClick={() => setSelectedChain('all')}
        >
          <Globe className="h-4 w-4 mr-1" />
          All Chains
        </Button>
        {SUPPORTED_CHAINS.map(chain => (
          <Button
            key={chain.chainId}
            variant={selectedChain === chain.chainId ? 'default' : 'outline'}
            size="sm"
            onClick={() => setSelectedChain(chain.chainId)}
            className="flex items-center gap-1"
          >
            <span className="text-lg">{getChainIcon(chain.chainId)}</span>
            {chain.name}
            {getSyncStatusBadge(chain.chainId)}
          </Button>
        ))}
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Total Proposals</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{proposals.length}</div>
            <p className="text-xs text-muted-foreground">Across all chains</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Active Voting</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{activeProposals.length}</div>
            <p className="text-xs text-muted-foreground">In progress</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Pending Sync</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{pendingProposals.length}</div>
            <p className="text-xs text-muted-foreground">Awaiting cross-chain sync</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium">Completed</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{completedProposals.length}</div>
            <p className="text-xs text-muted-foreground">Successfully executed</p>
          </CardContent>
        </Card>
      </div>

      {/* Proposals List */}
      <Card>
        <CardHeader>
          <CardTitle>Cross-Chain Proposals</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="active" className="space-y-4">
            <TabsList>
              <TabsTrigger value="active">Active ({activeProposals.length})</TabsTrigger>
              <TabsTrigger value="pending">Pending ({pendingProposals.length})</TabsTrigger>
              <TabsTrigger value="completed">Completed ({completedProposals.length})</TabsTrigger>
            </TabsList>

            <TabsContent value="active">
              <ScrollArea className="h-[400px]">
                <div className="space-y-4 pr-4">
                  {activeProposals.map((proposal) => (
                    <ProposalCard key={proposal.id} proposal={proposal} />
                  ))}
                  {activeProposals.length === 0 && (
                    <div className="text-center py-8 text-muted-foreground">
                      No active proposals
                    </div>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="pending">
              <ScrollArea className="h-[400px]">
                <div className="space-y-4 pr-4">
                  {pendingProposals.map((proposal) => (
                    <ProposalCard key={proposal.id} proposal={proposal} />
                  ))}
                  {pendingProposals.length === 0 && (
                    <div className="text-center py-8 text-muted-foreground">
                      No pending proposals
                    </div>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>

            <TabsContent value="completed">
              <ScrollArea className="h-[400px]">
                <div className="space-y-4 pr-4">
                  {completedProposals.map((proposal) => (
                    <ProposalCard key={proposal.id} proposal={proposal} />
                  ))}
                  {completedProposals.length === 0 && (
                    <div className="text-center py-8 text-muted-foreground">
                      No completed proposals
                    </div>
                  )}
                </div>
              </ScrollArea>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}

function ProposalCard({ proposal }: { proposal: CrossChainProposal }) {
  const getChainName = (chainId: string) => {
    return SUPPORTED_CHAINS.find(c => c.chainId === chainId)?.name || chainId;
  };

  const getChainIcon = (chainId: string) => {
    const icons: Record<string, string> = {
      ethereum: '⟠',
      polygon: '🟣',
      arbitrum: '🔵',
      optimism: '🔴',
      avalanche: '🔺',
      binance: '🟡',
    };
    return icons[chainId] || '🌐';
  };

  const mockVotes = Math.floor(Math.random() * 1000) + 100;
  const mockProgress = Math.floor(Math.random() * 100);

  return (
    <div className="p-4 border rounded-lg hover:bg-muted/50 transition-colors">
      <div className="flex items-start justify-between mb-2">
        <div className="flex-1">
          <h3 className="font-semibold">{proposal.title}</h3>
          <p className="text-sm text-muted-foreground mt-1">
            {proposal.description}
          </p>
        </div>
        <Badge variant={proposal.status === 'active' ? 'default' : 'secondary'}>
          {proposal.status}
        </Badge>
      </div>

      <div className="flex items-center gap-4 mt-3 text-sm">
        <div className="flex items-center gap-1">
          <span className="text-lg">{getChainIcon(proposal.originChain)}</span>
          <span>{getChainName(proposal.originChain)}</span>
        </div>
        
        {proposal.targetChains.length > 0 && (
          <>
            <span className="text-muted-foreground">→</span>
            <div className="flex items-center gap-1">
              {proposal.targetChains.map(chain => (
                <span key={chain} className="text-lg" title={getChainName(chain)}>
                  {getChainIcon(chain)}
                </span>
              ))}
            </div>
          </>
        )}

        <div className="flex items-center gap-1 text-muted-foreground">
          <Users className="h-3 w-3" />
          <span>{mockVotes} votes</span>
        </div>

        <div className="flex items-center gap-1 text-muted-foreground">
          <Clock className="h-3 w-3" />
          <span>{formatDistanceToNow(new Date(proposal.createdAt), { addSuffix: true })}</span>
        </div>
      </div>

      {proposal.status === 'active' && (
        <div className="mt-3 space-y-1">
          <div className="flex justify-between text-xs">
            <span>Voting Progress</span>
            <span>{mockProgress}%</span>
          </div>
          <Progress value={mockProgress} className="h-2" />
        </div>
      )}

      {Object.keys(proposal.syncStatus).length > 0 && (
        <div className="flex gap-2 mt-3">
          {Object.entries(proposal.syncStatus).map(([chain, status]) => (
            <Badge key={chain} variant="outline" className="text-xs">
              {getChainIcon(chain)} {status}
            </Badge>
          ))}
        </div>
      )}
    </div>
  );
}