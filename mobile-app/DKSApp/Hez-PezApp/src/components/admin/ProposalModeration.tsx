import { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Textarea } from '@/components/ui/textarea';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { CheckCircle, XCircle, AlertCircle, Clock, Eye } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';

interface Proposal {
  id: string;
  title: string;
  description: string;
  author: string;
  status: 'pending' | 'approved' | 'rejected' | 'flagged';
  created_at: string;
  category: string;
  votes_for: number;
  votes_against: number;
}

export default function ProposalModeration() {
  const [proposals] = useState<Proposal[]>([
    {
      id: '1',
      title: 'Implement Cross-Chain Bridge',
      description: 'Develop a secure bridge for cross-chain asset transfers...',
      author: 'alice.eth',
      status: 'pending',
      created_at: '2024-01-15',
      category: 'Technical',
      votes_for: 0,
      votes_against: 0
    },
    {
      id: '2',
      title: 'Treasury Allocation for Marketing',
      description: 'Allocate 50,000 tokens for Q1 marketing campaign...',
      author: 'bob.eth',
      status: 'pending',
      created_at: '2024-01-14',
      category: 'Treasury',
      votes_for: 0,
      votes_against: 0
    },
    {
      id: '3',
      title: 'Governance Token Burn Mechanism',
      description: 'Implement automatic token burn based on transaction fees...',
      author: 'charlie.eth',
      status: 'flagged',
      created_at: '2024-01-13',
      category: 'Tokenomics',
      votes_for: 45,
      votes_against: 12
    }
  ]);
  
  const [selectedProposal, setSelectedProposal] = useState<Proposal | null>(null);
  const [moderationNote, setModerationNote] = useState('');
  const { toast } = useToast();

  const handleApprove = (proposalId: string) => {
    toast({
      title: 'Proposal Approved',
      description: 'The proposal has been approved and is now live for voting'
    });
  };

  const handleReject = (proposalId: string) => {
    toast({
      title: 'Proposal Rejected',
      description: 'The proposal has been rejected with moderation notes',
      variant: 'destructive'
    });
  };

  const handleFlag = (proposalId: string) => {
    toast({
      title: 'Proposal Flagged',
      description: 'The proposal has been flagged for further review'
    });
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'approved': return 'default';
      case 'rejected': return 'destructive';
      case 'flagged': return 'secondary';
      default: return 'outline';
    }
  };

  const ProposalCard = ({ proposal }: { proposal: Proposal }) => (
    <Card>
      <CardHeader>
        <div className="flex items-start justify-between">
          <div>
            <CardTitle className="text-lg">{proposal.title}</CardTitle>
            <CardDescription>
              by {proposal.author} • {proposal.category}
            </CardDescription>
          </div>
          <Badge variant={getStatusColor(proposal.status)}>
            {proposal.status}
          </Badge>
        </div>
      </CardHeader>
      <CardContent>
        <p className="text-sm text-muted-foreground mb-4">
          {proposal.description}
        </p>
        
        <div className="flex items-center justify-between">
          <div className="text-sm text-muted-foreground">
            Created: {new Date(proposal.created_at).toLocaleDateString()}
          </div>
          
          <div className="flex gap-2">
            <Button
              size="sm"
              variant="outline"
              onClick={() => setSelectedProposal(proposal)}
            >
              <Eye className="h-4 w-4 mr-1" />
              Review
            </Button>
            
            {proposal.status === 'pending' && (
              <>
                <Button
                  size="sm"
                  variant="default"
                  onClick={() => handleApprove(proposal.id)}
                >
                  <CheckCircle className="h-4 w-4 mr-1" />
                  Approve
                </Button>
                <Button
                  size="sm"
                  variant="destructive"
                  onClick={() => handleReject(proposal.id)}
                >
                  <XCircle className="h-4 w-4 mr-1" />
                  Reject
                </Button>
              </>
            )}
            
            {proposal.status === 'flagged' && (
              <Button
                size="sm"
                variant="secondary"
                onClick={() => handleFlag(proposal.id)}
              >
                <AlertCircle className="h-4 w-4 mr-1" />
                Review Flag
              </Button>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Proposal Moderation Queue</CardTitle>
          <CardDescription>
            Review and moderate governance proposals before they go live
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="pending">
            <TabsList className="grid w-full grid-cols-4">
              <TabsTrigger value="pending">
                Pending <Badge className="ml-2" variant="outline">2</Badge>
              </TabsTrigger>
              <TabsTrigger value="flagged">
                Flagged <Badge className="ml-2" variant="secondary">1</Badge>
              </TabsTrigger>
              <TabsTrigger value="approved">
                Approved <Badge className="ml-2" variant="default">45</Badge>
              </TabsTrigger>
              <TabsTrigger value="rejected">
                Rejected <Badge className="ml-2" variant="destructive">12</Badge>
              </TabsTrigger>
            </TabsList>

            <TabsContent value="pending" className="space-y-4 mt-4">
              {proposals.filter(p => p.status === 'pending').map(proposal => (
                <ProposalCard key={proposal.id} proposal={proposal} />
              ))}
            </TabsContent>

            <TabsContent value="flagged" className="space-y-4 mt-4">
              {proposals.filter(p => p.status === 'flagged').map(proposal => (
                <ProposalCard key={proposal.id} proposal={proposal} />
              ))}
            </TabsContent>

            <TabsContent value="approved" className="space-y-4 mt-4">
              <div className="text-center py-8 text-muted-foreground">
                45 approved proposals
              </div>
            </TabsContent>

            <TabsContent value="rejected" className="space-y-4 mt-4">
              <div className="text-center py-8 text-muted-foreground">
                12 rejected proposals
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>

      {/* Review Dialog */}
      <Dialog open={!!selectedProposal} onOpenChange={() => setSelectedProposal(null)}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle>{selectedProposal?.title}</DialogTitle>
            <DialogDescription>
              Review proposal details and provide moderation feedback
            </DialogDescription>
          </DialogHeader>
          
          {selectedProposal && (
            <div className="space-y-4">
              <div>
                <h4 className="text-sm font-medium mb-2">Author</h4>
                <p className="text-sm text-muted-foreground">{selectedProposal.author}</p>
              </div>
              
              <div>
                <h4 className="text-sm font-medium mb-2">Category</h4>
                <Badge>{selectedProposal.category}</Badge>
              </div>
              
              <div>
                <h4 className="text-sm font-medium mb-2">Description</h4>
                <p className="text-sm text-muted-foreground">
                  {selectedProposal.description}
                </p>
              </div>
              
              <div>
                <h4 className="text-sm font-medium mb-2">Moderation Notes</h4>
                <Textarea
                  placeholder="Add notes about this proposal..."
                  value={moderationNote}
                  onChange={(e) => setModerationNote(e.target.value)}
                  rows={4}
                />
              </div>
              
              <div className="flex justify-end gap-2">
                <Button
                  variant="outline"
                  onClick={() => setSelectedProposal(null)}
                >
                  Cancel
                </Button>
                <Button
                  variant="secondary"
                  onClick={() => {
                    handleFlag(selectedProposal.id);
                    setSelectedProposal(null);
                  }}
                >
                  Flag for Review
                </Button>
                <Button
                  variant="destructive"
                  onClick={() => {
                    handleReject(selectedProposal.id);
                    setSelectedProposal(null);
                  }}
                >
                  Reject
                </Button>
                <Button
                  onClick={() => {
                    handleApprove(selectedProposal.id);
                    setSelectedProposal(null);
                  }}
                >
                  Approve
                </Button>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}