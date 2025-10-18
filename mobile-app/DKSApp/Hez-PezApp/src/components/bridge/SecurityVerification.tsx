import React, { useState } from 'react';
import { Shield, AlertTriangle, CheckCircle, Info, Lock, Key } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

interface SecurityCheck {
  name: string;
  status: 'passed' | 'warning' | 'failed';
  description: string;
}

export function SecurityVerification() {
  const [verifying, setVerifying] = useState(false);
  const [verificationProgress, setVerificationProgress] = useState(0);
  const [securityChecks, setSecurityChecks] = useState<SecurityCheck[]>([]);

  const runSecurityVerification = async () => {
    setVerifying(true);
    setVerificationProgress(0);
    setSecurityChecks([]);

    const checks: SecurityCheck[] = [
      {
        name: 'Smart Contract Audit',
        status: 'passed',
        description: 'Bridge contracts audited by CertiK'
      },
      {
        name: 'Liquidity Check',
        status: 'passed',
        description: 'Sufficient liquidity available on destination chain'
      },
      {
        name: 'Rate Limit Check',
        status: 'warning',
        description: 'Approaching daily transfer limit (80% used)'
      },
      {
        name: 'Address Verification',
        status: 'passed',
        description: 'Destination address format validated'
      },
      {
        name: 'Network Status',
        status: 'passed',
        description: 'All networks operating normally'
      },
      {
        name: 'Fee Estimation',
        status: 'passed',
        description: 'Gas fees within normal range'
      }
    ];

    // Simulate progressive verification
    for (let i = 0; i < checks.length; i++) {
      await new Promise(resolve => setTimeout(resolve, 500));
      setSecurityChecks(prev => [...prev, checks[i]]);
      setVerificationProgress((i + 1) / checks.length * 100);
    }

    setVerifying(false);
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'passed':
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case 'warning':
        return <AlertTriangle className="h-4 w-4 text-yellow-500" />;
      case 'failed':
        return <AlertTriangle className="h-4 w-4 text-red-500" />;
      default:
        return <Info className="h-4 w-4 text-blue-500" />;
    }
  };

  const securityTips = [
    'Always verify the destination address before bridging',
    'Start with small test amounts for new routes',
    'Check network congestion before large transfers',
    'Keep your private keys secure and never share them',
    'Use hardware wallets for large value transfers',
    'Monitor bridge announcements for maintenance windows'
  ];

  const auditReports = [
    { auditor: 'CertiK', date: '2024-10-01', score: 95, status: 'Passed' },
    { auditor: 'Quantstamp', date: '2024-09-15', score: 92, status: 'Passed' },
    { auditor: 'Trail of Bits', date: '2024-08-20', score: 94, status: 'Passed' }
  ];

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Shield className="h-5 w-5" />
          Security & Verification
        </CardTitle>
      </CardHeader>
      <CardContent>
        <Tabs defaultValue="verification" className="space-y-4">
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="verification">Verification</TabsTrigger>
            <TabsTrigger value="audits">Audits</TabsTrigger>
            <TabsTrigger value="tips">Security Tips</TabsTrigger>
          </TabsList>

          <TabsContent value="verification" className="space-y-4">
            <Alert>
              <Lock className="h-4 w-4" />
              <AlertTitle>Bridge Security</AlertTitle>
              <AlertDescription>
                Run security verification before bridging to ensure safe transfer
              </AlertDescription>
            </Alert>

            <Button 
              onClick={runSecurityVerification} 
              disabled={verifying}
              className="w-full"
            >
              {verifying ? 'Verifying...' : 'Run Security Check'}
            </Button>

            {verifying && (
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span>Verification Progress</span>
                  <span>{Math.round(verificationProgress)}%</span>
                </div>
                <Progress value={verificationProgress} />
              </div>
            )}

            {securityChecks.length > 0 && (
              <div className="space-y-2">
                {securityChecks.map((check, index) => (
                  <div
                    key={index}
                    className="flex items-start gap-3 p-3 border rounded-lg"
                  >
                    {getStatusIcon(check.status)}
                    <div className="flex-1">
                      <div className="font-medium">{check.name}</div>
                      <div className="text-sm text-muted-foreground">
                        {check.description}
                      </div>
                    </div>
                    <Badge 
                      variant={
                        check.status === 'passed' ? 'default' :
                        check.status === 'warning' ? 'secondary' : 'destructive'
                      }
                    >
                      {check.status}
                    </Badge>
                  </div>
                ))}
              </div>
            )}
          </TabsContent>

          <TabsContent value="audits" className="space-y-4">
            <Alert>
              <Key className="h-4 w-4" />
              <AlertTitle>Smart Contract Audits</AlertTitle>
              <AlertDescription>
                Our bridge contracts are regularly audited by leading security firms
              </AlertDescription>
            </Alert>

            <div className="space-y-3">
              {auditReports.map((audit, index) => (
                <div key={index} className="p-4 border rounded-lg">
                  <div className="flex items-center justify-between mb-2">
                    <div className="font-medium">{audit.auditor}</div>
                    <Badge variant="default">{audit.status}</Badge>
                  </div>
                  <div className="flex items-center justify-between text-sm text-muted-foreground">
                    <span>Audit Date: {audit.date}</span>
                    <span>Score: {audit.score}/100</span>
                  </div>
                  <Progress value={audit.score} className="mt-2" />
                </div>
              ))}
            </div>
          </TabsContent>

          <TabsContent value="tips" className="space-y-4">
            <Alert>
              <Info className="h-4 w-4" />
              <AlertTitle>Security Best Practices</AlertTitle>
              <AlertDescription>
                Follow these tips to ensure safe cross-chain transfers
              </AlertDescription>
            </Alert>

            <div className="space-y-2">
              {securityTips.map((tip, index) => (
                <div key={index} className="flex items-start gap-3 p-3 border rounded-lg">
                  <div className="flex items-center justify-center w-6 h-6 rounded-full bg-primary/10 text-primary text-xs font-bold">
                    {index + 1}
                  </div>
                  <div className="text-sm">{tip}</div>
                </div>
              ))}
            </div>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}