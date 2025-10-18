import { BlockchainAPI, CrossChainProposal } from './api';
import { ChainConfig } from './config';
import { chainWebSocketManager } from './websocket';

export interface SyncStatus {
  chainId: string;
  lastSyncTime: Date;
  proposalsSynced: number;
  status: 'idle' | 'syncing' | 'error';
  error?: string;
}

export class CrossChainDataSync {
  private syncIntervals: Map<string, NodeJS.Timeout> = new Map();
  private syncStatus: Map<string, SyncStatus> = new Map();

  async startSync(chains: ChainConfig[]) {
    for (const chain of chains) {
      this.startChainSync(chain);
    }
  }

  private startChainSync(chain: ChainConfig) {
    // Clear existing interval if any
    this.stopChainSync(chain.chainId);

    // Initial sync
    this.syncChainData(chain);

    // Set up periodic sync
    const interval = setInterval(() => {
      this.syncChainData(chain);
    }, 30000); // Sync every 30 seconds

    this.syncIntervals.set(chain.chainId, interval);

    // Subscribe to WebSocket events
    chainWebSocketManager.subscribe(chain.chainId, (data) => {
      if (data.type === 'proposal_update') {
        this.handleProposalUpdate(chain.chainId, data);
      }
    });
  }

  stopChainSync(chainId: string) {
    const interval = this.syncIntervals.get(chainId);
    if (interval) {
      clearInterval(interval);
      this.syncIntervals.delete(chainId);
    }
  }

  stopAllSync() {
    this.syncIntervals.forEach((interval, chainId) => {
      this.stopChainSync(chainId);
    });
  }

  private async syncChainData(chain: ChainConfig) {
    const status: SyncStatus = {
      chainId: chain.chainId,
      lastSyncTime: new Date(),
      proposalsSynced: 0,
      status: 'syncing'
    };

    this.syncStatus.set(chain.chainId, status);

    try {
      // Fetch proposals from the chain
      const proposals = await this.fetchChainProposals(chain);
      
      // Sync to database
      for (const proposal of proposals) {
        await this.syncProposal(chain.chainId, proposal);
        status.proposalsSynced++;
      }

      status.status = 'idle';
    } catch (error) {
      status.status = 'error';
      status.error = error instanceof Error ? error.message : 'Unknown error';
      console.error(`Sync error for ${chain.name}:`, error);
    }

    this.syncStatus.set(chain.chainId, status);
  }

  private async fetchChainProposals(chain: ChainConfig): Promise<any[]> {
    // Simulated chain data fetch
    // In production, this would call the actual chain RPC
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([
          {
            id: `${chain.chainId}-prop-1`,
            title: `${chain.name} Improvement Proposal`,
            description: 'Cross-chain governance proposal',
            status: 'active'
          }
        ]);
      }, 1000);
    });
  }

  private async syncProposal(chainId: string, proposalData: any) {
    try {
      await BlockchainAPI.createCrossChainProposal({
        proposalId: proposalData.id,
        originChain: chainId,
        targetChains: [],
        title: proposalData.title,
        description: proposalData.description,
        status: proposalData.status,
        syncStatus: { [chainId]: 'synced' },
        createdBy: 'system'
      });
    } catch (error) {
      // Handle duplicate key errors gracefully
      if (error instanceof Error && error.message.includes('duplicate')) {
        // Update existing proposal
        console.log(`Proposal ${proposalData.id} already exists, skipping...`);
      } else {
        throw error;
      }
    }
  }

  private async handleProposalUpdate(chainId: string, data: any) {
    console.log(`Handling proposal update from ${chainId}:`, data);
    // Handle real-time proposal updates
    await this.syncProposal(chainId, data.proposal);
  }

  getSyncStatus(chainId?: string): SyncStatus | SyncStatus[] {
    if (chainId) {
      return this.syncStatus.get(chainId) || {
        chainId,
        lastSyncTime: new Date(),
        proposalsSynced: 0,
        status: 'idle'
      };
    }
    return Array.from(this.syncStatus.values());
  }

  async forceSyncChain(chainId: string) {
    const chain = { chainId } as ChainConfig;
    await this.syncChainData(chain);
  }
}

export const crossChainDataSync = new CrossChainDataSync();

// Legacy exports for backward compatibility
export interface SyncState {
  proposals: any[];
  votes: Map<string, any>;
  treasury: any;
  latestBlock: any;
  isLoading: boolean;
  error: string | null;
  lastSync: number;
}

export const dataSyncService = {
  subscribe: (callback: (state: SyncState) => void) => {
    // Return empty unsubscribe function
    return () => {};
  },
  stopSync: () => {
    crossChainDataSync.stopAllSync();
  },
  refreshProposals: async () => {
    // No-op for now
  },
  refreshTreasury: async () => {
    // No-op for now
  }
};