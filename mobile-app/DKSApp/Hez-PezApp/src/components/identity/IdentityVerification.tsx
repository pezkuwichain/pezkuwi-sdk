import React, { useState } from 'react';
import { Shield, CheckCircle, AlertCircle, Lock, Eye, EyeOff } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Switch } from '@/components/ui/switch';
import { useIdentity } from '@/contexts/IdentityContext';
import { KYCData } from '@/lib/identity';

export function IdentityVerification() {
  const { profile, isVerifying, startKYC, updatePrivacySettings } = useIdentity();
  const [kycData, setKycData] = useState<KYCData>({});
  const [showForm, setShowForm] = useState(false);
  const [useZK, setUseZK] = useState(true);

  const handleSubmitKYC = async () => {
    await startKYC(kycData);
    setShowForm(false);
    setKycData({});
  };

  const getVerificationColor = () => {
    switch (profile?.verificationLevel) {
      case 'verified': return 'text-green-500';
      case 'advanced': return 'text-purple-500';
      case 'basic': return 'text-blue-500';
      default: return 'text-gray-500';
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Shield className="w-5 h-5" />
          Identity Verification
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Verification Status */}
        <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
          <div className="flex items-center gap-3">
            {profile?.verificationLevel === 'verified' ? (
              <CheckCircle className="w-6 h-6 text-green-500" />
            ) : (
              <AlertCircle className="w-6 h-6 text-yellow-500" />
            )}
            <div>
              <p className="font-medium">Verification Status</p>
              <p className={`text-sm ${getVerificationColor()}`}>
                {profile?.verificationLevel === 'none' ? 'Unverified' : 
                 profile?.verificationLevel?.charAt(0).toUpperCase() + profile?.verificationLevel?.slice(1)}
              </p>
            </div>
          </div>
          {profile?.verificationLevel !== 'verified' && (
            <Button onClick={() => setShowForm(!showForm)} disabled={isVerifying}>
              {isVerifying ? 'Verifying...' : 'Start Verification'}
            </Button>
          )}
        </div>

        {/* KYC Form */}
        {showForm && (
          <div className="space-y-4 p-4 border rounded-lg">
            <div className="flex items-center justify-between mb-4">
              <h3 className="font-semibold">Verification Form</h3>
              <div className="flex items-center gap-2">
                <Lock className="w-4 h-4" />
                <span className="text-sm">Privacy Mode</span>
                <Switch checked={useZK} onCheckedChange={setUseZK} />
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label>First Name</Label>
                <Input 
                  placeholder={useZK ? "Hidden with ZK Proof" : "John"}
                  value={kycData.firstName || ''}
                  onChange={(e) => setKycData({...kycData, firstName: e.target.value})}
                  disabled={useZK}
                />
              </div>
              <div>
                <Label>Last Name</Label>
                <Input 
                  placeholder={useZK ? "Hidden with ZK Proof" : "Doe"}
                  value={kycData.lastName || ''}
                  onChange={(e) => setKycData({...kycData, lastName: e.target.value})}
                  disabled={useZK}
                />
              </div>
            </div>

            <div>
              <Label>Email</Label>
              <Input 
                type="email"
                placeholder="email@example.com"
                value={kycData.email || ''}
                onChange={(e) => setKycData({...kycData, email: e.target.value})}
              />
            </div>

            <div>
              <Label>Country</Label>
              <Select 
                value={kycData.country || ''}
                onValueChange={(value) => setKycData({...kycData, country: value})}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select country" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="US">United States</SelectItem>
                  <SelectItem value="UK">United Kingdom</SelectItem>
                  <SelectItem value="CA">Canada</SelectItem>
                  <SelectItem value="AU">Australia</SelectItem>
                  <SelectItem value="DE">Germany</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div>
              <Label>Document Type</Label>
              <Select 
                value={kycData.documentType || ''}
                onValueChange={(value: any) => setKycData({...kycData, documentType: value})}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select document" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="passport">Passport</SelectItem>
                  <SelectItem value="driver_license">Driver's License</SelectItem>
                  <SelectItem value="national_id">National ID</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {useZK && (
              <div className="p-3 bg-blue-50 rounded-lg">
                <p className="text-sm text-blue-700">
                  🔐 Zero-Knowledge Proof enabled. Your personal data will be encrypted and only verification status will be stored on-chain.
                </p>
              </div>
            )}

            <div className="flex gap-2">
              <Button onClick={handleSubmitKYC} className="flex-1" disabled={isVerifying}>
                {isVerifying ? 'Verifying...' : 'Submit Verification'}
              </Button>
              <Button variant="outline" onClick={() => setShowForm(false)}>
                Cancel
              </Button>
            </div>
          </div>
        )}

        {/* Privacy Settings */}
        <div className="space-y-3">
          <h3 className="font-semibold">Privacy Settings</h3>
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <Label className="flex items-center gap-2">
                <Eye className="w-4 h-4" />
                Show Real Name
              </Label>
              <Switch 
                checked={profile?.privacySettings.showRealName}
                onCheckedChange={(checked) => updatePrivacySettings({showRealName: checked})}
              />
            </div>
            <div className="flex items-center justify-between">
              <Label className="flex items-center gap-2">
                <Eye className="w-4 h-4" />
                Show Email
              </Label>
              <Switch 
                checked={profile?.privacySettings.showEmail}
                onCheckedChange={(checked) => updatePrivacySettings({showEmail: checked})}
              />
            </div>
            <div className="flex items-center justify-between">
              <Label className="flex items-center gap-2">
                <Eye className="w-4 h-4" />
                Show Country
              </Label>
              <Switch 
                checked={profile?.privacySettings.showCountry}
                onCheckedChange={(checked) => updatePrivacySettings({showCountry: checked})}
              />
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}