import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { supabase } from '@/lib/supabase';
import { Database, Download, Upload, Clock, Shield, AlertTriangle, CheckCircle, HardDrive, Calendar } from 'lucide-react';
import { formatDistanceToNow } from 'date-fns';
import { useToast } from '@/hooks/use-toast';

interface BackupStats {
  totalBackups: number;
  totalSize: number;
  successRate: number;
  activeSchedules: number;
  recentBackups: any[];
}

export function BackupDashboard() {
  const [stats, setStats] = useState<BackupStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [creating, setCreating] = useState(false);
  const { toast } = useToast();

  useEffect(() => {
    loadBackupStats();
  }, []);

  const loadBackupStats = async () => {
    try {
      const { data } = await supabase.functions.invoke('backup-operations', {
        body: { action: 'get_backup_stats' }
      });
      if (data?.data) {
        setStats(data.data);
      }
    } catch (error) {
      console.error('Error loading backup stats:', error);
    } finally {
      setLoading(false);
    }
  };

  const createBackup = async (type: string) => {
    setCreating(true);
    try {
      const { data, error } = await supabase.functions.invoke('backup-operations', {
        body: {
          action: 'create_backup',
          data: {
            backupType: type,
            backupName: `${type.charAt(0).toUpperCase() + type.slice(1)} Backup - ${new Date().toLocaleDateString()}`
          }
        }
      });

      if (error) throw error;

      toast({
        title: 'Backup Created',
        description: 'Backup has been successfully created',
      });
      
      await loadBackupStats();
    } catch (error) {
      toast({
        title: 'Backup Failed',
        description: 'Failed to create backup',
        variant: 'destructive'
      });
    } finally {
      setCreating(false);
    }
  };

  const formatSize = (bytes: number) => {
    const sizes = ['B', 'KB', 'MB', 'GB'];
    if (bytes === 0) return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading backup dashboard...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Backups</CardTitle>
            <Database className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats?.totalBackups || 0}</div>
            <p className="text-xs text-muted-foreground">All backup types</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Storage Used</CardTitle>
            <HardDrive className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{formatSize(stats?.totalSize || 0)}</div>
            <p className="text-xs text-muted-foreground">Total backup size</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Success Rate</CardTitle>
            <CheckCircle className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{Math.round(stats?.successRate || 0)}%</div>
            <Progress value={stats?.successRate || 0} className="mt-2" />
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Active Schedules</CardTitle>
            <Calendar className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats?.activeSchedules || 0}</div>
            <p className="text-xs text-muted-foreground">Automated backups</p>
          </CardContent>
        </Card>
      </div>

      <div className="flex gap-4">
        <Button onClick={() => createBackup('full')} disabled={creating}>
          <Database className="mr-2 h-4 w-4" />
          Create Full Backup
        </Button>
        <Button onClick={() => createBackup('incremental')} variant="outline" disabled={creating}>
          <Upload className="mr-2 h-4 w-4" />
          Incremental Backup
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Recent Backups</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {stats?.recentBackups?.map((backup) => (
              <div key={backup.id} className="flex items-center justify-between p-4 border rounded-lg">
                <div className="flex items-center gap-4">
                  <div className={`p-2 rounded-lg ${
                    backup.status === 'completed' ? 'bg-green-100' : 
                    backup.status === 'failed' ? 'bg-red-100' : 'bg-yellow-100'
                  }`}>
                    {backup.status === 'completed' ? <CheckCircle className="h-4 w-4 text-green-600" /> :
                     backup.status === 'failed' ? <AlertTriangle className="h-4 w-4 text-red-600" /> :
                     <Clock className="h-4 w-4 text-yellow-600" />}
                  </div>
                  <div>
                    <p className="font-medium">{backup.backup_name}</p>
                    <div className="flex items-center gap-4 text-sm text-muted-foreground">
                      <span>{formatDistanceToNow(new Date(backup.created_at))} ago</span>
                      <Badge variant="outline">{backup.backup_type}</Badge>
                      {backup.backup_size && <span>{formatSize(backup.backup_size)}</span>}
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Button size="sm" variant="ghost">
                    <Download className="h-4 w-4" />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}