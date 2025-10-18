import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { UserCheck, TrendingUp, Users } from 'lucide-react';

interface FollowingListProps {
  userId?: string;
}

const FollowingList: React.FC<FollowingListProps> = ({ userId }) => {
  const { t } = useTranslation();
  const [selectedTab, setSelectedTab] = useState('following');

  const following = [
    {
      id: 1,
      name: 'Leyla Zana',
      username: '@leyla_zana',
      avatar: 'https://images.unsplash.com/photo-1494790108377-be9c29b29330',
      reputation: 12500,
      isVerified: true,
      badges: 3,
      followers: 2341
    },
    {
      id: 2,
      name: 'Mazlum Doğan',
      username: '@mazlum_dogan',
      avatar: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d',
      reputation: 9800,
      isVerified: true,
      badges: 5,
      followers: 1876
    },
    {
      id: 3,
      name: 'Sakine Cansız',
      username: '@sakine_cansiz',
      avatar: 'https://images.unsplash.com/photo-1438761681033-6461ffad8d80',
      reputation: 11200,
      isVerified: true,
      badges: 4,
      followers: 3102
    }
  ];

  const followers = [
    {
      id: 4,
      name: 'Ahmet Kaya',
      username: '@ahmet_kaya',
      avatar: 'https://images.unsplash.com/photo-1500648767791-00dcc994a43e',
      reputation: 7600,
      isVerified: false,
      badges: 2,
      followers: 892
    },
    {
      id: 5,
      name: 'Ciwan Haco',
      username: '@ciwan_haco',
      avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e',
      reputation: 8900,
      isVerified: true,
      badges: 3,
      followers: 1543
    }
  ];

  const UserCard = ({ user }: { user: any }) => (
    <div className="flex items-center justify-between p-4 border rounded-lg hover:bg-gray-50 transition-colors">
      <div className="flex items-center gap-3">
        <Avatar className="w-12 h-12">
          <AvatarImage src={user.avatar} />
          <AvatarFallback className="bg-gradient-to-br from-green-500 to-yellow-500">
            {user.name.split(' ').map((n: string) => n[0]).join('')}
          </AvatarFallback>
        </Avatar>
        <div>
          <div className="flex items-center gap-2">
            <span className="font-semibold">{user.name}</span>
            {user.isVerified && <UserCheck className="w-4 h-4 text-green-500" />}
          </div>
          <div className="text-sm text-gray-600">{user.username}</div>
          <div className="flex items-center gap-3 mt-1 text-xs text-gray-500">
            <span className="flex items-center gap-1">
              <TrendingUp className="w-3 h-3" />
              {user.reputation}
            </span>
            <span className="flex items-center gap-1">
              <Users className="w-3 h-3" />
              {user.followers}
            </span>
            {user.badges > 0 && (
              <Badge variant="secondary" className="text-xs">
                {user.badges} badges
              </Badge>
            )}
          </div>
        </div>
      </div>
      <Button size="sm" variant="outline">
        {t('profile.viewProfile')}
      </Button>
    </div>
  );

  return (
    <Card className="border-green-200">
      <CardHeader>
        <CardTitle>{t('profile.connections')}</CardTitle>
      </CardHeader>
      <CardContent>
        <Tabs value={selectedTab} onValueChange={setSelectedTab}>
          <TabsList className="grid w-full grid-cols-2 bg-green-50">
            <TabsTrigger value="following">
              {t('profile.following')} ({following.length})
            </TabsTrigger>
            <TabsTrigger value="followers">
              {t('profile.followers')} ({followers.length})
            </TabsTrigger>
          </TabsList>
          
          <TabsContent value="following" className="space-y-3 mt-4">
            {following.map(user => (
              <UserCard key={user.id} user={user} />
            ))}
          </TabsContent>
          
          <TabsContent value="followers" className="space-y-3 mt-4">
            {followers.map(user => (
              <UserCard key={user.id} user={user} />
            ))}
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
};

export default FollowingList;