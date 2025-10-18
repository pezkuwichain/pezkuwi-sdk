import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { supabase } from '@/lib/supabase';
import { useToast } from '@/hooks/use-toast';
import { Shield, TrendingUp, AlertTriangle, Activity, DollarSign, Gift, Zap } from 'lucide-react';
import { MEVRewardsConfig } from './MEVRewardsConfig';
import { useAuth } from '@/contexts/AuthContext';

interface ProtectionConfig {
  chain_id: string;
  chain_name: string;
  protection_level: string;
  private_pool_enabled: boolean;
  max_slippage: number;
}

interface AttackStats {
  total_attacks_detected: number;
  total_loss_prevented: number;
  prevention_success_rate: number;
  most_common_attack: string;
}

export function MEVProtectionDashboard() {
  const { user } = useAuth();
  const [configs, setConfigs] = useState<ProtectionConfig[]>([]);
  const [stats, setStats] = useState<AttackStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedChain, setSelectedChain] = useState('1');

  useEffect(() => {
    loadProtectionData();
  }, [user]);

  const loadProtectionData = async () => {
    try {
      // Load protection configs
      const { data: configData } = await supabase
        .from('mev_protection_configs')
        .select('*');
      
      if (configData) setConfigs(configData);

      // Load attack statistics if user is logged in
      if (user) {
        const { data } = await supabase.functions.invoke('mev-protection', {
          body: { action: 'get_attack_history', userId: user.id }
        });
        
        if (data?.statistics) setStats(data.statistics);
      }
    } catch (error) {
      console.error('Error loading protection data:', error);
    } finally {
      setLoading(false);
    }
  };

  const getProtectionColor = (level: string) => {
    switch (level) {
      case 'high': return 'text-green-500';
      case 'medium': return 'text-yellow-500';
      case 'low': return 'text-red-500';
      default: return 'text-gray-500';
    }
  };

  const getProtectionIcon = (level: string) => {
    switch (level) {
      case 'high': return <Shield className="w-5 h-5" />;
      case 'medium': return <Shield className="w-5 h-5" />;
      case 'low': return <AlertTriangle className="w-5 h-5" />;
      default: return <Activity className="w-5 h-5" />;
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Protection Status Alert */}
      <Alert className="border-green-500 bg-green-50 dark:bg-green-950">
        <Shield className="h-4 w-4 text-green-600" />
        <AlertDescription className="text-green-800 dark:text-green-200">
          MEV Protection is active. Your transactions are being monitored and protected from sandwich attacks and frontrunning.
        </AlertDescription>
      </Alert>

      {/* Statistics Cards */}
      {stats && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Attacks Detected</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.total_attacks_detected}</div>
              <p className="text-xs text-muted-foreground mt-1">Total MEV attacks identified</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Loss Prevented</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">${stats.total_loss_prevented.toFixed(2)}</div>
              <p className="text-xs text-muted-foreground mt-1">Saved from MEV extraction</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Success Rate</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stats.prevention_success_rate.toFixed(1)}%</div>
              <Progress value={stats.prevention_success_rate} className="mt-2" />
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-sm font-medium">Most Common</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold capitalize">{stats.most_common_attack}</div>
              <p className="text-xs text-muted-foreground mt-1">Attack type</p>
            </CardContent>
          </Card>
        </div>
      )}

      {/* Chain Protection Status */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Zap className="w-5 h-5" />
            Protection Status by Chain
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {configs.map((config) => (
              <div
                key={config.chain_id}
                className="border rounded-lg p-4 hover:shadow-md transition-shadow cursor-pointer"
                onClick={() => setSelectedChain(config.chain_id)}
              >
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-semibold">{config.chain_name}</h3>
                  <div className={`flex items-center gap-1 ${getProtectionColor(config.protection_level)}`}>
                    {getProtectionIcon(config.protection_level)}
                    <span className="text-sm capitalize">{config.protection_level}</span>
                  </div>
                </div>
                
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">Private Pool</span>
                    <Badge variant={config.private_pool_enabled ? "success" : "secondary"}>
                      {config.private_pool_enabled ? 'Enabled' : 'Disabled'}
                    </Badge>
                  </div>
                  
                  <div className="flex items-center justify-between text-sm">
                    <span className="text-muted-foreground">Max Slippage</span>
                    <span className="font-medium">{config.max_slippage}%</span>
                  </div>
                </div>

                <Button 
                  className="w-full mt-3" 
                  size="sm"
                  variant={selectedChain === config.chain_id ? "default" : "outline"}
                >
                  {selectedChain === config.chain_id ? 'Active' : 'Select'}
                </Button>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Protection Tips */}
      <Card>
        <CardHeader>
          <CardTitle>Protection Best Practices</CardTitle>
        </CardHeader>
        <CardContent>
          <ul className="space-y-2">
            <li className="flex items-start gap-2">
              <Shield className="w-4 h-4 text-green-500 mt-0.5" />
              <span className="text-sm">Always use private pools for high-value transactions</span>
            </li>
            <li className="flex items-start gap-2">
              <Shield className="w-4 h-4 text-green-500 mt-0.5" />
              <span className="text-sm">Enable commit-reveal for sensitive operations</span>
            </li>
            <li className="flex items-start gap-2">
              <Shield className="w-4 h-4 text-green-500 mt-0.5" />
              <span className="text-sm">Monitor gas prices and use appropriate tips</span>
            </li>
            <li className="flex items-start gap-2">
              <Shield className="w-4 h-4 text-green-500 mt-0.5" />
              <span className="text-sm">Avoid round number amounts that attract bots</span>
            </li>
          </ul>
        </CardContent>
      </Card>
    </div>
  );
}