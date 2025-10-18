import React, { useState, useEffect } from 'react';
import { ArrowDownUp, Activity, Clock, DollarSign, TrendingUp, CheckCircle, AlertCircle, LogIn, User, Shield, Globe, Zap, Coins, Gavel, Droplets, Send } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import HeroSection from './HeroSection';
import HEZTokenomics from './hez/HEZTokenomics';
import StakingInterface from './hez/StakingInterface';
import ValidatorDashboard from './hez/ValidatorDashboard';
import GovernanceInterface from './GovernanceInterface';
import { TreasuryOverview } from './treasury/TreasuryOverview';
import { ProposalsListLive } from './governance/ProposalsListLive';
import { BridgeInterface } from './bridge/BridgeInterface';
import DualTokenBridge from './bridge/DualTokenBridge';
import ParachainAuctions from './auctions/ParachainAuctions';
import { TransactionHistory } from './bridge/TransactionHistory';
import { SecurityVerification } from './bridge/SecurityVerification';
import { ChainSelector } from './ChainSelector';
import { MEVProtectionDashboard } from './mev/MEVProtectionDashboard';
import { SandwichDetector } from './mev/SandwichDetector';
import { PrivatePoolManager } from './mev/PrivatePoolManager';
import { MEVRewardsConfig } from './mev/MEVRewardsConfig';
import { ValidatorSelection } from './validators/ValidatorSelection';
import { LiquidityMining } from './liquidity/LiquidityMining';
import { CrossChainMessaging } from './messaging/CrossChainMessaging';
import { SUPPORTED_CHAINS } from '@/services/blockchain/config';
import { useAuth } from '@/contexts/AuthContext';
import { LoginModal } from './auth/LoginModal';
import { UserMenu } from './auth/UserMenu';
import { UserProfileEdit } from './profile/UserProfileEdit';
import ProtectedRoute from './auth/ProtectedRoute';

