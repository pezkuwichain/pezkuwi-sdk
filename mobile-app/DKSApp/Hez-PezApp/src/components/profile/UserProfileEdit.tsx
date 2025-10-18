import React, { useState } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { 
  User, 
  Mail, 
  Wallet, 
  Shield, 
  Save, 
  Camera,
  Award,
  Link,
  Globe
} from 'lucide-react';
import { useWallet } from '@/contexts/WalletContext';

export const UserProfileEdit: React.FC = () => {
  const { user, profile, updateProfile } = useAuth();
  const { address, connectWallet } = useWallet();
  const [loading, setLoading] = useState(false);
  const [success, setSuccess] = useState('');
  const [error, setError] = useState('');
  
  const [formData, setFormData] = useState({
    username: profile?.username || '',
    full_name: profile?.full_name || '',
    bio: profile?.bio || '',
    avatar_url: profile?.avatar_url || '',
    wallet_address: profile?.wallet_address || address || ''
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError('');
    setSuccess('');

    const { error } = await updateProfile(formData);
    
    if (error) {
      setError(error.message || 'Failed to update profile');
    } else {
      setSuccess('Profile updated successfully!');
    }
    
    setLoading(false);
  };

  const handleConnectWallet = async () => {
    try {
      await connectWallet();
      if (address) {
        setFormData({ ...formData, wallet_address: address });
      }
    } catch (err: any) {
      setError(err.message || 'Failed to connect wallet');
    }
  };

  const getInitials = (name?: string) => {
    if (!name) return user?.email?.charAt(0).toUpperCase() || 'U';
    return name.split(' ').map(n => n[0]).join('').toUpperCase();
  };

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      <Card>
        <CardHeader>
          <CardTitle>Profile Settings</CardTitle>
          <CardDescription>
            Manage your profile information and preferences
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs defaultValue="profile" className="w-full">
            <TabsList className="grid w-full grid-cols-3">
              <TabsTrigger value="profile">Profile</TabsTrigger>
              <TabsTrigger value="wallet">Wallet</TabsTrigger>
              <TabsTrigger value="permissions">Permissions</TabsTrigger>
            </TabsList>

            <TabsContent value="profile">
              <form onSubmit={handleSubmit} className="space-y-6">
                <div className="flex items-center space-x-4">
                  <Avatar className="h-20 w-20">
                    <AvatarImage src={formData.avatar_url} />
                    <AvatarFallback>{getInitials(formData.full_name)}</AvatarFallback>
                  </Avatar>
                  <div className="space-y-2">
                    <Label htmlFor="avatar_url">Avatar URL</Label>
                    <div className="flex space-x-2">
                      <Input
                        id="avatar_url"
                        placeholder="https://example.com/avatar.jpg"
                        value={formData.avatar_url}
                        onChange={(e) => setFormData({ ...formData, avatar_url: e.target.value })}
                      />
                      <Button type="button" variant="outline" size="icon">
                        <Camera className="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="username">Username</Label>
                    <div className="relative">
                      <User className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                      <Input
                        id="username"
                        className="pl-10"
                        value={formData.username}
                        onChange={(e) => setFormData({ ...formData, username: e.target.value })}
                      />
                    </div>
                  </div>

                  <div className="space-y-2">
                    <Label htmlFor="full_name">Full Name</Label>
                    <Input
                      id="full_name"
                      value={formData.full_name}
                      onChange={(e) => setFormData({ ...formData, full_name: e.target.value })}
                    />
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="email">Email</Label>
                  <div className="relative">
                    <Mail className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                    <Input
                      id="email"
                      type="email"
                      className="pl-10"
                      value={user?.email || ''}
                      disabled
                    />
                  </div>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="bio">Bio</Label>
                  <Textarea
                    id="bio"
                    placeholder="Tell us about yourself..."
                    rows={4}
                    value={formData.bio}
                    onChange={(e) => setFormData({ ...formData, bio: e.target.value })}
                  />
                </div>

                {success && (
                  <Alert>
                    <AlertDescription>{success}</AlertDescription>
                  </Alert>
                )}

                {error && (
                  <Alert variant="destructive">
                    <AlertDescription>{error}</AlertDescription>
                  </Alert>
                )}

                <Button type="submit" disabled={loading}>
                  <Save className="mr-2 h-4 w-4" />
                  Save Changes
                </Button>
              </form>
            </TabsContent>

            <TabsContent value="wallet">
              <div className="space-y-6">
                <div className="space-y-2">
                  <Label>Connected Wallet</Label>
                  <div className="flex items-center space-x-2">
                    <div className="relative flex-1">
                      <Wallet className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
                      <Input
                        className="pl-10"
                        value={formData.wallet_address || 'No wallet connected'}
                        disabled
                      />
                    </div>
                    <Button onClick={handleConnectWallet} variant="outline">
                      <Link className="mr-2 h-4 w-4" />
                      {formData.wallet_address ? 'Change' : 'Connect'}
                    </Button>
                  </div>
                </div>

                <Alert>
                  <Globe className="h-4 w-4" />
                  <AlertDescription>
                    Connecting your wallet enables on-chain governance participation and token management.
                  </AlertDescription>
                </Alert>
              </div>
            </TabsContent>

            <TabsContent value="permissions">
              <div className="space-y-6">
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="font-medium">Current Role</p>
                      <p className="text-sm text-muted-foreground">Your role determines your governance permissions</p>
                    </div>
                    <Badge variant={profile?.role === 'admin' ? 'destructive' : 'default'}>
                      {profile?.role || 'member'}
                    </Badge>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <p className="font-medium">Reputation Score</p>
                      <p className="text-sm text-muted-foreground">Earned through participation and contributions</p>
                    </div>
                    <div className="flex items-center gap-2">
                      <Award className="h-4 w-4 text-yellow-500" />
                      <span className="font-semibold">{profile?.reputation_score || 0}</span>
                    </div>
                  </div>
                </div>

                <div className="border rounded-lg p-4 space-y-3">
                  <div className="flex items-center gap-2">
                    <Shield className="h-4 w-4" />
                    <p className="font-medium">Your Permissions</p>
                  </div>
                  <div className="grid grid-cols-2 gap-2">
                    {['vote', 'create_proposal', 'delegate', 'moderate', 'treasury_access'].map((perm) => (
                      <div key={perm} className="flex items-center gap-2">
                        <div className={`h-2 w-2 rounded-full ${
                          profile?.role === 'admin' || 
                          (profile?.role === 'moderator' && ['vote', 'create_proposal', 'delegate', 'moderate'].includes(perm)) ||
                          (profile?.role === 'delegate' && ['vote', 'create_proposal', 'delegate'].includes(perm)) ||
                          (profile?.role === 'member' && perm === 'vote')
                            ? 'bg-green-500' : 'bg-gray-300'
                        }`} />
                        <span className="text-sm capitalize">{perm.replace('_', ' ')}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </TabsContent>
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
};