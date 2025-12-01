// ferox-desktop/src/components/ui/ScenarioCard.tsx
// Scenario card components for displaying security testing scenarios

import { ReactNode, useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  Target,
  Shield,
  AlertTriangle,
  CheckCircle,
  Clock,
  Tag,
} from "lucide-react";

// Scenario severity/priority levels
type ScenarioSeverity = "critical" | "high" | "medium" | "low" | "info";

// Scenario status
type ScenarioStatus = "pending" | "in_progress" | "completed" | "failed";

interface ScenarioCardProps {
  number?: number;
  title: string;
  description?: string;
  severity?: ScenarioSeverity;
  status?: ScenarioStatus;
  tags?: string[];
  children?: ReactNode;
  className?: string;
  collapsible?: boolean;
  defaultExpanded?: boolean;
}

const severityConfig: Record<
  ScenarioSeverity,
  { label: string; className: string }
> = {
  critical: { label: "Critical", className: "badge-critical" },
  high: { label: "High", className: "badge-high" },
  medium: { label: "Medium", className: "badge-medium" },
  low: { label: "Low", className: "badge-low" },
  info: { label: "Info", className: "badge-info" },
};

const statusConfig: Record<
  ScenarioStatus,
  { label: string; icon: typeof Clock; className: string }
> = {
  pending: { label: "Pending", icon: Clock, className: "status-pending" },
  in_progress: {
    label: "In Progress",
    icon: Target,
    className: "status-in-progress",
  },
  completed: {
    label: "Completed",
    icon: CheckCircle,
    className: "status-completed",
  },
  failed: { label: "Failed", icon: AlertTriangle, className: "status-failed" },
};

export function ScenarioCard({
  number,
  title,
  description,
  severity,
  status,
  tags,
  children,
  className = "",
  collapsible = false,
  defaultExpanded = true,
}: ScenarioCardProps) {
  const [isExpanded, setIsExpanded] = useState(defaultExpanded);

  const handleToggle = () => {
    if (collapsible) {
      setIsExpanded(!isExpanded);
    }
  };

  return (
    <div className={`scenario-card ${className}`}>
      {/* Header */}
      <div
        className={`scenario-header ${collapsible ? "cursor-pointer" : ""}`}
        onClick={handleToggle}
      >
        <div className="scenario-header-main">
          {collapsible && (
            <span className="scenario-expand-icon">
              {isExpanded ? (
                <ChevronDown size={18} />
              ) : (
                <ChevronRight size={18} />
              )}
            </span>
          )}
          {number !== undefined && (
            <span className="scenario-number">{number}</span>
          )}
          <h3 className="scenario-title">{title}</h3>
        </div>

        <div className="scenario-header-meta">
          {severity && (
            <span
              className={`scenario-badge ${severityConfig[severity].className}`}
            >
              <Shield size={12} />
              {severityConfig[severity].label}
            </span>
          )}
          {status && (
            <span
              className={`scenario-status ${statusConfig[status].className}`}
            >
              {(() => {
                const StatusIcon = statusConfig[status].icon;
                return <StatusIcon size={12} />;
              })()}
              {statusConfig[status].label}
            </span>
          )}
        </div>
      </div>

      {/* Tags */}
      {tags && tags.length > 0 && (
        <div className="scenario-tags">
          {tags.map((tag, index) => (
            <span key={index} className="scenario-tag">
              <Tag size={10} />
              {tag}
            </span>
          ))}
        </div>
      )}

      {/* Description */}
      {description && (!collapsible || isExpanded) && (
        <p className="scenario-description">{description}</p>
      )}

      {/* Content */}
      {children && (!collapsible || isExpanded) && (
        <div className="scenario-content">{children}</div>
      )}
    </div>
  );
}

// Sub-scenario component for nested scenarios
interface SubScenarioProps {
  id: string;
  title: string;
  description?: string;
  children?: ReactNode;
  className?: string;
}

export function SubScenario({
  id,
  title,
  description,
  children,
  className = "",
}: SubScenarioProps) {
  return (
    <div className={`sub-scenario ${className}`}>
      <div className="sub-scenario-header">
        <span className="sub-scenario-id">{id}</span>
        <h4 className="sub-scenario-title">{title}</h4>
      </div>
      {description && (
        <p className="sub-scenario-description">{description}</p>
      )}
      {children && <div className="sub-scenario-content">{children}</div>}
    </div>
  );
}

// Scenario group for organizing multiple scenarios
interface ScenarioGroupProps {
  title: string;
  description?: string;
  children: ReactNode;
  className?: string;
}

export function ScenarioGroup({
  title,
  description,
  children,
  className = "",
}: ScenarioGroupProps) {
  return (
    <section className={`scenario-group ${className}`}>
      <div className="scenario-group-header">
        <h2 className="scenario-group-title">{title}</h2>
        {description && (
          <p className="scenario-group-description">{description}</p>
        )}
      </div>
      <div className="scenario-group-content">{children}</div>
    </section>
  );
}

// Feature grid for displaying feature cards
interface Feature {
  icon?: ReactNode;
  title: string;
  description: string;
}

interface FeatureGridProps {
  features: Feature[];
  columns?: 2 | 3 | 4;
  className?: string;
}

export function FeatureGrid({
  features,
  columns = 3,
  className = "",
}: FeatureGridProps) {
  const gridCols = {
    2: "grid-cols-1 md:grid-cols-2",
    3: "grid-cols-1 md:grid-cols-2 lg:grid-cols-3",
    4: "grid-cols-1 md:grid-cols-2 lg:grid-cols-4",
  };

  return (
    <div className={`feature-grid ${gridCols[columns]} ${className}`}>
      {features.map((feature, index) => (
        <div key={index} className="feature-card">
          {feature.icon && <div className="feature-icon">{feature.icon}</div>}
          <h4 className="feature-title">{feature.title}</h4>
          <p className="feature-description">{feature.description}</p>
        </div>
      ))}
    </div>
  );
}

// Progress indicator for scenarios
interface ScenarioProgressProps {
  completed: number;
  total: number;
  className?: string;
}

export function ScenarioProgress({
  completed,
  total,
  className = "",
}: ScenarioProgressProps) {
  const percentage = total > 0 ? Math.round((completed / total) * 100) : 0;

  return (
    <div className={`scenario-progress ${className}`}>
      <div className="scenario-progress-header">
        <span className="scenario-progress-label">Progress</span>
        <span className="scenario-progress-value">
          {completed}/{total} ({percentage}%)
        </span>
      </div>
      <div className="scenario-progress-bar">
        <div
          className="scenario-progress-fill"
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
}
