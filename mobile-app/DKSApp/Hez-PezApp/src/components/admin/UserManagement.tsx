import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Search, Filter, MoreVertical, Shield, Ban, Edit, Eye } from 'lucide-react';
import { supabase } from '@/lib/supabase';
import { useToast } from '@/hooks/use-toast';

interface User {
  id: string;
  email: string;
  username: string;
  full_name: string;
  created_at: string;
  last_sign_in_at: string;
  permissions: string[];
  status: 'active' | 'suspended' | 'banned';
}

export default function UserManagement() {
  const [users, setUsers] = useState<User[]>([]);
  const [filteredUsers, setFilteredUsers] = useState<User[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [loading, setLoading] = useState(true);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const { toast } = useToast();

  useEffect(() => {
    fetchUsers();
  }, []);

  useEffect(() => {
    filterUsers();
  }, [searchQuery, statusFilter, users]);

  const fetchUsers = async () => {
    try {
      const { data: profiles } = await supabase
        .from('profiles')
        .select('*')
        .order('created_at', { ascending: false });

      const { data: permissions } = await supabase
        .from('governance_permissions')
        .select('user_id, permission_type');

      const usersWithPermissions = profiles?.map(profile => {
        const userPerms = permissions?.filter(p => p.user_id === profile.id)
          .map(p => p.permission_type) || [];
        
        return {
          ...profile,
          permissions: userPerms,
          status: 'active' as const,
          email: profile.email || 'N/A',
          last_sign_in_at: profile.updated_at
        };
      }) || [];

      setUsers(usersWithPermissions);
      setFilteredUsers(usersWithPermissions);
    } catch (error) {
      console.error('Error fetching users:', error);
      toast({
        title: 'Error',
        description: 'Failed to fetch users',
        variant: 'destructive'
      });
    } finally {
      setLoading(false);
    }
  };

  const filterUsers = () => {
    let filtered = [...users];

    if (searchQuery) {
      filtered = filtered.filter(user =>
        user.username?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        user.email?.toLowerCase().includes(searchQuery.toLowerCase()) ||
        user.full_name?.toLowerCase().includes(searchQuery.toLowerCase())
      );
    }

    if (statusFilter !== 'all') {
      filtered = filtered.filter(user => user.status === statusFilter);
    }

    setFilteredUsers(filtered);
  };

  const handleSuspendUser = async (userId: string) => {
    try {
      // Log the action
      await supabase.from('activity_logs').insert({
        user_id: userId,
        action: 'user_suspended',
        details: { suspended_by: 'admin' }
      });

      toast({
        title: 'User Suspended',
        description: 'User has been suspended successfully'
      });

      fetchUsers();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to suspend user',
        variant: 'destructive'
      });
    }
  };

  const handleGrantPermission = async (userId: string, permission: string) => {
    try {
      await supabase.from('governance_permissions').insert({
        user_id: userId,
        permission_type: permission,
        granted_by: (await supabase.auth.getUser()).data.user?.id
      });

      toast({
        title: 'Permission Granted',
        description: `${permission} permission granted successfully`
      });

      fetchUsers();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to grant permission',
        variant: 'destructive'
      });
    }
  };

  const handleRevokePermission = async (userId: string, permission: string) => {
    try {
      await supabase
        .from('governance_permissions')
        .delete()
        .eq('user_id', userId)
        .eq('permission_type', permission);

      toast({
        title: 'Permission Revoked',
        description: `${permission} permission revoked successfully`
      });

      fetchUsers();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to revoke permission',
        variant: 'destructive'
      });
    }
  };

  if (loading) {
    return <div className="flex justify-center p-8">Loading users...</div>;
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>User Management</CardTitle>
        <div className="flex gap-4 mt-4">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-muted-foreground h-4 w-4" />
            <Input
              placeholder="Search users..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-10"
            />
          </div>
          <Select value={statusFilter} onValueChange={setStatusFilter}>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Filter by status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Users</SelectItem>
              <SelectItem value="active">Active</SelectItem>
              <SelectItem value="suspended">Suspended</SelectItem>
              <SelectItem value="banned">Banned</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>User</TableHead>
              <TableHead>Email</TableHead>
              <TableHead>Permissions</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Joined</TableHead>
              <TableHead>Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {filteredUsers.map((user) => (
              <TableRow key={user.id}>
                <TableCell className="font-medium">
                  {user.username || user.full_name || 'Anonymous'}
                </TableCell>
                <TableCell>{user.email}</TableCell>
                <TableCell>
                  <div className="flex gap-1 flex-wrap">
                    {user.permissions.map(perm => (
                      <Badge key={perm} variant="secondary">{perm}</Badge>
                    ))}
                    {user.permissions.length === 0 && (
                      <span className="text-muted-foreground">None</span>
                    )}
                  </div>
                </TableCell>
                <TableCell>
                  <Badge variant={user.status === 'active' ? 'default' : 'destructive'}>
                    {user.status}
                  </Badge>
                </TableCell>
                <TableCell>
                  {new Date(user.created_at).toLocaleDateString()}
                </TableCell>
                <TableCell>
                  <Dialog>
                    <DialogTrigger asChild>
                      <Button variant="ghost" size="sm" onClick={() => setSelectedUser(user)}>
                        <MoreVertical className="h-4 w-4" />
                      </Button>
                    </DialogTrigger>
                    <DialogContent>
                      <DialogHeader>
                        <DialogTitle>Manage User</DialogTitle>
                        <DialogDescription>
                          {user.username || user.full_name || 'Anonymous User'}
                        </DialogDescription>
                      </DialogHeader>
                      <div className="space-y-4">
                        <div>
                          <h4 className="text-sm font-medium mb-2">Permissions</h4>
                          <div className="space-y-2">
                            {['admin', 'moderator', 'validator', 'proposer'].map(perm => (
                              <div key={perm} className="flex items-center justify-between">
                                <span className="capitalize">{perm}</span>
                                {user.permissions.includes(perm) ? (
                                  <Button
                                    size="sm"
                                    variant="destructive"
                                    onClick={() => handleRevokePermission(user.id, perm)}
                                  >
                                    Revoke
                                  </Button>
                                ) : (
                                  <Button
                                    size="sm"
                                    onClick={() => handleGrantPermission(user.id, perm)}
                                  >
                                    Grant
                                  </Button>
                                )}
                              </div>
                            ))}
                          </div>
                        </div>
                        <div className="flex gap-2">
                          <Button
                            variant="destructive"
                            onClick={() => handleSuspendUser(user.id)}
                            disabled={user.status === 'suspended'}
                          >
                            <Ban className="h-4 w-4 mr-2" />
                            Suspend User
                          </Button>
                        </div>
                      </div>
                    </DialogContent>
                  </Dialog>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}