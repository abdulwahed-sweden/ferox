import { useState } from 'react';
import { useDashboardStore } from '../store';
import {
  FileText,
  Download,
  Calendar,
  Clock,
  CheckCircle2,
  AlertTriangle,
  BarChart3,
  PieChart,
  Shield,
  Key,
  Monitor,
} from 'lucide-react';
import { clsx } from 'clsx';

interface ReportTemplate {
  id: string;
  name: string;
  description: string;
  icon: typeof FileText;
  type: 'executive' | 'technical' | 'compliance';
}

const reportTemplates: ReportTemplate[] = [
  {
    id: 'executive-summary',
    name: 'Executive Summary',
    description: 'High-level overview of the engagement for leadership',
    icon: BarChart3,
    type: 'executive',
  },
  {
    id: 'technical-findings',
    name: 'Technical Findings',
    description: 'Detailed technical analysis of vulnerabilities found',
    icon: FileText,
    type: 'technical',
  },
  {
    id: 'attack-path',
    name: 'Attack Path Analysis',
    description: 'Visual representation of the attack chain',
    icon: Shield,
    type: 'technical',
  },
  {
    id: 'credentials-report',
    name: 'Credentials Report',
    description: 'Summary of all harvested credentials and password analysis',
    icon: Key,
    type: 'technical',
  },
  {
    id: 'mitre-mapping',
    name: 'MITRE ATT&CK Mapping',
    description: 'Techniques used mapped to MITRE framework',
    icon: PieChart,
    type: 'compliance',
  },
  {
    id: 'session-activity',
    name: 'Session Activity Log',
    description: 'Chronological log of all session activities',
    icon: Monitor,
    type: 'technical',
  },
];

interface GeneratedReport {
  id: string;
  name: string;
  generatedAt: Date;
  format: string;
  size: string;
}

