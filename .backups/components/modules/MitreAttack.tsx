/**
 * MitreAttack - MITRE ATT&CK Matrix View (Placeholder)
 * For demo/training purposes only
 */

import { Grid3X3, Target } from 'lucide-react';

interface MitreAttackProps {
  sessionId?: string;
}

// MITRE ATT&CK Tactics (14 columns)
const tactics = [
  'Reconnaissance',
  'Resource Dev',
  'Initial Access',
  'Execution',
  'Persistence',
  'Priv Escalation',
  'Defense Evasion',
  'Credential Access',
  'Discovery',
  'Lateral Movement',
  'Collection',
  'C2',
  'Exfiltration',
  'Impact',
];

export function MitreAttack({ sessionId: _sessionId }: MitreAttackProps) {
  // Mock coverage stats
  const mockStats = {
    totalTechniques: 201,
    covered: 47,
    inProgress: 12,
    planned: 28,
  };

  const coveragePercent = Math.round((mockStats.covered / mockStats.totalTechniques) * 100);

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Grid3X3 className="text-purple-400" size={22} />
            <h2 className="text-lg font-semibold text-text-primary">MITRE ATT&CK Matrix</h2>
            <span className="text-xs bg-purple-500/20 text-purple-400 px-2 py-0.5 rounded">PLACEHOLDER</span>
          </div>
          <div className="flex items-center gap-2">
            <span className="text-xs text-text-muted">Coverage:</span>
            <span className="text-sm font-bold text-purple-400">{coveragePercent}%</span>
          </div>
        </div>
        <p className="text-xs text-text-muted mt-2">
          Map your techniques to the MITRE ATT&CK framework
        </p>
      </div>

      {/* Stats Summary */}
      <div className="p-4 border-b border-dark-600 grid grid-cols-4 gap-3">
        <div className="bg-dark-700/50 border border-dark-600 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-text-primary">{mockStats.totalTechniques}</div>
          <div className="text-xs text-text-muted">Total Techniques</div>
        </div>
        <div className="bg-green-500/10 border border-green-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-green-400">{mockStats.covered}</div>
          <div className="text-xs text-text-muted">Covered</div>
        </div>
        <div className="bg-yellow-500/10 border border-yellow-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-yellow-400">{mockStats.inProgress}</div>
          <div className="text-xs text-text-muted">In Progress</div>
        </div>
        <div className="bg-blue-500/10 border border-blue-500/20 rounded-lg p-3 text-center">
          <div className="text-2xl font-bold text-blue-400">{mockStats.planned}</div>
          <div className="text-xs text-text-muted">Planned</div>
        </div>
      </div>

      {/* Placeholder Matrix Grid */}
      <div className="flex-1 overflow-auto p-4">
        <div className="min-w-max">
          {/* Tactics Header */}
          <div className="grid grid-cols-14 gap-1 mb-2">
            {tactics.map((tactic, i) => (
              <div
                key={i}
                className="bg-purple-500/20 text-purple-400 text-[10px] font-medium p-2 rounded text-center truncate"
                title={tactic}
              >
                {tactic}
              </div>
            ))}
          </div>

          {/* Placeholder technique cells */}
          <div className="grid grid-cols-14 gap-1">
            {tactics.map((_, colIdx) => (
              <div key={colIdx} className="space-y-1">
                {[...Array(Math.floor(Math.random() * 8) + 3)].map((_, rowIdx) => {
                  const status = Math.random();
                  let bgColor = 'bg-dark-700';
                  if (status > 0.8) bgColor = 'bg-green-500/30';
                  else if (status > 0.6) bgColor = 'bg-yellow-500/30';
                  else if (status > 0.4) bgColor = 'bg-dark-600';

                  return (
                    <div
                      key={rowIdx}
                      className={`${bgColor} h-6 rounded text-[9px] text-text-muted flex items-center justify-center`}
                    >
                      T{1000 + colIdx * 100 + rowIdx}
                    </div>
                  );
                })}
              </div>
            ))}
          </div>
        </div>

        {/* Legend */}
        <div className="mt-6 flex items-center justify-center gap-6">
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded bg-green-500/30" />
            <span className="text-xs text-text-muted">Covered</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded bg-yellow-500/30" />
            <span className="text-xs text-text-muted">In Progress</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded bg-dark-600" />
            <span className="text-xs text-text-muted">Planned</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 rounded bg-dark-700" />
            <span className="text-xs text-text-muted">Not Covered</span>
          </div>
        </div>

        {/* Coming soon notice */}
        <div className="mt-8 text-center">
          <Target size={32} className="mx-auto mb-3 text-purple-400/30" />
          <p className="text-sm text-text-muted">
            Full interactive MITRE ATT&CK matrix with technique details and execution tracking coming soon
          </p>
        </div>
      </div>
    </div>
  );
}

export default MitreAttack;
