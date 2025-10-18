import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Skeleton } from '@/components/ui/skeleton';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { useTranslation } from 'react-i18next';
import { 
  DollarSign, 
  TrendingUp, 
  TrendingDown, 
  PieChart,
  Activity,
  AlertCircle,
  CheckCircle,
  Clock,
  ArrowUpRight,
  ArrowDownRight,
  RefreshCw,
  Wifi,
  WifiOff
} from 'lucide-react';
import { useWebSocket } from '@/contexts/WebSocketContext';
import { TreasuryInfo } from '@/services/blockchain/config';

interface TreasuryMetrics {
  totalBalance: number;
  monthlyIncome: number;
  monthlyExpenses: number;
  pendingProposals: number;
  approvedBudget: number;
  healthScore: number;
}

interface BudgetCategory {
  id: string;
  name: string;
  allocated: number;
  spent: number;
  remaining: number;
  color: string;
}

export const TreasuryOverview: React.FC = () => {
  const { t } = useTranslation();
  const { syncState, isConnected, refreshTreasury, subscribe, unsubscribe } = useWebSocket();
  const [metrics, setMetrics] = useState<TreasuryMetrics>({
    totalBalance: 0,
    monthlyIncome: 0,
    monthlyExpenses: 0,
    pendingProposals: 0,
    approvedBudget: 0,
    healthScore: 0
  });

  useEffect(() => {
    // Update metrics when treasury data changes
    if (syncState.treasury) {
      const balance = parseInt(syncState.treasury.balance) / 1e12; // Convert from smallest unit
      const proposals = syncState.treasury.proposals;
      const approvals = syncState.treasury.approvals;
      
      // Calculate health score based on treasury metrics
      const healthScore = calculateHealthScore(balance, proposals, approvals);
      
      setMetrics({
        totalBalance: balance,
        monthlyIncome: balance * 0.06, // Estimated based on staking rewards
        monthlyExpenses: parseInt(syncState.treasury.burn) / 1e12,
        pendingProposals: proposals,
        approvedBudget: approvals * 50000, // Average proposal size
        healthScore
      });
    }
  }, [syncState.treasury]);

  useEffect(() => {
    const handleTreasuryUpdate = (treasury: TreasuryInfo) => {
      console.log('Treasury updated:', treasury);
    };

    subscribe('treasury', handleTreasuryUpdate);
    return () => unsubscribe('treasury', handleTreasuryUpdate);
  }, [subscribe, unsubscribe]);

  const calculateHealthScore = (balance: number, proposals: number, approvals: number): number => {
    // Simple health score calculation
    const balanceScore = Math.min(balance / 5000000 * 100, 100);
    const proposalScore = Math.max(100 - (proposals * 5), 0);
    const approvalRate = approvals > 0 ? (approvals / (proposals + approvals)) * 100 : 50;
    return Math.round((balanceScore * 0.5 + proposalScore * 0.2 + approvalRate * 0.3));
  };

  const [categories] = useState<BudgetCategory[]>([
    { id: '1', name: 'Development', allocated: 500000, spent: 320000, remaining: 180000, color: 'bg-blue-500' },
    { id: '2', name: 'Marketing', allocated: 200000, spent: 150000, remaining: 50000, color: 'bg-purple-500' },
    { id: '3', name: 'Operations', allocated: 300000, spent: 180000, remaining: 120000, color: 'bg-green-500' },
    { id: '4', name: 'Community', allocated: 150000, spent: 80000, remaining: 70000, color: 'bg-yellow-500' },
    { id: '5', name: 'Research', allocated: 250000, spent: 100000, remaining: 150000, color: 'bg-pink-500' },
    { id: '6', name: 'Infrastructure', allocated: 400000, spent: 350000, remaining: 50000, color: 'bg-indigo-500' }
  ]);

  const getHealthStatus = (score: number) => {
    if (score >= 80) return { label: 'Excellent', color: 'text-green-500', icon: CheckCircle };
    if (score >= 60) return { label: 'Good', color: 'text-blue-500', icon: Activity };
    if (score >= 40) return { label: 'Fair', color: 'text-yellow-500', icon: AlertCircle };
    return { label: 'Critical', color: 'text-red-500', icon: AlertCircle };
  };

  const healthStatus = getHealthStatus(metrics.healthScore);
  const HealthIcon = healthStatus.icon;

  const formatBalance = (balance: number) => {
    if (balance >= 1000000) return `$${(balance / 1000000).toFixed(2)}M`;
    if (balance >= 1000) return `$${(balance / 1000).toFixed(0)}k`;
    return `$${balance.toFixed(0)}`;
  };

  if (syncState.isLoading && !syncState.treasury) {
    return (
      <div className="space-y-6">
        <Card>
          <CardHeader>
            <Skeleton className="h-8 w-48" />
          </CardHeader>
          <CardContent>
            <Skeleton className="h-32 w-full" />
          </CardContent>
        </Card>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {[1, 2, 3, 4].map(i => (
            <Card key={i}>
              <CardContent className="p-6">
                <Skeleton className="h-20 w-full" />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    );
  }

  if (syncState.error && !syncState.treasury) {
    return (
      <Alert className="bg-red-900/20 border-red-800">
        <AlertCircle className="h-4 w-4" />
        <AlertDescription>
          {syncState.error}
          <Button 
            onClick={refreshTreasury} 
            size="sm" 
            variant="outline" 
            className="ml-4"
          >
            <RefreshCw className="w-4 h-4 mr-2" />
            Retry
          </Button>
        </AlertDescription>
      </Alert>
    );
  }

  return (
    <div className="space-y-6">
      {/* Treasury Health Score */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Treasury Health</span>
            <HealthIcon className={`h-6 w-6 ${healthStatus.color}`} />
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-2xl font-bold">{metrics.healthScore}%</span>
              <Badge className={healthStatus.color}>{healthStatus.label}</Badge>
            </div>
            <Progress value={metrics.healthScore} className="h-3" />
            <div className="grid grid-cols-2 gap-4 text-sm">
              <div>
                <p className="text-muted-foreground">Runway</p>
                <p className="font-semibold">20.8 months</p>
              </div>
              <div>
                <p className="text-muted-foreground">Burn Rate</p>
                <p className="font-semibold">$120k/month</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Total Balance</p>
                <p className="text-2xl font-bold">${(metrics.totalBalance / 1000000).toFixed(2)}M</p>
                <p className="text-xs text-green-500 flex items-center mt-1">
                  <ArrowUpRight className="h-3 w-3 mr-1" />
                  +12.5% this month
                </p>
              </div>
              <DollarSign className="h-8 w-8 text-green-500" />
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Monthly Income</p>
                <p className="text-2xl font-bold">${(metrics.monthlyIncome / 1000).toFixed(0)}k</p>
                <p className="text-xs text-green-500 flex items-center mt-1">
                  <TrendingUp className="h-3 w-3 mr-1" />
                  +8.3% vs last month
                </p>
              </div>
              <TrendingUp className="h-8 w-8 text-blue-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Monthly Expenses</p>
                <p className="text-2xl font-bold">${(metrics.monthlyExpenses / 1000).toFixed(0)}k</p>
                <p className="text-xs text-red-500 flex items-center mt-1">
                  <ArrowDownRight className="h-3 w-3 mr-1" />
                  -5.2% vs last month
                </p>
              </div>
              <TrendingDown className="h-8 w-8 text-red-500" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent className="p-6">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Pending Proposals</p>
                <p className="text-2xl font-bold">{metrics.pendingProposals}</p>
                <p className="text-xs text-yellow-500 flex items-center mt-1">
                  <Clock className="h-3 w-3 mr-1" />
                  $450k requested
                </p>
              </div>
              <Clock className="h-8 w-8 text-yellow-500" />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Budget Categories */}
      <Card>
        <CardHeader>
          <CardTitle>Budget Allocation by Category</CardTitle>
          <CardDescription>Current quarter budget utilization</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {categories.map((category) => {
              const utilization = (category.spent / category.allocated) * 100;
              return (
                <div key={category.id} className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span className="font-medium">{category.name}</span>
                    <div className="flex items-center gap-4">
                      <span className="text-muted-foreground">
                        ${(category.spent / 1000).toFixed(0)}k / ${(category.allocated / 1000).toFixed(0)}k
                      </span>
                      <Badge variant={utilization > 80 ? 'destructive' : 'secondary'}>
                        {utilization.toFixed(0)}%
                      </Badge>
                    </div>
                  </div>
                  <Progress value={utilization} className="h-2" />
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>
    </div>
  );
};