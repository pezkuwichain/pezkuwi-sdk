import React, { useState } from 'react';
import { 
  Vote, Users, Gavel, FileText, TrendingUpIcon, 
  Clock, CheckCircle, XCircle, AlertCircle, 
  BarChart3, PieChart, Activity, Shield
} from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Badge } from '../ui/badge';
import { Progress } from '../ui/progress';

interface GovernanceStats {
  activeProposals: number;
  activeElections: number;
  totalVoters: number;
  participationRate: number;
  parliamentMembers: number;
  diwanMembers: number;
  nextElection: string;
  treasuryBalance: string;
}

const GovernanceOverview: React.FC = () => {
  const [stats] = useState<GovernanceStats>({
    activeProposals: 12,
    activeElections: 2,
    totalVoters: 15234,
    participationRate: 68.5,
    parliamentMembers: 27,
    diwanMembers: 9,
    nextElection: '15 days',
    treasuryBalance: '2.5M PEZ'
  });

  const [recentActivity] = useState([
    { type: 'proposal', action: 'New proposal submitted', title: 'Treasury Allocation Update', time: '2 hours ago' },
    { type: 'vote', action: 'Vote cast', title: 'Infrastructure Development Fund', time: '3 hours ago' },
    { type: 'election', action: 'Election started', title: 'Parliamentary Elections 2024', time: '1 day ago' },
    { type: 'approved', action: 'Proposal approved', title: 'Community Grant Program', time: '2 days ago' }
  ]);

  const getActivityIcon = (type: string) => {
    switch(type) {
      case 'proposal': return <FileText className="w-4 h-4 text-blue-400" />;
      case 'vote': return <Vote className="w-4 h-4 text-purple-400" />;
      case 'election': return <Users className="w-4 h-4 text-cyan-400" />;
      case 'approved': return <CheckCircle className="w-4 h-4 text-green-400" />;
      default: return <Activity className="w-4 h-4 text-gray-400" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card className="bg-gray-900/50 border-gray-800">
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Active Proposals</p>
                <p className="text-2xl font-bold text-white mt-1">{stats.activeProposals}</p>
                <p className="text-xs text-green-400 mt-2">+3 this week</p>
              </div>
              <div className="p-3 bg-blue-500/10 rounded-lg">
                <FileText className="w-6 h-6 text-blue-400" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="bg-gray-900/50 border-gray-800">
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Active Elections</p>
                <p className="text-2xl font-bold text-white mt-1">{stats.activeElections}</p>
                <p className="text-xs text-cyan-400 mt-2">Next in {stats.nextElection}</p>
              </div>
              <div className="p-3 bg-cyan-500/10 rounded-lg">
                <Users className="w-6 h-6 text-cyan-400" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="bg-gray-900/50 border-gray-800">
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Participation Rate</p>
                <p className="text-2xl font-bold text-white mt-1">{stats.participationRate}%</p>
                <Progress value={stats.participationRate} className="mt-2 h-1" />
              </div>
              <div className="p-3 bg-kurdish-green/10 rounded-lg">
                <TrendingUpIcon className="w-6 h-6 text-kurdish-green" />
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="bg-gray-900/50 border-gray-800">
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Treasury Balance</p>
                <p className="text-2xl font-bold text-white mt-1">{stats.treasuryBalance}</p>
                <p className="text-xs text-yellow-400 mt-2">Available for proposals</p>
              </div>
              <div className="p-3 bg-yellow-500/10 rounded-lg">
                <Shield className="w-6 h-6 text-yellow-400" />
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Government Bodies */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card className="bg-gray-900/50 border-gray-800">
          <CardHeader>
            <CardTitle className="text-white flex items-center">
              <Gavel className="w-5 h-5 mr-2 text-purple-400" />
              Parliament Status
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Active Members</span>
                <span className="text-white font-semibold">{stats.parliamentMembers}/27</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Current Session</span>
                <Badge className="bg-green-500/10 text-green-400 border-green-500/20">In Session</Badge>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Pending Votes</span>
                <span className="text-white font-semibold">5</span>
              </div>
              <div className="pt-2 border-t border-gray-800">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Quorum Status</span>
                  <span className="text-green-400">Met (85%)</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <Card className="bg-gray-900/50 border-gray-800">
          <CardHeader>
            <CardTitle className="text-white flex items-center">
              <Shield className="w-5 h-5 mr-2 text-cyan-400" />
              Dîwan (Constitutional Court)
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Active Judges</span>
                <span className="text-white font-semibold">{stats.diwanMembers}/9</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Pending Reviews</span>
                <span className="text-white font-semibold">3</span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Recent Decisions</span>
                <span className="text-white font-semibold">12</span>
              </div>
              <div className="pt-2 border-t border-gray-800">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-400">Next Hearing</span>
                  <span className="text-cyan-400">Tomorrow, 14:00 UTC</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Recent Activity */}
      <Card className="bg-gray-900/50 border-gray-800">
        <CardHeader>
          <CardTitle className="text-white flex items-center">
            <Activity className="w-5 h-5 mr-2 text-purple-400" />
            Recent Governance Activity
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {recentActivity.map((activity, index) => (
              <div key={index} className="flex items-start space-x-3 p-3 rounded-lg hover:bg-gray-800/50 transition-colors">
                {getActivityIcon(activity.type)}
                <div className="flex-1">
                  <p className="text-sm text-gray-300">{activity.action}</p>
                  <p className="text-xs text-white font-medium mt-1">{activity.title}</p>
                </div>
                <span className="text-xs text-gray-500">{activity.time}</span>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Voting Power Distribution */}
      <Card className="bg-gray-900/50 border-gray-800">
        <CardHeader>
          <CardTitle className="text-white flex items-center">
            <PieChart className="w-5 h-5 mr-2 text-purple-400" />
            Voting Power Distribution
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Direct Votes</span>
                <span className="text-white font-semibold">45%</span>
              </div>
              <Progress value={45} className="h-2 bg-gray-800" />
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Delegated Votes</span>
                <span className="text-white font-semibold">35%</span>
              </div>
              <Progress value={35} className="h-2 bg-gray-800" />
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-gray-400">Proxy Votes</span>
                <span className="text-white font-semibold">20%</span>
              </div>
              <Progress value={20} className="h-2 bg-gray-800" />
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default GovernanceOverview;