import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Badge } from '@/components/ui/badge';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Progress } from '@/components/ui/progress';
import { UserCheck, Award, TrendingUp, Users, Edit, Share2, Settings } from 'lucide-react';
import ProfileCustomization from './ProfileCustomization';
import ActivityFeed from './ActivityFeed';
import FollowingList from './FollowingList';

interface UserProfileProps {
  userId?: string;
  isOwnProfile?: boolean;
}

const UserProfile: React.FC<UserProfileProps> = ({ userId, isOwnProfile = false }) => {
  const { t } = useTranslation();
  const [isEditing, setIsEditing] = useState(false);
  const [isFollowing, setIsFollowing] = useState(false);

  // Mock user data
  const userData = {
    name: 'Şêrko Bêkes',
    username: '@sherko_bekes',
    avatar: 'https://images.unsplash.com/photo-1633332755192-727a05c4013d',
    bio: 'Governance participant and community advocate for decentralized decision-making',
    reputation: 8750,
    level: 'Gold',
    joinDate: '2024-01-15',
    location: 'Kurdistan',
    followers: 1234,
    following: 567,
    proposals: 42,
    votes: 328,
    delegatedPower: 15000,
    badges: [
      { id: 1, name: 'Early Adopter', icon: '🌟', color: 'bg-yellow-500' },
      { id: 2, name: 'Proposal Master', icon: '📝', color: 'bg-blue-500' },
      { id: 3, name: 'Community Leader', icon: '👥', color: 'bg-green-500' },
      { id: 4, name: 'Verified Identity', icon: '✓', color: 'bg-purple-500' }
    ],
    stats: {
      proposalsCreated: 42,
      votescast: 328,
      delegationsReceived: 89,
      successRate: 78
    }
  };

  return (
    <div className="container mx-auto px-4 py-8 max-w-7xl">
      {/* Profile Header */}
      <Card className="mb-6 border-green-200">
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row items-start gap-6">
            <Avatar className="w-32 h-32 border-4 border-green-500">
              <AvatarImage src={userData.avatar} />
              <AvatarFallback className="text-2xl bg-gradient-to-br from-green-500 to-yellow-500">
                {userData.name.split(' ').map(n => n[0]).join('')}
              </AvatarFallback>
            </Avatar>
            
            <div className="flex-1">
              <div className="flex items-start justify-between mb-4">
                <div>
                  <h1 className="text-3xl font-bold flex items-center gap-2">
                    {userData.name}
                    <UserCheck className="w-6 h-6 text-green-500" />
                  </h1>
                  <p className="text-gray-600">{userData.username}</p>
                </div>
                
                <div className="flex gap-2">
                  {isOwnProfile ? (
                    <>
                      <Button 
                        variant="outline" 
                        size="sm"
                        onClick={() => setIsEditing(true)}
                      >
                        <Edit className="w-4 h-4 mr-2" />
                        {t('profile.edit')}
                      </Button>
                      <Button variant="outline" size="sm">
                        <Settings className="w-4 h-4 mr-2" />
                        {t('profile.settings')}
                      </Button>
                    </>
                  ) : (
                    <>
                      <Button 
                        variant={isFollowing ? "outline" : "default"}
                        size="sm"
                        onClick={() => setIsFollowing(!isFollowing)}
                        className={isFollowing ? "" : "bg-green-600 hover:bg-green-700"}
                      >
                        {isFollowing ? t('profile.unfollow') : t('profile.follow')}
                      </Button>
                      <Button variant="outline" size="sm">
                        <Share2 className="w-4 h-4 mr-2" />
                        {t('profile.share')}
                      </Button>
                    </>
                  )}
                </div>
              </div>
              
              <p className="text-gray-700 mb-4">{userData.bio}</p>
              
              <div className="flex flex-wrap gap-4 text-sm">
                <div className="flex items-center gap-2">
                  <TrendingUp className="w-4 h-4 text-green-600" />
                  <span className="font-semibold">{userData.reputation}</span>
                  <span className="text-gray-600">{t('profile.reputation')}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Award className="w-4 h-4 text-yellow-600" />
                  <span className="font-semibold">{userData.level}</span>
                  <span className="text-gray-600">{t('profile.level')}</span>
                </div>
                <div className="flex items-center gap-2">
                  <Users className="w-4 h-4 text-blue-600" />
                  <span className="font-semibold">{userData.followers}</span>
                  <span className="text-gray-600">{t('profile.followers')}</span>
                </div>
              </div>
              
              {/* Badges */}
              <div className="flex flex-wrap gap-2 mt-4">
                {userData.badges.map(badge => (
                  <Badge key={badge.id} className={`${badge.color} text-white`}>
                    <span className="mr-1">{badge.icon}</span>
                    {badge.name}
                  </Badge>
                ))}
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <Card className="border-green-200">
          <CardContent className="p-4">
            <div className="text-2xl font-bold text-green-600">{userData.stats.proposalsCreated}</div>
            <div className="text-sm text-gray-600">{t('profile.proposalsCreated')}</div>
          </CardContent>
        </Card>
        <Card className="border-yellow-200">
          <CardContent className="p-4">
            <div className="text-2xl font-bold text-yellow-600">{userData.stats.votescast}</div>
            <div className="text-sm text-gray-600">{t('profile.votesCast')}</div>
          </CardContent>
        </Card>
        <Card className="border-red-200">
          <CardContent className="p-4">
            <div className="text-2xl font-bold text-red-600">{userData.stats.delegationsReceived}</div>
            <div className="text-sm text-gray-600">{t('profile.delegationsReceived')}</div>
          </CardContent>
        </Card>
        <Card className="border-blue-200">
          <CardContent className="p-4">
            <div className="text-2xl font-bold text-blue-600">{userData.stats.successRate}%</div>
            <div className="text-sm text-gray-600">{t('profile.successRate')}</div>
            <Progress value={userData.stats.successRate} className="mt-2" />
          </CardContent>
        </Card>
      </div>

      {/* Tabs */}
      <Tabs defaultValue="activity" className="space-y-4">
        <TabsList className="grid w-full grid-cols-3 bg-green-50">
          <TabsTrigger value="activity">{t('profile.activity')}</TabsTrigger>
          <TabsTrigger value="following">{t('profile.following')}</TabsTrigger>
          <TabsTrigger value="achievements">{t('profile.achievements')}</TabsTrigger>
        </TabsList>
        
        <TabsContent value="activity">
          <ActivityFeed userId={userId} />
        </TabsContent>
        
        <TabsContent value="following">
          <FollowingList userId={userId} />
        </TabsContent>
        
        <TabsContent value="achievements">
          <Card className="border-green-200">
            <CardHeader>
              <CardTitle>{t('profile.achievementsTitle')}</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {userData.badges.map(badge => (
                  <div key={badge.id} className="text-center p-4 border rounded-lg">
                    <div className="text-4xl mb-2">{badge.icon}</div>
                    <div className="font-semibold">{badge.name}</div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>

      {/* Profile Customization Modal */}
      {isEditing && (
        <ProfileCustomization 
          isOpen={isEditing} 
          onClose={() => setIsEditing(false)}
          userData={userData}
        />
      )}
    </div>
  );
};

export default UserProfile;