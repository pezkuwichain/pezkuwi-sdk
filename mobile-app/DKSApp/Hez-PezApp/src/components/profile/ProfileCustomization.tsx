import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import { Label } from '@/components/ui/label';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent } from '@/components/ui/card';
import { Camera, Palette, Shield, Bell } from 'lucide-react';

interface ProfileCustomizationProps {
  isOpen: boolean;
  onClose: () => void;
  userData: any;
}

const ProfileCustomization: React.FC<ProfileCustomizationProps> = ({ 
  isOpen, 
  onClose, 
  userData 
}) => {
  const { t } = useTranslation();
  const [profileData, setProfileData] = useState({
    name: userData.name,
    username: userData.username,
    bio: userData.bio,
    location: userData.location,
    website: '',
    twitter: '',
    github: ''
  });

  const themes = [
    { id: 1, name: 'Kurdish', colors: ['bg-green-500', 'bg-red-500', 'bg-yellow-500'] },
    { id: 2, name: 'Ocean', colors: ['bg-blue-500', 'bg-cyan-500', 'bg-teal-500'] },
    { id: 3, name: 'Sunset', colors: ['bg-orange-500', 'bg-pink-500', 'bg-purple-500'] },
    { id: 4, name: 'Forest', colors: ['bg-green-600', 'bg-emerald-500', 'bg-lime-500'] }
  ];

  const handleSave = () => {
    console.log('Saving profile:', profileData);
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>{t('profile.customizeProfile')}</DialogTitle>
        </DialogHeader>

        <Tabs defaultValue="basic" className="mt-4">
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger value="basic">{t('profile.basic')}</TabsTrigger>
            <TabsTrigger value="theme">{t('profile.theme')}</TabsTrigger>
            <TabsTrigger value="privacy">{t('profile.privacy')}</TabsTrigger>
            <TabsTrigger value="notifications">{t('profile.notifications')}</TabsTrigger>
          </TabsList>

          <TabsContent value="basic" className="space-y-4">
            <div className="flex items-center gap-4">
              <Avatar className="w-20 h-20">
                <AvatarImage src={userData.avatar} />
                <AvatarFallback>
                  {userData.name.split(' ').map((n: string) => n[0]).join('')}
                </AvatarFallback>
              </Avatar>
              <Button variant="outline">
                <Camera className="w-4 h-4 mr-2" />
                {t('profile.changeAvatar')}
              </Button>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label htmlFor="name">{t('profile.name')}</Label>
                <Input
                  id="name"
                  value={profileData.name}
                  onChange={(e) => setProfileData({...profileData, name: e.target.value})}
                />
              </div>
              <div>
                <Label htmlFor="username">{t('profile.username')}</Label>
                <Input
                  id="username"
                  value={profileData.username}
                  onChange={(e) => setProfileData({...profileData, username: e.target.value})}
                />
              </div>
            </div>

            <div>
              <Label htmlFor="bio">{t('profile.bio')}</Label>
              <Textarea
                id="bio"
                value={profileData.bio}
                onChange={(e) => setProfileData({...profileData, bio: e.target.value})}
                rows={3}
              />
            </div>

            <div>
              <Label htmlFor="location">{t('profile.location')}</Label>
              <Input
                id="location"
                value={profileData.location}
                onChange={(e) => setProfileData({...profileData, location: e.target.value})}
              />
            </div>

            <div className="space-y-2">
              <Label>{t('profile.socialLinks')}</Label>
              <Input
                placeholder="Website URL"
                value={profileData.website}
                onChange={(e) => setProfileData({...profileData, website: e.target.value})}
              />
              <Input
                placeholder="Twitter handle"
                value={profileData.twitter}
                onChange={(e) => setProfileData({...profileData, twitter: e.target.value})}
              />
              <Input
                placeholder="GitHub username"
                value={profileData.github}
                onChange={(e) => setProfileData({...profileData, github: e.target.value})}
              />
            </div>
          </TabsContent>

          <TabsContent value="theme" className="space-y-4">
            <div className="flex items-center gap-2 mb-4">
              <Palette className="w-5 h-5 text-green-600" />
              <h3 className="font-semibold">{t('profile.chooseTheme')}</h3>
            </div>
            <div className="grid grid-cols-2 gap-4">
              {themes.map(theme => (
                <Card key={theme.id} className="cursor-pointer hover:border-green-500">
                  <CardContent className="p-4">
                    <div className="font-medium mb-2">{theme.name}</div>
                    <div className="flex gap-2">
                      {theme.colors.map((color, idx) => (
                        <div key={idx} className={`w-8 h-8 rounded ${color}`} />
                      ))}
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          </TabsContent>

          <TabsContent value="privacy" className="space-y-4">
            <div className="flex items-center gap-2 mb-4">
              <Shield className="w-5 h-5 text-green-600" />
              <h3 className="font-semibold">{t('profile.privacySettings')}</h3>
            </div>
            <div className="space-y-3">
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" defaultChecked />
                <span>{t('profile.showEmail')}</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" defaultChecked />
                <span>{t('profile.showActivity')}</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" />
                <span>{t('profile.allowMessages')}</span>
              </label>
            </div>
          </TabsContent>

          <TabsContent value="notifications" className="space-y-4">
            <div className="flex items-center gap-2 mb-4">
              <Bell className="w-5 h-5 text-green-600" />
              <h3 className="font-semibold">{t('profile.notificationSettings')}</h3>
            </div>
            <div className="space-y-3">
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" defaultChecked />
                <span>{t('profile.proposalUpdates')}</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" defaultChecked />
                <span>{t('profile.voteReminders')}</span>
              </label>
              <label className="flex items-center gap-2">
                <input type="checkbox" className="rounded" />
                <span>{t('profile.newFollowers')}</span>
              </label>
            </div>
          </TabsContent>
        </Tabs>

        <div className="flex justify-end gap-2 mt-6">
          <Button variant="outline" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button onClick={handleSave} className="bg-green-600 hover:bg-green-700">
            {t('common.save')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
};

export default ProfileCustomization;