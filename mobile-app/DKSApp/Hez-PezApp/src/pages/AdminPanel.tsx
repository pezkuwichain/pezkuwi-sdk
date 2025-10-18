import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuth } from '@/contexts/AuthContext';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Button } from '@/components/ui/button';
import { Shield, Users, Activity, BarChart3, FileText, Settings, Database, HardDrive } from 'lucide-react';
import { Alert, AlertDescription } from '@/components/ui/alert';
import UserManagement from '@/components/admin/UserManagement';
import PermissionsManager from '@/components/admin/PermissionsManager';
import ActivityLogs from '@/components/admin/ActivityLogs';
import SystemAnalytics from '@/components/admin/SystemAnalytics';
import ProposalModeration from '@/components/admin/ProposalModeration';
import { BackupDashboard } from '@/components/backup/BackupDashboard';
import { BackupScheduler } from '@/components/backup/BackupScheduler';
import { RecoveryManager } from '@/components/backup/RecoveryManager';
import { DisasterRecoveryPlan } from '@/components/backup/DisasterRecoveryPlan';
import { DataExport } from '@/components/backup/DataExport';
import { supabase } from '@/lib/supabase';

export default function AdminPanel() {
  const { user } = useAuth();
  const navigate = useNavigate();
  const [isAdmin, setIsAdmin] = useState(false);
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState({
    totalUsers: 0,
    activeProposals: 0,
    todayActivity: 0,
    pendingModeration: 0
  });

  useEffect(() => {
    checkAdminAccess();
    fetchDashboardStats();
  }, [user]);

  const checkAdminAccess = async () => {
    if (!user) {
      navigate('/');
      return;
    }

    try {
      const { data, error } = await supabase
        .from('governance_permissions')
        .select('permission_type')
        .eq('user_id', user.id)
        .eq('permission_type', 'admin')
        .single();

      if (error || !data) {
        navigate('/');
        return;
      }

      setIsAdmin(true);
    } catch (error) {
      console.error('Error checking admin access:', error);
      navigate('/');
    } finally {
      setLoading(false);
    }
  };

  const fetchDashboardStats = async () => {
    try {
      // Fetch user count
      const { count: userCount } = await supabase
        .from('profiles')
        .select('*', { count: 'exact', head: true });

      // Fetch activity logs from today
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      
      const { count: activityCount } = await supabase
        .from('activity_logs')
        .select('*', { count: 'exact', head: true })
        .gte('created_at', today.toISOString());

      setStats({
        totalUsers: userCount || 0,
        activeProposals: 12,
        todayActivity: activityCount || 0,
        pendingModeration: 3
      });
    } catch (error) {
      console.error('Error fetching stats:', error);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (!isAdmin) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Alert>
          <AlertDescription>
            You don't have permission to access this page.
          </AlertDescription>
        </Alert>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Admin Panel</h1>
        <p className="text-muted-foreground">Manage users, permissions, and system settings</p>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Users</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.totalUsers}</div>
            <p className="text-xs text-muted-foreground">+12% from last month</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Proposals</CardTitle>
            <FileText className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.activeProposals}</div>
            <p className="text-xs text-muted-foreground">3 pending review</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Today's Activity</CardTitle>
            <Activity className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.todayActivity}</div>
            <p className="text-xs text-muted-foreground">Actions logged today</p>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Pending Moderation</CardTitle>
            <Shield className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.pendingModeration}</div>
            <p className="text-xs text-muted-foreground">Requires attention</p>
          </CardContent>
        </Card>
      </div>

      {/* Main Admin Tabs */}
      <Tabs defaultValue="users" className="space-y-4">
        <TabsList className="grid w-full grid-cols-6">
          <TabsTrigger value="users">Users</TabsTrigger>
          <TabsTrigger value="permissions">Permissions</TabsTrigger>
          <TabsTrigger value="activity">Activity</TabsTrigger>
          <TabsTrigger value="moderation">Moderation</TabsTrigger>
          <TabsTrigger value="analytics">Analytics</TabsTrigger>
          <TabsTrigger value="backup">Backup</TabsTrigger>
        </TabsList>

        <TabsContent value="users">
          <UserManagement />
        </TabsContent>

        <TabsContent value="permissions">
          <PermissionsManager />
        </TabsContent>

        <TabsContent value="activity">
          <ActivityLogs />
        </TabsContent>

        <TabsContent value="moderation">
          <ProposalModeration />
        </TabsContent>

        <TabsContent value="analytics">
          <SystemAnalytics />
        </TabsContent>

        <TabsContent value="backup" className="space-y-6">
          <div className="mb-6">
            <h2 className="text-2xl font-bold mb-2">Backup & Recovery</h2>
            <p className="text-muted-foreground">Manage system backups, recovery points, and disaster recovery procedures</p>
          </div>
          
          <Tabs defaultValue="dashboard" className="space-y-4">
            <TabsList>
              <TabsTrigger value="dashboard">Dashboard</TabsTrigger>
              <TabsTrigger value="scheduler">Scheduler</TabsTrigger>
              <TabsTrigger value="recovery">Recovery</TabsTrigger>
              <TabsTrigger value="export">Export</TabsTrigger>
              <TabsTrigger value="drp">Disaster Plan</TabsTrigger>
            </TabsList>
            
            <TabsContent value="dashboard">
              <BackupDashboard />
            </TabsContent>
            
            <TabsContent value="scheduler">
              <BackupScheduler />
            </TabsContent>
            
            <TabsContent value="recovery">
              <RecoveryManager />
            </TabsContent>
            
            <TabsContent value="export">
              <DataExport />
            </TabsContent>
            
            <TabsContent value="drp">
              <DisasterRecoveryPlan />
            </TabsContent>
          </Tabs>
        </TabsContent>
      </Tabs>
    </div>
  );
}