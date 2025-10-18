import React, { useState } from 'react';
import { AlertTriangle, Shield, Search, TrendingDown, TrendingUp } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { supabase } from '@/lib/supabase';
import { useAuth } from '@/contexts/AuthContext';

interface DetectionResult {
  protection_needed: boolean;
  risk_level: string;
  attacks_detected: {
    sandwich: { detected: boolean; confidence: number; details: any };
    frontrun: { detected: boolean; confidence: number; details: any };
  };
  recommended_protection: any;
  protection_strategies: string[];
}

export function SandwichDetector() {
  const { user } = useAuth();
  const [txHash, setTxHash] = useState('');
  const [analyzing, setAnalyzing] = useState(false);
  const [result, setResult] = useState<DetectionResult | null>(null);
  const [recentAttacks, setRecentAttacks] = useState<any[]>([]);

  const analyzeTransaction = async () => {
    if (!txHash) return;
    
    setAnalyzing(true);
    try {
      // Simulate transaction data for analysis
      const mockTransaction = {
        hash: txHash,
        value: Math.random() * 50000,
        gasPrice: Math.random() * 200,
        data: '0x' + Math.random().toString(16).substr(2, 8),
        to: '0x' + Math.random().toString(16).substr(2, 40),
        nonce: Math.floor(Math.random() * 10)
      };

      const { data } = await supabase.functions.invoke('mev-protection', {
        body: {
          action: 'analyze_transaction',
          transaction: mockTransaction,
          chainId: '1',
          userId: user?.id
        }
      });

      if (data) {
        setResult(data);
        
        // Load recent attacks if detected
        if (data.protection_needed && user) {
          const { data: attacks } = await supabase
            .from('mev_attacks_detected')
            .select('*')
            .eq('user_id', user.id)
            .order('detected_at', { ascending: false })
            .limit(5);
          
          if (attacks) setRecentAttacks(attacks);
        }
      }
    } catch (error) {
      console.error('Error analyzing transaction:', error);
    } finally {
      setAnalyzing(false);
    }
  };

  const getRiskColor = (level: string) => {
    switch (level) {
      case 'high': return 'destructive';
      case 'medium': return 'warning';
      case 'low': return 'success';
      default: return 'secondary';
    }
  };

  return (
    <div className="space-y-6">
      {/* Analysis Input */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Search className="w-5 h-5" />
            Transaction Analysis
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex gap-2">
            <Input
              placeholder="Enter transaction hash or paste transaction data..."
              value={txHash}
              onChange={(e) => setTxHash(e.target.value)}
              className="flex-1"
            />
            <Button onClick={analyzeTransaction} disabled={analyzing}>
              {analyzing ? 'Analyzing...' : 'Analyze'}
            </Button>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            Analyze any transaction for potential MEV attacks before submission
          </p>
        </CardContent>
      </Card>

      {/* Detection Results */}
      {result && (
        <>
          <Alert className={result.protection_needed ? 'border-red-500' : 'border-green-500'}>
            <AlertTriangle className="h-4 w-4" />
            <AlertTitle>
              {result.protection_needed ? 'MEV Attack Detected!' : 'Transaction Appears Safe'}
            </AlertTitle>
            <AlertDescription>
              {result.protection_needed 
                ? 'Your transaction is vulnerable to MEV attacks. Enable protection before submitting.'
                : 'No immediate MEV threats detected, but always use caution with high-value transactions.'}
            </AlertDescription>
          </Alert>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Sandwich Attack Detection */}
            <Card className={result.attacks_detected.sandwich.detected ? 'border-red-500' : ''}>
              <CardHeader>
                <CardTitle className="text-lg flex items-center justify-between">
                  <span className="flex items-center gap-2">
                    <TrendingDown className="w-4 h-4" />
                    Sandwich Attack
                  </span>
                  <Badge variant={result.attacks_detected.sandwich.detected ? 'destructive' : 'success'}>
                    {result.attacks_detected.sandwich.detected ? 'Detected' : 'Clear'}
                  </Badge>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Confidence</span>
                    <span className="font-medium">
                      {result.attacks_detected.sandwich.confidence.toFixed(1)}%
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span>Risk Level</span>
                    <Badge variant={getRiskColor(result.attacks_detected.sandwich.details?.risk_level)}>
                      {result.attacks_detected.sandwich.details?.risk_level || 'Low'}
                    </Badge>
                  </div>
                  <div className="text-sm text-muted-foreground mt-2">
                    {result.attacks_detected.sandwich.details?.recommended_action || 'Standard submission'}
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Frontrunning Detection */}
            <Card className={result.attacks_detected.frontrun.detected ? 'border-orange-500' : ''}>
              <CardHeader>
                <CardTitle className="text-lg flex items-center justify-between">
                  <span className="flex items-center gap-2">
                    <TrendingUp className="w-4 h-4" />
                    Frontrunning
                  </span>
                  <Badge variant={result.attacks_detected.frontrun.detected ? 'warning' : 'success'}>
                    {result.attacks_detected.frontrun.detected ? 'Detected' : 'Clear'}
                  </Badge>
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Confidence</span>
                    <span className="font-medium">
                      {result.attacks_detected.frontrun.confidence.toFixed(1)}%
                    </span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span>Vulnerability</span>
                    <Badge variant={getRiskColor(result.attacks_detected.frontrun.details?.vulnerability)}>
                      {result.attacks_detected.frontrun.details?.vulnerability || 'Low'}
                    </Badge>
                  </div>
                  <div className="text-sm text-muted-foreground mt-2">
                    {result.attacks_detected.frontrun.details?.suggested_protection || 'None needed'}
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Protection Recommendations */}
          {result.protection_needed && (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Shield className="w-5 h-5 text-green-500" />
                  Recommended Protection
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {result.recommended_protection.pool && (
                    <div className="p-3 bg-green-50 dark:bg-green-950 rounded-lg">
                      <div className="font-medium">Use {result.recommended_protection.pool.pool_name}</div>
                      <div className="text-sm text-muted-foreground">
                        Success Rate: {result.recommended_protection.pool.success_rate}%
                      </div>
                    </div>
                  )}
                  
                  <div className="space-y-2">
                    <div className="text-sm font-medium">Protection Strategies:</div>
                    {result.protection_strategies.map((strategy, idx) => (
                      <div key={idx} className="flex items-center gap-2 text-sm">
                        <Shield className="w-3 h-3 text-green-500" />
                        <span>{strategy}</span>
                      </div>
                    ))}
                  </div>

                  <div className="flex justify-between items-center pt-2 border-t">
                    <span className="text-sm">Recommended Tip</span>
                    <span className="font-medium">
                      {result.recommended_protection.optimal_tip} ETH
                    </span>
                  </div>
                </div>

                <Button className="w-full mt-4" size="lg">
                  Enable Protection & Submit
                </Button>
              </CardContent>
            </Card>
          )}
        </>
      )}

      {/* Recent Attacks */}
      {recentAttacks.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>Recent Attack History</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {recentAttacks.map((attack) => (
                <div key={attack.id} className="flex items-center justify-between p-2 border rounded">
                  <div>
                    <Badge variant={getRiskColor('high')} className="mb-1">
                      {attack.attack_type}
                    </Badge>
                    <div className="text-sm text-muted-foreground">
                      {new Date(attack.detected_at).toLocaleString()}
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm font-medium">
                      ${attack.potential_loss?.toFixed(2)} saved
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {attack.prevented ? 'Prevented' : 'Not prevented'}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}