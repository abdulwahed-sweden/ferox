import { useQuery } from '@tanstack/react-query';
import { useApi, queryKeys } from '../hooks/useApi';
import { Shield, CheckCircle2, AlertTriangle, Info } from 'lucide-react';
import { clsx } from 'clsx';

const MITRE_TACTICS = [
  { id: 'TA0001', name: 'Initial Access', short: 'Initial' },
  { id: 'TA0002', name: 'Execution', short: 'Exec' },
  { id: 'TA0003', name: 'Persistence', short: 'Persist' },
  { id: 'TA0004', name: 'Privilege Escalation', short: 'PrivEsc' },
  { id: 'TA0005', name: 'Defense Evasion', short: 'Evasion' },
  { id: 'TA0006', name: 'Credential Access', short: 'Creds' },
  { id: 'TA0007', name: 'Discovery', short: 'Disc' },
  { id: 'TA0008', name: 'Lateral Movement', short: 'Lateral' },
  { id: 'TA0009', name: 'Collection', short: 'Collect' },
  { id: 'TA0010', name: 'Exfiltration', short: 'Exfil' },
  { id: 'TA0011', name: 'Command & Control', short: 'C2' },
];

// Sample techniques for demo
const SAMPLE_TECHNIQUES = [
  { id: 'T1059.001', name: 'PowerShell', tactic: 'Execution', used: true, risk: 'medium' },
  { id: 'T1059.003', name: 'Command Shell', tactic: 'Execution', used: true, risk: 'low' },
  { id: 'T1003.001', name: 'LSASS Memory', tactic: 'Credential Access', used: true, risk: 'high' },
  { id: 'T1003.002', name: 'SAM', tactic: 'Credential Access', used: true, risk: 'medium' },
  { id: 'T1021.002', name: 'SMB/Admin Shares', tactic: 'Lateral Movement', used: true, risk: 'medium' },
  { id: 'T1021.001', name: 'RDP', tactic: 'Lateral Movement', used: false, risk: 'low' },
  { id: 'T1547.001', name: 'Registry Run Keys', tactic: 'Persistence', used: true, risk: 'medium' },
  { id: 'T1053.005', name: 'Scheduled Task', tactic: 'Persistence', used: false, risk: 'medium' },
  { id: 'T1548.002', name: 'UAC Bypass', tactic: 'Privilege Escalation', used: true, risk: 'high' },
  { id: 'T1134.001', name: 'Token Impersonation', tactic: 'Privilege Escalation', used: false, risk: 'high' },
  { id: 'T1082', name: 'System Info Discovery', tactic: 'Discovery', used: true, risk: 'low' },
  { id: 'T1083', name: 'File/Dir Discovery', tactic: 'Discovery', used: true, risk: 'low' },
  { id: 'T1071.001', name: 'Web Protocols', tactic: 'Command & Control', used: true, risk: 'low' },
  { id: 'T1105', name: 'Ingress Tool Transfer', tactic: 'Command & Control', used: true, risk: 'medium' },
  { id: 'T1027', name: 'Obfuscation', tactic: 'Defense Evasion', used: true, risk: 'low' },
  { id: 'T1562.001', name: 'Disable Security Tools', tactic: 'Defense Evasion', used: false, risk: 'high' },
];

interface TechniqueCardProps {
  id: string;
  name: string;
  tactic: string;
  used: boolean;
  risk: 'low' | 'medium' | 'high';
}

function TechniqueCard({ id, name, used, risk }: TechniqueCardProps) {
  return (
    <div
      className={clsx(
        'p-2 rounded text-xs cursor-pointer transition-all',
        used
          ? risk === 'high'
            ? 'bg-danger/20 border border-danger/50 text-danger'
            : risk === 'medium'
            ? 'bg-warning/20 border border-warning/50 text-warning'
            : 'bg-ferox-green/20 border border-ferox-green/50 text-ferox-green'
          : 'bg-dark-700 border border-dark-600 text-text-muted hover:border-dark-400'
      )}
    >
      <p className="font-medium truncate" title={name}>{name}</p>
      <p className="text-[10px] opacity-70">{id}</p>
    </div>
  );
}

