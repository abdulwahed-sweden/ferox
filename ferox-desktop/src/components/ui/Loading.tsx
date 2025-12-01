// src/components/ui/Loading.tsx
// Loading state components: Spinner, Skeleton, ProgressBar

import { cn } from '@/lib/utils';

// ============================================================================
// Spinner Component
// ============================================================================

interface SpinnerProps {
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  className?: string;
  color?: 'default' | 'primary' | 'white';
}

const spinnerSizes = {
  xs: 'w-3 h-3',
  sm: 'w-4 h-4',
  md: 'w-6 h-6',
  lg: 'w-8 h-8',
  xl: 'w-12 h-12',
};

const spinnerColors = {
  default: 'border-content-tertiary border-t-content-primary',
  primary: 'border-primary-soft border-t-primary-new',
  white: 'border-white/30 border-t-white',
};

export function Spinner({ size = 'md', className, color = 'default' }: SpinnerProps) {
  return (
    <div
      className={cn(
        'inline-block rounded-full border-2 animate-spin',
        spinnerSizes[size],
        spinnerColors[color],
        className
      )}
      role="status"
      aria-label="Loading"
    >
      <span className="sr-only">Loading...</span>
    </div>
  );
}

// ============================================================================
// Skeleton Component
// ============================================================================

interface SkeletonProps {
  className?: string;
  variant?: 'text' | 'circular' | 'rectangular' | 'rounded';
  width?: string | number;
  height?: string | number;
  animation?: 'pulse' | 'wave' | 'none';
}

export function Skeleton({
  className,
  variant = 'text',
  width,
  height,
  animation = 'pulse',
}: SkeletonProps) {
  const variantClasses = {
    text: 'rounded',
    circular: 'rounded-full',
    rectangular: 'rounded-none',
    rounded: 'rounded-lg',
  };

  const animationClasses = {
    pulse: 'animate-pulse',
    wave: 'animate-shimmer',
    none: '',
  };

  const style: React.CSSProperties = {
    width: width,
    height: variant === 'text' && !height ? '1em' : height,
  };

  return (
    <div
      className={cn(
        'bg-surface-hover',
        variantClasses[variant],
        animationClasses[animation],
        className
      )}
      style={style}
      role="presentation"
      aria-hidden="true"
    />
  );
}

// Preset skeleton shapes for common use cases
export function SkeletonText({ lines = 3, className }: { lines?: number; className?: string }) {
  return (
    <div className={cn('space-y-2', className)}>
      {Array.from({ length: lines }).map((_, i) => (
        <Skeleton
          key={i}
          variant="text"
          width={i === lines - 1 ? '60%' : '100%'}
          height="0.875rem"
        />
      ))}
    </div>
  );
}

export function SkeletonCard({ className }: { className?: string }) {
  return (
    <div className={cn('p-4 rounded-xl bg-surface-default border border-border-subtle', className)}>
      <div className="flex items-center gap-3 mb-4">
        <Skeleton variant="circular" width={40} height={40} />
        <div className="flex-1">
          <Skeleton variant="text" width="60%" height="1rem" className="mb-2" />
          <Skeleton variant="text" width="40%" height="0.75rem" />
        </div>
      </div>
      <SkeletonText lines={2} />
    </div>
  );
}

export function SkeletonTable({ rows = 5, columns = 4, className }: { rows?: number; columns?: number; className?: string }) {
  return (
    <div className={cn('space-y-2', className)}>
      {/* Header */}
      <div className="flex gap-4 p-3 bg-surface-hover rounded-lg">
        {Array.from({ length: columns }).map((_, i) => (
          <Skeleton key={i} variant="text" className="flex-1" height="0.875rem" />
        ))}
      </div>
      {/* Rows */}
      {Array.from({ length: rows }).map((_, rowIndex) => (
        <div key={rowIndex} className="flex gap-4 p-3">
          {Array.from({ length: columns }).map((_, colIndex) => (
            <Skeleton key={colIndex} variant="text" className="flex-1" height="0.875rem" />
          ))}
        </div>
      ))}
    </div>
  );
}

// ============================================================================
// Progress Bar Component
// ============================================================================

