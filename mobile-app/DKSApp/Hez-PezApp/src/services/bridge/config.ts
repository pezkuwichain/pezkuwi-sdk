import { BridgeConfig } from './types';

export const BRIDGE_CONFIGS: BridgeConfig[] = [
  {
    fromChain: 'ethereum',
    toChain: 'polygon',
    minAmount: 0.01,
    maxAmount: 1000,
    fee: 0.001,
    estimatedTime: 10,
    supported: true
  },
  {
    fromChain: 'ethereum',
    toChain: 'arbitrum',
    minAmount: 0.01,
    maxAmount: 1000,
    fee: 0.0005,
    estimatedTime: 5,
    supported: true
  },
  {
    fromChain: 'ethereum',
    toChain: 'optimism',
    minAmount: 0.01,
    maxAmount: 1000,
    fee: 0.0005,
    estimatedTime: 5,
    supported: true
  },
  {
    fromChain: 'polygon',
    toChain: 'ethereum',
    minAmount: 10,
    maxAmount: 100000,
    fee: 0.002,
    estimatedTime: 30,
    supported: true
  },
  {
    fromChain: 'polygon',
    toChain: 'binance',
    minAmount: 10,
    maxAmount: 100000,
    fee: 0.001,
    estimatedTime: 15,
    supported: true
  },
  {
    fromChain: 'binance',
    toChain: 'polygon',
    minAmount: 0.1,
    maxAmount: 10000,
    fee: 0.001,
    estimatedTime: 15,
    supported: true
  },
  {
    fromChain: 'avalanche',
    toChain: 'ethereum',
    minAmount: 1,
    maxAmount: 10000,
    fee: 0.002,
    estimatedTime: 20,
    supported: true
  },
  {
    fromChain: 'arbitrum',
    toChain: 'optimism',
    minAmount: 0.01,
    maxAmount: 1000,
    fee: 0.0003,
    estimatedTime: 10,
    supported: true
  }
];

export const getBridgeConfig = (fromChain: string, toChain: string): BridgeConfig | undefined => {
  return BRIDGE_CONFIGS.find(
    config => config.fromChain === fromChain && config.toChain === toChain
  );
};

export const getSupportedDestinations = (fromChain: string): string[] => {
  return BRIDGE_CONFIGS
    .filter(config => config.fromChain === fromChain && config.supported)
    .map(config => config.toChain);
};

export const SUPPORTED_TOKENS = [
  { symbol: 'ETH', name: 'Ethereum', chains: ['ethereum', 'arbitrum', 'optimism'] },
  { symbol: 'MATIC', name: 'Polygon', chains: ['polygon'] },
  { symbol: 'BNB', name: 'Binance Coin', chains: ['binance'] },
  { symbol: 'AVAX', name: 'Avalanche', chains: ['avalanche'] },
  { symbol: 'USDC', name: 'USD Coin', chains: ['ethereum', 'polygon', 'arbitrum', 'optimism', 'avalanche', 'binance'] },
  { symbol: 'USDT', name: 'Tether', chains: ['ethereum', 'polygon', 'arbitrum', 'optimism', 'avalanche', 'binance'] },
  { symbol: 'DAI', name: 'Dai', chains: ['ethereum', 'polygon', 'arbitrum', 'optimism'] },
];