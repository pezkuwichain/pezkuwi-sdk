import React, { useState } from 'react';
import { Wallet, Chrome, Smartphone, Copy, Check, ExternalLink } from 'lucide-react';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { useWallet } from '@/contexts/WalletContext';
import { formatAddress } from '@/lib/wallet';

interface WalletModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const WalletModal: React.FC<WalletModalProps> = ({ isOpen, onClose }) => {
  const { connectMetaMask, connectWalletConnect, isConnected, address } = useWallet();
  const [copied, setCopied] = useState(false);

  const handleCopyAddress = () => {
    if (address) {
      navigator.clipboard.writeText(address);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleMetaMaskConnect = async () => {
    await connectMetaMask();
    if (isConnected) onClose();
  };

  const handleWalletConnectConnect = async () => {
    await connectWalletConnect();
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <Wallet className="h-5 w-5 text-kesk" />
            Connect Wallet
          </DialogTitle>
          <DialogDescription>
            Connect your wallet to interact with PezkuwiChain governance
          </DialogDescription>
        </DialogHeader>

        {!isConnected ? (
          <Tabs defaultValue="browser" className="w-full">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="browser">Browser Wallet</TabsTrigger>
              <TabsTrigger value="mobile">Mobile Wallet</TabsTrigger>
            </TabsList>
            
            <TabsContent value="browser" className="space-y-4">
              <Button
                onClick={handleMetaMaskConnect}
                className="w-full justify-start bg-kesk hover:bg-kesk/90"
              >
                <Chrome className="mr-2 h-5 w-5" />
                MetaMask
              </Button>
              <div className="text-sm text-muted-foreground">
                Don't have MetaMask?{' '}
                <a
                  href="https://metamask.io/download/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-kesk hover:underline"
                >
                  Download here
                </a>
              </div>
            </TabsContent>
            
            <TabsContent value="mobile" className="space-y-4">
              <Button
                onClick={handleWalletConnectConnect}
                className="w-full justify-start bg-zer hover:bg-zer/90"
              >
                <Smartphone className="mr-2 h-5 w-5" />
                WalletConnect
              </Button>
              <div className="text-sm text-muted-foreground">
                Scan QR code with your mobile wallet to connect
              </div>
            </TabsContent>
          </Tabs>
        ) : (
          <div className="space-y-4">
            <div className="flex items-center justify-between p-3 border rounded-lg">
              <div>
                <div className="text-sm text-muted-foreground">Connected Address</div>
                <div className="font-mono font-medium">{formatAddress(address!)}</div>
              </div>
              <Button
                size="icon"
                variant="ghost"
                onClick={handleCopyAddress}
              >
                {copied ? (
                  <Check className="h-4 w-4 text-kesk" />
                ) : (
                  <Copy className="h-4 w-4" />
                )}
              </Button>
            </div>
            <Button
              variant="outline"
              className="w-full"
              onClick={() => window.open('https://explorer.pezkuwichain.app/address/' + address, '_blank')}
            >
              <ExternalLink className="mr-2 h-4 w-4" />
              View on Explorer
            </Button>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
};