export function ReportsPage() {
  const { sessions, credentials } = useDashboardStore();
  const [selectedTemplate, setSelectedTemplate] = useState<string | null>(null);
  const [generatedReports] = useState<GeneratedReport[]>([
    {
      id: '1',
      name: 'Executive Summary - Q4 Engagement',
      generatedAt: new Date(Date.now() - 86400000),
      format: 'PDF',
      size: '2.4 MB',
    },
    {
      id: '2',
      name: 'Technical Findings v1.2',
      generatedAt: new Date(Date.now() - 172800000),
      format: 'PDF',
      size: '8.1 MB',
    },
  ]);

  const stats = {
    totalSessions: sessions.length,
    activeSessions: sessions.filter((s) => s.status === 'active').length,
    totalCredentials: credentials.length,
    criticalFindings: credentials.filter((c) => c.sensitivity === 'critical').length,
  };

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Quick Stats for Report */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div className="card">
          <div className="flex items-center gap-3">
            <Monitor size={24} className="text-ferox-green" />
            <div>
              <p className="text-2xl font-bold text-text-primary">{stats.totalSessions}</p>
              <p className="text-sm text-text-secondary">Sessions</p>
            </div>
          </div>
        </div>
        <div className="card">
          <div className="flex items-center gap-3">
            <Key size={24} className="text-info" />
            <div>
              <p className="text-2xl font-bold text-text-primary">{stats.totalCredentials}</p>
              <p className="text-sm text-text-secondary">Credentials</p>
            </div>
          </div>
        </div>
        <div className="card">
          <div className="flex items-center gap-3">
            <AlertTriangle size={24} className="text-danger" />
            <div>
              <p className="text-2xl font-bold text-text-primary">{stats.criticalFindings}</p>
              <p className="text-sm text-text-secondary">Critical</p>
            </div>
          </div>
        </div>
        <div className="card">
          <div className="flex items-center gap-3">
            <CheckCircle2 size={24} className="text-ferox-green" />
            <div>
              <p className="text-2xl font-bold text-text-primary">{stats.activeSessions}</p>
              <p className="text-sm text-text-secondary">Active</p>
            </div>
          </div>
        </div>
      </div>

      {/* Report Templates */}
      <div className="card">
        <h3 className="text-lg font-semibold text-text-primary mb-4">
          Generate Report
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {reportTemplates.map((template) => {
            const Icon = template.icon;
            const isSelected = selectedTemplate === template.id;
            return (
              <button
                key={template.id}
                onClick={() => setSelectedTemplate(isSelected ? null : template.id)}
                className={clsx(
                  'p-4 rounded-lg border text-left transition-all',
                  isSelected
                    ? 'bg-ferox-green/10 border-ferox-green'
                    : 'bg-dark-800 border-dark-600 hover:border-dark-400'
                )}
              >
                <div className="flex items-start gap-3">
                  <div
                    className={clsx(
                      'p-2 rounded-lg',
                      isSelected ? 'bg-ferox-green/20' : 'bg-dark-700'
                    )}
                  >
                    <Icon
                      size={20}
                      className={isSelected ? 'text-ferox-green' : 'text-text-secondary'}
                    />
                  </div>
                  <div className="flex-1 min-w-0">
                    <p
                      className={clsx(
                        'font-medium',
                        isSelected ? 'text-ferox-green' : 'text-text-primary'
                      )}
                    >
                      {template.name}
                    </p>
                    <p className="text-xs text-text-muted mt-1 line-clamp-2">
                      {template.description}
                    </p>
                    <span
                      className={clsx(
                        'badge mt-2',
                        template.type === 'executive' && 'badge-info',
                        template.type === 'technical' && 'badge-warning',
                        template.type === 'compliance' && 'badge-success'
                      )}
                    >
                      {template.type}
                    </span>
                  </div>
                </div>
              </button>
            );
          })}
        </div>

        {selectedTemplate && (
          <div className="mt-4 flex items-center justify-between p-4 bg-dark-800 rounded-lg border border-dark-600">
            <div className="flex items-center gap-4">
              <select className="input">
                <option value="pdf">PDF Format</option>
                <option value="html">HTML Format</option>
                <option value="json">JSON Export</option>
              </select>
              <label className="flex items-center gap-2 text-sm text-text-secondary">
                <input type="checkbox" className="rounded border-dark-500" />
                Include screenshots
              </label>
            </div>
            <button className="btn-primary">
              <FileText size={16} />
              Generate Report
            </button>
          </div>
        )}
      </div>

      {/* Generated Reports */}
      <div className="card">
        <h3 className="text-lg font-semibold text-text-primary mb-4">
          Generated Reports
        </h3>
        {generatedReports.length > 0 ? (
          <div className="space-y-3">
            {generatedReports.map((report) => (
              <div
                key={report.id}
                className="flex items-center justify-between p-4 bg-dark-800 rounded-lg"
              >
                <div className="flex items-center gap-4">
                  <div className="p-2 bg-dark-700 rounded-lg">
                    <FileText size={20} className="text-info" />
                  </div>
                  <div>
                    <p className="font-medium text-text-primary">{report.name}</p>
                    <div className="flex items-center gap-3 text-xs text-text-muted mt-1">
                      <span className="flex items-center gap-1">
                        <Calendar size={12} />
                        {report.generatedAt.toLocaleDateString()}
                      </span>
                      <span className="flex items-center gap-1">
                        <Clock size={12} />
                        {report.generatedAt.toLocaleTimeString()}
                      </span>
                      <span className="badge badge-gray">{report.format}</span>
                      <span>{report.size}</span>
                    </div>
                  </div>
                </div>
                <button className="btn-outline">
                  <Download size={16} />
                  Download
                </button>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-8 text-text-muted">
            <FileText size={48} className="mx-auto mb-4 opacity-50" />
            <p>No reports generated yet</p>
            <p className="text-sm">Select a template above to generate a report</p>
          </div>
        )}
      </div>
    </div>
  );
}