const AppLayout: React.FC = () => {
  const { user, profile, loading } = useAuth();
  const navigate = useNavigate();
  const [selectedChain, setSelectedChain] = useState(SUPPORTED_CHAINS[0]);
  const [totalVolume, setTotalVolume] = useState(1234567);
  const [showLoginModal, setShowLoginModal] = useState(false);
  const [activeTab, setActiveTab] = useState('bridge');

  const handleChainChange = (chain: any) => {
    setSelectedChain(chain);
  };

  const getStatusBadge = (status: string) => {
    const colors = {
      operational: 'bg-green-500',
      degraded: 'bg-yellow-500',
      down: 'bg-red-500'
    };
    return colors[status as keyof typeof colors] || 'bg-gray-500';
  };

  const getTransactionStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'failed':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      default:
        return <Clock className="h-4 w-4 text-yellow-500" />;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-900 to-black text-white">
      <div className="absolute top-4 right-4 z-50">
        {user ? (
          <UserMenu />
        ) : (
          <Button onClick={() => setShowLoginModal(true)} variant="outline">
            <LogIn className="mr-2 h-4 w-4" />
            Login
          </Button>
        )}
      </div>

      <LoginModal isOpen={showLoginModal} onClose={() => setShowLoginModal(false)} />
      
      <HeroSection />
      
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
        {/* Chain Selector */}
        <div className="flex justify-center mb-8">
          <ChainSelector onChainChange={handleChainChange} />
        </div>

        <div className="text-center mb-12">
          <h2 className="text-4xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent">
            Multi-Chain Bridge & Governance
          </h2>
          <p className="text-gray-400 text-lg">
            Transfer assets across {SUPPORTED_CHAINS.length} blockchain networks
          </p>
          {user && profile && (
            <div className="mt-4 flex items-center justify-center gap-2">
              <span className="text-sm text-gray-400">Welcome back,</span>
              <span className="font-medium">{profile.full_name || profile.username || user.email}</span>
              <Badge variant={profile.role === 'admin' ? 'destructive' : 'default'}>
                {profile.role}
              </Badge>
            </div>
          )}
        </div>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <Card className="bg-gray-800/50 border-gray-700">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-gray-400">
                Total Volume (24h)
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-white">
                ${totalVolume.toLocaleString()}
              </div>
              <p className="text-xs text-green-500 flex items-center mt-1">
                <TrendingUp className="h-3 w-3 mr-1" />
                +12.5%
              </p>
            </CardContent>
          </Card>

          <Card className="bg-gray-800/50 border-gray-700">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-gray-400">
                Active Chains
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-white">
                {SUPPORTED_CHAINS.length}
              </div>
              <p className="text-xs text-gray-500 mt-1">
                Networks connected
              </p>
            </CardContent>
          </Card>

          <Card className="bg-gray-800/50 border-gray-700">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-gray-400">
                Your Reputation
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-white">
                {profile?.reputation_score || 0}
              </div>
              <p className="text-xs text-gray-500 mt-1">
                Governance score
              </p>
            </CardContent>
          </Card>

          <Card className="bg-gray-800/50 border-gray-700">
            <CardHeader className="pb-3">
              <CardTitle className="text-sm font-medium text-gray-400">
                Success Rate
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold text-white">99.8%</div>
              <p className="text-xs text-gray-500 mt-1">
                Last 7 days
              </p>
            </CardContent>
          </Card>
        </div>

        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-6 lg:grid-cols-12 mb-8 bg-gray-800">
            <TabsTrigger value="bridge">Bridge</TabsTrigger>
            <TabsTrigger value="dual-bridge">Dual</TabsTrigger>
            <TabsTrigger value="auctions">Auctions</TabsTrigger>
            <TabsTrigger value="hez">HEZ</TabsTrigger>
            <TabsTrigger value="staking">Staking</TabsTrigger>
            <TabsTrigger value="validators">Validators</TabsTrigger>
            <TabsTrigger value="liquidity">Liquidity</TabsTrigger>
            <TabsTrigger value="messaging">XCM</TabsTrigger>
            <TabsTrigger value="mev">MEV</TabsTrigger>
            <TabsTrigger value="governance">Gov</TabsTrigger>
            <TabsTrigger value="status">Status</TabsTrigger>
            <TabsTrigger value="profile">Profile</TabsTrigger>
          </TabsList>

          <TabsContent value="bridge">
            <div className="space-y-8">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <BridgeInterface />
                <SecurityVerification />
              </div>
              <TransactionHistory />
            </div>
          </TabsContent>
          
          <TabsContent value="dual-bridge">
            <DualTokenBridge />
          </TabsContent>
          
          <TabsContent value="auctions">
            <ParachainAuctions />
          </TabsContent>
          
          <TabsContent value="hez">
            <HEZTokenomics />
          </TabsContent>
          <TabsContent value="staking">
            <div className="space-y-6">
              <Tabs defaultValue="stake" className="w-full">
                <TabsList className="grid w-full grid-cols-2 bg-gray-800">
                  <TabsTrigger value="stake">Stake & Nominate</TabsTrigger>
                  <TabsTrigger value="validator">Validator Dashboard</TabsTrigger>
                </TabsList>
                <TabsContent value="stake">
                  <StakingInterface />
                </TabsContent>
                <TabsContent value="validator">
                  {user ? (
                    <ValidatorDashboard />
                  ) : (
                    <Alert>
                      <Shield className="h-4 w-4" />
                      <AlertDescription>
                        Please login to access validator dashboard
                      </AlertDescription>
                    </Alert>
                  )}
                </TabsContent>
              </Tabs>
            </div>
          </TabsContent>
          
          <TabsContent value="validators">
            <ValidatorSelection />
          </TabsContent>
          
          <TabsContent value="liquidity">
            <LiquidityMining />
          </TabsContent>
          
          <TabsContent value="messaging">
            <CrossChainMessaging />
          </TabsContent>
          
          <TabsContent value="mev">
            <div className="space-y-6">
              <div className="text-center mb-8">
                <h3 className="text-2xl font-bold mb-2 flex items-center justify-center gap-2">
                  <Zap className="w-6 h-6 text-yellow-500" />
                  MEV Protection Suite
                </h3>
                <p className="text-gray-400">
                  Protect your transactions from sandwich attacks, frontrunning, and other MEV exploits
                </p>
              </div>
              
              <Tabs defaultValue="protection" className="w-full">
                <TabsList className="grid w-full grid-cols-4 bg-gray-800">
                  <TabsTrigger value="protection">Protection</TabsTrigger>
                  <TabsTrigger value="detector">Detector</TabsTrigger>
                  <TabsTrigger value="pools">Private Pools</TabsTrigger>
                  <TabsTrigger value="rewards">MEV Rewards</TabsTrigger>
                </TabsList>
                <TabsContent value="protection">
                  <MEVProtectionDashboard />
                </TabsContent>
                <TabsContent value="detector">
                  <SandwichDetector />
                </TabsContent>
                <TabsContent value="pools">
                  <PrivatePoolManager />
                </TabsContent>
                <TabsContent value="rewards">
                  <MEVRewardsConfig />
                </TabsContent>
              </Tabs>
            </div>
          </TabsContent>

          <TabsContent value="governance">
            {user ? (
              <ProtectedRoute requiredPermission="vote">
                <div className="space-y-6">
                  <GovernanceInterface />
                  <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <TreasuryOverview />
                    <ProposalsListLive />
                  </div>
                </div>
              </ProtectedRoute>
            ) : (
              <Alert>
                <User className="h-4 w-4" />
                <AlertDescription>
                  Please login to participate in governance
                </AlertDescription>
              </Alert>
            )}
          </TabsContent>

          <TabsContent value="status">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {SUPPORTED_CHAINS.map((chain) => (
                <Card key={chain.chainId} className="bg-gray-800/50 border-gray-700">
                  <CardHeader>
                    <div className="flex items-center justify-between">
                      <CardTitle className="text-lg">
                        {chain.name}
                      </CardTitle>
                      <Badge className="bg-green-500 text-white">
                        Active
                      </Badge>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <div className="space-y-2">
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-400">Native Token</span>
                        <span>{chain.nativeToken}</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-400">Block Time</span>
                        <span>{chain.blockTime}s</span>
                      </div>
                      <div className="flex justify-between text-sm">
                        <span className="text-gray-400">Status</span>
                        <span className="text-green-400">Connected</span>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          </TabsContent>

          <TabsContent value="profile">
            {user ? (
              <UserProfileEdit />
            ) : (
              <Alert>
                <User className="h-4 w-4" />
                <AlertDescription>
                  Please login to view and edit your profile
                </AlertDescription>
              </Alert>
            )}
          </TabsContent>

          <TabsContent value="history">
            <Card className="bg-gray-800/50 border-gray-700">
              <CardHeader>
                <CardTitle>Transaction History</CardTitle>
              </CardHeader>
              <CardContent>
                {user ? (
                  <div className="text-center py-8 text-gray-400">
                    Your transaction history will appear here
                  </div>
                ) : (
                  <Alert>
                    <Activity className="h-4 w-4" />
                    <AlertDescription>
                      Connect your wallet to view your transaction history
                    </AlertDescription>
                  </Alert>
                )}
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
      
      {/* Floating Admin Button for Admins */}
      {profile?.role === 'admin' && (
        <Button
          onClick={() => navigate('/admin')}
          className="fixed bottom-6 right-6 h-14 w-14 rounded-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 shadow-lg z-50"
          size="icon"
        >
          <Shield className="h-6 w-6" />
        </Button>
      )}
    </div>
  );
};

export default AppLayout;