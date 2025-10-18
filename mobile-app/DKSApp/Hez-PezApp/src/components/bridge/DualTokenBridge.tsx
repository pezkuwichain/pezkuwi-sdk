import React, { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { ArrowDownUp, Loader2, Shield, Zap } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';

const DualTokenBridge = () => {
  const { toast } = useToast();
  const [bridging, setBridging] = useState(false);
  const [token, setToken] = useState('HEZ');
  const [amount, setAmount] = useState('');
  const [sourceChain, setSourceChain] = useState('polkadot');
  const [targetChain, setTargetChain] = useState('ethereum');

  const chains = [
    { id: 'polkadot', name: 'Polkadot', symbol: 'DOT' },
    { id: 'ethereum', name: 'Ethereum', symbol: 'ETH' },
    { id: 'kusama', name: 'Kusama', symbol: 'KSM' },
    { id: 'moonbeam', name: 'Moonbeam', symbol: 'GLMR' },
    { id: 'acala', name: 'Acala', symbol: 'ACA' }
  ];

  const bridgeStats = {
    HEZ: { locked: '2.5M', daily: '150K', fee: '0.1%' },
    PEZ: { locked: '10M', daily: '500K', fee: '0.2%' }
  };

  const handleBridge = async () => {
    setBridging(true);
    await new Promise(resolve => setTimeout(resolve, 2000));
    toast({
      title: "Bridge Transaction Initiated",
      description: `Bridging ${amount} ${token} from ${sourceChain} to ${targetChain}`
    });
    setBridging(false);
    setAmount('');
  };

  const swapChains = () => {
    setSourceChain(targetChain);
    setTargetChain(sourceChain);
  };

  return (
    <div className="space-y-6">
      <Tabs defaultValue="bridge" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="bridge">Bridge</TabsTrigger>
          <TabsTrigger value="liquidity">Liquidity</TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
        </TabsList>

        <TabsContent value="bridge" className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Cross-Chain Bridge</CardTitle>
              <CardDescription>Transfer HEZ and PEZ tokens between chains</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <Label>Token</Label>
                  <Select value={token} onValueChange={setToken}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="HEZ">HEZ (Native)</SelectItem>
                      <SelectItem value="PEZ">PEZ (Utility)</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div>
                  <Label>Amount</Label>
                  <Input
                    type="number"
                    placeholder="0.00"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                  />
                </div>
              </div>

              <div className="space-y-2">
                <div className="flex items-center gap-2">
                  <Select value={sourceChain} onValueChange={setSourceChain}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {chains.map(chain => (
                        <SelectItem key={chain.id} value={chain.id}>
                          {chain.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <Button variant="ghost" size="icon" onClick={swapChains}>
                    <ArrowDownUp className="h-4 w-4" />
                  </Button>
                  <Select value={targetChain} onValueChange={setTargetChain}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {chains.map(chain => (
                        <SelectItem key={chain.id} value={chain.id}>
                          {chain.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div className="bg-muted p-4 rounded-lg space-y-2">
                <div className="flex justify-between text-sm">
                  <span>Bridge Fee:</span>
                  <span>{bridgeStats[token as keyof typeof bridgeStats].fee}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span>Estimated Time:</span>
                  <span>~5 minutes</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span>Security:</span>
                  <Badge variant="outline" className="gap-1">
                    <Shield className="h-3 w-3" />
                    Multi-sig Protected
                  </Badge>
                </div>
              </div>

              <Button 
                className="w-full" 
                onClick={handleBridge}
                disabled={!amount || bridging}
              >
                {bridging ? (
                  <>
                    <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                    Bridging...
                  </>
                ) : (
                  <>
                    <Zap className="mr-2 h-4 w-4" />
                    Bridge {token}
                  </>
                )}
              </Button>
            </CardContent>
          </Card>

          <div className="grid grid-cols-2 gap-4">
            <Card>
              <CardHeader>
                <CardTitle>HEZ Statistics</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Total Locked:</span>
                  <span className="font-bold">{bridgeStats.HEZ.locked}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">24h Volume:</span>
                  <span>{bridgeStats.HEZ.daily}</span>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>PEZ Statistics</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Total Locked:</span>
                  <span className="font-bold">{bridgeStats.PEZ.locked}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">24h Volume:</span>
                  <span>{bridgeStats.PEZ.daily}</span>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="liquidity">
          <Card>
            <CardHeader>
              <CardTitle>Liquidity Pools</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {['HEZ/DOT', 'PEZ/USDT', 'HEZ/ETH', 'PEZ/HEZ'].map(pair => (
                  <div key={pair} className="flex items-center justify-between p-4 border rounded-lg">
                    <div>
                      <div className="font-semibold">{pair}</div>
                      <div className="text-sm text-muted-foreground">TVL: ${Math.floor(Math.random() * 10000000)}</div>
                    </div>
                    <div className="text-right">
                      <div className="text-green-600">APY: {(Math.random() * 50).toFixed(2)}%</div>
                      <Button size="sm" variant="outline">Add Liquidity</Button>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="history">
          <Card>
            <CardHeader>
              <CardTitle>Bridge History</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                {[1, 2, 3].map(i => (
                  <div key={i} className="flex items-center justify-between p-3 border rounded">
                    <div>
                      <div className="font-medium">1000 HEZ</div>
                      <div className="text-sm text-muted-foreground">Polkadot → Ethereum</div>
                    </div>
                    <Badge variant="outline">Completed</Badge>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default DualTokenBridge;