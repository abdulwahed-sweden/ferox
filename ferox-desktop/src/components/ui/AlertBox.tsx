// ferox-desktop/src/components/ui/AlertBox.tsx
// Alert box components for warnings, info, success, and danger messages

import { ReactNode } from "react";
import {
  AlertTriangle,
  Info,
  CheckCircle,
  XCircle,
  LucideIcon,
} from "lucide-react";

type AlertVariant = "warning" | "info" | "success" | "danger";

interface AlertBoxProps {
  variant: AlertVariant;
  title?: string;
  children: ReactNode;
  className?: string;
  showIcon?: boolean;
}

const variantConfig: Record<
  AlertVariant,
  {
    icon: LucideIcon;
    className: string;
  }
> = {
  warning: {
    icon: AlertTriangle,
    className: "alert-warning",
  },
  info: {
    icon: Info,
    className: "alert-info",
  },
  success: {
    icon: CheckCircle,
    className: "alert-success",
  },
  danger: {
    icon: XCircle,
    className: "alert-danger",
  },
};

export function AlertBox({
  variant,
  title,
  children,
  className = "",
  showIcon = true,
}: AlertBoxProps) {
  const config = variantConfig[variant];
  const Icon = config.icon;

  return (
    <div className={`alert-box ${config.className} ${className}`}>
      {showIcon && (
        <Icon className="alert-icon flex-shrink-0" size={20} strokeWidth={2} />
      )}
      <div className="flex-1 min-w-0">
        {title && <div className="alert-title">{title}</div>}
        <div className="alert-content">{children}</div>
      </div>
    </div>
  );
}

// Convenience components for each variant
interface SimpleAlertProps {
  title?: string;
  children: ReactNode;
  className?: string;
  showIcon?: boolean;
}

export function WarningBox(props: SimpleAlertProps) {
  return <AlertBox variant="warning" {...props} />;
}

export function InfoBox(props: SimpleAlertProps) {
  return <AlertBox variant="info" {...props} />;
}

export function SuccessBox(props: SimpleAlertProps) {
  return <AlertBox variant="success" {...props} />;
}

export function DangerBox(props: SimpleAlertProps) {
  return <AlertBox variant="danger" {...props} />;
}

// Inline alert for smaller notifications
interface InlineAlertProps {
  variant: AlertVariant;
  children: ReactNode;
  className?: string;
}

export function InlineAlert({
  variant,
  children,
  className = "",
}: InlineAlertProps) {
  const config = variantConfig[variant];
  const Icon = config.icon;

  return (
    <span
      className={`inline-flex items-center gap-1.5 text-sm ${config.className} px-2 py-1 rounded ${className}`}
    >
      <Icon size={14} strokeWidth={2} />
      <span>{children}</span>
    </span>
  );
}