export function MitrePage() {
  const api = useApi();

  const { } = useQuery({
    queryKey: queryKeys.mitre,
    queryFn: api.getMitreCoverage,
  });

  const usedCount = SAMPLE_TECHNIQUES.filter((t) => t.used).length;
  const totalCount = SAMPLE_TECHNIQUES.length;
  const coveragePercent = Math.round((usedCount / totalCount) * 100);

  // Group techniques by tactic
  const techniquesByTactic = MITRE_TACTICS.reduce((acc, tactic) => {
    acc[tactic.name] = SAMPLE_TECHNIQUES.filter((t) => t.tactic === tactic.name);
    return acc;
  }, {} as Record<string, typeof SAMPLE_TECHNIQUES>);

  return (
    <div className="space-y-6 animate-fade-in">
      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="card">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-ferox-green/10 rounded-lg">
              <Shield size={24} className="text-ferox-green" />
            </div>
            <div>
              <p className="text-2xl font-bold text-text-primary">{coveragePercent}%</p>
              <p className="text-sm text-text-secondary">Coverage</p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-info/10 rounded-lg">
              <CheckCircle2 size={24} className="text-info" />
            </div>
            <div>
              <p className="text-2xl font-bold text-text-primary">{usedCount}</p>
              <p className="text-sm text-text-secondary">Techniques Used</p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-warning/10 rounded-lg">
              <Info size={24} className="text-warning" />
            </div>
            <div>
              <p className="text-2xl font-bold text-text-primary">
                {MITRE_TACTICS.filter((t) => techniquesByTactic[t.name]?.some((tech) => tech.used)).length}
              </p>
              <p className="text-sm text-text-secondary">Tactics Covered</p>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="flex items-center gap-3">
            <div className="p-3 bg-danger/10 rounded-lg">
              <AlertTriangle size={24} className="text-danger" />
            </div>
            <div>
              <p className="text-2xl font-bold text-text-primary">
                {SAMPLE_TECHNIQUES.filter((t) => t.used && t.risk === 'high').length}
              </p>
              <p className="text-sm text-text-secondary">High Risk</p>
            </div>
          </div>
        </div>
      </div>

      {/* Legend */}
      <div className="flex items-center gap-6 text-sm">
        <span className="flex items-center gap-2">
          <div className="w-4 h-4 rounded bg-ferox-green/20 border border-ferox-green/50" />
          Used (Low Risk)
        </span>
        <span className="flex items-center gap-2">
          <div className="w-4 h-4 rounded bg-warning/20 border border-warning/50" />
          Used (Medium Risk)
        </span>
        <span className="flex items-center gap-2">
          <div className="w-4 h-4 rounded bg-danger/20 border border-danger/50" />
          Used (High Risk)
        </span>
        <span className="flex items-center gap-2">
          <div className="w-4 h-4 rounded bg-dark-700 border border-dark-600" />
          Available
        </span>
      </div>

      {/* MITRE Matrix */}
      <div className="card overflow-x-auto">
        <h3 className="text-lg font-semibold text-text-primary mb-4">
          MITRE ATT&CK Matrix
        </h3>

        <div className="grid grid-cols-11 gap-2 min-w-[1000px]">
          {/* Tactic headers */}
          {MITRE_TACTICS.map((tactic) => (
            <div
              key={tactic.id}
              className="text-center p-2 bg-dark-800 rounded-lg"
            >
              <p className="text-xs font-medium text-text-primary">{tactic.short}</p>
              <p className="text-[10px] text-text-muted">{tactic.id}</p>
            </div>
          ))}

          {/* Technique rows */}
          {MITRE_TACTICS.map((tactic) => (
            <div key={`col-${tactic.id}`} className="space-y-2">
              {techniquesByTactic[tactic.name]?.map((tech) => (
                <TechniqueCard
                  key={tech.id}
                  id={tech.id}
                  name={tech.name}
                  tactic={tech.tactic}
                  used={tech.used}
                  risk={tech.risk as 'low' | 'medium' | 'high'}
                />
              )) || <div className="h-16" />}
            </div>
          ))}
        </div>
      </div>

      {/* Used techniques list */}
      <div className="card">
        <h3 className="text-lg font-semibold text-text-primary mb-4">
          Techniques Used This Session
        </h3>
        <div className="space-y-2">
          {SAMPLE_TECHNIQUES.filter((t) => t.used).map((tech) => (
            <div
              key={tech.id}
              className="flex items-center justify-between p-3 bg-dark-800 rounded-lg"
            >
              <div className="flex items-center gap-3">
                <span
                  className={clsx(
                    'w-2 h-2 rounded-full',
                    tech.risk === 'high' && 'bg-danger',
                    tech.risk === 'medium' && 'bg-warning',
                    tech.risk === 'low' && 'bg-ferox-green'
                  )}
                />
                <div>
                  <p className="text-text-primary font-medium">{tech.name}</p>
                  <p className="text-xs text-text-muted">{tech.id} - {tech.tactic}</p>
                </div>
              </div>
              <span
                className={clsx(
                  'badge',
                  tech.risk === 'high' && 'badge-danger',
                  tech.risk === 'medium' && 'badge-warning',
                  tech.risk === 'low' && 'badge-success'
                )}
              >
                {tech.risk} risk
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
