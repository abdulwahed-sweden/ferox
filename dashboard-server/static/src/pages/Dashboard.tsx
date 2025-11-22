import { useQuery } from '@tanstack/react-query';
import { useApi, queryKeys } from '../hooks/useApi';
import { useDashboardStore } from '../store';
import {
  Monitor,
  Key,
  Target,
  Shield,
  TrendingUp,
  Clock,
  CheckCircle2,
  Circle,
} from 'lucide-react';
import { clsx } from 'clsx';
import { SkeletonDashboard } from '../components/Skeleton';

interface StatCardProps {
  title: string;
  value: number | string;
  icon: React.ReactNode;
  trend?: string;
  color: 'green' | 'blue' | 'yellow' | 'purple';
}

function StatCard({ title, value, icon, trend, color }: StatCardProps) {
  const colorClasses = {
    green: 'text-ferox-green bg-ferox-green/10 border-ferox-green/30',
    blue: 'text-info bg-info/10 border-info/30',
    yellow: 'text-warning bg-warning/10 border-warning/30',
    purple: 'text-purple-400 bg-purple-400/10 border-purple-400/30',
  };

  return (
    <div className="card-hover">
      <div className="flex items-start justify-between">
        <div>
          <p className="text-text-secondary text-sm mb-1">{title}</p>
          <p className="text-3xl font-bold text-text-primary">{value}</p>
          {trend && (
            <div className="flex items-center gap-1 mt-2 text-xs text-ferox-green">
              <TrendingUp size={12} />
              <span>{trend}</span>
            </div>
          )}
        </div>
        <div className={clsx('p-3 rounded-lg border', colorClasses[color])}>
          {icon}
        </div>
      </div>
    </div>
  );
}

const attackPhases = [
  { id: 'initial', name: 'Initial Access', status: 'complete' },
  { id: 'privesc', name: 'Privilege Escalation', status: 'complete' },
  { id: 'creds', name: 'Credential Harvest', status: 'in_progress' },
  { id: 'persist', name: 'Persistence', status: 'pending' },
  { id: 'lateral', name: 'Lateral Movement', status: 'pending' },
];

function AttackTimeline() {
  return (
    <div className="card">
      <h3 className="text-lg font-semibold text-text-primary mb-4">
        Active Operation Timeline
      </h3>
      <div className="relative">
        {/* Progress bar background */}
        <div className="absolute top-4 left-0 right-0 h-1 bg-dark-600 rounded" />
        {/* Progress bar fill */}
        <div className="absolute top-4 left-0 w-[45%] h-1 bg-ferox-green rounded" />

        <div className="flex justify-between relative">
          {attackPhases.map((phase) => (
            <div key={phase.id} className="flex flex-col items-center">
              <div
                className={clsx(
                  'w-8 h-8 rounded-full flex items-center justify-center z-10',
                  phase.status === 'complete' && 'bg-ferox-green text-dark-900',
                  phase.status === 'in_progress' &&
                    'bg-warning text-dark-900 animate-pulse',
                  phase.status === 'pending' && 'bg-dark-600 text-text-muted'
                )}
              >
                {phase.status === 'complete' ? (
                  <CheckCircle2 size={16} />
                ) : phase.status === 'in_progress' ? (
                  <Clock size={16} />
                ) : (
                  <Circle size={16} />
                )}
              </div>
              <span
                className={clsx(
                  'text-xs mt-2 text-center max-w-[80px]',
                  phase.status === 'in_progress'
                    ? 'text-warning font-medium'
                    : 'text-text-secondary'
                )}
              >
                {phase.name}
              </span>
            </div>
          ))}
        </div>
      </div>
      <div className="mt-6 flex items-center justify-between text-sm">
        <span className="text-text-secondary">
          Current Phase: <span className="text-warning font-medium">Credential Harvest</span>
        </span>
        <span className="text-text-muted">ETA: ~15 minutes</span>
      </div>
    </div>
  );
}

function RecentActivity() {
  const { sessions } = useDashboardStore();

  const recentSessions = [...sessions]
    .sort((a, b) => new Date(b.last_seen).getTime() - new Date(a.last_seen).getTime())
    .slice(0, 5);

  return (
    <div className="card">
      <h3 className="text-lg font-semibold text-text-primary mb-4">
        Recent Sessions
      </h3>
      <div className="space-y-3">
        {recentSessions.map((session) => (
          <div
            key={session.id}
            className="flex items-center justify-between p-3 bg-dark-800 rounded-lg"
          >
            <div className="flex items-center gap-3">
              <div
                className={clsx(
                  'status-dot',
                  session.status === 'active' && 'status-active',
                  session.status === 'sleeping' && 'status-sleeping',
                  session.status === 'dead' && 'status-dead'
                )}
              />
              <div>
                <p className="text-text-primary font-medium">{session.hostname}</p>
                <p className="text-text-muted text-xs">{session.ip_address}</p>
              </div>
            </div>
            <div className="text-right">
              <span
                className={clsx(
                  'badge',
                  session.privileges === 'system' || session.privileges === 'root'
                    ? 'badge-danger'
                    : session.privileges === 'administrator'
                    ? 'badge-warning'
                    : 'badge-gray'
                )}
              >
                {session.privileges}
              </span>
              <p className="text-text-muted text-xs mt-1">
                {session.username}
              </p>
            </div>
          </div>
        ))}
        {recentSessions.length === 0 && (
          <p className="text-text-muted text-center py-4">No active sessions</p>
        )}
      </div>
    </div>
  );
}

export function DashboardPage() {
  const api = useApi();
  const { sessions, credentials } = useDashboardStore();

  const { data: stats, isLoading } = useQuery({
    queryKey: queryKeys.stats,
    queryFn: api.getStats,
    refetchInterval: 5000,
  });

  const activeSessions = sessions.filter((s) => s.status === 'active').length;

  // Show skeleton while initial data is loading
  if (isLoading && sessions.length === 0) {
    return <SkeletonDashboard />;
  }

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard
          title="Active Sessions"
          value={activeSessions}
          icon={<Monitor size={24} />}
          trend="+2 this hour"
          color="green"
        />
        <StatCard
          title="Credentials Collected"
          value={stats?.credentials_collected || credentials.length}
          icon={<Key size={24} />}
          trend="+5 today"
          color="blue"
        />
        <StatCard
          title="Targets Discovered"
          value={stats?.targets_discovered || 12}
          icon={<Target size={24} />}
          color="yellow"
        />
        <StatCard
          title="MITRE Coverage"
          value={`${Math.round(stats?.mitre_coverage || 15)}%`}
          icon={<Shield size={24} />}
          color="purple"
        />
      </div>

      {/* Attack Timeline */}
      <AttackTimeline />

      {/* Two column layout */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <RecentActivity />

        {/* Quick Actions */}
        <div className="card">
          <h3 className="text-lg font-semibold text-text-primary mb-4">
            Quick Actions
          </h3>
          <div className="grid grid-cols-2 gap-3">
            <button className="btn-outline justify-center py-3">
              <Target size={18} />
              <span>Discover Targets</span>
            </button>
            <button className="btn-outline justify-center py-3">
              <Key size={18} />
              <span>Harvest Credentials</span>
            </button>
            <button className="btn-outline justify-center py-3">
              <Shield size={18} />
              <span>Install Persistence</span>
            </button>
            <button className="btn-outline justify-center py-3">
              <Monitor size={18} />
              <span>Lateral Movement</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
