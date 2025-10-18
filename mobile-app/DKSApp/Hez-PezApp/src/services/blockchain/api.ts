import { supabase } from '@/lib/supabase';
import { ChainConfig } from './config';

export interface CrossChainProposal {
  id: string;
  proposalId: string;
  originChain: string;
  targetChains: string[];
  title: string;
  description: string;
  status: string;
  syncStatus: Record<string, string>;
  createdBy: string;
  createdAt: Date;
}

export interface BridgeTransaction {
  id: string;
  fromChain: string;
  toChain: string;
  fromAddress: string;
  toAddress: string;
  amount: number;
  token: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  txHash?: string;
  bridgeFee?: number;
  createdAt: Date;
  completedAt?: Date;
}

export class BlockchainAPI {
  static async getChainConfigs(): Promise<ChainConfig[]> {
    const { data, error } = await supabase
      .from('chain_configs')
      .select('*')
      .eq('is_active', true)
      .order('name');

    if (error) throw error;
    return data || [];
  }

  static async saveWalletConnection(
    chainId: string,
    walletAddress: string,
    walletType: string
  ) {
    const { data: user } = await supabase.auth.getUser();
    if (!user?.user) throw new Error('User not authenticated');

    const { error } = await supabase
      .from('wallet_connections')
      .upsert({
        user_id: user.user.id,
        chain_id: chainId,
        wallet_address: walletAddress,
        wallet_type: walletType,
        last_connected: new Date().toISOString()
      });

    if (error) throw error;
  }

  static async getUserWallets() {
    const { data: user } = await supabase.auth.getUser();
    if (!user?.user) return [];

    const { data, error } = await supabase
      .from('wallet_connections')
      .select('*')
      .eq('user_id', user.user.id)
      .order('last_connected', { ascending: false });

    if (error) throw error;
    return data || [];
  }

  static async createCrossChainProposal(
    proposal: Omit<CrossChainProposal, 'id' | 'createdAt'>
  ) {
    const { data: user } = await supabase.auth.getUser();
    if (!user?.user) throw new Error('User not authenticated');

    const { data, error } = await supabase
      .from('cross_chain_proposals')
      .insert({
        proposal_id: proposal.proposalId,
        origin_chain: proposal.originChain,
        target_chains: proposal.targetChains,
        title: proposal.title,
        description: proposal.description,
        status: proposal.status,
        sync_status: proposal.syncStatus,
        created_by: user.user.id
      })
      .select()
      .single();

    if (error) throw error;
    return data;
  }

  static async getCrossChainProposals(chainId?: string) {
    let query = supabase
      .from('cross_chain_proposals')
      .select('*')
      .order('created_at', { ascending: false });

    if (chainId) {
      query = query.or(`origin_chain.eq.${chainId},target_chains.cs.{${chainId}}`);
    }

    const { data, error } = await query;
    if (error) throw error;
    return data || [];
  }

  static async createBridgeTransaction(
    transaction: Omit<BridgeTransaction, 'id' | 'createdAt'>
  ) {
    const { data: user } = await supabase.auth.getUser();
    if (!user?.user) throw new Error('User not authenticated');

    const { data, error } = await supabase
      .from('bridge_transactions')
      .insert({
        user_id: user.user.id,
        from_chain: transaction.fromChain,
        to_chain: transaction.toChain,
        from_address: transaction.fromAddress,
        to_address: transaction.toAddress,
        amount: transaction.amount,
        token: transaction.token,
        status: transaction.status,
        tx_hash: transaction.txHash,
        bridge_fee: transaction.bridgeFee
      })
      .select()
      .single();

    if (error) throw error;
    return data;
  }

  static async getBridgeTransactions() {
    const { data: user } = await supabase.auth.getUser();
    if (!user?.user) return [];

    const { data, error } = await supabase
      .from('bridge_transactions')
      .select('*')
      .eq('user_id', user.user.id)
      .order('created_at', { ascending: false });

    if (error) throw error;
    return data || [];
  }
}