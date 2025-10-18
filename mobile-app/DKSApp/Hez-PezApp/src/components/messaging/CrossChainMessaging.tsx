import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Send, Globe, CheckCircle, Clock, AlertCircle, ArrowRight, Copy, ExternalLink } from 'lucide-react';

interface Message {
  id: string;
  from: string;
  to: string;
  content: string;
  timestamp: string;
  status: 'pending' | 'confirmed' | 'failed';
  txHash: string;
  fee: string;
}

export function CrossChainMessaging() {
  const [selectedChain, setSelectedChain] = useState('polkadot');
  const [messageContent, setMessageContent] = useState('');
  const [recipient, setRecipient] = useState('');
  const [gasEstimate, setGasEstimate] = useState('0.05');

  const chains = [
    { id: 'polkadot', name: 'Polkadot', status: 'online', latency: '1.2s' },
    { id: 'kusama', name: 'Kusama', status: 'online', latency: '0.8s' },
    { id: 'ethereum', name: 'Ethereum', status: 'online', latency: '15s' },
    { id: 'moonbeam', name: 'Moonbeam', status: 'online', latency: '2.5s' },
    { id: 'acala', name: 'Acala', status: 'online', latency: '1.5s' },
    { id: 'astar', name: 'Astar', status: 'maintenance', latency: '-' },
  ];

  const messages: Message[] = [
    { id: '1', from: 'Polkadot', to: 'Ethereum', content: 'Transfer notification: 100 HEZ bridged', timestamp: '2 mins ago', status: 'confirmed', txHash: '0x1234...5678', fee: '0.02 HEZ' },
    { id: '2', from: 'Kusama', to: 'Moonbeam', content: 'Smart contract execution request', timestamp: '5 mins ago', status: 'confirmed', txHash: '0x8765...4321', fee: '0.01 HEZ' },
    { id: '3', from: 'Acala', to: 'Polkadot', content: 'Liquidity pool update: New pair added', timestamp: '10 mins ago', status: 'pending', txHash: '0xabcd...efgh', fee: '0.03 HEZ' },
    { id: '4', from: 'Ethereum', to: 'Kusama', content: 'Oracle data feed update', timestamp: '15 mins ago', status: 'confirmed', txHash: '0xijkl...mnop', fee: '0.05 HEZ' },
    { id: '5', from: 'Moonbeam', to: 'Acala', content: 'Cross-chain governance proposal', timestamp: '30 mins ago', status: 'failed', txHash: '0xqrst...uvwx', fee: '0.02 HEZ' },
  ];

  const handleSendMessage = () => {
    console.log('Sending message:', { chain: selectedChain, recipient, content: messageContent });
    setMessageContent('');
    setRecipient('');
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'confirmed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'pending':
        return <Clock className="h-4 w-4 text-yellow-500" />;
      case 'failed':
        return <AlertCircle className="h-4 w-4 text-red-500" />;
      default:
        return null;
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row gap-4 items-start md:items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold">Cross-Chain Messaging</h2>
          <p className="text-muted-foreground mt-2">Send messages and data across different blockchains</p>
        </div>
        <div className="flex gap-2">
          <Badge variant="outline" className="px-3 py-1">
            <div className="w-2 h-2 bg-green-500 rounded-full mr-2 animate-pulse" />
            XCM Active
          </Badge>
          <Badge variant="outline" className="px-3 py-1">
            6 Chains Connected
          </Badge>
        </div>
      </div>

      <div className="grid md:grid-cols-3 gap-6">
        <div className="md:col-span-2 space-y-4">
          <Card>
            <CardHeader>
              <CardTitle>Send Cross-Chain Message</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid md:grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium mb-2 block">Source Chain</label>
                  <Select value={selectedChain} onValueChange={setSelectedChain}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {chains.map((chain) => (
                        <SelectItem key={chain.id} value={chain.id}>
                          <div className="flex items-center justify-between w-full">
                            <span>{chain.name}</span>
                            {chain.status === 'online' ? (
                              <Badge variant="outline" className="ml-2">Online</Badge>
                            ) : (
                              <Badge variant="secondary" className="ml-2">Maintenance</Badge>
                            )}
                          </div>
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>

                <div>
                  <label className="text-sm font-medium mb-2 block">Destination Chain</label>
                  <Select>
                    <SelectTrigger>
                      <SelectValue placeholder="Select destination" />
                    </SelectTrigger>
                    <SelectContent>
                      {chains.filter(c => c.id !== selectedChain).map((chain) => (
                        <SelectItem key={chain.id} value={chain.id}>
                          {chain.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Recipient Address</label>
                <Input
                  placeholder="Enter destination address"
                  value={recipient}
                  onChange={(e) => setRecipient(e.target.value)}
                />
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Message Content</label>
                <Textarea
                  placeholder="Enter your message or data payload"
                  value={messageContent}
                  onChange={(e) => setMessageContent(e.target.value)}
                  rows={4}
                />
              </div>

              <div className="flex items-center justify-between p-4 bg-secondary/20 rounded-lg">
                <div>
                  <p className="text-sm text-muted-foreground">Estimated Fee</p>
                  <p className="font-semibold">{gasEstimate} HEZ</p>
                </div>
                <div>
                  <p className="text-sm text-muted-foreground">Delivery Time</p>
                  <p className="font-semibold">~2-15 seconds</p>
                </div>
              </div>

              <Button 
                className="w-full" 
                size="lg"
                onClick={handleSendMessage}
                disabled={!messageContent || !recipient}
              >
                <Send className="mr-2 h-4 w-4" />
                Send Cross-Chain Message
              </Button>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Recent Messages</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {messages.map((message) => (
                <div key={message.id} className="p-4 border rounded-lg space-y-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline">{message.from}</Badge>
                      <ArrowRight className="h-4 w-4 text-muted-foreground" />
                      <Badge variant="outline">{message.to}</Badge>
                    </div>
                    <div className="flex items-center gap-2">
                      {getStatusIcon(message.status)}
                      <span className="text-sm capitalize">{message.status}</span>
                    </div>
                  </div>

                  <p className="text-sm">{message.content}</p>

                  <div className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-4">
                      <span className="text-muted-foreground">{message.timestamp}</span>
                      <span className="text-muted-foreground">Fee: {message.fee}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Button
                        size="sm"
                        variant="ghost"
                        onClick={() => copyToClipboard(message.txHash)}
                      >
                        <Copy className="h-3 w-3 mr-1" />
                        {message.txHash}
                      </Button>
                      <Button size="sm" variant="ghost">
                        <ExternalLink className="h-3 w-3" />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        </div>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Globe className="h-5 w-5" />
                Network Status
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              {chains.map((chain) => (
                <div key={chain.id} className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <div className={`w-2 h-2 rounded-full ${
                      chain.status === 'online' ? 'bg-green-500' : 'bg-yellow-500'
                    }`} />
                    <span className="text-sm">{chain.name}</span>
                  </div>
                  <span className="text-sm text-muted-foreground">{chain.latency}</span>
                </div>
              ))}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Message Stats</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Total Sent</span>
                <span className="font-semibold">1,247</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Success Rate</span>
                <span className="font-semibold text-green-600">98.5%</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Avg. Delivery</span>
                <span className="font-semibold">3.2s</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Total Fees</span>
                <span className="font-semibold">62.4 HEZ</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}