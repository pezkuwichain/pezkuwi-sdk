import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Calendar } from '@/components/ui/calendar';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Search, Calendar as CalendarIcon, Download, Filter, Activity } from 'lucide-react';
import { format } from 'date-fns';
import { supabase } from '@/lib/supabase';
import { useToast } from '@/hooks/use-toast';

interface ActivityLog {
  id: string;
  user_id: string;
  action: string;
  details: any;
  ip_address: string;
  user_agent: string;
  created_at: string;
  user?: {
    username: string;
    email: string;
  };
}

const ACTION_TYPES = {
  login: { label: 'Login', color: 'default' },
  logout: { label: 'Logout', color: 'secondary' },
  proposal_created: { label: 'Proposal Created', color: 'default' },
  proposal_voted: { label: 'Voted on Proposal', color: 'default' },
  permission_granted: { label: 'Permission Granted', color: 'default' },
  permission_revoked: { label: 'Permission Revoked', color: 'destructive' },
  user_suspended: { label: 'User Suspended', color: 'destructive' },
  profile_updated: { label: 'Profile Updated', color: 'secondary' },
  wallet_connected: { label: 'Wallet Connected', color: 'default' },
  treasury_withdrawal: { label: 'Treasury Withdrawal', color: 'destructive' }
};

export default function ActivityLogs() {
  const [logs, setLogs] = useState<ActivityLog[]>([]);
  const [filteredLogs, setFilteredLogs] = useState<ActivityLog[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [actionFilter, setActionFilter] = useState('all');
  const [dateRange, setDateRange] = useState<{
    from: Date | undefined;
    to: Date | undefined;
  }>({ from: undefined, to: undefined });
  const { toast } = useToast();

  useEffect(() => {
    fetchLogs();
  }, []);

  useEffect(() => {
    filterLogs();
  }, [searchQuery, actionFilter, dateRange, logs]);

  const fetchLogs = async () => {
    try {
      const { data } = await supabase
        .from('activity_logs')
        .select(`
          *,
          profiles!activity_logs_user_id_fkey (
            username,
            email
          )
        `)
        .order('created_at', { ascending: false })
        .limit(100);

      const formattedLogs = data?.map(log => ({
        ...log,
        user: log.profiles
      })) || [];

      setLogs(formattedLogs);
      setFilteredLogs(formattedLogs);
    } catch (error) {
      console.error('Error fetching logs:', error);
      toast({
        title: 'Error',
        description: 'Failed to fetch activity logs',
        variant: 'destructive'
      });
    } finally {
      setLoading(false);
    }
  };

  const filterLogs = () => {
    let filtered = [...logs];

    if (searchQuery) {
      filtered = filtered.filter(log =>
        log.user?.username?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        log.user?.email?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        log.action.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    if (actionFilter !== 'all') {
      filtered = filtered.filter(log => log.action === actionFilter);
    }

    if (dateRange.from) {
      filtered = filtered.filter(log => 
        new Date(log.created_at) >= dateRange.from!
      );
    }

    if (dateRange.to) {
      filtered = filtered.filter(log => 
        new Date(log.created_at) <= dateRange.to!
      );
    }

    setFilteredLogs(filtered);
  };

  const exportLogs = () => {
    const csv = [
      ['Date', 'User', 'Action', 'Details', 'IP Address'].join(','),
      ...filteredLogs.map(log => [
        new Date(log.created_at).toISOString(),
        log.user?.email || 'Unknown',
        log.action,
        JSON.stringify(log.details),
        log.ip_address || 'N/A'
      ].join(','))
    ].join('\n');

    const blob = new Blob([csv], { type: 'text/csv' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `activity-logs-${format(new Date(), 'yyyy-MM-dd')}.csv`;
    a.click();
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading activity logs...</div>;
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>Activity Logs</CardTitle>
            <CardDescription>Monitor all user activities and system events</CardDescription>
          </div>
          <Button onClick={exportLogs} variant="outline">
            <Download className="h-4 w-4 mr-2" />
            Export CSV
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        <div className="flex gap-4 mb-6">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
            <Input
              placeholder="Search logs..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
          
          <Select value={actionFilter} onValueChange={setActionFilter}>
            <SelectTrigger className="w-[200px]">
              <SelectValue placeholder="Filter by action" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Actions</SelectItem>
              {Object.entries(ACTION_TYPES).map(([key, config]) => (
                <SelectItem key={key} value={key}>{config.label}</SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Popover>
            <PopoverTrigger asChild>
              <Button variant="outline">
                <CalendarIcon className="h-4 w-4 mr-2" />
                Date Range
              </Button>
            </PopoverTrigger>
            <PopoverContent className="w-auto p-0" align="end">
              <Calendar
                mode="range"
                selected={{ from: dateRange.from, to: dateRange.to }}
                onSelect={(range: any) => setDateRange(range || { from: undefined, to: undefined })}
              />
            </PopoverContent>
          </Popover>
        </div>

        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Timestamp</TableHead>
                <TableHead>User</TableHead>
                <TableHead>Action</TableHead>
                <TableHead>Details</TableHead>
                <TableHead>IP Address</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredLogs.map((log) => (
                <TableRow key={log.id}>
                  <TableCell className="text-sm">
                    {format(new Date(log.created_at), 'MMM dd, HH:mm:ss')}
                  </TableCell>
                  <TableCell>
                    <div>
                      <div className="font-medium text-sm">
                        {log.user?.username || 'Unknown'}
                      </div>
                      <div className="text-xs text-muted-foreground">
                        {log.user?.email}
                      </div>
                    </div>
                  </TableCell>
                  <TableCell>
                    <Badge variant={ACTION_TYPES[log.action as keyof typeof ACTION_TYPES]?.color as any || 'outline'}>
                      {ACTION_TYPES[log.action as keyof typeof ACTION_TYPES]?.label || log.action}
                    </Badge>
                  </TableCell>
                  <TableCell className="max-w-[200px]">
                    <div className="text-sm truncate">
                      {log.details ? JSON.stringify(log.details) : '-'}
                    </div>
                  </TableCell>
                  <TableCell className="text-sm text-muted-foreground">
                    {log.ip_address || 'N/A'}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>

        {filteredLogs.length === 0 && (
          <div className="text-center py-8 text-muted-foreground">
            No activity logs found
          </div>
        )}
      </CardContent>
    </Card>
  );
}