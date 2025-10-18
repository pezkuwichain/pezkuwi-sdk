import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import { Slider } from '@/components/ui/slider';
import { Label } from '@/components/ui/label';
import { supabase } from '@/lib/supabase';
import { useToast } from '@/hooks/use-toast';
import { Shield, TrendingUp, Users, Settings } from 'lucide-react';

export function MEVRewardsConfig() {
  const { toast } = useToast();
  const [config, setConfig] = useState({
    optIn: false,
    sharePercentage: 50,
    minThreshold: 0.001,
    autoCompound: false
  });
  const [loading, setLoading] = useState(false);

  const updateConfig = async () => {
    setLoading(true);
    try {
      const { data, error } = await supabase.functions.invoke('mev-rewards', {
        body: {
          action: 'update-config',
          data: {
            userId: 'user-1',
            optIn: config.optIn,
            sharePercentage: config.sharePercentage,
            minThreshold: config.minThreshold,
            autoCompound: config.autoCompound
          }
        }
      });

      if (!error && data?.success) {
        toast({
          title: "Configuration Updated",
          description: "Your MEV rewards settings have been saved"
        });
      }
    } catch (err) {
      toast({
        title: "Error",
        description: "Failed to update configuration",
        variant: "destructive"
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Settings className="h-5 w-5" />
          MEV Rewards Configuration
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <Label>Opt-in to MEV Rewards</Label>
            <p className="text-sm text-muted-foreground">Share MEV profits with validators</p>
          </div>
          <Switch
            checked={config.optIn}
            onCheckedChange={(checked) => setConfig({...config, optIn: checked})}
          />
        </div>

        <div className="space-y-2">
          <Label>Profit Share: {config.sharePercentage}%</Label>
          <Slider
            value={[config.sharePercentage]}
            onValueChange={([value]) => setConfig({...config, sharePercentage: value})}
            min={0}
            max={100}
            step={5}
            disabled={!config.optIn}
          />
          <div className="flex justify-between text-xs text-muted-foreground">
            <span>Keep 100%</span>
            <span>50/50 Split</span>
            <span>Give 100%</span>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <div>
            <Label>Auto-Compound Rewards</Label>
            <p className="text-sm text-muted-foreground">Automatically reinvest earnings</p>
          </div>
          <Switch
            checked={config.autoCompound}
            onCheckedChange={(checked) => setConfig({...config, autoCompound: checked})}
            disabled={!config.optIn}
          />
        </div>

        <Button 
          onClick={updateConfig} 
          disabled={loading || !config.optIn}
          className="w-full"
        >
          Save Configuration
        </Button>
      </CardContent>
    </Card>
  );
}