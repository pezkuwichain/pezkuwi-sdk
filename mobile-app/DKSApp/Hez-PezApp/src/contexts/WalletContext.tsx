import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import { PEZKUWICHAIN_NETWORK, WALLET_ERRORS, initialWalletState, WalletState } from '@/lib/wallet';

interface WalletContextType extends WalletState {
  connectMetaMask: () => Promise<void>;
  connectWalletConnect: () => Promise<void>;
  disconnect: () => void;
  switchNetwork: () => Promise<void>;
  signTransaction: (tx: any) => Promise<string>;
  signMessage: (message: string) => Promise<string>;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [walletState, setWalletState] = useState<WalletState>(initialWalletState);

  const updateBalance = useCallback(async (address: string, provider: any) => {
    try {
      const balance = await provider.request({
        method: 'eth_getBalance',
        params: [address, 'latest']
      });
      setWalletState(prev => ({ ...prev, balance }));
    } catch (error) {
      console.error('Failed to fetch balance:', error);
    }
  }, []);

  const connectMetaMask = useCallback(async () => {
    if (!window.ethereum) {
      setWalletState(prev => ({ ...prev, error: WALLET_ERRORS.NO_WALLET }));
      return;
    }

    try {
      const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
      const chainId = await window.ethereum.request({ method: 'eth_chainId' });
      
      setWalletState({
        isConnected: true,
        address: accounts[0],
        balance: '0',
        chainId,
        provider: window.ethereum,
        error: null
      });

      await updateBalance(accounts[0], window.ethereum);
    } catch (error: any) {
      setWalletState(prev => ({
        ...prev,
        error: error.code === 4001 ? WALLET_ERRORS.USER_REJECTED : WALLET_ERRORS.CONNECTION_FAILED
      }));
    }
  }, [updateBalance]);

  const connectWalletConnect = useCallback(async () => {
    // WalletConnect implementation placeholder
    setWalletState(prev => ({
      ...prev,
      error: 'WalletConnect integration coming soon'
    }));
  }, []);

  const disconnect = useCallback(() => {
    setWalletState(initialWalletState);
  }, []);

  const switchNetwork = useCallback(async () => {
    if (!walletState.provider) return;

    try {
      await walletState.provider.request({
        method: 'wallet_switchEthereumChain',
        params: [{ chainId: PEZKUWICHAIN_NETWORK.chainId }]
      });
    } catch (error: any) {
      if (error.code === 4902) {
        try {
          await walletState.provider.request({
            method: 'wallet_addEthereumChain',
            params: [PEZKUWICHAIN_NETWORK]
          });
        } catch (addError) {
          setWalletState(prev => ({ ...prev, error: WALLET_ERRORS.NETWORK_ERROR }));
        }
      }
    }
  }, [walletState.provider]);

  const signTransaction = useCallback(async (tx: any): Promise<string> => {
    if (!walletState.provider || !walletState.address) {
      throw new Error('Wallet not connected');
    }

    try {
      const result = await walletState.provider.request({
        method: 'eth_sendTransaction',
        params: [{ ...tx, from: walletState.address }]
      });
      return result;
    } catch (error: any) {
      throw new Error(error.message || WALLET_ERRORS.TRANSACTION_FAILED);
    }
  }, [walletState.provider, walletState.address]);

  const signMessage = useCallback(async (message: string): Promise<string> => {
    if (!walletState.provider || !walletState.address) {
      throw new Error('Wallet not connected');
    }

    try {
      const result = await walletState.provider.request({
        method: 'personal_sign',
        params: [message, walletState.address]
      });
      return result;
    } catch (error: any) {
      throw new Error(error.message || 'Failed to sign message');
    }
  }, [walletState.provider, walletState.address]);

  useEffect(() => {
    if (window.ethereum) {
      window.ethereum.on('accountsChanged', (accounts: string[]) => {
        if (accounts.length === 0) {
          disconnect();
        } else {
          setWalletState(prev => ({ ...prev, address: accounts[0] }));
          updateBalance(accounts[0], window.ethereum);
        }
      });

      window.ethereum.on('chainChanged', (chainId: string) => {
        setWalletState(prev => ({ ...prev, chainId }));
      });
    }
  }, [disconnect, updateBalance]);

  return (
    <WalletContext.Provider value={{
      ...walletState,
      connectMetaMask,
      connectWalletConnect,
      disconnect,
      switchNetwork,
      signTransaction,
      signMessage
    }}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within WalletProvider');
  }
  return context;
};