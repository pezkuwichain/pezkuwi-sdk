// Wallet configuration and utilities for PezkuwiChain
export const PEZKUWICHAIN_NETWORK = {
  chainId: '0x2329', // 9001 in hex
  chainName: 'PezkuwiChain',
  nativeCurrency: {
    name: 'Pezkuwi',
    symbol: 'PZK',
    decimals: 18
  },
  rpcUrls: ['https://rpc.pezkuwichain.app'],
  blockExplorerUrls: ['https://explorer.pezkuwichain.app']
};

export const WALLET_ERRORS = {
  NO_WALLET: 'No wallet detected. Please install MetaMask or use WalletConnect.',
  CONNECTION_FAILED: 'Failed to connect wallet. Please try again.',
  NETWORK_ERROR: 'Failed to switch network. Please add PezkuwiChain manually.',
  TRANSACTION_FAILED: 'Transaction failed. Please check your balance and try again.',
  USER_REJECTED: 'User rejected the request.'
};

export const formatAddress = (address: string): string => {
  if (!address) return '';
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
};

export const formatBalance = (balance: string, decimals = 18): string => {
  if (!balance) return '0';
  const value = parseFloat(balance) / Math.pow(10, decimals);
  return value.toFixed(4);
};

export interface WalletState {
  isConnected: boolean;
  address: string | null;
  balance: string;
  chainId: string | null;
  provider: any;
  error: string | null;
}

export const initialWalletState: WalletState = {
  isConnected: false,
  address: null,
  balance: '0',
  chainId: null,
  provider: null,
  error: null
};