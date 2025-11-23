/**
 * TaskScheduler - Simulated Task Scheduler
 * For demo/training purposes only - tasks stored in memory
 */

import { useState } from 'react';
import { Clock, Plus, Trash2, Play, Pause, CheckCircle, XCircle, AlertCircle } from 'lucide-react';
import { clsx } from 'clsx';
import toast from 'react-hot-toast';

interface ScheduledTask {
  id: string;
  name: string;
  command: string;
  schedule: string;
  status: 'pending' | 'running' | 'completed' | 'failed' | 'paused';
  lastRun: Date | null;
  nextRun: Date;
  createdAt: Date;
}

interface TaskSchedulerProps {
  sessionId: string;
}

export function TaskScheduler({ sessionId: _sessionId }: TaskSchedulerProps) {
  const [tasks, setTasks] = useState<ScheduledTask[]>([
    {
      id: '1',
      name: 'Beacon Heartbeat',
      command: 'send_heartbeat()',
      schedule: 'Every 30 seconds',
      status: 'running',
      lastRun: new Date(Date.now() - 30000),
      nextRun: new Date(Date.now() + 30000),
      createdAt: new Date(Date.now() - 3600000),
    },
    {
      id: '2',
      name: 'Credential Harvest',
      command: 'harvest_creds --all',
      schedule: 'Every 5 minutes',
      status: 'pending',
      lastRun: null,
      nextRun: new Date(Date.now() + 180000),
      createdAt: new Date(Date.now() - 1800000),
    },
    {
      id: '3',
      name: 'Network Scan',
      command: 'scan 192.168.1.0/24',
      schedule: 'Once',
      status: 'completed',
      lastRun: new Date(Date.now() - 600000),
      nextRun: new Date(Date.now() + 99999999),
      createdAt: new Date(Date.now() - 7200000),
    },
  ]);

  const [showAddModal, setShowAddModal] = useState(false);
  const [newTask, setNewTask] = useState({ name: '', command: '', schedule: 'once' });

  const handleAddTask = () => {
    if (!newTask.name || !newTask.command) {
      toast.error('Please fill in all fields');
      return;
    }

    const task: ScheduledTask = {
      id: Date.now().toString(),
      name: newTask.name,
      command: newTask.command,
      schedule: newTask.schedule === 'once' ? 'Once' : `Every ${newTask.schedule}`,
      status: 'pending',
      lastRun: null,
      nextRun: new Date(Date.now() + 60000),
      createdAt: new Date(),
    };

    setTasks(prev => [...prev, task]);
    setNewTask({ name: '', command: '', schedule: 'once' });
    setShowAddModal(false);
    toast.success('Task scheduled');
  };

  const handleDeleteTask = (id: string) => {
    setTasks(prev => prev.filter(t => t.id !== id));
    toast.success('Task removed');
  };

  const handleToggleTask = (id: string) => {
    setTasks(prev => prev.map(t => {
      if (t.id !== id) return t;
      return {
        ...t,
        status: t.status === 'paused' ? 'pending' : 'paused',
      };
    }));
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'running': return <Play size={14} className="text-green-400" />;
      case 'pending': return <Clock size={14} className="text-blue-400" />;
      case 'completed': return <CheckCircle size={14} className="text-green-400" />;
      case 'failed': return <XCircle size={14} className="text-red-400" />;
      case 'paused': return <Pause size={14} className="text-yellow-400" />;
      default: return <AlertCircle size={14} className="text-text-muted" />;
    }
  };

  const getStatusBadgeClass = (status: string) => {
    switch (status) {
      case 'running': return 'bg-green-500/20 text-green-400';
      case 'pending': return 'bg-blue-500/20 text-blue-400';
      case 'completed': return 'bg-green-500/20 text-green-400';
      case 'failed': return 'bg-red-500/20 text-red-400';
      case 'paused': return 'bg-yellow-500/20 text-yellow-400';
      default: return 'bg-dark-600 text-text-muted';
    }
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Clock className="text-orange-400" size={20} />
            <h2 className="text-lg font-semibold text-text-primary">Task Scheduler</h2>
            <span className="text-xs bg-orange-500/20 text-orange-400 px-2 py-0.5 rounded">SIMULATION</span>
          </div>
          <button
            onClick={() => setShowAddModal(true)}
            className="px-3 py-1.5 bg-orange-500 text-white rounded text-sm font-medium flex items-center gap-1.5 hover:bg-orange-600 transition-colors"
          >
            <Plus size={14} />
            Add Task
          </button>
        </div>
        <p className="text-xs text-text-muted mt-1">Simulated task scheduling (memory only)</p>
      </div>

      {/* Task List */}
      <div className="flex-1 overflow-y-auto p-4">
        {tasks.length === 0 ? (
          <div className="h-full flex items-center justify-center text-text-muted">
            <div className="text-center">
              <Clock size={48} className="mx-auto mb-4 opacity-20" />
              <p>No scheduled tasks</p>
              <p className="text-xs mt-1">Click "Add Task" to create one</p>
            </div>
          </div>
        ) : (
          <div className="space-y-3">
            {tasks.map(task => (
              <div key={task.id} className="bg-dark-800 rounded-lg p-4 border border-dark-600">
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-3">
                    {getStatusIcon(task.status)}
                    <div>
                      <div className="text-sm font-medium text-text-primary">{task.name}</div>
                      <code className="text-xs text-text-muted font-mono">{task.command}</code>
                    </div>
                  </div>
                  <span className={clsx('text-xs px-2 py-0.5 rounded', getStatusBadgeClass(task.status))}>
                    {task.status}
                  </span>
                </div>

                <div className="mt-3 grid grid-cols-3 gap-4 text-xs">
                  <div>
                    <div className="text-text-muted">Schedule</div>
                    <div className="text-text-primary">{task.schedule}</div>
                  </div>
                  <div>
                    <div className="text-text-muted">Last Run</div>
                    <div className="text-text-primary">
                      {task.lastRun ? task.lastRun.toLocaleTimeString() : 'Never'}
                    </div>
                  </div>
                  <div>
                    <div className="text-text-muted">Next Run</div>
                    <div className="text-text-primary">
                      {task.status === 'completed' ? 'N/A' : task.nextRun.toLocaleTimeString()}
                    </div>
                  </div>
                </div>

                <div className="mt-3 flex items-center gap-2 border-t border-dark-600 pt-3">
                  <button
                    onClick={() => handleToggleTask(task.id)}
                    className="px-2 py-1 rounded text-xs flex items-center gap-1 bg-dark-700 text-text-secondary hover:text-text-primary transition-colors"
                  >
                    {task.status === 'paused' ? <Play size={12} /> : <Pause size={12} />}
                    {task.status === 'paused' ? 'Resume' : 'Pause'}
                  </button>
                  <button
                    onClick={() => handleDeleteTask(task.id)}
                    className="px-2 py-1 rounded text-xs flex items-center gap-1 bg-dark-700 text-text-secondary hover:text-red-400 transition-colors"
                  >
                    <Trash2 size={12} />
                    Remove
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Add Task Modal */}
      {showAddModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-dark-800 rounded-lg p-6 w-full max-w-md border border-dark-600">
            <h3 className="text-lg font-semibold text-text-primary mb-4">Schedule New Task</h3>

            <div className="space-y-4">
              <div>
                <label className="block text-xs text-text-secondary mb-1">Task Name</label>
                <input
                  type="text"
                  value={newTask.name}
                  onChange={e => setNewTask(prev => ({ ...prev, name: e.target.value }))}
                  placeholder="My Task"
                  className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-orange-400/50 focus:outline-none"
                />
              </div>

              <div>
                <label className="block text-xs text-text-secondary mb-1">Command</label>
                <input
                  type="text"
                  value={newTask.command}
                  onChange={e => setNewTask(prev => ({ ...prev, command: e.target.value }))}
                  placeholder="run_module --args"
                  className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary font-mono focus:border-orange-400/50 focus:outline-none"
                />
              </div>

              <div>
                <label className="block text-xs text-text-secondary mb-1">Schedule</label>
                <select
                  value={newTask.schedule}
                  onChange={e => setNewTask(prev => ({ ...prev, schedule: e.target.value }))}
                  className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-orange-400/50 focus:outline-none"
                >
                  <option value="once">Run Once</option>
                  <option value="30 seconds">Every 30 seconds</option>
                  <option value="1 minute">Every 1 minute</option>
                  <option value="5 minutes">Every 5 minutes</option>
                  <option value="15 minutes">Every 15 minutes</option>
                  <option value="1 hour">Every 1 hour</option>
                </select>
              </div>
            </div>

            <div className="mt-6 flex items-center gap-3 justify-end">
              <button
                onClick={() => setShowAddModal(false)}
                className="px-4 py-2 rounded text-sm text-text-secondary hover:text-text-primary transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleAddTask}
                className="px-4 py-2 bg-orange-500 text-white rounded text-sm font-medium hover:bg-orange-600 transition-colors"
              >
                Schedule Task
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default TaskScheduler;
