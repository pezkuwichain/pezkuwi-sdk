import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Input } from '@/components/ui/input';
import { Shield, TrendingUp, Users, Zap, Search, Filter } from 'lucide-react';

interface Validator {
  id: string;
  name: string;
  commission: number;
  apy: number;
  totalStaked: string;
  nominators: number;
  uptime: number;
  era: number;
  identity: boolean;
  slashingHistory: number;
  performance: number;
}

export function ValidatorSelection() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedValidators, setSelectedValidators] = useState<string[]>([]);
  const [filterBy, setFilterBy] = useState('all');
  const [stakeAmount, setStakeAmount] = useState('1000');

  const validators: Validator[] = [
    { id: '1', name: 'Polkadot Foundation', commission: 1, apy: 14.5, totalStaked: '2.5M HEZ', nominators: 1250, uptime: 99.9, era: 365, identity: true, slashingHistory: 0, performance: 98 },
    { id: '2', name: 'Web3 Validator', commission: 3, apy: 13.8, totalStaked: '1.8M HEZ', nominators: 890, uptime: 99.7, era: 300, identity: true, slashingHistory: 0, performance: 96 },
    { id: '3', name: 'Staking Pro', commission: 2, apy: 14.2, totalStaked: '3.1M HEZ', nominators: 1450, uptime: 99.8, era: 400, identity: true, slashingHistory: 0, performance: 97 },
    { id: '4', name: 'Secure Node', commission: 5, apy: 12.5, totalStaked: '900K HEZ', nominators: 450, uptime: 99.5, era: 250, identity: true, slashingHistory: 1, performance: 94 },
    { id: '5', name: 'Validator Hub', commission: 1.5, apy: 14.3, totalStaked: '2.2M HEZ', nominators: 1100, uptime: 99.95, era: 420, identity: true, slashingHistory: 0, performance: 99 },
    { id: '6', name: 'Chain Guardian', commission: 2.5, apy: 13.9, totalStaked: '1.5M HEZ', nominators: 750, uptime: 99.6, era: 280, identity: true, slashingHistory: 0, performance: 95 },
  ];

  const filteredValidators = validators.filter(v => {
    const matchesSearch = v.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesFilter = filterBy === 'all' || 
      (filterBy === 'lowCommission' && v.commission <= 2) ||
      (filterBy === 'highApy' && v.apy >= 14) ||
      (filterBy === 'verified' && v.identity);
    return matchesSearch && matchesFilter;
  });

  const handleSelectValidator = (id: string) => {
    if (selectedValidators.includes(id)) {
      setSelectedValidators(selectedValidators.filter(v => v !== id));
    } else if (selectedValidators.length < 16) {
      setSelectedValidators([...selectedValidators, id]);
    }
  };

  const calculateRewards = () => {
    const avgApy = selectedValidators.reduce((sum, id) => {
      const validator = validators.find(v => v.id === id);
      return sum + (validator?.apy || 0);
    }, 0) / (selectedValidators.length || 1);
    
    return ((parseFloat(stakeAmount) || 0) * avgApy / 100).toFixed(2);
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row gap-4 items-start md:items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold">Validator Selection</h2>
          <p className="text-muted-foreground mt-2">Choose up to 16 validators to nominate your stake</p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => setFilterBy('all')}>All</Button>
          <Button variant="outline" onClick={() => setFilterBy('lowCommission')}>Low Fee</Button>
          <Button variant="outline" onClick={() => setFilterBy('highApy')}>High APY</Button>
          <Button variant="outline" onClick={() => setFilterBy('verified')}>Verified</Button>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-4">
        <Card className="md:col-span-2">
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Available Validators</CardTitle>
              <div className="relative w-64">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search validators..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="pl-10"
                />
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-4">
            {filteredValidators.map((validator) => (
              <div
                key={validator.id}
                className={`p-4 rounded-lg border cursor-pointer transition-all ${
                  selectedValidators.includes(validator.id) 
                    ? 'border-primary bg-primary/5' 
                    : 'hover:border-primary/50'
                }`}
                onClick={() => handleSelectValidator(validator.id)}
              >
                <div className="flex items-start justify-between">
                  <div className="space-y-2 flex-1">
                    <div className="flex items-center gap-2">
                      <h3 className="font-semibold">{validator.name}</h3>
                      {validator.identity && <Badge variant="secondary">Verified</Badge>}
                      {validator.slashingHistory === 0 && <Badge variant="outline">No Slashes</Badge>}
                    </div>
                    
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                      <div>
                        <p className="text-muted-foreground">Commission</p>
                        <p className="font-medium">{validator.commission}%</p>
                      </div>
                      <div>
                        <p className="text-muted-foreground">APY</p>
                        <p className="font-medium text-green-600">{validator.apy}%</p>
                      </div>
                      <div>
                        <p className="text-muted-foreground">Total Staked</p>
                        <p className="font-medium">{validator.totalStaked}</p>
                      </div>
                      <div>
                        <p className="text-muted-foreground">Nominators</p>
                        <p className="font-medium">{validator.nominators}</p>
                      </div>
                    </div>

                    <div className="flex items-center gap-4">
                      <div className="flex items-center gap-1 text-sm">
                        <Zap className="h-3 w-3" />
                        <span>Uptime: {validator.uptime}%</span>
                      </div>
                      <div className="flex items-center gap-1 text-sm">
                        <TrendingUp className="h-3 w-3" />
                        <span>Performance: {validator.performance}%</span>
                      </div>
                    </div>
                  </div>
                  
                  <Button
                    size="sm"
                    variant={selectedValidators.includes(validator.id) ? "default" : "outline"}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleSelectValidator(validator.id);
                    }}
                  >
                    {selectedValidators.includes(validator.id) ? 'Selected' : 'Select'}
                  </Button>
                </div>
              </div>
            ))}
          </CardContent>
        </Card>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Nomination Summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <p className="text-sm text-muted-foreground mb-2">Selected Validators</p>
                <div className="flex items-center justify-between">
                  <span className="text-2xl font-bold">{selectedValidators.length}/16</span>
                  <Progress value={(selectedValidators.length / 16) * 100} className="w-24" />
                </div>
              </div>

              <div>
                <p className="text-sm text-muted-foreground mb-2">Stake Amount (HEZ)</p>
                <Input
                  type="number"
                  value={stakeAmount}
                  onChange={(e) => setStakeAmount(e.target.value)}
                  placeholder="Enter amount"
                />
              </div>

              <div className="p-4 bg-secondary/20 rounded-lg">
                <p className="text-sm text-muted-foreground">Estimated Annual Rewards</p>
                <p className="text-2xl font-bold text-green-600">{calculateRewards()} HEZ</p>
              </div>

              <Button 
                className="w-full" 
                size="lg"
                disabled={selectedValidators.length === 0}
              >
                Nominate {selectedValidators.length} Validators
              </Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Shield className="h-5 w-5" />
                Selection Tips
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-2 text-sm">
              <p>• Diversify across multiple validators</p>
              <p>• Check commission rates and APY</p>
              <p>• Prefer validators with verified identity</p>
              <p>• Review slashing history</p>
              <p>• Consider uptime and performance</p>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}