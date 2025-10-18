import React, { useState } from 'react';
import { Server, Activity, Award, AlertTriangle, TrendingUp, Users } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';

interface ValidatorMetrics {
  era: number;
  points: number;
  blocks: number;
  slashes: number;
  rewards: number;
}

const ValidatorDashboard: React.FC = () => {
  const [isActive, setIsActive] = useState(true);
  const [commission, setCommission] = useState(5);

  const currentEra = 420;
  const metrics: ValidatorMetrics[] = [
    { era: 420, points: 980, blocks: 45, slashes: 0, rewards: 125.5 },
    { era: 419, points: 920, blocks: 42, slashes: 0, rewards: 118.2 },
    { era: 418, points: 950, blocks: 44, slashes: 0, rewards: 122.1 },
    { era: 417, points: 890, blocks: 40, slashes: 0, rewards: 114.3 },
    { era: 416, points: 970, blocks: 46, slashes: 0, rewards: 124.6 }
  ];

  const nominators = [
    { address: '1X2Y...3Z4A', stake: 50000, rewards: 4.25 },
    { address: '2Y3Z...4A5B', stake: 35000, rewards: 2.98 },
    { address: '3Z4A...5B6C', stake: 28000, rewards: 2.38 },
    { address: '4A5B...6C7D', stake: 22000, rewards: 1.87 },
    { address: '5B6C...7D8E', stake: 18000, rewards: 1.53 }
  ];

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="bg-gray-950/50 border-gray-800">
          <CardContent className="p-4">
            <div className="flex items-center justify-between mb-2">
              <Server className="w-5 h-5 text-blue-400" />
              <Badge variant={isActive ? "default" : "secondary"}>
                {isActive ? 'Active' : 'Inactive'}
              </Badge>
            </div>
            <div className="text-2xl font-bold text-white">Validator</div>
            <div className="text-sm text-gray-400">Kurdistan Node</div>
          </CardContent>
        </Card>

        <Card className="bg-gray-950/50 border-gray-800">
          <CardContent className="p-4">
            <div className="flex items-center justify-between mb-2">
              <Activity className="w-5 h-5 text-green-400" />
              <span className="text-sm text-gray-400">Era {currentEra}</span>
            </div>
            <div className="text-2xl font-bold text-white">99.9%</div>
            <div className="text-sm text-gray-400">Uptime</div>
          </CardContent>
        </Card>

        <Card className="bg-gray-950/50 border-gray-800">
          <CardContent className="p-4">
            <div className="flex items-center justify-between mb-2">
              <Users className="w-5 h-5 text-purple-400" />
              <span className="text-sm text-gray-400">+12 this era</span>
            </div>
            <div className="text-2xl font-bold text-white">245</div>
            <div className="text-sm text-gray-400">Nominators</div>
          </CardContent>
        </Card>

        <Card className="bg-gray-950/50 border-gray-800">
          <CardContent className="p-4">
            <div className="flex items-center justify-between mb-2">
              <Award className="w-5 h-5 text-yellow-400" />
              <Badge className="bg-yellow-500/20 text-yellow-400">{commission}%</Badge>
            </div>
            <div className="text-2xl font-bold text-white">1.5M HEZ</div>
            <div className="text-sm text-gray-400">Total Stake</div>
          </CardContent>
        </Card>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <Card className="bg-gray-950/50 border-gray-800">
          <CardHeader>
            <CardTitle className="flex items-center justify-between">
              <span>Performance Metrics</span>
              <TrendingUp className="w-5 h-5 text-green-400" />
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-3">
              {metrics.map((metric) => (
                <div key={metric.era} className="flex items-center justify-between p-3 bg-gray-900/50 rounded-lg">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-white font-semibold">Era {metric.era}</span>
                      {metric.era === currentEra && (
                        <Badge variant="outline" className="text-xs text-green-400">Current</Badge>
                      )}
                    </div>
                    <div className="text-sm text-gray-400">
                      {metric.blocks} blocks • {metric.points} points
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-green-400 font-semibold">+{metric.rewards} HEZ</div>
                    {metric.slashes > 0 && (
                      <div className="text-red-400 text-sm">-{metric.slashes} HEZ</div>
                    )}
                  </div>
                </div>
              ))}
            </div>

            <div className="p-3 bg-blue-900/20 rounded-lg border border-blue-500/30">
              <div className="flex items-center justify-between mb-2">
                <span className="text-gray-400">Average Era Points</span>
                <span className="text-white font-bold">942</span>
              </div>
              <Progress value={94.2} className="h-2" />
            </div>
          </CardContent>
        </Card>

        <Card className="bg-gray-950/50 border-gray-800">
          <CardHeader>
            <CardTitle>Top Nominators</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3">
            {nominators.map((nominator, index) => (
              <div key={nominator.address} className="flex items-center justify-between p-3 bg-gray-900/50 rounded-lg">
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 rounded-full bg-gradient-to-r from-blue-500 to-purple-500 flex items-center justify-center text-white font-bold text-sm">
                    {index + 1}
                  </div>
                  <div>
                    <div className="text-white font-medium">{nominator.address}</div>
                    <div className="text-sm text-gray-400">
                      {(nominator.stake / 1000).toFixed(0)}K HEZ staked
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="text-green-400 font-semibold">+{nominator.rewards} HEZ</div>
                  <div className="text-xs text-gray-500">Last era</div>
                </div>
              </div>
            ))}

            <Button className="w-full mt-4" variant="outline">
              View All Nominators
            </Button>
          </CardContent>
        </Card>
      </div>

      <Alert className="bg-yellow-900/20 border-yellow-500/30">
        <AlertTriangle className="h-4 w-4 text-yellow-400" />
        <AlertDescription className="text-gray-300">
          <strong>Validator Responsibilities:</strong> Maintain 98%+ uptime, validate blocks accurately, 
          participate in governance. Failure may result in slashing of stake.
        </AlertDescription>
      </Alert>
    </div>
  );
};

export default ValidatorDashboard;