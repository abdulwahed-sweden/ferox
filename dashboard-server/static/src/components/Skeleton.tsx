import { clsx } from 'clsx';

interface SkeletonProps {
  className?: string;
}

export function Skeleton({ className }: SkeletonProps) {
  return (
    <div
      className={clsx(
        'animate-pulse bg-dark-600 rounded',
        className
      )}
    />
  );
}

export function SkeletonCard() {
  return (
    <div className="card space-y-3">
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-3">
          <Skeleton className="w-10 h-10 rounded-lg" />
          <div className="space-y-2">
            <Skeleton className="w-32 h-4" />
            <Skeleton className="w-20 h-3" />
          </div>
        </div>
        <Skeleton className="w-16 h-5 rounded-full" />
      </div>
      <div className="space-y-2">
        <Skeleton className="w-full h-3" />
        <Skeleton className="w-3/4 h-3" />
      </div>
    </div>
  );
}

export function SkeletonStatCard() {
  return (
    <div className="card">
      <div className="flex items-start justify-between">
        <div className="space-y-2">
          <Skeleton className="w-24 h-3" />
          <Skeleton className="w-16 h-8" />
        </div>
        <Skeleton className="w-12 h-12 rounded-lg" />
      </div>
    </div>
  );
}

export function SkeletonTableRow() {
  return (
    <tr className="table-row">
      <td className="table-cell">
        <div className="flex items-center gap-3">
          <Skeleton className="w-2.5 h-2.5 rounded-full" />
          <div className="space-y-1">
            <Skeleton className="w-24 h-4" />
            <Skeleton className="w-20 h-3" />
          </div>
        </div>
      </td>
      <td className="table-cell">
        <Skeleton className="w-16 h-4" />
      </td>
      <td className="table-cell">
        <div className="space-y-1">
          <Skeleton className="w-20 h-4" />
          <Skeleton className="w-12 h-4 rounded-full" />
        </div>
      </td>
      <td className="table-cell">
        <Skeleton className="w-16 h-5 rounded-full" />
      </td>
      <td className="table-cell">
        <Skeleton className="w-14 h-4" />
      </td>
      <td className="table-cell">
        <Skeleton className="w-6 h-6 rounded" />
      </td>
    </tr>
  );
}

export function SkeletonSessionList() {
  return (
    <div className="card overflow-hidden">
      <table className="w-full">
        <thead className="bg-dark-800">
          <tr>
            <th className="table-header">Host</th>
            <th className="table-header">OS</th>
            <th className="table-header">User</th>
            <th className="table-header">Credentials</th>
            <th className="table-header">Last Seen</th>
            <th className="table-header w-12"></th>
          </tr>
        </thead>
        <tbody>
          {[...Array(5)].map((_, i) => (
            <SkeletonTableRow key={i} />
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function SkeletonDashboard() {
  return (
    <div className="space-y-6">
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {[...Array(4)].map((_, i) => (
          <SkeletonStatCard key={i} />
        ))}
      </div>

      {/* Timeline */}
      <div className="card">
        <Skeleton className="w-48 h-5 mb-4" />
        <div className="flex justify-between">
          {[...Array(5)].map((_, i) => (
            <div key={i} className="flex flex-col items-center">
              <Skeleton className="w-8 h-8 rounded-full" />
              <Skeleton className="w-16 h-3 mt-2" />
            </div>
          ))}
        </div>
      </div>

      {/* Two column */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card space-y-3">
          <Skeleton className="w-32 h-5" />
          {[...Array(4)].map((_, i) => (
            <div key={i} className="flex items-center justify-between p-3 bg-dark-800 rounded-lg">
              <div className="flex items-center gap-3">
                <Skeleton className="w-2.5 h-2.5 rounded-full" />
                <div className="space-y-1">
                  <Skeleton className="w-24 h-4" />
                  <Skeleton className="w-16 h-3" />
                </div>
              </div>
              <div className="space-y-1 text-right">
                <Skeleton className="w-16 h-4 rounded-full" />
                <Skeleton className="w-12 h-3" />
              </div>
            </div>
          ))}
        </div>

        <div className="card">
          <Skeleton className="w-32 h-5 mb-4" />
          <div className="grid grid-cols-2 gap-3">
            {[...Array(4)].map((_, i) => (
              <Skeleton key={i} className="h-12 rounded-lg" />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

export function LoadingSpinner({ size = 24 }: { size?: number }) {
  return (
    <svg
      className="animate-spin text-ferox-green"
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <circle
        className="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        strokeWidth="4"
      />
      <path
        className="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  );
}

export function EmptyState({
  icon: Icon,
  title,
  description,
  action,
}: {
  icon: React.ComponentType<{ size: number; className?: string }>;
  title: string;
  description?: string;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex flex-col items-center justify-center py-12 text-center">
      <Icon size={48} className="text-text-muted opacity-50 mb-4" />
      <h3 className="text-lg font-medium text-text-primary mb-1">{title}</h3>
      {description && <p className="text-sm text-text-muted mb-4">{description}</p>}
      {action}
    </div>
  );
}

export function ConnectionBanner({ isConnected }: { isConnected: boolean }) {
  if (isConnected) return null;

  return (
    <div className="fixed top-0 left-0 right-0 z-50 bg-danger/90 text-white py-2 px-4 text-center text-sm">
      <div className="flex items-center justify-center gap-2">
        <LoadingSpinner size={16} />
        <span>Disconnected from server. Attempting to reconnect...</span>
      </div>
    </div>
  );
}
