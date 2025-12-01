// src/components/cli/SystemAlerts.tsx
// System alerts display component matching the design reference

import { AlertTriangle, Info, CheckCircle, XCircle, X } from "lucide-react";
import { Alert } from "@/types/cli-commands";

interface SystemAlertsProps {
  alerts: Alert[];
  onDismiss?: (id: string) => void;
  className?: string;
}

const alertStyles = {
  warning: {
    containerClass: "alert-warning",
    icon: AlertTriangle,
  },
  info: {
    containerClass: "alert-info",
    icon: Info,
  },
  success: {
    containerClass: "alert-success",
    icon: CheckCircle,
  },
  danger: {
    containerClass: "alert-danger",
    icon: XCircle,
  },
};

export function SystemAlerts({
  alerts,
  onDismiss,
  className = "",
}: SystemAlertsProps) {
  if (alerts.length === 0) {
    return null;
  }

  return (
    <div className={`space-y-4 ${className}`}>
      <h3 className="font-mono text-lg font-semibold text-content-primary">
        System Alerts
      </h3>

      <div className="space-y-3">
        {alerts.map((alert) => {
          const style = alertStyles[alert.type];
          const Icon = style.icon;

          return (
            <div key={alert.id} className={`alert-box ${style.containerClass}`}>
              <Icon className="alert-icon flex-shrink-0" size={20} />

              <div className="flex-1 min-w-0">
                <div className="alert-title">{alert.title}</div>
                <div className="alert-content">{alert.message}</div>
                {alert.timestamp && (
                  <div className="mt-1 text-xs opacity-70">
                    {alert.timestamp.toLocaleTimeString()}
                  </div>
                )}
              </div>

              {alert.dismissible && onDismiss && (
                <button
                  onClick={() => onDismiss(alert.id)}
                  className="p-1 rounded hover:bg-black/10 transition-colors flex-shrink-0"
                  title="Dismiss"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

// Single alert component for individual use
interface SingleAlertProps {
  type: Alert["type"];
  title: string;
  message: string;
  onDismiss?: () => void;
  className?: string;
}

export function SingleAlert({
  type,
  title,
  message,
  onDismiss,
  className = "",
}: SingleAlertProps) {
  const style = alertStyles[type];
  const Icon = style.icon;

  return (
    <div className={`alert-box ${style.containerClass} ${className}`}>
      <Icon className="alert-icon flex-shrink-0" size={20} />

      <div className="flex-1 min-w-0">
        <div className="alert-title">{title}</div>
        <div className="alert-content">{message}</div>
      </div>

      {onDismiss && (
        <button
          onClick={onDismiss}
          className="p-1 rounded hover:bg-black/10 transition-colors flex-shrink-0"
          title="Dismiss"
        >
          <X className="w-4 h-4" />
        </button>
      )}
    </div>
  );
}

export default SystemAlerts;
