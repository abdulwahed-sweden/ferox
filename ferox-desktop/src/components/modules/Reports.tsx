/**
 * Reports - Report Generation & Export (Placeholder)
 * For demo/training purposes only
 */

import {
  ClipboardList,
  Download,
  FileJson,
  FileText,
  FileCode,
  Clock,
  CheckCircle,
} from "lucide-react";
import toast from "react-hot-toast";

interface ReportsProps {
  sessionId?: string;
}

// Mock recent reports
const mockReports = [
  {
    id: 1,
    name: "Network Scan Report",
    type: "JSON",
    date: "2024-01-15 14:30",
    size: "245 KB",
    status: "completed",
  },
  {
    id: 2,
    name: "Credential Dump Summary",
    type: "HTML",
    date: "2024-01-15 12:15",
    size: "128 KB",
    status: "completed",
  },
  {
    id: 3,
    name: "Post-Exploitation Log",
    type: "JSON",
    date: "2024-01-14 18:45",
    size: "512 KB",
    status: "completed",
  },
  {
    id: 4,
    name: "MITRE Coverage Report",
    type: "PDF",
    date: "2024-01-14 10:00",
    size: "1.2 MB",
    status: "completed",
  },
  {
    id: 5,
    name: "Full Assessment Report",
    type: "HTML",
    date: "2024-01-13 16:30",
    size: "3.4 MB",
    status: "completed",
  },
];

export function Reports({ sessionId: _sessionId }: ReportsProps) {
  const handleExport = (format: string) => {
    toast.loading(`Generating ${format} report...`, { id: `export-${format}` });
    setTimeout(() => {
      toast.success(`${format} report exported successfully`, {
        id: `export-${format}`,
      });
    }, 2000);
  };

  const handleDownload = (report: (typeof mockReports)[0]) => {
    toast.success(`Downloading ${report.name}...`);
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <ClipboardList className="text-emerald-400" size={22} />
            <h2 className="text-lg font-semibold text-text-primary">
              Reports & Export
            </h2>
            <span className="text-xs bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded">
              PLACEHOLDER
            </span>
          </div>
        </div>
        <p className="text-xs text-text-muted mt-2">
          Generate and export assessment reports in various formats
        </p>
      </div>

      {/* Export Actions */}
      <div className="p-4 border-b border-dark-600">
        <h3 className="text-sm font-medium text-text-primary mb-3">
          Quick Export
        </h3>
        <div className="flex items-center gap-3">
          <button
            onClick={() => handleExport("JSON")}
            className="flex-1 py-3 bg-dark-700 border border-dark-600 rounded-lg hover:bg-dark-600 hover:border-dark-500 transition-colors flex items-center justify-center gap-2"
          >
            <FileJson size={18} className="text-warning-text" />
            <span className="text-sm font-medium text-text-primary">
              Export JSON
            </span>
          </button>
          <button
            onClick={() => handleExport("HTML")}
            className="flex-1 py-3 bg-dark-700 border border-dark-600 rounded-lg hover:bg-dark-600 hover:border-dark-500 transition-colors flex items-center justify-center gap-2"
          >
            <FileCode size={18} className="text-warning-text" />
            <span className="text-sm font-medium text-text-primary">
              Export HTML
            </span>
          </button>
          <button
            onClick={() => handleExport("PDF")}
            className="flex-1 py-3 bg-dark-700 border border-dark-600 rounded-lg hover:bg-dark-600 hover:border-dark-500 transition-colors flex items-center justify-center gap-2"
          >
            <FileText size={18} className="text-danger-text" />
            <span className="text-sm font-medium text-text-primary">
              Export PDF
            </span>
          </button>
        </div>
      </div>

      {/* Report Options */}
      <div className="p-4 border-b border-dark-600">
        <h3 className="text-sm font-medium text-text-primary mb-3">
          Report Options
        </h3>
        <div className="grid grid-cols-2 gap-3">
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              defaultChecked
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Include credentials
            </span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              defaultChecked
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Include network map
            </span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              defaultChecked
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Include MITRE mapping
            </span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Include raw logs
            </span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              defaultChecked
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Include screenshots
            </span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              className="w-4 h-4 rounded bg-dark-700 border-dark-600 text-emerald-500"
            />
            <span className="text-sm text-text-secondary">
              Redact sensitive data
            </span>
          </label>
        </div>
      </div>

      {/* Recent Reports */}
      <div className="flex-1 overflow-auto p-4">
        <h3 className="text-sm font-medium text-text-primary mb-3">
          Recent Reports
        </h3>
        <div className="space-y-2">
          {mockReports.map((report) => (
            <div
              key={report.id}
              className="bg-dark-800 rounded-lg p-4 border border-dark-600 flex items-center justify-between hover:bg-dark-700/50 transition-colors"
            >
              <div className="flex items-center gap-3">
                {report.type === "JSON" && (
                  <FileJson size={20} className="text-warning-text" />
                )}
                {report.type === "HTML" && (
                  <FileCode size={20} className="text-warning-text" />
                )}
                {report.type === "PDF" && (
                  <FileText size={20} className="text-danger-text" />
                )}
                <div>
                  <div className="text-sm font-medium text-text-primary">
                    {report.name}
                  </div>
                  <div className="text-xs text-text-muted flex items-center gap-2">
                    <Clock size={10} />
                    {report.date}
                    <span className="text-dark-500">•</span>
                    {report.size}
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <span className="flex items-center gap-1 text-xs text-success-text">
                  <CheckCircle size={12} />
                  {report.status}
                </span>
                <button
                  onClick={() => handleDownload(report)}
                  className="p-2 hover:bg-dark-600 rounded transition-colors"
                  title="Download"
                >
                  <Download
                    size={16}
                    className="text-text-muted hover:text-text-primary"
                  />
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Placeholder notice */}
        <div className="mt-8 text-center">
          <ClipboardList
            size={32}
            className="mx-auto mb-3 text-emerald-400/30"
          />
          <p className="text-sm text-text-muted">
            Full report generation with customizable templates coming soon
          </p>
        </div>
      </div>
    </div>
  );
}

export default Reports;
