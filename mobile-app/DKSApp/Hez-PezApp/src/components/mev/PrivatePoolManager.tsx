import React, { useState, useEffect } from 'react';
import { Server, Zap, CheckCircle, XCircle, TrendingUp, Clock } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Switch } from '@/components/ui/switch';
import { Label } from '@/components/ui/label';
import { supabase } from '@/lib/supabase';

interface PrivatePool {
  id: string;
  pool_name: string;
  chain_id: string;
  pool_type: string;
  min_tip: number;
  success_rate: number;
  avg_inclusion_time: number;
  is_active: boolean;
  features: string[];
}

export function PrivatePoolManager() {
  const [pools, setPools] = useState<PrivatePool[]>([]);
  const [selectedPool, setSelectedPool] = useState<string | null>(null);
  const [autoSelect, setAutoSelect] = useState(true);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadPools();
  }, []);

  const loadPools = async () => {
    try {
      const { data } = await supabase
        .from('private_pools')
        .select('*')
        .eq('is_active', true)
        .order('success_rate', { ascending: false });
      
      if (data) {
        setPools(data);
        // Auto-select best pool
        if (data.length > 0 && autoSelect) {
          setSelectedPool(data[0].id);
        }
      }
    } catch (error) {
      console.error('Error loading pools:', error);
    } finally {
      setLoading(false);
    }
  };

  const getPoolTypeColor = (type: string) => {
    const colors: Record<string, string> = {
      flashbots: 'bg-purple-500',
      bloxroute: 'bg-blue-500',
      eden: 'bg-green-500',
      nodereal: 'bg-orange-500'
    };
    return colors[type] || 'bg-gray-500';
  };

  const selectPool = (poolId: string) => {
    setSelectedPool(poolId);
    setAutoSelect(false);
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
      {/* Auto-Select Toggle */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Pool Selection Mode</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex items-center justify-between">
            <div className="space-y-0.5">
              <Label htmlFor="auto-select">Automatic Pool Selection</Label>
              <p className="text-sm text-muted-foreground">
                Automatically choose the best performing pool for each transaction
              </p>
            </div>
            <Switch
              id="auto-select"
              checked={autoSelect}
              onCheckedChange={setAutoSelect}
            />
          </div>
        </CardContent>
      </Card>

      {/* Pool Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {pools.map((pool) => (
          <Card 
            key={pool.id}
            className={`cursor-pointer transition-all ${
              selectedPool === pool.id ? 'ring-2 ring-primary' : 'hover:shadow-lg'
            }`}
            onClick={() => selectPool(pool.id)}
          >
            <CardHeader>
              <div className="flex items-center justify-between">
                <CardTitle className="text-lg flex items-center gap-2">
                  <Server className="w-5 h-5" />
                  {pool.pool_name}
                </CardTitle>
                <Badge className={getPoolTypeColor(pool.pool_type)}>
                  {pool.pool_type}
                </Badge>
              </div>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                {/* Success Rate */}
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-muted-foreground">Success Rate</span>
                    <span className="font-medium">{pool.success_rate}%</span>
                  </div>
                  <Progress value={pool.success_rate} className="h-2" />
                </div>

                {/* Stats */}
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <div className="flex items-center gap-1">
                    <Zap className="w-3 h-3 text-yellow-500" />
                    <span className="text-muted-foreground">Min Tip:</span>
                    <span className="font-medium">{pool.min_tip} ETH</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <Clock className="w-3 h-3 text-blue-500" />
                    <span className="text-muted-foreground">Avg Time:</span>
                    <span className="font-medium">{pool.avg_inclusion_time}s</span>
                  </div>
                </div>

                {/* Features */}
                <div className="flex flex-wrap gap-1">
                  {pool.features.map((feature, idx) => (
                    <Badge key={idx} variant="outline" className="text-xs">
                      {feature}
                    </Badge>
                  ))}
                </div>

                {/* Selection Status */}
                <div className="pt-2 border-t">
                  {selectedPool === pool.id ? (
                    <div className="flex items-center gap-2 text-green-600">
                      <CheckCircle className="w-4 h-4" />
                      <span className="text-sm font-medium">Selected</span>
                    </div>
                  ) : (
                    <Button 
                      size="sm" 
                      variant="outline" 
                      className="w-full"
                      onClick={(e) => {
                        e.stopPropagation();
                        selectPool(pool.id);
                      }}
                    >
                      Select Pool
                    </Button>
                  )}
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Pool Comparison */}
      <Card>
        <CardHeader>
          <CardTitle>Pool Performance Comparison</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {pools.map((pool) => (
              <div key={pool.id} className="flex items-center gap-3">
                <div className="w-32 text-sm font-medium truncate">{pool.pool_name}</div>
                <div className="flex-1">
                  <Progress value={pool.success_rate} className="h-6" />
                </div>
                <div className="text-sm font-medium w-12 text-right">
                  {pool.success_rate}%
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}