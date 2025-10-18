import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { supabase } from '@/lib/supabase';
import { Calendar, Clock, Settings, Trash2, Plus } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';

interface BackupSchedule {
  id: string;
  schedule_name: string;
  backup_type: string;
  frequency: string;
  is_active: boolean;
  last_run: string | null;
  next_run: string | null;
  retention_days: number;
}

export function BackupScheduler() {
  const [schedules, setSchedules] = useState<BackupSchedule[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    schedule_name: '',
    backup_type: 'full',
    frequency: 'daily',
    retention_days: 30
  });
  const { toast } = useToast();

  useEffect(() => {
    loadSchedules();
  }, []);

  const loadSchedules = async () => {
    try {
      const { data, error } = await supabase
        .from('backup_schedules')
        .select('*')
        .order('created_at', { ascending: false });

      if (error) throw error;
      setSchedules(data || []);
    } catch (error) {
      console.error('Error loading schedules:', error);
    } finally {
      setLoading(false);
    }
  };

  const createSchedule = async () => {
    try {
      const nextRun = calculateNextRun(formData.frequency);
      
      const { error } = await supabase
        .from('backup_schedules')
        .insert({
          ...formData,
          next_run: nextRun,
          cron_expression: getCronExpression(formData.frequency)
        });

      if (error) throw error;

      toast({
        title: 'Schedule Created',
        description: 'Backup schedule has been created successfully',
      });

      setShowForm(false);
      setFormData({
        schedule_name: '',
        backup_type: 'full',
        frequency: 'daily',
        retention_days: 30
      });
      await loadSchedules();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to create backup schedule',
        variant: 'destructive'
      });
    }
  };

  const toggleSchedule = async (id: string, isActive: boolean) => {
    try {
      const { error } = await supabase
        .from('backup_schedules')
        .update({ is_active: isActive })
        .eq('id', id);

      if (error) throw error;
      await loadSchedules();
    } catch (error) {
      console.error('Error toggling schedule:', error);
    }
  };

  const deleteSchedule = async (id: string) => {
    try {
      const { error } = await supabase
        .from('backup_schedules')
        .delete()
        .eq('id', id);

      if (error) throw error;
      
      toast({
        title: 'Schedule Deleted',
        description: 'Backup schedule has been removed',
      });
      
      await loadSchedules();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to delete schedule',
        variant: 'destructive'
      });
    }
  };

  const calculateNextRun = (frequency: string) => {
    const now = new Date();
    switch (frequency) {
      case 'hourly':
        now.setHours(now.getHours() + 1);
        break;
      case 'daily':
        now.setDate(now.getDate() + 1);
        break;
      case 'weekly':
        now.setDate(now.getDate() + 7);
        break;
      case 'monthly':
        now.setMonth(now.getMonth() + 1);
        break;
    }
    return now.toISOString();
  };

  const getCronExpression = (frequency: string) => {
    switch (frequency) {
      case 'hourly': return '0 * * * *';
      case 'daily': return '0 0 * * *';
      case 'weekly': return '0 0 * * 0';
      case 'monthly': return '0 0 1 * *';
      default: return '0 0 * * *';
    }
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading schedules...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h3 className="text-lg font-semibold">Backup Schedules</h3>
        <Button onClick={() => setShowForm(!showForm)}>
          <Plus className="mr-2 h-4 w-4" />
          New Schedule
        </Button>
      </div>

      {showForm && (
        <Card>
          <CardHeader>
            <CardTitle>Create Backup Schedule</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label>Schedule Name</Label>
              <Input
                value={formData.schedule_name}
                onChange={(e) => setFormData({ ...formData, schedule_name: e.target.value })}
                placeholder="e.g., Daily Database Backup"
              />
            </div>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>Backup Type</Label>
                <Select
                  value={formData.backup_type}
                  onValueChange={(value) => setFormData({ ...formData, backup_type: value })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="full">Full Backup</SelectItem>
                    <SelectItem value="incremental">Incremental</SelectItem>
                    <SelectItem value="differential">Differential</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div>
                <Label>Frequency</Label>
                <Select
                  value={formData.frequency}
                  onValueChange={(value) => setFormData({ ...formData, frequency: value })}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="hourly">Hourly</SelectItem>
                    <SelectItem value="daily">Daily</SelectItem>
                    <SelectItem value="weekly">Weekly</SelectItem>
                    <SelectItem value="monthly">Monthly</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            <div>
              <Label>Retention Days</Label>
              <Input
                type="number"
                value={formData.retention_days}
                onChange={(e) => setFormData({ ...formData, retention_days: parseInt(e.target.value) })}
              />
            </div>
            <div className="flex gap-2">
              <Button onClick={createSchedule}>Create Schedule</Button>
              <Button variant="outline" onClick={() => setShowForm(false)}>Cancel</Button>
            </div>
          </CardContent>
        </Card>
      )}

      <div className="space-y-4">
        {schedules.map((schedule) => (
          <Card key={schedule.id}>
            <CardContent className="flex items-center justify-between p-6">
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <h4 className="font-semibold">{schedule.schedule_name}</h4>
                  <Badge variant={schedule.is_active ? 'default' : 'secondary'}>
                    {schedule.is_active ? 'Active' : 'Inactive'}
                  </Badge>
                </div>
                <div className="flex items-center gap-4 text-sm text-muted-foreground">
                  <span className="flex items-center gap-1">
                    <Settings className="h-3 w-3" />
                    {schedule.backup_type}
                  </span>
                  <span className="flex items-center gap-1">
                    <Clock className="h-3 w-3" />
                    {schedule.frequency}
                  </span>
                  <span className="flex items-center gap-1">
                    <Calendar className="h-3 w-3" />
                    Retention: {schedule.retention_days} days
                  </span>
                </div>
                {schedule.next_run && (
                  <p className="text-xs text-muted-foreground">
                    Next run: {new Date(schedule.next_run).toLocaleString()}
                  </p>
                )}
              </div>
              <div className="flex items-center gap-4">
                <Switch
                  checked={schedule.is_active}
                  onCheckedChange={(checked) => toggleSchedule(schedule.id, checked)}
                />
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => deleteSchedule(schedule.id)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}