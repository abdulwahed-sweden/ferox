/**
 * TaskScheduler - Simulated Task Scheduler
 * For demo/training purposes only - tasks stored in memory
 */

import { useState, useEffect } from "react";
import {
  Clock,
  Plus,
  Trash2,
  Play,
  Pause,
  CheckCircle,
  XCircle,
  AlertCircle,
  RefreshCw,
  Zap,
} from "lucide-react";
import { clsx } from "clsx";
import toast from "react-hot-toast";
import { simulateScheduledTasks } from "../../lib/tauri";
import type { SimulatedTask } from "../../types";

interface TaskSchedulerProps {
  sessionId: string;
}

export function TaskScheduler({ sessionId }: TaskSchedulerProps) {
  const [tasks, setTasks] = useState<SimulatedTask[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [showAddModal, setShowAddModal] = useState(false);
  const [newTask, setNewTask] = useState({
    name: "",
    command: "",
    schedule: "once",
  });

  const loadTasks = async () => {
    setIsLoading(true);
    try {
      const data = await simulateScheduledTasks(sessionId);
      setTasks(data);
    } catch (error) {
      console.error("Failed to load tasks:", error);
      toast.error("Failed to load scheduled tasks");
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    loadTasks();
  }, [sessionId]);

  const handleAddTask = () => {
    if (!newTask.name || !newTask.command) {
      toast.error("Please fill in all fields");
      return;
    }

    const task: SimulatedTask = {
      id: `task-${Date.now()}`,
      name: newTask.name,
      command: newTask.command,
      schedule:
        newTask.schedule === "once" ? "Once" : `Every ${newTask.schedule}`,
      status: "pending",
      last_run: null,
      next_run: new Date(Date.now() + 60000).toISOString(),
      created_at: new Date().toISOString(),
      run_count: 0,
      last_result: null,
      priority: "normal",
    };

    setTasks((prev) => [...prev, task]);
    setNewTask({ name: "", command: "", schedule: "once" });
    setShowAddModal(false);
    toast.success("Task scheduled");
  };

  const handleDeleteTask = (id: string) => {
    setTasks((prev) => prev.filter((t) => t.id !== id));
    toast.success("Task removed");
  };

  const handleToggleTask = (id: string) => {
    setTasks((prev) =>
      prev.map((t) => {
        if (t.id !== id) return t;
        return {
          ...t,
          status: t.status === "paused" ? "pending" : "paused",
        } as SimulatedTask;
      }),
    );
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "running":
        return <Play size={14} className="text-success-text" />;
      case "pending":
        return <Clock size={14} className="text-info-text" />;
      case "completed":
        return <CheckCircle size={14} className="text-success-text" />;
      case "failed":
        return <XCircle size={14} className="text-danger-text" />;
      case "paused":
        return <Pause size={14} className="text-warning-text" />;
      default:
        return <AlertCircle size={14} className="text-text-muted" />;
    }
  };

  const getStatusBadgeClass = (status: string) => {
    switch (status) {
      case "running":
        return "bg-success-soft text-success-text";
      case "pending":
        return "bg-info-soft text-info-text";
      case "completed":
        return "bg-success-soft text-success-text";
      case "failed":
        return "bg-danger-soft text-danger-text";
      case "paused":
        return "bg-warning-soft text-warning-text";
      default:
        return "bg-dark-600 text-text-muted";
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case "critical":
        return "text-danger-text";
      case "high":
        return "text-warning-text";
      case "normal":
        return "text-info-text";
      case "low":
        return "text-text-muted";
      default:
        return "text-text-muted";
    }
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600 bg-dark-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Clock className="text-warning-text" size={20} />
            <h2 className="text-lg font-semibold text-text-primary">
              Task Scheduler
            </h2>
            <span className="text-xs bg-warning-soft text-warning-text px-2 py-0.5 rounded">
              SIMULATION
            </span>
            {isLoading && (
              <RefreshCw
                size={12}
                className="text-warning-text animate-spin ml-2"
              />
            )}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={loadTasks}
              disabled={isLoading}
              className="px-3 py-1.5 bg-dark-700 text-text-secondary rounded text-xs font-medium flex items-center gap-1.5 hover:text-text-primary transition-colors"
            >
              <RefreshCw
                size={12}
                className={isLoading ? "animate-spin" : ""}
              />
              Refresh
            </button>
            <button
              onClick={() => setShowAddModal(true)}
              className="px-3 py-1.5 bg-warning-soft text-warning-text rounded text-sm font-medium flex items-center gap-1.5 hover:bg-warning-soft transition-colors"
            >
              <Plus size={14} />
              Add Task
            </button>
          </div>
        </div>
        <p className="text-xs text-text-muted mt-1">
          Simulated task scheduling (memory only)
        </p>
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
            {tasks.map((task) => (
              <div
                key={task.id}
                className="bg-dark-800 rounded-lg p-4 border border-dark-600"
              >
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-3">
                    {getStatusIcon(task.status)}
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-medium text-text-primary">
                          {task.name}
                        </span>
                        <span title={task.priority}>
                          <Zap
                            size={10}
                            className={getPriorityColor(task.priority)}
                          />
                        </span>
                      </div>
                      <code className="text-xs text-text-muted font-mono">
                        {task.command}
                      </code>
                    </div>
                  </div>
                  <span
                    className={clsx(
                      "text-xs px-2 py-0.5 rounded",
                      getStatusBadgeClass(task.status),
                    )}
                  >
                    {task.status}
                  </span>
                </div>

                <div className="mt-3 grid grid-cols-4 gap-4 text-xs">
                  <div>
                    <div className="text-text-muted">Schedule</div>
                    <div className="text-text-primary">{task.schedule}</div>
                  </div>
                  <div>
                    <div className="text-text-muted">Last Run</div>
                    <div className="text-text-primary">
                      {task.last_run
                        ? new Date(task.last_run).toLocaleTimeString()
                        : "Never"}
                    </div>
                  </div>
                  <div>
                    <div className="text-text-muted">Next Run</div>
                    <div className="text-text-primary">
                      {task.status === "completed"
                        ? "N/A"
                        : new Date(task.next_run).toLocaleTimeString()}
                    </div>
                  </div>
                  <div>
                    <div className="text-text-muted">Run Count</div>
                    <div className="text-text-primary">{task.run_count}</div>
                  </div>
                </div>

                {task.last_result && (
                  <div className="mt-2 text-xs">
                    <span className="text-text-muted">Result: </span>
                    <span
                      className={
                        task.last_result.includes("Error")
                          ? "text-danger-text"
                          : "text-success-text"
                      }
                    >
                      {task.last_result}
                    </span>
                  </div>
                )}

                <div className="mt-3 flex items-center gap-2 border-t border-dark-600 pt-3">
                  <button
                    onClick={() => handleToggleTask(task.id)}
                    className="px-2 py-1 rounded text-xs flex items-center gap-1 bg-dark-700 text-text-secondary hover:text-text-primary transition-colors"
                  >
                    {task.status === "paused" ? (
                      <Play size={12} />
                    ) : (
                      <Pause size={12} />
                    )}
                    {task.status === "paused" ? "Resume" : "Pause"}
                  </button>
                  <button
                    onClick={() => handleDeleteTask(task.id)}
                    className="px-2 py-1 rounded text-xs flex items-center gap-1 bg-dark-700 text-text-secondary hover:text-danger-text transition-colors"
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
            <h3 className="text-lg font-semibold text-text-primary mb-4">
              Schedule New Task
            </h3>

            <div className="space-y-4">
              <div>
                <label className="block text-xs text-text-secondary mb-1">
                  Task Name
                </label>
                <input
                  type="text"
                  value={newTask.name}
                  onChange={(e) =>
                    setNewTask((prev) => ({ ...prev, name: e.target.value }))
                  }
                  placeholder="My Task"
                  className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary focus:border-orange-400/50 focus:outline-none"
                />
              </div>

              <div>
                <label className="block text-xs text-text-secondary mb-1">
                  Command
                </label>
                <input
                  type="text"
                  value={newTask.command}
                  onChange={(e) =>
                    setNewTask((prev) => ({ ...prev, command: e.target.value }))
                  }
                  placeholder="run_module --args"
                  className="w-full px-3 py-2 bg-dark-700 border border-dark-600 rounded text-sm text-text-primary font-mono focus:border-orange-400/50 focus:outline-none"
                />
              </div>

              <div>
                <label className="block text-xs text-text-secondary mb-1">
                  Schedule
                </label>
                <select
                  value={newTask.schedule}
                  onChange={(e) =>
                    setNewTask((prev) => ({
                      ...prev,
                      schedule: e.target.value,
                    }))
                  }
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
                className="px-4 py-2 bg-warning-soft text-warning-text rounded text-sm font-medium hover:bg-warning-soft transition-colors"
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
