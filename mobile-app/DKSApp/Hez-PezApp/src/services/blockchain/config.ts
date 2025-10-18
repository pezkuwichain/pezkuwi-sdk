export interface ChainConfig {
  chainId: string;
  name: string;
  rpcUrl: string;
  explorerUrl?: string;
  nativeToken: string;
  iconUrl?: string;
  blockTime: number;
  isActive: boolean;
}

export interface WalletConnection {
  id: string;
  chainId: string;
  walletAddress: string;
  walletType: string;
  isPrimary: boolean;
  lastConnected: Date;
}

export const SUPPORTED_CHAINS: ChainConfig[] = [
  {
    chainId: 'ethereum',
    name: 'Ethereum',
    rpcUrl: 'https://mainnet.infura.io/v3/',
    explorerUrl: 'https://etherscan.io',
    nativeToken: 'ETH',
    iconUrl: '/chains/ethereum.svg',
    blockTime: 12,
    isActive: true
  },
  {
    chainId: 'polygon',
    name: 'Polygon',
    rpcUrl: 'https://polygon-rpc.com',
    explorerUrl: 'https://polygonscan.com',
    nativeToken: 'MATIC',
    iconUrl: '/chains/polygon.svg',
    blockTime: 2,
    isActive: true
  },
  {
    chainId: 'arbitrum',
    name: 'Arbitrum',
    rpcUrl: 'https://arb1.arbitrum.io/rpc',
    explorerUrl: 'https://arbiscan.io',
    nativeToken: 'ETH',
    iconUrl: '/chains/arbitrum.svg',
    blockTime: 1,
    isActive: true
  },
  {
    chainId: 'optimism',
    name: 'Optimism',
    rpcUrl: 'https://mainnet.optimism.io',
    explorerUrl: 'https://optimistic.etherscan.io',
    nativeToken: 'ETH',
    iconUrl: '/chains/optimism.svg',
    blockTime: 2,
    isActive: true
  },
  {
    chainId: 'avalanche',
    name: 'Avalanche',
    rpcUrl: 'https://api.avax.network/ext/bc/C/rpc',
    explorerUrl: 'https://snowtrace.io',
    nativeToken: 'AVAX',
    iconUrl: '/chains/avalanche.svg',
    blockTime: 2,
    isActive: true
  },
  {
    chainId: 'binance',
    name: 'BNB Chain',
    rpcUrl: 'https://bsc-dataseed.binance.org',
    explorerUrl: 'https://bscscan.com',
    nativeToken: 'BNB',
    iconUrl: '/chains/binance.svg',
    blockTime: 3,
    isActive: true
  }
];

export const getChainConfig = (chainId: string): ChainConfig | undefined => {
  return SUPPORTED_CHAINS.find(chain => chain.chainId === chainId);
};

export const getActiveChains = (): ChainConfig[] => {
  return SUPPORTED_CHAINS.filter(chain => chain.isActive);
};