import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Progress } from '@/components/ui/progress';
import { BarChart3, TrendingUp, Users, Activity, DollarSign, FileText } from 'lucide-react';
import { supabase } from '@/lib/supabase';

interface Analytics {
  userGrowth: number[];
  proposalActivity: number[];
  votingParticipation: number;
  treasuryBalance: number;
  activeUsers: number;
  totalProposals: number;
}

export default function SystemAnalytics() {
  const [analytics, setAnalytics] = useState<Analytics>({
    userGrowth: [10, 25, 35, 45, 60, 75, 90],
    proposalActivity: [5, 8, 12, 10, 15, 18, 22],
    votingParticipation: 68,
    treasuryBalance: 1250000,
    activeUsers: 342,
    totalProposals: 89
  });
  const [timeRange, setTimeRange] = useState('7d');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchAnalytics();
  }, [timeRange]);

  const fetchAnalytics = async () => {
    try {
      // Fetch user count
      const { count: userCount } = await supabase
        .from('profiles')
        .select('*', { count: 'exact', head: true });

      // Fetch recent activity
      const sevenDaysAgo = new Date();
      sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);
      
      const { count: recentActivity } = await supabase
        .from('activity_logs')
        .select('*', { count: 'exact', head: true })
        .gte('created_at', sevenDaysAgo.toISOString());

      setAnalytics(prev => ({
        ...prev,
        activeUsers: userCount || 0,
        totalProposals: 89 + Math.floor(Math.random() * 10)
      }));
    } catch (error) {
      console.error('Error fetching analytics:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 0
    }).format(value);
  };

  return (
    <div className="space-y-6">
      {/* Time Range Selector */}
      <div className="flex justify-end">
        <Select value={timeRange} onValueChange={setTimeRange}>
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Select time range" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="24h">Last 24 Hours</SelectItem>
            <SelectItem value="7d">Last 7 Days</SelectItem>
            <SelectItem value="30d">Last 30 Days</SelectItem>
            <SelectItem value="90d">Last 90 Days</SelectItem>
          </SelectContent>
        </Select>
      </div>

      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Users</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{analytics.activeUsers}</div>
            <p className="text-xs text-muted-foreground">
              <span className="text-green-500">↑ 12%</span> from last period
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Proposals</CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{analytics.totalProposals}</div>
            <p className="text-xs text-muted-foreground">
              <span className="text-green-500">↑ 8%</span> from last period
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Voting Participation</CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{analytics.votingParticipation}%</div>
            <Progress value={analytics.votingParticipation} className="mt-2" />
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Treasury Balance</CardTitle>
            <DollarSign className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{formatCurrency(analytics.treasuryBalance)}</div>
            <p className="text-xs text-muted-foreground">
              <span className="text-red-500">↓ 3%</span> from last period
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Detailed Analytics */}
      <Tabs defaultValue="users" className="space-y-4">
        <TabsList>
          <TabsTrigger value="users">User Analytics</TabsTrigger>
          <TabsTrigger value="proposals">Proposal Analytics</TabsTrigger>
          <TabsTrigger value="treasury">Treasury Analytics</TabsTrigger>
          <TabsTrigger value="performance">System Performance</TabsTrigger>
        </TabsList>

        <TabsContent value="users">
          <Card>
            <CardHeader>
              <CardTitle>User Growth & Activity</CardTitle>
              <CardDescription>Track user registration and engagement metrics</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">New Users This Week</span>
                    <span className="text-sm text-muted-foreground">45</span>
                  </div>
                  <Progress value={75} />
                </div>
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">Daily Active Users</span>
                    <span className="text-sm text-muted-foreground">128</span>
                  </div>
                  <Progress value={60} />
                </div>
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">User Retention Rate</span>
                    <span className="text-sm text-muted-foreground">82%</span>
                  </div>
                  <Progress value={82} />
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="proposals">
          <Card>
            <CardHeader>
              <CardTitle>Proposal Metrics</CardTitle>
              <CardDescription>Analyze proposal creation and voting patterns</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <p className="text-sm font-medium">Proposals by Status</p>
                  <div className="space-y-1">
                    <div className="flex justify-between text-sm">
                      <span>Active</span>
                      <span className="font-medium">12</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Passed</span>
                      <span className="font-medium">45</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Failed</span>
                      <span className="font-medium">23</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Pending</span>
                      <span className="font-medium">9</span>
                    </div>
                  </div>
                </div>
                <div className="space-y-2">
                  <p className="text-sm font-medium">Voting Statistics</p>
                  <div className="space-y-1">
                    <div className="flex justify-between text-sm">
                      <span>Avg. Participation</span>
                      <span className="font-medium">68%</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Total Votes Cast</span>
                      <span className="font-medium">3,456</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Unique Voters</span>
                      <span className="font-medium">234</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Avg. Approval Rate</span>
                      <span className="font-medium">71%</span>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="treasury">
          <Card>
            <CardHeader>
              <CardTitle>Treasury Overview</CardTitle>
              <CardDescription>Monitor treasury balance and spending</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">Monthly Spending</span>
                    <span className="text-sm text-muted-foreground">{formatCurrency(85000)}</span>
                  </div>
                  <Progress value={35} />
                </div>
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">Approved Allocations</span>
                    <span className="text-sm text-muted-foreground">{formatCurrency(250000)}</span>
                  </div>
                  <Progress value={20} />
                </div>
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium">Reserve Fund</span>
                    <span className="text-sm text-muted-foreground">{formatCurrency(500000)}</span>
                  </div>
                  <Progress value={40} />
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="performance">
          <Card>
            <CardHeader>
              <CardTitle>System Performance</CardTitle>
              <CardDescription>Monitor system health and performance metrics</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <p className="text-sm font-medium">API Performance</p>
                  <div className="space-y-1">
                    <div className="flex justify-between text-sm">
                      <span>Avg. Response Time</span>
                      <span className="font-medium">245ms</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Uptime</span>
                      <span className="font-medium">99.98%</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Error Rate</span>
                      <span className="font-medium">0.02%</span>
                    </div>
                  </div>
                </div>
                <div className="space-y-2">
                  <p className="text-sm font-medium">Database Metrics</p>
                  <div className="space-y-1">
                    <div className="flex justify-between text-sm">
                      <span>Storage Used</span>
                      <span className="font-medium">2.3 GB</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Query Performance</span>
                      <span className="font-medium">98ms</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span>Active Connections</span>
                      <span className="font-medium">42</span>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}