import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Droplets, TrendingUp, Clock, Coins, ArrowUpDown, Info } from 'lucide-react';

interface Pool {
  id: string;
  pair: string;
  tvl: string;
  apr: number;
  rewards: string;
  volume24h: string;
  myLiquidity: string;
  myRewards: string;
}

export function LiquidityMining() {
  const [selectedPool, setSelectedPool] = useState<string | null>(null);
  const [depositAmount, setDepositAmount] = useState('');
  const [withdrawAmount, setWithdrawAmount] = useState('');

  const pools: Pool[] = [
    { id: '1', pair: 'HEZ/USDT', tvl: '$5.2M', apr: 45.2, rewards: '1000 HEZ/day', volume24h: '$890K', myLiquidity: '$2,500', myRewards: '125 HEZ' },
    { id: '2', pair: 'HEZ/PEZ', tvl: '$3.8M', apr: 62.5, rewards: '1500 HEZ/day', volume24h: '$650K', myLiquidity: '$0', myRewards: '0 HEZ' },
    { id: '3', pair: 'PEZ/USDT', tvl: '$2.1M', apr: 38.7, rewards: '800 PEZ/day', volume24h: '$420K', myLiquidity: '$1,200', myRewards: '85 PEZ' },
    { id: '4', pair: 'HEZ/DOT', tvl: '$4.5M', apr: 52.3, rewards: '1200 HEZ/day', volume24h: '$750K', myLiquidity: '$3,800', myRewards: '210 HEZ' },
    { id: '5', pair: 'HEZ/ETH', tvl: '$6.7M', apr: 41.8, rewards: '900 HEZ/day', volume24h: '$1.2M', myLiquidity: '$0', myRewards: '0 HEZ' },
    { id: '6', pair: 'PEZ/DOT', tvl: '$1.5M', apr: 55.6, rewards: '600 PEZ/day', volume24h: '$280K', myLiquidity: '$800', myRewards: '45 PEZ' },
  ];

  const totalLiquidity = pools.reduce((sum, pool) => {
    const value = parseFloat(pool.myLiquidity.replace(/[$,]/g, '')) || 0;
    return sum + value;
  }, 0);

  const handleDeposit = () => {
    console.log('Depositing:', depositAmount);
    setDepositAmount('');
  };

  const handleWithdraw = () => {
    console.log('Withdrawing:', withdrawAmount);
    setWithdrawAmount('');
  };

  const handleClaimRewards = (poolId: string) => {
    console.log('Claiming rewards for pool:', poolId);
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row gap-4 items-start md:items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold">Liquidity Mining</h2>
          <p className="text-muted-foreground mt-2">Provide liquidity and earn rewards</p>
        </div>
        <div className="flex gap-4">
          <Card className="px-4 py-2">
            <p className="text-sm text-muted-foreground">Total Value Locked</p>
            <p className="text-xl font-bold">$28.3M</p>
          </Card>
          <Card className="px-4 py-2">
            <p className="text-sm text-muted-foreground">My Total Liquidity</p>
            <p className="text-xl font-bold">${totalLiquidity.toLocaleString()}</p>
          </Card>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-6">
        <div className="md:col-span-2 space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Active Pools</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {pools.map((pool) => (
                <div
                  key={pool.id}
                  className={`p-4 rounded-lg border cursor-pointer transition-all ${
                    selectedPool === pool.id ? 'border-primary bg-primary/5' : 'hover:border-primary/50'
                  }`}
                  onClick={() => setSelectedPool(pool.id)}
                >
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="flex -space-x-2">
                        <div className="w-8 h-8 rounded-full bg-gradient-to-r from-blue-500 to-purple-500" />
                        <div className="w-8 h-8 rounded-full bg-gradient-to-r from-green-500 to-teal-500" />
                      </div>
                      <div>
                        <h3 className="font-semibold text-lg">{pool.pair}</h3>
                        <Badge variant="secondary" className="mt-1">APR {pool.apr}%</Badge>
                      </div>
                    </div>
                    {parseFloat(pool.myLiquidity.replace(/[$,]/g, '')) > 0 && (
                      <Badge variant="default">Staking</Badge>
                    )}
                  </div>

                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                      <p className="text-muted-foreground">TVL</p>
                      <p className="font-medium">{pool.tvl}</p>
                    </div>
                    <div>
                      <p className="text-muted-foreground">24h Volume</p>
                      <p className="font-medium">{pool.volume24h}</p>
                    </div>
                    <div>
                      <p className="text-muted-foreground">Rewards</p>
                      <p className="font-medium text-green-600">{pool.rewards}</p>
                    </div>
                    <div>
                      <p className="text-muted-foreground">My Liquidity</p>
                      <p className="font-medium">{pool.myLiquidity}</p>
                    </div>
                  </div>

                  {parseFloat(pool.myLiquidity.replace(/[$,]/g, '')) > 0 && (
                    <div className="mt-3 p-3 bg-secondary/20 rounded-lg flex items-center justify-between">
                      <div>
                        <p className="text-sm text-muted-foreground">Unclaimed Rewards</p>
                        <p className="font-semibold">{pool.myRewards}</p>
                      </div>
                      <Button 
                        size="sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleClaimRewards(pool.id);
                        }}
                      >
                        Claim
                      </Button>
                    </div>
                  )}
                </div>
              ))}
            </CardContent>
          </Card>
        </div>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Manage Liquidity</CardTitle>
            </CardHeader>
            <CardContent>
              <Tabs defaultValue="deposit">
                <TabsList className="grid w-full grid-cols-2">
                  <TabsTrigger value="deposit">Deposit</TabsTrigger>
                  <TabsTrigger value="withdraw">Withdraw</TabsTrigger>
                </TabsList>
                
                <TabsContent value="deposit" className="space-y-4">
                  <div>
                    <p className="text-sm text-muted-foreground mb-2">Amount to Deposit</p>
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={depositAmount}
                      onChange={(e) => setDepositAmount(e.target.value)}
                    />
                  </div>
                  <Button className="w-full" onClick={handleDeposit} disabled={!selectedPool}>
                    Add Liquidity
                  </Button>
                </TabsContent>
                
                <TabsContent value="withdraw" className="space-y-4">
                  <div>
                    <p className="text-sm text-muted-foreground mb-2">Amount to Withdraw</p>
                    <Input
                      type="number"
                      placeholder="0.00"
                      value={withdrawAmount}
                      onChange={(e) => setWithdrawAmount(e.target.value)}
                    />
                  </div>
                  <Button className="w-full" variant="destructive" onClick={handleWithdraw} disabled={!selectedPool}>
                    Remove Liquidity
                  </Button>
                </TabsContent>
              </Tabs>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5" />
                Yield Farming Stats
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Total Earned</span>
                <span className="font-semibold">465 HEZ</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Daily Earnings</span>
                <span className="font-semibold text-green-600">~12.5 HEZ</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Average APR</span>
                <span className="font-semibold">48.3%</span>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Info className="h-5 w-5" />
                Tips
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              <p>• Higher APR pools may have more risk</p>
              <p>• Consider impermanent loss</p>
              <p>• Claim rewards regularly</p>
              <p>• Monitor pool TVL changes</p>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}