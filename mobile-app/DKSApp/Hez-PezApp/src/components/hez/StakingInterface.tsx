import React, { useState } from 'react';
import { Shield, Users, Zap, AlertCircle, ChevronRight } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';

interface Validator {
  address: string;
  name: string;
  commission: number;
  totalStake: number;
  nominators: number;
  uptime: number;
  era_points: number;
}

const StakingInterface: React.FC = () => {
  const [stakeAmount, setStakeAmount] = useState('');
  const [selectedValidators, setSelectedValidators] = useState<string[]>([]);
  const [bondedAmount, setBondedAmount] = useState(0);
  const [isStaking, setIsStaking] = useState(false);

  const validators: Validator[] = [
    { address: '1A2B...3C4D', name: 'Kurdistan Node', commission: 5, totalStake: 1500000, nominators: 245, uptime: 99.9, era_points: 980 },
    { address: '2B3C...4D5E', name: 'Erbil Validator', commission: 7, totalStake: 1200000, nominators: 189, uptime: 99.7, era_points: 920 },
    { address: '3C4D...5E6F', name: 'Slemani Stake', commission: 3, totalStake: 980000, nominators: 156, uptime: 99.8, era_points: 950 },
    { address: '4D5E...6F7G', name: 'Duhok Secure', commission: 10, totalStake: 750000, nominators: 98, uptime: 98.5, era_points: 850 },
    { address: '5E6F...7G8H', name: 'Halabja Trust', commission: 8, totalStake: 620000, nominators: 76, uptime: 99.2, era_points: 890 }
  ];

  const handleStake = () => {
    if (!stakeAmount || selectedValidators.length === 0) return;
    setIsStaking(true);
    setTimeout(() => {
      setBondedAmount(bondedAmount + parseFloat(stakeAmount));
      setStakeAmount('');
      setIsStaking(false);
    }, 2000);
  };

  const toggleValidator = (address: string) => {
    setSelectedValidators(prev =>
      prev.includes(address)
        ? prev.filter(v => v !== address)
        : [...prev, address].slice(0, 16) // Max 16 validators like DOT
    );
  };

  return (
    <div className="space-y-6">
      <Card className="bg-gray-950/50 border-gray-800">
        <CardHeader>
          <CardTitle className="text-2xl">HEZ Staking Dashboard</CardTitle>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="nominate" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="nominate">Nominate</TabsTrigger>
              <TabsTrigger value="validate">Validate</TabsTrigger>
              <TabsTrigger value="rewards">Rewards</TabsTrigger>
            </TabsList>

            <TabsContent value="nominate" className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card className="bg-gray-900/50 border-gray-700">
                  <CardContent className="p-4">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-gray-400">Available Balance</span>
                      <Shield className="w-4 h-4 text-blue-400" />
                    </div>
                    <div className="text-2xl font-bold text-white">10,000 HEZ</div>
                  </CardContent>
                </Card>
                <Card className="bg-gray-900/50 border-gray-700">
                  <CardContent className="p-4">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-gray-400">Bonded Amount</span>
                      <Zap className="w-4 h-4 text-yellow-400" />
                    </div>
                    <div className="text-2xl font-bold text-white">{bondedAmount} HEZ</div>
                  </CardContent>
                </Card>
                <Card className="bg-gray-900/50 border-gray-700">
                  <CardContent className="p-4">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-gray-400">Estimated APY</span>
                      <Users className="w-4 h-4 text-green-400" />
                    </div>
                    <div className="text-2xl font-bold text-white">8.5%</div>
                  </CardContent>
                </Card>
              </div>

              <div className="space-y-4">
                <div className="flex gap-4">
                  <Input
                    type="number"
                    placeholder="Amount to stake (HEZ)"
                    value={stakeAmount}
                    onChange={(e) => setStakeAmount(e.target.value)}
                    className="flex-1"
                  />
                  <Button 
                    onClick={handleStake}
                    disabled={!stakeAmount || selectedValidators.length === 0 || isStaking}
                    className="bg-blue-600 hover:bg-blue-700"
                  >
                    {isStaking ? 'Staking...' : 'Stake HEZ'}
                  </Button>
                </div>

                {selectedValidators.length === 0 && (
                  <Alert>
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription>
                      Select at least one validator to nominate (max 16)
                    </AlertDescription>
                  </Alert>
                )}

                <div className="space-y-2">
                  <h3 className="text-lg font-semibold text-white mb-3">Select Validators</h3>
                  {validators.map((validator) => (
                    <div
                      key={validator.address}
                      onClick={() => toggleValidator(validator.address)}
                      className={`p-4 rounded-lg border cursor-pointer transition-all ${
                        selectedValidators.includes(validator.address)
                          ? 'bg-blue-900/30 border-blue-500'
                          : 'bg-gray-900/50 border-gray-700 hover:border-gray-600'
                      }`}
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1">
                          <div className="flex items-center gap-2 mb-1">
                            <span className="font-semibold text-white">{validator.name}</span>
                            <Badge variant="outline" className="text-xs">
                              {validator.commission}% fee
                            </Badge>
                          </div>
                          <div className="text-sm text-gray-400">
                            {validator.address} • {validator.nominators} nominators
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-sm text-gray-400">Total Stake</div>
                          <div className="font-semibold text-white">
                            {(validator.totalStake / 1000000).toFixed(1)}M HEZ
                          </div>
                          <div className="flex items-center gap-2 mt-1">
                            <Badge variant="outline" className="text-xs text-green-400">
                              {validator.uptime}% uptime
                            </Badge>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </TabsContent>

            <TabsContent value="validate" className="space-y-4">
              <Alert>
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>
                  Minimum 1,000 HEZ required to run a validator node. Validators must maintain high uptime or face slashing.
                </AlertDescription>
              </Alert>
              <Button className="w-full" size="lg">
                <ChevronRight className="w-4 h-4 mr-2" />
                Setup Validator Node
              </Button>
            </TabsContent>

            <TabsContent value="rewards" className="space-y-4">
              <Card className="bg-gray-900/50 border-gray-700">
                <CardContent className="p-4">
                  <h3 className="text-lg font-semibold text-white mb-4">Era Rewards History</h3>
                  <div className="space-y-2">
                    {[1, 2, 3, 4, 5].map((era) => (
                      <div key={era} className="flex justify-between items-center p-2 bg-gray-800/50 rounded">
                        <span className="text-gray-400">Era {420 - era}</span>
                        <span className="text-green-400 font-semibold">+{(Math.random() * 10 + 5).toFixed(2)} HEZ</span>
                      </div>
                    ))}
                  </div>
                </CardContent>
              </Card>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
};

export default StakingInterface;