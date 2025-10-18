import React from 'react';
import { useTranslation } from 'react-i18next';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ScrollArea } from '@/components/ui/scroll-area';
import { FileText, Vote, Users, Award, MessageSquare, TrendingUp } from 'lucide-react';

interface ActivityFeedProps {
  userId?: string;
}

const ActivityFeed: React.FC<ActivityFeedProps> = ({ userId }) => {
  const { t } = useTranslation();

  const activities = [
    {
      id: 1,
      type: 'proposal',
      icon: FileText,
      title: 'Created proposal: Treasury Allocation for Community Development',
      time: '2 hours ago',
      details: 'Requesting 50,000 PZK for local education initiatives',
      color: 'text-blue-600'
    },
    {
      id: 2,
      type: 'vote',
      icon: Vote,
      title: 'Voted YES on: Network Upgrade Proposal #127',
      time: '5 hours ago',
      details: 'Supporting technical improvements to consensus mechanism',
      color: 'text-green-600'
    },
    {
      id: 3,
      type: 'delegation',
      icon: Users,
      title: 'Received delegation from 12 community members',
      time: '1 day ago',
      details: 'Total delegated power: 5,000 PZK',
      color: 'text-purple-600'
    },
    {
      id: 4,
      type: 'achievement',
      icon: Award,
      title: 'Earned badge: Proposal Master',
      time: '2 days ago',
      details: 'Created 10 successful proposals',
      color: 'text-yellow-600'
    },
    {
      id: 5,
      type: 'comment',
      icon: MessageSquare,
      title: 'Commented on: Environmental Protection Initiative',
      time: '3 days ago',
      details: 'Provided feedback on implementation timeline',
      color: 'text-indigo-600'
    },
    {
      id: 6,
      type: 'reputation',
      icon: TrendingUp,
      title: 'Reputation increased by 250 points',
      time: '4 days ago',
      details: 'Successful proposal implementation',
      color: 'text-red-600'
    }
  ];

  return (
    <Card className="border-green-200">
      <CardContent className="p-6">
        <h3 className="text-lg font-semibold mb-4">{t('profile.recentActivity')}</h3>
        <ScrollArea className="h-[500px] pr-4">
          <div className="space-y-4">
            {activities.map((activity) => {
              const Icon = activity.icon;
              return (
                <div key={activity.id} className="flex gap-4 p-4 border rounded-lg hover:bg-gray-50 transition-colors">
                  <div className={`mt-1 ${activity.color}`}>
                    <Icon className="w-5 h-5" />
                  </div>
                  <div className="flex-1">
                    <div className="font-medium">{activity.title}</div>
                    <div className="text-sm text-gray-600 mt-1">{activity.details}</div>
                    <div className="text-xs text-gray-500 mt-2">{activity.time}</div>
                  </div>
                  {activity.type === 'achievement' && (
                    <Badge className="bg-yellow-100 text-yellow-800">New Badge</Badge>
                  )}
                  {activity.type === 'reputation' && (
                    <Badge className="bg-green-100 text-green-800">+250</Badge>
                  )}
                </div>
              );
            })}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
};

export default ActivityFeed;