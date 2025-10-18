import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { FileText, Download, CheckCircle, AlertTriangle, Shield, Clock } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';

interface RecoveryStep {
  id: number;
  title: string;
  description: string;
  estimatedTime: string;
  priority: 'critical' | 'high' | 'medium' | 'low';
  status: 'pending' | 'in_progress' | 'completed';
}

export function DisasterRecoveryPlan() {
  const [executingPlan, setExecutingPlan] = useState(false);
  const [currentStep, setCurrentStep] = useState(0);
  const { toast } = useToast();

  const recoverySteps: RecoveryStep[] = [
    {
      id: 1,
      title: 'Assess System Status',
      description: 'Evaluate the extent of system failure and identify affected components',
      estimatedTime: '5-10 minutes',
      priority: 'critical',
      status: 'pending'
    },
    {
      id: 2,
      title: 'Activate Backup Systems',
      description: 'Switch to backup infrastructure and verify connectivity',
      estimatedTime: '10-15 minutes',
      priority: 'critical',
      status: 'pending'
    },
    {
      id: 3,
      title: 'Restore Database',
      description: 'Restore database from most recent backup point',
      estimatedTime: '30-60 minutes',
      priority: 'high',
      status: 'pending'
    },
    {
      id: 4,
      title: 'Verify Data Integrity',
      description: 'Run integrity checks on restored data',
      estimatedTime: '15-20 minutes',
      priority: 'high',
      status: 'pending'
    },
    {
      id: 5,
      title: 'Restore Application Services',
      description: 'Bring application services back online',
      estimatedTime: '20-30 minutes',
      priority: 'medium',
      status: 'pending'
    },
    {
      id: 6,
      title: 'System Testing',
      description: 'Perform comprehensive system testing',
      estimatedTime: '30-45 minutes',
      priority: 'medium',
      status: 'pending'
    }
  ];

  const executeDRP = () => {
    setExecutingPlan(true);
    setCurrentStep(1);
    
    toast({
      title: 'Disaster Recovery Initiated',
      description: 'Following the disaster recovery plan procedures',
    });

    // Simulate step progression
    const interval = setInterval(() => {
      setCurrentStep(prev => {
        if (prev >= recoverySteps.length) {
          clearInterval(interval);
          setExecutingPlan(false);
          toast({
            title: 'Recovery Complete',
            description: 'System has been successfully recovered',
          });
          return prev;
        }
        return prev + 1;
      });
    }, 3000);
  };

  const downloadPlan = () => {
    toast({
      title: 'Downloading DRP',
      description: 'Disaster Recovery Plan PDF downloaded',
    });
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'critical': return 'destructive';
      case 'high': return 'default';
      case 'medium': return 'secondary';
      default: return 'outline';
    }
  };

  const progress = (currentStep / recoverySteps.length) * 100;

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Disaster Recovery Plan</CardTitle>
          <CardDescription>
            Comprehensive procedures for system recovery in case of catastrophic failure
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex gap-4 mb-6">
            <Button onClick={executeDRP} disabled={executingPlan}>
              <Shield className="mr-2 h-4 w-4" />
              {executingPlan ? 'Executing Recovery...' : 'Execute DRP'}
            </Button>
            <Button variant="outline" onClick={downloadPlan}>
              <Download className="mr-2 h-4 w-4" />
              Download Plan
            </Button>
          </div>

          {executingPlan && (
            <div className="mb-6 space-y-2">
              <div className="flex justify-between text-sm">
                <span>Recovery Progress</span>
                <span>{Math.round(progress)}%</span>
              </div>
              <Progress value={progress} />
            </div>
          )}

          <div className="space-y-4">
            {recoverySteps.map((step, index) => (
              <div
                key={step.id}
                className={`p-4 border rounded-lg ${
                  executingPlan && index < currentStep
                    ? 'bg-green-50 border-green-200'
                    : executingPlan && index === currentStep
                    ? 'bg-blue-50 border-blue-200'
                    : ''
                }`}
              >
                <div className="flex items-start justify-between">
                  <div className="flex items-start gap-3">
                    <div className="mt-1">
                      {executingPlan && index < currentStep ? (
                        <CheckCircle className="h-5 w-5 text-green-600" />
                      ) : executingPlan && index === currentStep ? (
                        <Clock className="h-5 w-5 text-blue-600 animate-pulse" />
                      ) : (
                        <div className="h-5 w-5 rounded-full border-2 border-gray-300" />
                      )}
                    </div>
                    <div>
                      <h4 className="font-semibold">{step.title}</h4>
                      <p className="text-sm text-muted-foreground mt-1">{step.description}</p>
                      <div className="flex items-center gap-4 mt-2">
                        <Badge variant={getPriorityColor(step.priority)}>
                          {step.priority}
                        </Badge>
                        <span className="text-xs text-muted-foreground">
                          Est. {step.estimatedTime}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      <Alert>
        <AlertTriangle className="h-4 w-4" />
        <AlertDescription>
          This disaster recovery plan should be tested quarterly to ensure all procedures are current and functional.
          Last test: 2 weeks ago - All systems recovered successfully.
        </AlertDescription>
      </Alert>
    </div>
  );
}