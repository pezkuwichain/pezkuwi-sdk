import React, { useState, useEffect } from 'react';
import { ArrowDownUp, Loader2, Info, Clock, DollarSign } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { BridgeService } from '@/services/bridge/bridgeService';
import { BridgeQuote } from '@/services/bridge/types';
import { SUPPORTED_CHAINS } from '@/services/blockchain/config';
import { getSupportedDestinations, SUPPORTED_TOKENS } from '@/services/bridge/config';
import { useToast } from '@/hooks/use-toast';

export function BridgeInterface() {
  const [fromChain, setFromChain] = useState('ethereum');
  const [toChain, setToChain] = useState('polygon');
  const [token, setToken] = useState('USDC');
  const [amount, setAmount] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [quote, setQuote] = useState<BridgeQuote | null>(null);
  const [loading, setLoading] = useState(false);
  const [quoteLoading, setQuoteLoading] = useState(false);
  const [availableDestinations, setAvailableDestinations] = useState<string[]>([]);
  const [availableTokens, setAvailableTokens] = useState<string[]>([]);
  const { toast } = useToast();

  useEffect(() => {
    updateAvailableDestinations();
    updateAvailableTokens();
  }, [fromChain]);

  useEffect(() => {
    if (amount && parseFloat(amount) > 0) {
      fetchQuote();
    } else {
      setQuote(null);
    }
  }, [fromChain, toChain, token, amount]);

  const updateAvailableDestinations = () => {
    const destinations = getSupportedDestinations(fromChain);
    setAvailableDestinations(destinations);
    
    if (!destinations.includes(toChain) && destinations.length > 0) {
      setToChain(destinations[0]);
    }
  };

  const updateAvailableTokens = () => {
    const tokens = SUPPORTED_TOKENS
      .filter(t => t.chains.includes(fromChain))
      .map(t => t.symbol);
    setAvailableTokens(tokens);
    
    if (!tokens.includes(token) && tokens.length > 0) {
      setToken(tokens[0]);
    }
  };

  const fetchQuote = async () => {
    if (!amount || parseFloat(amount) <= 0) return;

    setQuoteLoading(true);
    try {
      const quoteData = await BridgeService.getQuote(
        fromChain,
        toChain,
        token,
        parseFloat(amount)
      );
      setQuote(quoteData);
    } catch (error) {
      console.error('Error fetching quote:', error);
    } finally {
      setQuoteLoading(false);
    }
  };

  const handleSwapChains = () => {
    const temp = fromChain;
    setFromChain(toChain);
    setToChain(temp);
  };

  const handleBridge = async () => {
    if (!quote || !toAddress) {
      toast({
        title: 'Invalid Input',
        description: 'Please fill in all required fields',
        variant: 'destructive',
      });
      return;
    }

    const validation = BridgeService.validateBridgeAmount(
      fromChain,
      toChain,
      parseFloat(amount)
    );

    if (!validation.valid) {
      toast({
        title: 'Invalid Amount',
        description: validation.error,
        variant: 'destructive',
      });
      return;
    }

    setLoading(true);
    try {
      await BridgeService.executeBridge(quote, toAddress);
      
      toast({
        title: 'Bridge Initiated',
        description: `Transferring ${amount} ${token} from ${fromChain} to ${toChain}`,
      });

      // Reset form
      setAmount('');
      setToAddress('');
      setQuote(null);
    } catch (error) {
      toast({
        title: 'Bridge Failed',
        description: 'Failed to initiate bridge transaction',
        variant: 'destructive',
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Cross-Chain Bridge</CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-4">
          {/* From Chain */}
          <div className="space-y-2">
            <Label>From Chain</Label>
            <Select value={fromChain} onValueChange={setFromChain}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {SUPPORTED_CHAINS.map(chain => (
                  <SelectItem key={chain.chainId} value={chain.chainId}>
                    {chain.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Swap Button */}
          <div className="flex justify-center">
            <Button
              variant="ghost"
              size="icon"
              onClick={handleSwapChains}
              className="rounded-full"
            >
              <ArrowDownUp className="h-4 w-4" />
            </Button>
          </div>

          {/* To Chain */}
          <div className="space-y-2">
            <Label>To Chain</Label>
            <Select value={toChain} onValueChange={setToChain}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {availableDestinations.map(chainId => {
                  const chain = SUPPORTED_CHAINS.find(c => c.chainId === chainId);
                  return chain ? (
                    <SelectItem key={chain.chainId} value={chain.chainId}>
                      {chain.name}
                    </SelectItem>
                  ) : null;
                })}
              </SelectContent>
            </Select>
          </div>

          {/* Token Selection */}
          <div className="space-y-2">
            <Label>Token</Label>
            <Select value={token} onValueChange={setToken}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {availableTokens.map(tokenSymbol => (
                  <SelectItem key={tokenSymbol} value={tokenSymbol}>
                    {tokenSymbol}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* Amount */}
          <div className="space-y-2">
            <Label>Amount</Label>
            <Input
              type="number"
              placeholder="0.00"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              min="0"
              step="0.01"
            />
          </div>

          {/* Destination Address */}
          <div className="space-y-2">
            <Label>Destination Address</Label>
            <Input
              placeholder="0x..."
              value={toAddress}
              onChange={(e) => setToAddress(e.target.value)}
            />
          </div>

          {/* Quote Display */}
          {quote && !quoteLoading && (
            <Alert>
              <Info className="h-4 w-4" />
              <AlertDescription>
                <div className="space-y-2 mt-2">
                  <div className="flex justify-between">
                    <span>You will receive:</span>
                    <span className="font-semibold">
                      {quote.toAmount.toFixed(4)} {quote.toToken}
                    </span>
                  </div>
                  <div className="flex justify-between text-sm text-muted-foreground">
                    <span className="flex items-center gap-1">
                      <DollarSign className="h-3 w-3" />
                      Bridge Fee:
                    </span>
                    <span>{quote.fee.toFixed(6)} {token}</span>
                  </div>
                  <div className="flex justify-between text-sm text-muted-foreground">
                    <span className="flex items-center gap-1">
                      <Clock className="h-3 w-3" />
                      Estimated Time:
                    </span>
                    <span>{quote.estimatedTime} minutes</span>
                  </div>
                  <div className="flex gap-1 mt-2">
                    <Badge variant="outline" className="text-xs">
                      Route: {quote.route.join(' → ')}
                    </Badge>
                  </div>
                </div>
              </AlertDescription>
            </Alert>
          )}

          {/* Bridge Button */}
          <Button
            onClick={handleBridge}
            disabled={loading || !quote || !toAddress}
            className="w-full"
            size="lg"
          >
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Processing Bridge...
              </>
            ) : (
              'Bridge Tokens'
            )}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}