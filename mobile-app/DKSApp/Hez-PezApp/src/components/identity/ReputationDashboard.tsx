import React from 'react';
import { Trophy, Star, TrendingUp, Award, Shield, Users } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { useIdentity } from '@/contexts/IdentityContext';
import { VERIFICATION_LEVELS } from '@/lib/identity';

export function ReputationDashboard() {
  const { profile, refreshReputation } = useIdentity();

  const getNextLevel = () => {
    const currentScore = profile?.reputationScore || 0;
    if (currentScore < 100) return { level: 'basic', required: 100 };
    if (currentScore < 500) return { level: 'advanced', required: 500 };
    if (currentScore < 1000) return { level: 'verified', required: 1000 };
    return null;
  };

  const nextLevel = getNextLevel();
  const progressToNext = nextLevel 
    ? ((profile?.reputationScore || 0) / nextLevel.required) * 100
    : 100;

  return (
    <div className="space-y-6">
      {/* Reputation Score Card */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Trophy className="w-5 h-5" />
            Reputation Score
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-3xl font-bold">{profile?.reputationScore || 0}</p>
                <p className="text-sm text-gray-500">Total Points</p>
              </div>
              <div className="text-right">
                <Badge className={`bg-${VERIFICATION_LEVELS[profile?.verificationLevel || 'none'].color}-100`}>
                  {VERIFICATION_LEVELS[profile?.verificationLevel || 'none'].label}
                </Badge>
              </div>
            </div>

            {nextLevel && (
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span>Progress to {nextLevel.level}</span>
                  <span>{profile?.reputationScore || 0} / {nextLevel.required}</span>
                </div>
                <Progress value={progressToNext} className="h-2" />
              </div>
            )}

            <div className="grid grid-cols-3 gap-2 pt-4 border-t">
              <div className="text-center">
                <Star className="w-5 h-5 mx-auto mb-1 text-yellow-500" />
                <p className="text-xs text-gray-500">Badges</p>
                <p className="font-semibold">{profile?.badges.length || 0}</p>
              </div>
              <div className="text-center">
                <Users className="w-5 h-5 mx-auto mb-1 text-blue-500" />
                <p className="text-xs text-gray-500">Roles</p>
                <p className="font-semibold">{profile?.roles.length || 0}</p>
              </div>
              <div className="text-center">
                <TrendingUp className="w-5 h-5 mx-auto mb-1 text-green-500" />
                <p className="text-xs text-gray-500">Level</p>
                <p className="font-semibold">{profile?.verificationLevel || 'None'}</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Badges Grid */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Award className="w-5 h-5" />
            Earned Badges
          </CardTitle>
        </CardHeader>
        <CardContent>
          {profile?.badges && profile.badges.length > 0 ? (
            <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
              {profile.badges.map((badge) => (
                <div 
                  key={badge.id}
                  className="p-4 border rounded-lg hover:shadow-md transition-shadow"
                >
                  <div className="text-2xl mb-2">{badge.icon}</div>
                  <h4 className="font-semibold text-sm">{badge.name}</h4>
                  <p className="text-xs text-gray-500 mt-1">{badge.description}</p>
                  <Badge 
                    variant="outline" 
                    className="mt-2 text-xs"
                    style={{ borderColor: badge.color, color: badge.color }}
                  >
                    {badge.category}
                  </Badge>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8 text-gray-500">
              <Award className="w-12 h-12 mx-auto mb-2 text-gray-300" />
              <p>No badges earned yet</p>
              <p className="text-sm mt-1">Complete verification to earn your first badges!</p>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Roles & Permissions */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Shield className="w-5 h-5" />
            Roles & Permissions
          </CardTitle>
        </CardHeader>
        <CardContent>
          {profile?.roles && profile.roles.length > 0 ? (
            <div className="space-y-3">
              {profile.roles.map((role) => (
                <div key={role.id} className="p-3 border rounded-lg">
                  <div className="flex items-center justify-between mb-2">
                    <h4 className="font-semibold">{role.name}</h4>
                    <Badge variant="secondary">Active</Badge>
                  </div>
                  <div className="flex flex-wrap gap-1">
                    {role.permissions.map((permission) => (
                      <Badge key={permission} variant="outline" className="text-xs">
                        {permission.replace(/_/g, ' ')}
                      </Badge>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center py-8 text-gray-500">
              <Shield className="w-12 h-12 mx-auto mb-2 text-gray-300" />
              <p>No roles assigned</p>
              <p className="text-sm mt-1">Verify your identity to unlock governance roles</p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}