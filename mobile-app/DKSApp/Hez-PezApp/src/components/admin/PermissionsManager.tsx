import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { Shield, Users, Key, Lock } from 'lucide-react';
import { supabase } from '@/lib/supabase';
import { useToast } from '@/hooks/use-toast';

interface Permission {
  id: string;
  user_id: string;
  permission_type: string;
  granted_at: string;
  granted_by: string;
  user?: {
    username: string;
    email: string;
  };
}

const PERMISSION_TYPES = {
  admin: {
    label: 'Administrator',
    description: 'Full system access and user management',
    color: 'destructive'
  },
  moderator: {
    label: 'Moderator',
    description: 'Content moderation and proposal review',
    color: 'secondary'
  },
  validator: {
    label: 'Validator',
    description: 'Validate transactions and proposals',
    color: 'default'
  },
  proposer: {
    label: 'Proposer',
    description: 'Create and submit governance proposals',
    color: 'outline'
  },
  voter: {
    label: 'Voter',
    description: 'Participate in governance voting',
    color: 'outline'
  },
  treasury: {
    label: 'Treasury Manager',
    description: 'Manage treasury funds and allocations',
    color: 'secondary'
  }
};

export default function PermissionsManager() {
  const [permissions, setPermissions] = useState<Permission[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedPermType, setSelectedPermType] = useState('all');
  const { toast } = useToast();

  useEffect(() => {
    fetchPermissions();
  }, []);

  const fetchPermissions = async () => {
    try {
      const { data } = await supabase
        .from('governance_permissions')
        .select(`
          *,
          profiles!governance_permissions_user_id_fkey (
            username,
            email
          )
        `)
        .order('granted_at', { ascending: false });

      const formattedPerms = data?.map(perm => ({
        ...perm,
        user: perm.profiles
      })) || [];

      setPermissions(formattedPerms);
    } catch (error) {
      console.error('Error fetching permissions:', error);
      toast({
        title: 'Error',
        description: 'Failed to fetch permissions',
        variant: 'destructive'
      });
    } finally {
      setLoading(false);
    }
  };

  const handleBulkRevoke = async (permissionType: string) => {
    try {
      await supabase
        .from('governance_permissions')
        .delete()
        .eq('permission_type', permissionType);

      toast({
        title: 'Permissions Revoked',
        description: `All ${permissionType} permissions have been revoked`,
      });

      fetchPermissions();
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to revoke permissions',
        variant: 'destructive'
      });
    }
  };

  const getPermissionStats = () => {
    const stats: Record<string, number> = {};
    Object.keys(PERMISSION_TYPES).forEach(type => {
      stats[type] = permissions.filter(p => p.permission_type === type).length;
    });
    return stats;
  };

  const filteredPermissions = selectedPermType === 'all' 
    ? permissions 
    : permissions.filter(p => p.permission_type === selectedPermType);

  const stats = getPermissionStats();

  if (loading) {
    return <div className="flex justify-center p-8">Loading permissions...</div>;
  }

  return (
    <div className="space-y-6">
      {/* Permission Overview Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {Object.entries(PERMISSION_TYPES).map(([key, config]) => (
          <Card key={key}>
            <CardHeader className="pb-3">
              <div className="flex items-center justify-between">
                <CardTitle className="text-base">{config.label}</CardTitle>
                <Badge variant={config.color as any}>{stats[key] || 0}</Badge>
              </div>
              <CardDescription className="text-xs">
                {config.description}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex justify-between items-center">
                <span className="text-sm text-muted-foreground">
                  Active users
                </span>
                <Button
                  size="sm"
                  variant="ghost"
                  onClick={() => setSelectedPermType(key)}
                >
                  View
                </Button>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Permission Management */}
      <Card>
        <CardHeader>
          <CardTitle>Permission Assignments</CardTitle>
          <CardDescription>
            View and manage all permission assignments
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs value={selectedPermType} onValueChange={setSelectedPermType}>
            <TabsList className="grid w-full grid-cols-7">
              <TabsTrigger value="all">All</TabsTrigger>
              {Object.keys(PERMISSION_TYPES).map(type => (
                <TabsTrigger key={type} value={type} className="capitalize">
                  {type}
                </TabsTrigger>
              ))}
            </TabsList>

            <TabsContent value={selectedPermType} className="mt-4">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>User</TableHead>
                    <TableHead>Permission</TableHead>
                    <TableHead>Granted At</TableHead>
                    <TableHead>Granted By</TableHead>
                    <TableHead>Actions</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filteredPermissions.map((perm) => (
                    <TableRow key={perm.id}>
                      <TableCell>
                        <div>
                          <div className="font-medium">
                            {perm.user?.username || 'Unknown User'}
                          </div>
                          <div className="text-sm text-muted-foreground">
                            {perm.user?.email}
                          </div>
                        </div>
                      </TableCell>
                      <TableCell>
                        <Badge variant={PERMISSION_TYPES[perm.permission_type as keyof typeof PERMISSION_TYPES]?.color as any}>
                          {perm.permission_type}
                        </Badge>
                      </TableCell>
                      <TableCell>
                        {new Date(perm.granted_at).toLocaleDateString()}
                      </TableCell>
                      <TableCell>
                        {perm.granted_by ? 'Admin' : 'System'}
                      </TableCell>
                      <TableCell>
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={async () => {
                            await supabase
                              .from('governance_permissions')
                              .delete()
                              .eq('id', perm.id);
                            
                            toast({
                              title: 'Permission Revoked',
                              description: 'Permission has been revoked successfully'
                            });
                            
                            fetchPermissions();
                          }}
                        >
                          Revoke
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>

              {selectedPermType !== 'all' && filteredPermissions.length > 0 && (
                <div className="mt-4 flex justify-end">
                  <Button
                    variant="destructive"
                    onClick={() => handleBulkRevoke(selectedPermType)}
                  >
                    Revoke All {selectedPermType} Permissions
                  </Button>
                </div>
              )}
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}