interface ProgressBarProps {
  value: number;
  max?: number;
  size?: 'sm' | 'md' | 'lg';
  color?: 'primary' | 'success' | 'warning' | 'danger';
  showLabel?: boolean;
  label?: string;
  animated?: boolean;
  indeterminate?: boolean;
  className?: string;
}

const progressSizes = {
  sm: 'h-1',
  md: 'h-2',
  lg: 'h-3',
};

const progressColors = {
  primary: 'bg-primary-new',
  success: 'bg-success-new',
  warning: 'bg-warning-new',
  danger: 'bg-danger-new',
};

export function ProgressBar({
  value,
  max = 100,
  size = 'md',
  color = 'primary',
  showLabel = false,
  label,
  animated = false,
  indeterminate = false,
  className,
}: ProgressBarProps) {
  const percentage = Math.min(100, Math.max(0, (value / max) * 100));

  return (
    <div className={cn('w-full', className)}>
      {(showLabel || label) && (
        <div className="flex justify-between mb-1">
          {label && <span className="text-sm text-content-secondary">{label}</span>}
          {showLabel && (
            <span className="text-sm font-mono text-content-tertiary">
              {indeterminate ? 'Loading...' : `${Math.round(percentage)}%`}
            </span>
          )}
        </div>
      )}
      <div
        className={cn(
          'w-full bg-surface-hover rounded-full overflow-hidden',
          progressSizes[size]
        )}
        role="progressbar"
        aria-valuenow={indeterminate ? undefined : value}
        aria-valuemin={0}
        aria-valuemax={max}
      >
        <div
          className={cn(
            'h-full rounded-full transition-all duration-300 ease-out',
            progressColors[color],
            animated && 'animate-pulse',
            indeterminate && 'animate-progress-indeterminate'
          )}
          style={{
            width: indeterminate ? '50%' : `${percentage}%`,
          }}
        />
      </div>
    </div>
  );
}

// ============================================================================
// Loading Overlay Component
// ============================================================================

interface LoadingOverlayProps {
  visible: boolean;
  message?: string;
  fullScreen?: boolean;
  blur?: boolean;
  className?: string;
}

export function LoadingOverlay({
  visible,
  message = 'Loading...',
  fullScreen = false,
  blur = true,
  className,
}: LoadingOverlayProps) {
  if (!visible) return null;

  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center gap-4',
        fullScreen ? 'fixed inset-0 z-50' : 'absolute inset-0 z-10',
        blur ? 'bg-surface-default/80 backdrop-blur-sm' : 'bg-surface-default/90',
        className
      )}
      role="alert"
      aria-busy="true"
      aria-live="polite"
    >
      <Spinner size="lg" color="primary" />
      {message && (
        <p className="text-sm text-content-secondary font-medium">{message}</p>
      )}
    </div>
  );
}

// ============================================================================
// Button Loading State
// ============================================================================

interface LoadingButtonContentProps {
  loading: boolean;
  children: React.ReactNode;
  loadingText?: string;
  spinnerSize?: SpinnerProps['size'];
}

export function LoadingButtonContent({
  loading,
  children,
  loadingText,
  spinnerSize = 'sm',
}: LoadingButtonContentProps) {
  if (!loading) return <>{children}</>;

  return (
    <span className="flex items-center gap-2">
      <Spinner size={spinnerSize} color="white" />
      {loadingText && <span>{loadingText}</span>}
    </span>
  );
}

// ============================================================================
// Empty State Component
// ============================================================================

interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: React.ReactNode;
  className?: string;
}

export function EmptyState({
  icon,
  title,
  description,
  action,
  className,
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        'flex flex-col items-center justify-center py-12 px-6 text-center',
        className
      )}
    >
      {icon && (
        <div className="w-16 h-16 rounded-full bg-surface-hover flex items-center justify-center mb-4 text-content-tertiary">
          {icon}
        </div>
      )}
      <h3 className="text-lg font-semibold text-content-primary mb-2">{title}</h3>
      {description && (
        <p className="text-sm text-content-secondary max-w-md mb-6">{description}</p>
      )}
      {action}
    </div>
  );
}
