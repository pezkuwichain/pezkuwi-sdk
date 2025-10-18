export interface BridgeConfig {
  fromChain: string;
  toChain: string;
  minAmount: number;
  maxAmount: number;
  fee: number;
  estimatedTime: number; // in minutes
  supported: boolean;
}

export interface BridgeQuote {
  fromChain: string;
  toChain: string;
  fromToken: string;
  toToken: string;
  fromAmount: number;
  toAmount: number;
  fee: number;
  estimatedTime: number;
  route: string[];
}

export interface BridgeTransaction {
  id: string;
  fromChain: string;
  toChain: string;
  fromAddress: string;
  toAddress: string;
  fromToken: string;
  toToken: string;
  fromAmount: number;
  toAmount: number;
  fee: number;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  txHash?: string;
  createdAt: Date;
  completedAt?: Date;
  error?: string;
}

export interface TokenBalance {
  chain: string;
  token: string;
  symbol: string;
  balance: number;
  decimals: number;
  usdValue?: number;
}