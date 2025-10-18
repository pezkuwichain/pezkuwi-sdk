import React, { useState, useEffect } from 'react';
import { Shield, Users, Lock, TrendingUp, Coins, Award } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';

const HEZTokenomics: React.FC = () => {
  const [inflationRate, setInflationRate] = useState(10);
  const [stakedPercentage, setStakedPercentage] = useState(65);
  const [validatorRewards, setValidatorRewards] = useState(0);
  const [nominatorRewards, setNominatorRewards] = useState(0);

  // Calculate rewards based on staking percentage (DOT model)
  useEffect(() => {
    const idealStake = 75;
    const actualInflation = stakedPercentage <= idealStake 
      ? inflationRate * (stakedPercentage / idealStake)
      : inflationRate * Math.pow(2, (idealStake - stakedPercentage) / 17);
    
    const totalRewards = actualInflation;
    setValidatorRewards(totalRewards * 0.2); // 20% commission
    setNominatorRewards(totalRewards * 0.8); // 80% to nominators
  }, [inflationRate, stakedPercentage]);

  const distribution = [
    { name: 'Staking Rewards', percentage: 60, color: 'from-blue-500 to-blue-600' },
    { name: 'Treasury', percentage: 15, color: 'from-purple-500 to-purple-600' },
    { name: 'Parachain Slot Auction', percentage: 15, color: 'from-cyan-500 to-cyan-600' },
    { name: 'Development Fund', percentage: 10, color: 'from-teal-500 to-teal-600' }
  ];

  return (
    <div className="space-y-6">
      <div className="text-center mb-8">
        <h2 className="text-4xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
          HEZ Token Economics
        </h2>
        <p className="text-gray-400 text-lg max-w-3xl mx-auto">
          Native token with DOT-inspired NPoS consensus, inflation model, and governance
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Staking Model */}
        <Card className="bg-gray-950/50 border-gray-800">
          <CardHeader>
            <CardTitle className="flex items-center">
              <Shield className="w-5 h-5 mr-2 text-blue-400" />
              NPoS Staking Model
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <span className="text-gray-400">Target Staking Rate</span>
                <Badge variant="outline" className="text-blue-400">75%</Badge>
              </div>
              <div className="flex justify-between items-center">
                <span className="text-gray-400">Current Staked</span>
                <span className="text-white font-semibold">{stakedPercentage}%</span>
              </div>
              <Progress value={stakedPercentage} className="h-2" />
            </div>

            <div className="grid grid-cols-2 gap-4 mt-6">
              <div className="p-3 bg-blue-900/20 rounded-lg border border-blue-500/30">
                <div className="flex items-center mb-2">
                  <Award className="w-4 h-4 text-blue-400 mr-2" />
                  <span className="text-sm text-gray-400">Validator APY</span>
                </div>
                <div className="text-xl font-bold text-white">
                  {validatorRewards.toFixed(2)}%
                </div>
              </div>
              <div className="p-3 bg-purple-900/20 rounded-lg border border-purple-500/30">
                <div className="flex items-center mb-2">
                  <Users className="w-4 h-4 text-purple-400 mr-2" />
                  <span className="text-sm text-gray-400">Nominator APY</span>
                </div>
                <div className="text-xl font-bold text-white">
                  {nominatorRewards.toFixed(2)}%
                </div>
              </div>
            </div>

            <div className="mt-4 p-3 bg-gray-900/50 rounded-lg">
              <div className="text-sm text-gray-400 mb-2">Staking Simulation</div>
              <input
                type="range"
                min="0"
                max="100"
                value={stakedPercentage}
                onChange={(e) => setStakedPercentage(parseInt(e.target.value))}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-gray-500 mt-1">
                <span>0%</span>
                <span>25%</span>
                <span>50%</span>
                <span>75%</span>
                <span>100%</span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Inflation & Distribution */}
        <Card className="bg-gray-950/50 border-gray-800">
          <CardHeader>
            <CardTitle className="flex items-center">
              <TrendingUp className="w-5 h-5 mr-2 text-purple-400" />
              Inflation & Distribution
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="p-4 bg-gradient-to-r from-blue-900/20 to-purple-900/20 rounded-lg border border-blue-500/30">
              <div className="flex justify-between items-center mb-2">
                <span className="text-gray-400">Annual Inflation</span>
                <span className="text-2xl font-bold text-white">{inflationRate}%</span>
              </div>
              <div className="text-sm text-gray-400">
                Dynamic adjustment based on staking ratio
              </div>
            </div>

            <div className="space-y-3">
              {distribution.map((item) => (
                <div key={item.name} className="flex items-center justify-between p-3 bg-gray-900/50 rounded-lg">
                  <div className="flex items-center">
                    <div className={`w-3 h-3 rounded-full bg-gradient-to-r ${item.color} mr-3`}></div>
                    <span className="text-gray-300">{item.name}</span>
                  </div>
                  <span className="text-white font-semibold">{item.percentage}%</span>
                </div>
              ))}
            </div>

            <div className="mt-4 p-3 bg-yellow-900/20 rounded-lg border border-yellow-500/30">
              <div className="flex items-center mb-2">
                <Lock className="w-4 h-4 text-yellow-400 mr-2" />
                <span className="text-sm text-gray-300">Slashing Conditions</span>
              </div>
              <ul className="text-xs text-gray-400 space-y-1">
                <li>• Offline: 0.1% per session</li>
                <li>• Equivocation: 10% stake</li>
                <li>• Governance abuse: 5% stake</li>
              </ul>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default HEZTokenomics;