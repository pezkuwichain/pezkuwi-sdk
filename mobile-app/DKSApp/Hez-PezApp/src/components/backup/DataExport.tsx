import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Label } from '@/components/ui/label';
import { Checkbox } from '@/components/ui/checkbox';
import { Calendar } from '@/components/ui/calendar';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { supabase } from '@/lib/supabase';
import { Download, FileJson, FileText, Table, CalendarIcon } from 'lucide-react';
import { format } from 'date-fns';
import { useToast } from '@/hooks/use-toast';
import { cn } from '@/lib/utils';

export function DataExport() {
  const [exportType, setExportType] = useState('proposals');
  const [exportFormat, setExportFormat] = useState('json');
  const [dateRange, setDateRange] = useState<{ from: Date | undefined; to: Date | undefined }>({
    from: undefined,
    to: undefined
  });
  const [selectedOptions, setSelectedOptions] = useState({
    includeComments: true,
    includeVotes: true,
    includeAttachments: false,
    includeMetadata: true
  });
  const [exporting, setExporting] = useState(false);
  const { toast } = useToast();

  const handleExport = async () => {
    setExporting(true);
    try {
      const { data, error } = await supabase.functions.invoke('backup-operations', {
        body: {
          action: 'export_proposals',
          data: {
            startDate: dateRange.from?.toISOString(),
            endDate: dateRange.to?.toISOString(),
            format: exportFormat,
            options: selectedOptions
          }
        }
      });

      if (error) throw error;

      toast({
        title: 'Export Complete',
        description: `Data exported successfully in ${exportFormat.toUpperCase()} format`,
      });

      // Simulate download
      const blob = new Blob([JSON.stringify(data)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${exportType}-export-${Date.now()}.${exportFormat}`;
      a.click();
    } catch (error) {
      toast({
        title: 'Export Failed',
        description: 'Failed to export data',
        variant: 'destructive'
      });
    } finally {
      setExporting(false);
    }
  };

  const getFormatIcon = (format: string) => {
    switch (format) {
      case 'json': return <FileJson className="h-4 w-4" />;
      case 'csv': return <Table className="h-4 w-4" />;
      case 'xml': return <FileText className="h-4 w-4" />;
      default: return <FileText className="h-4 w-4" />;
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Export Data</CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="grid grid-cols-2 gap-4">
          <div>
            <Label>Export Type</Label>
            <Select value={exportType} onValueChange={setExportType}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="proposals">Proposals</SelectItem>
                <SelectItem value="users">User Data</SelectItem>
                <SelectItem value="votes">Voting Records</SelectItem>
                <SelectItem value="treasury">Treasury Transactions</SelectItem>
                <SelectItem value="full">Full System Export</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div>
            <Label>Export Format</Label>
            <Select value={exportFormat} onValueChange={setExportFormat}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="json">JSON</SelectItem>
                <SelectItem value="csv">CSV</SelectItem>
                <SelectItem value="xml">XML</SelectItem>
                <SelectItem value="sql">SQL Dump</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>

        <div>
          <Label>Date Range</Label>
          <div className="flex gap-2">
            <Popover>
              <PopoverTrigger asChild>
                <Button variant="outline" className={cn("justify-start text-left font-normal", !dateRange.from && "text-muted-foreground")}>
                  <CalendarIcon className="mr-2 h-4 w-4" />
                  {dateRange.from ? format(dateRange.from, "PPP") : "From date"}
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-auto p-0" align="start">
                <Calendar
                  mode="single"
                  selected={dateRange.from}
                  onSelect={(date) => setDateRange({ ...dateRange, from: date })}
                />
              </PopoverContent>
            </Popover>
            <Popover>
              <PopoverTrigger asChild>
                <Button variant="outline" className={cn("justify-start text-left font-normal", !dateRange.to && "text-muted-foreground")}>
                  <CalendarIcon className="mr-2 h-4 w-4" />
                  {dateRange.to ? format(dateRange.to, "PPP") : "To date"}
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-auto p-0" align="start">
                <Calendar
                  mode="single"
                  selected={dateRange.to}
                  onSelect={(date) => setDateRange({ ...dateRange, to: date })}
                />
              </PopoverContent>
            </Popover>
          </div>
        </div>

        <div className="space-y-3">
          <Label>Export Options</Label>
          <div className="space-y-2">
            <div className="flex items-center space-x-2">
              <Checkbox
                checked={selectedOptions.includeComments}
                onCheckedChange={(checked) => setSelectedOptions({ ...selectedOptions, includeComments: checked as boolean })}
              />
              <label className="text-sm">Include comments and discussions</label>
            </div>
            <div className="flex items-center space-x-2">
              <Checkbox
                checked={selectedOptions.includeVotes}
                onCheckedChange={(checked) => setSelectedOptions({ ...selectedOptions, includeVotes: checked as boolean })}
              />
              <label className="text-sm">Include voting records</label>
            </div>
            <div className="flex items-center space-x-2">
              <Checkbox
                checked={selectedOptions.includeAttachments}
                onCheckedChange={(checked) => setSelectedOptions({ ...selectedOptions, includeAttachments: checked as boolean })}
              />
              <label className="text-sm">Include file attachments</label>
            </div>
            <div className="flex items-center space-x-2">
              <Checkbox
                checked={selectedOptions.includeMetadata}
                onCheckedChange={(checked) => setSelectedOptions({ ...selectedOptions, includeMetadata: checked as boolean })}
              />
              <label className="text-sm">Include system metadata</label>
            </div>
          </div>
        </div>

        <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
          <div className="flex items-center gap-2">
            {getFormatIcon(exportFormat)}
            <span className="text-sm font-medium">
              Export as {exportFormat.toUpperCase()}
            </span>
          </div>
          <Button onClick={handleExport} disabled={exporting}>
            <Download className="mr-2 h-4 w-4" />
            {exporting ? 'Exporting...' : 'Export Data'}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}