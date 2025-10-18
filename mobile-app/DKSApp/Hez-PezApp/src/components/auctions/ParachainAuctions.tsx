import React, { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Trophy, TrendingUp, Clock, Users, Gavel, AlertCircle } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';
import { Alert, AlertDescription } from '@/components/ui/alert';

const ParachainAuctions = () => {
  const { toast } = useToast();
  const [bidAmount, setBidAmount] = useState('');
  const [selectedSlot, setSelectedSlot] = useState<number | null>(null);

  const currentAuctions = [
    {
      id: 1,
      slotRange: '13-20',
      currentBid: '2.5M HEZ',
      leadingProject: 'DeFi Hub',
      endsIn: '2 days',
      participants: 8,
      progress: 75
    },
    {
      id: 2,
      slotRange: '21-28',
      currentBid: '1.8M HEZ',
      leadingProject: 'GameFi Network',
      endsIn: '5 days',
      participants: 5,
      progress: 45
    }
  ];

  const upcomingSlots = [
    { range: '29-36', startDate: '2024-02-01', minBid: '1M HEZ' },
    { range: '37-44', startDate: '2024-03-01', minBid: '1M HEZ' },
    { range: '45-52', startDate: '2024-04-01', minBid: '1M HEZ' }
  ];

  const crowdloans = [
    {
      project: 'DeFi Hub',
      raised: '2.5M HEZ',
      contributors: 1250,
      rewards: '10:1 DHB',
      target: '3M HEZ',
      progress: 83
    },
    {
      project: 'GameFi Network',
      raised: '1.8M HEZ',
      contributors: 890,
      rewards: '5:1 GFN',
      target: '2.5M HEZ',
      progress: 72
    },
    {
      project: 'Privacy Chain',
      raised: '950K HEZ',
      contributors: 450,
      rewards: '8:1 PRV',
      target: '2M HEZ',
      progress: 47
    }
  ];

  const handleBid = () => {
    if (!bidAmount || !selectedSlot) {
      toast({
        title: "Error",
        description: "Please select a slot and enter bid amount",
        variant: "destructive"
      });
      return;
    }
    toast({
      title: "Bid Placed",
      description: `Successfully bid ${bidAmount} HEZ for slot ${selectedSlot}`
    });
    setBidAmount('');
    setSelectedSlot(null);
  };

  const handleContribute = (project: string) => {
    toast({
      title: "Contribution Successful",
      description: `Contributed to ${project} crowdloan`
    });
  };

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Auctions</CardTitle>
            <Gavel className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">2</div>
            <p className="text-xs text-muted-foreground">Slots being auctioned</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Locked</CardTitle>
            <Trophy className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">4.3M HEZ</div>
            <p className="text-xs text-muted-foreground">In current auctions</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Participants</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">2,590</div>
            <p className="text-xs text-muted-foreground">Active contributors</p>
          </CardContent>
        </Card>
      </div>

      <Tabs defaultValue="auctions" className="w-full">
        <TabsList className="grid w-full grid-cols-3">
          <TabsTrigger value="auctions">Live Auctions</TabsTrigger>
          <TabsTrigger value="crowdloans">Crowdloans</TabsTrigger>
          <TabsTrigger value="upcoming">Upcoming Slots</TabsTrigger>
        </TabsList>

        <TabsContent value="auctions" className="space-y-4">
          <Alert>
            <AlertCircle className="h-4 w-4" />
            <AlertDescription>
              Parachain slots are leased for 96-week periods. Win an auction to secure a slot for your project.
            </AlertDescription>
          </Alert>

          {currentAuctions.map(auction => (
            <Card key={auction.id}>
              <CardHeader>
                <div className="flex items-center justify-between">
                  <div>
                    <CardTitle>Slot {auction.slotRange}</CardTitle>
                    <CardDescription>Ends in {auction.endsIn}</CardDescription>
                  </div>
                  <Badge variant="outline">
                    <Clock className="mr-1 h-3 w-3" />
                    Live
                  </Badge>
                </div>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Current Leader:</span>
                    <span className="font-semibold">{auction.leadingProject}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span>Highest Bid:</span>
                    <span className="font-semibold text-green-600">{auction.currentBid}</span>
                  </div>
                  <div className="flex justify-between text-sm">
                    <span>Participants:</span>
                    <span>{auction.participants}</span>
                  </div>
                </div>
                
                <Progress value={auction.progress} className="h-2" />
                
                <div className="flex gap-2">
                  <Input
                    placeholder="Enter bid amount"
                    value={selectedSlot === auction.id ? bidAmount : ''}
                    onChange={(e) => {
                      setBidAmount(e.target.value);
                      setSelectedSlot(auction.id);
                    }}
                  />
                  <Button onClick={handleBid}>Place Bid</Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </TabsContent>

        <TabsContent value="crowdloans" className="space-y-4">
          <Alert>
            <TrendingUp className="h-4 w-4" />
            <AlertDescription>
              Support projects by contributing HEZ to their crowdloan. Earn project tokens as rewards!
            </AlertDescription>
          </Alert>

          {crowdloans.map(loan => (
            <Card key={loan.project}>
              <CardHeader>
                <div className="flex items-center justify-between">
                  <CardTitle>{loan.project}</CardTitle>
                  <Badge variant="secondary">{loan.rewards}</Badge>
                </div>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Raised:</span>
                    <span className="font-semibold">{loan.raised} / {loan.target}</span>
                  </div>
                  <Progress value={loan.progress} className="h-2" />
                  <div className="flex justify-between text-sm text-muted-foreground">
                    <span>{loan.contributors} contributors</span>
                    <span>{loan.progress}% funded</span>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <Input placeholder="Contribution amount" />
                  <Button onClick={() => handleContribute(loan.project)}>
                    Contribute
                  </Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </TabsContent>

        <TabsContent value="upcoming" className="space-y-4">
          {upcomingSlots.map(slot => (
            <Card key={slot.range}>
              <CardHeader>
                <CardTitle>Slot {slot.range}</CardTitle>
                <CardDescription>Auction starts: {slot.startDate}</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-muted-foreground">Minimum Bid</p>
                    <p className="text-lg font-semibold">{slot.minBid}</p>
                  </div>
                  <Button variant="outline">Set Reminder</Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default ParachainAuctions;