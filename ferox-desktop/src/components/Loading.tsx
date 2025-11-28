import { clsx } from "clsx";

/**
 * Spinner component for loading indicators
 */
export function Spinner({
  size = "md",
  className,
}: {
  size?: "sm" | "md" | "lg";
  className?: string;
}) {
  const sizeClasses = {
    sm: "w-4 h-4 border-2",
    md: "w-6 h-6 border-2",
    lg: "w-8 h-8 border-3",
  };

  return (
    <div
      className={clsx(
        "animate-spin rounded-full border-ferox-green border-t-transparent",
        sizeClasses[size],
        className,
      )}
    />
  );
}

/**
 * Full-page loading overlay
 */
export function LoadingOverlay({
  message = "Loading...",
}: {
  message?: string;
}) {
  return (
    <div className="absolute inset-0 flex flex-col items-center justify-center bg-dark-900/80 backdrop-blur-sm z-50">
      <Spinner size="lg" />
      <p className="mt-3 text-sm text-text-muted">{message}</p>
    </div>
  );
}

/**
 * Inline loading indicator
 */
export function LoadingInline({
  message,
  size = "sm",
}: {
  message?: string;
  size?: "sm" | "md";
}) {
  return (
    <div className="flex items-center gap-2 text-text-muted">
      <Spinner size={size} />
      {message && <span className="text-xs">{message}</span>}
    </div>
  );
}

/**
 * Skeleton line for text placeholders
 */
export function SkeletonLine({
  width = "100%",
  height = "1rem",
  className,
}: {
  width?: string | number;
  height?: string | number;
  className?: string;
}) {
  return (
    <div
      className={clsx("skeleton rounded", className)}
      style={{ width, height }}
    />
  );
}

/**
 * Skeleton circle for avatar placeholders
 */
export function SkeletonCircle({
  size = 32,
  className,
}: {
  size?: number;
  className?: string;
}) {
  return (
    <div
      className={clsx("skeleton rounded-full", className)}
      style={{ width: size, height: size }}
    />
  );
}

/**
 * Session item skeleton for loading state
 */
export function SessionSkeleton() {
  return (
    <div className="flex items-center gap-2 px-2 py-1.5 animate-pulse">
      <div className="w-5" /> {/* Chevron placeholder */}
      <SkeletonCircle size={8} /> {/* Status dot */}
      <SkeletonCircle size={16} /> {/* OS icon */}
      <div className="flex-1 min-w-0">
        <SkeletonLine width="70%" height="0.875rem" className="mb-1" />
        <SkeletonLine width="50%" height="0.75rem" />
      </div>
    </div>
  );
}

/**
 * Multiple session skeletons
 */
export function SessionListSkeleton({ count = 3 }: { count?: number }) {
  return (
    <div className="py-1">
      {Array.from({ length: count }).map((_, i) => (
        <SessionSkeleton key={i} />
      ))}
    </div>
  );
}

/**
 * Terminal skeleton for loading state
 */
export function TerminalSkeleton() {
  return (
    <div className="h-full bg-dark-900 p-4 animate-pulse">
      <div className="space-y-2">
        <SkeletonLine width="40%" height="1rem" />
        <SkeletonLine width="60%" height="1rem" />
        <SkeletonLine width="35%" height="1rem" />
        <SkeletonLine width="55%" height="1rem" />
        <SkeletonLine width="45%" height="1rem" />
      </div>
      <div className="mt-4 flex items-center gap-2">
        <SkeletonLine width="80px" height="1rem" />
        <div className="w-2 h-4 bg-ferox-green/50 animate-pulse" />{" "}
        {/* Cursor */}
      </div>
    </div>
  );
}

/**
 * Panel skeleton with header
 */
export function PanelSkeleton({
  header = true,
  lines = 5,
}: {
  header?: boolean;
  lines?: number;
}) {
  return (
    <div className="h-full flex flex-col animate-pulse">
      {header && (
        <div className="p-3 border-b border-dark-600">
          <SkeletonLine width="120px" height="1.25rem" />
        </div>
      )}
      <div className="flex-1 p-4 space-y-3">
        {Array.from({ length: lines }).map((_, i) => (
          <SkeletonLine
            key={i}
            width={`${Math.random() * 40 + 40}%`}
            height="0.875rem"
          />
        ))}
      </div>
    </div>
  );
}

/**
 * Button loading state
 */
export function ButtonLoading({
  children,
  loading,
  className,
  disabled,
  ...props
}: React.ButtonHTMLAttributes<HTMLButtonElement> & {
  loading?: boolean;
}) {
  return (
    <button
      className={clsx(
        "relative inline-flex items-center justify-center transition-all",
        loading && "cursor-wait",
        className,
      )}
      disabled={disabled || loading}
      {...props}
    >
      {loading && (
        <span className="absolute inset-0 flex items-center justify-center">
          <Spinner size="sm" />
        </span>
      )}
      <span className={clsx(loading && "invisible")}>{children}</span>
    </button>
  );
}
