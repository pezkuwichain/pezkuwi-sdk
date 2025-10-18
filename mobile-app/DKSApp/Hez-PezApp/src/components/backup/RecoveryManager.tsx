import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { supabase } from '@/lib/supabase';
import { RotateCcw, AlertTriangle, Clock, CheckCircle, Database, Calendar } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';
import { format } from 'date-fns';

interface RecoveryLog {
  id: string;
  backup_id: string;
  recovery_type: string;
  recovery_point: string;
  status: string;
  started_at: string;
  completed_at: string | null;
  affected_records: number | null;
}

interface Backup {
  id: string;
  backup_name: string;
  backup_type: string;
  created_at: string;
  backup_size: number;
}

export function RecoveryManager() {
  const [backups, setBackups] = useState<Backup[]>([]);
  const [recoveryLogs, setRecoveryLogs] = useState<RecoveryLog[]>([]);
  const [selectedBackup, setSelectedBackup] = useState<string>('');
  const [recoveryType, setRecoveryType] = useState<string>('full');
  const [loading, setLoading] = useState(true);
  const [recovering, setRecovering] = useState(false);
  const { toast } = useToast();

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [backupsRes, logsRes] = await Promise.all([
        supabase
          .from('backup_metadata')
          .select('*')
          .eq('status', 'completed')
          .order('created_at', { ascending: false })
          .limit(20),
        supabase
          .from('recovery_logs')
          .select('*')
          .order('started_at', { ascending: false })
          .limit(10)
      ]);

      if (backupsRes.data) setBackups(backupsRes.data);
      if (logsRes.data) setRecoveryLogs(logsRes.data);
    } catch (error) {
      console.error('Error loading recovery data:', error);
    } finally {
      setLoading(false);
    }
  };

  const initiateRecovery = async () => {
    if (!selectedBackup) {
      toast({
        title: 'Error',
        description: 'Please select a backup to recover from',
        variant: 'destructive'
      });
      return;
    }

    setRecovering(true);
    try {
      const backup = backups.find(b => b.id === selectedBackup);
      const { data, error } = await supabase.functions.invoke('backup-operations', {
        body: {
          action: 'initiate_recovery',
          data: {
            backupId: selectedBackup,
            recoveryType,
            recoveryPoint: backup?.created_at
          }
        }
      });

      if (error) throw error;

      toast({
        title: 'Recovery Started',
        description: 'Recovery process has been initiated. This may take several minutes.',
      });

      await loadData();
    } catch (error) {
      toast({
        title: 'Recovery Failed',
        description: 'Failed to initiate recovery process',
        variant: 'destructive'
      });
    } finally {
      setRecovering(false);
    }
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading recovery manager...</div>;
  }

  return (
    <div className="space-y-6">
      <Alert>
        <AlertTriangle className="h-4 w-4" />
        <AlertDescription>
          Recovery operations can affect system availability. Ensure you have verified the backup integrity before proceeding.
        </AlertDescription>
      </Alert>

      <Card>
        <CardHeader>
          <CardTitle>Point-in-Time Recovery</CardTitle>
          <CardDescription>
            Restore your system to a specific backup point
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="text-sm font-medium">Select Backup</label>
              <Select value={selectedBackup} onValueChange={setSelectedBackup}>
                <SelectTrigger>
                  <SelectValue placeholder="Choose a backup" />
                </SelectTrigger>
                <SelectContent>
                  {backups.map((backup) => (
                    <SelectItem key={backup.id} value={backup.id}>
                      <div className="flex flex-col">
                        <span>{backup.backup_name}</span>
                        <span className="text-xs text-muted-foreground">
                          {format(new Date(backup.created_at), 'PPp')}
                        </span>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <div>
              <label className="text-sm font-medium">Recovery Type</label>
              <Select value={recoveryType} onValueChange={setRecoveryType}>
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="full">Full Recovery</SelectItem>
                  <SelectItem value="partial">Partial Recovery</SelectItem>
                  <SelectItem value="point_in_time">Point-in-Time</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          {selectedBackup && (
            <div className="p-4 bg-muted rounded-lg">
              <h4 className="font-medium mb-2">Backup Details</h4>
              {(() => {
                const backup = backups.find(b => b.id === selectedBackup);
                return backup ? (
                  <div className="space-y-1 text-sm">
                    <p>Type: {backup.backup_type}</p>
                    <p>Created: {format(new Date(backup.created_at), 'PPp')}</p>
                    <p>Size: {(backup.backup_size / 1024 / 1024).toFixed(2)} MB</p>
                  </div>
                ) : null;
              })()}
            </div>
          )}

          <Button 
            onClick={initiateRecovery} 
            disabled={recovering || !selectedBackup}
            className="w-full"
          >
            <RotateCcw className="mr-2 h-4 w-4" />
            {recovering ? 'Initiating Recovery...' : 'Start Recovery'}
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Recovery History</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {recoveryLogs.map((log) => (
              <div key={log.id} className="flex items-center justify-between p-4 border rounded-lg">
                <div className="flex items-center gap-4">
                  <div className={`p-2 rounded-lg ${
                    log.status === 'completed' ? 'bg-green-100' : 
                    log.status === 'failed' ? 'bg-red-100' : 'bg-yellow-100'
                  }`}>
                    {log.status === 'completed' ? <CheckCircle className="h-4 w-4 text-green-600" /> :
                     log.status === 'failed' ? <AlertTriangle className="h-4 w-4 text-red-600" /> :
                     <Clock className="h-4 w-4 text-yellow-600" />}
                  </div>
                  <div>
                    <p className="font-medium">
                      {log.recovery_type.charAt(0).toUpperCase() + log.recovery_type.slice(1)} Recovery
                    </p>
                    <div className="flex items-center gap-4 text-sm text-muted-foreground">
                      <span className="flex items-center gap-1">
                        <Calendar className="h-3 w-3" />
                        {format(new Date(log.started_at), 'PP')}
                      </span>
                      {log.affected_records && (
                        <span className="flex items-center gap-1">
                          <Database className="h-3 w-3" />
                          {log.affected_records} records
                        </span>
                      )}
                    </div>
                  </div>
                </div>
                <Badge variant={log.status === 'completed' ? 'default' : 
                              log.status === 'failed' ? 'destructive' : 'secondary'}>
                  {log.status}
                </Badge>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}