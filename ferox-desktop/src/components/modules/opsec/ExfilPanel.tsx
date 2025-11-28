// ferox-desktop/src/components/modules/opsec/ExfilPanel.tsx
// Exfiltration Panel

import { useState, useEffect } from 'react';
import {
  Upload,
  Download,
  Clock,
  Info,
  RefreshCw,
  Wifi,
  Cloud,
  Globe,
  MessageSquare,
} from 'lucide-react';
import { useOpsec } from '../../../hooks/useOpsec';
import type {
  ExfilChannel,
  ExfilChannelInfo,
  ExfilSession,
  ExfilOptions,
} from '../../../types/opsec';

const CHANNEL_ICONS: Record<ExfilChannel, React.ElementType> = {
  Dns: Globe,
  HttpsPost: Upload,
  HttpsGet: Download,
  Icmp: Wifi,
  CloudStorage: Cloud,
  Webhook: MessageSquare,
  Steganography: Clock,
  Pastebin: Globe,
  WebSocket: Wifi,
};

export function ExfilPanel() {
  const { listExfilChannels, startExfil, getExfilSessions, loading } = useOpsec();
  const [channels, setChannels] = useState<ExfilChannelInfo[]>([]);
  const [sessions, setSessions] = useState<ExfilSession[]>([]);
  const [selectedChannel, setSelectedChannel] = useState<ExfilChannel>('HttpsPost');
  const [options, setOptions] = useState<ExfilOptions>({
    channel: 'HttpsPost',
    endpoint: '',
    chunkSize: 1024,
    delayMs: 2000,
    jitterPercent: 30,
    encryption: true,
  });
  const [dataToExfil, setDataToExfil] = useState('');

  useEffect(() => {
    loadChannels();
    loadSessions();
  }, []);

  const loadChannels = async () => {
    try {
      const result = await listExfilChannels();
      setChannels(result);
    } catch (e) {
      console.error('Failed to load channels:', e);
    }
  };

  const loadSessions = async () => {
    try {
      const result = await getExfilSessions();
      setSessions(result);
    } catch (e) {
      console.error('Failed to load sessions:', e);
    }
  };

  const handleStartExfil = async () => {
    try {
      const session = await startExfil(
        { ...options, channel: selectedChannel },
        dataToExfil
      );
      setSessions((prev) => [...prev, session]);
    } catch (e) {
      console.error('Exfiltration failed:', e);
    }
  };

  const selectedChannelInfo = channels.find((c) => c.channel === selectedChannel);

  return (
    <div className="space-y-6">
      {/* Channel Selection */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Exfiltration Channel</h3>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
          {channels.map((channel) => {
            const Icon = CHANNEL_ICONS[channel.channel] || Upload;
            return (
              <button
                key={channel.channel}
                onClick={() => {
                  setSelectedChannel(channel.channel);
                  setOptions((prev) => ({ ...prev, channel: channel.channel }));
                }}
                className={`p-3 rounded-lg border text-left transition-colors ${
                  selectedChannel === channel.channel
                    ? 'border-cyan-400 bg-cyan-400/10'
                    : 'border-dark-600 hover:border-dark-500'
                }`}
              >
                <div className="flex items-center gap-2 mb-2">
                  <Icon className="w-4 h-4" />
                  <span className="font-medium text-sm">{channel.channel}</span>
                </div>
                <div className="flex items-center gap-2 text-xs">
                  <span className="text-text-muted">Stealth:</span>
                  <div className="flex-1 h-1 bg-dark-700 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-cyan-400 rounded-full"
                      style={{ width: `${channel.stealthRating * 10}%` }}
                    />
                  </div>
                  <span>{channel.stealthRating}/10</span>
                </div>
                <div className="flex items-center gap-2 text-xs mt-1">
                  <span className="text-text-muted">Speed:</span>
                  <div className="flex-1 h-1 bg-dark-700 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-success-text rounded-full"
                      style={{ width: `${channel.bandwidthRating * 10}%` }}
                    />
                  </div>
                  <span>{channel.bandwidthRating}/10</span>
                </div>
              </button>
            );
          })}
        </div>

        {/* Channel Info */}
        {selectedChannelInfo && (
          <div className="mt-4 p-3 bg-dark-700/50 rounded-lg">
            <div className="flex items-center justify-between text-sm">
              <span className="text-text-muted">MITRE ATT&CK:</span>
              <span className="font-mono">{selectedChannelInfo.mitreId}</span>
            </div>
            <div className="flex items-center justify-between text-sm mt-1">
              <span className="text-text-muted">Max Chunk Size:</span>
              <span>{(selectedChannelInfo.maxChunkSize / 1024).toFixed(0)} KB</span>
            </div>
          </div>
        )}
      </div>

      {/* Configuration */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Configuration</h3>

        <div className="space-y-4">
          {/* Endpoint */}
          <div>
            <label className="text-sm text-text-secondary block mb-1">
              Endpoint / Domain
            </label>
            <input
              type="text"
              value={options.endpoint}
              onChange={(e) => setOptions((prev) => ({ ...prev, endpoint: e.target.value }))}
              placeholder={
                selectedChannel === 'Dns'
                  ? 'exfil.example.com'
                  : 'https://api.example.com/collect'
              }
              className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded-lg
                focus:outline-none focus:border-cyan-400"
            />
          </div>

          {/* Chunk Size */}
          <div>
            <label className="text-sm text-text-secondary block mb-1">
              Chunk Size: {options.chunkSize} bytes
            </label>
            <input
              type="range"
              value={options.chunkSize}
              onChange={(e) =>
                setOptions((prev) => ({ ...prev, chunkSize: parseInt(e.target.value) }))
              }
              min={64}
              max={selectedChannelInfo?.maxChunkSize || 4096}
              step={64}
              className="w-full"
            />
          </div>

          {/* Delay */}
          <div>
            <label className="text-sm text-text-secondary block mb-1">
              Delay: {options.delayMs}ms
            </label>
            <input
              type="range"
              value={options.delayMs}
              onChange={(e) =>
                setOptions((prev) => ({ ...prev, delayMs: parseInt(e.target.value) }))
              }
              min={100}
              max={30000}
              step={100}
              className="w-full"
            />
          </div>

          {/* Jitter */}
          <div>
            <label className="text-sm text-text-secondary block mb-1">
              Jitter: {options.jitterPercent}%
            </label>
            <input
              type="range"
              value={options.jitterPercent}
              onChange={(e) =>
                setOptions((prev) => ({ ...prev, jitterPercent: parseInt(e.target.value) }))
              }
              min={0}
              max={50}
              step={5}
              className="w-full"
            />
          </div>

          {/* Encryption Toggle */}
          <div className="flex items-center justify-between p-3 bg-dark-700/50 rounded-lg">
            <div>
              <p className="font-medium">Encryption</p>
              <p className="text-xs text-text-muted">Encrypt data before transmission</p>
            </div>
            <button
              onClick={() =>
                setOptions((prev) => ({ ...prev, encryption: !prev.encryption }))
              }
              className={`w-12 h-6 rounded-full transition-colors ${
                options.encryption ? 'bg-cyan-600' : 'bg-dark-600'
              }`}
            >
              <div
                className={`w-5 h-5 bg-white rounded-full transition-transform ${
                  options.encryption ? 'translate-x-6' : 'translate-x-0.5'
                }`}
              />
            </button>
          </div>
        </div>
      </div>

      {/* Data Input */}
      <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
        <h3 className="text-lg font-semibold mb-4">Data to Exfiltrate</h3>
        <textarea
          value={dataToExfil}
          onChange={(e) => setDataToExfil(e.target.value)}
          placeholder="Enter data or file path..."
          rows={4}
          className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded-lg
            focus:outline-none focus:border-cyan-400 resize-none"
        />
        <div className="flex items-center justify-between mt-2">
          <p className="text-xs text-text-muted">
            {dataToExfil.length > 0
              ? `${dataToExfil.length} characters (~${Math.ceil(
                  dataToExfil.length / options.chunkSize
                )} chunks)`
              : 'No data entered'}
          </p>
          <button className="text-xs text-cyan-400 hover:underline">
            Select File...
          </button>
        </div>
      </div>

      {/* Start Button */}
      <button
        onClick={handleStartExfil}
        disabled={loading || !options.endpoint || !dataToExfil}
        className="w-full flex items-center justify-center gap-2 px-4 py-3 bg-cyan-600 hover:bg-cyan-500
          rounded-lg font-medium disabled:opacity-50 transition-colors"
      >
        <Upload className="w-4 h-4" />
        {loading ? 'Starting...' : 'Start Exfiltration'}
      </button>

      {/* Active Sessions */}
      {sessions.length > 0 && (
        <div className="bg-dark-800 rounded-lg p-4 border border-dark-600">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold">Active Sessions</h3>
            <button
              onClick={loadSessions}
              className="p-1 hover:bg-dark-700 rounded transition-colors"
            >
              <RefreshCw className="w-4 h-4" />
            </button>
          </div>

          <div className="space-y-2">
            {sessions.map((session) => (
              <div
                key={session.sessionId}
                className="p-3 bg-dark-700/50 rounded-lg border border-dark-600"
              >
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-sm">{session.sessionId.slice(0, 8)}</span>
                    <span className="text-xs text-text-muted">{session.channel}</span>
                  </div>
                  <span
                    className={`px-2 py-0.5 rounded text-xs ${
                      session.status === 'Completed'
                        ? 'bg-success-soft text-success-text'
                        : session.status === 'Failed'
                        ? 'bg-danger-soft text-danger-text'
                        : session.status === 'InProgress'
                        ? 'bg-info-soft text-info-text'
                        : 'bg-warning-soft text-warning-text'
                    }`}
                  >
                    {session.status}
                  </span>
                </div>

                {/* Progress Bar */}
                <div className="mb-2">
                  <div className="flex items-center justify-between text-xs mb-1">
                    <span className="text-text-muted">
                      {session.chunksSent}/{session.chunksTotal} chunks
                    </span>
                    <span>
                      {((session.bytesSent / session.totalBytes) * 100).toFixed(0)}%
                    </span>
                  </div>
                  <div className="h-1.5 bg-dark-700 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-cyan-400 rounded-full transition-all"
                      style={{
                        width: `${(session.bytesSent / session.totalBytes) * 100}%`,
                      }}
                    />
                  </div>
                </div>

                <div className="flex items-center justify-between text-xs text-text-muted">
                  <span>
                    {(session.bytesSent / 1024).toFixed(1)} / {(session.totalBytes / 1024).toFixed(1)} KB
                  </span>
                  <span>Started: {new Date(session.startedAt).toLocaleTimeString()}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Tips */}
      <div className="bg-dark-800/50 rounded-lg p-4 border border-dark-600">
        <h4 className="font-medium mb-2 flex items-center gap-2">
          <Info className="w-4 h-4 text-cyan-400" />
          Exfiltration Tips
        </h4>
        <ul className="text-sm text-text-secondary space-y-1">
          <li>• DNS exfil is stealthy but slow (~63 bytes/query)</li>
          <li>• Cloud storage blends with normal traffic</li>
          <li>• Higher jitter makes timing analysis harder</li>
          <li>• Match channel to target network policies</li>
        </ul>
      </div>
    </div>
  );
}
