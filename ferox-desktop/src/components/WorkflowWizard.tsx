import { useCallback } from "react";
import { useWorkflowStore } from "../store/workflowStore";
import {
  targetTypeLabels,
  scopeLabels,
  scopeDescriptions,
  intensityLabels,
  intensityDescriptions,
  discoveryTypeLabels,
  discoveryTypeColors,
} from "../types/workflow";
import type {
  AssessmentTargetType,
  AssessmentScope,
  ScanIntensity,
  WorkflowModule,
  Discovery,
  WorkflowProgress,
} from "../types/workflow";
import { clsx } from "clsx";
import {
  Target,
  Radar,
  Settings,
  PlayCircle,
  CheckCircle,
  ChevronRight,
  ChevronLeft,
  Zap,
  Globe,
  Layout,
  Network,
  Shield,
  Clock,
  FileText,
  Pause,
  Square,
  RotateCcw,
  Download,
  Check,
  AlertTriangle,
  Info,
} from "lucide-react";

// Step indicator component
function StepIndicator({
  steps,
  currentStep,
}: {
  steps: { id: string; label: string; icon: React.ReactNode }[];
  currentStep: string;
}) {
  const currentIndex = steps.findIndex((s) => s.id === currentStep);

  return (
    <div className="flex items-center justify-between mb-8">
      {steps.map((step, index) => {
        const isCompleted = index < currentIndex;
        const isCurrent = step.id === currentStep;
        const isUpcoming = index > currentIndex;

        return (
          <div key={step.id} className="flex items-center flex-1">
            <div className="flex flex-col items-center">
              <div
                className={clsx(
                  "w-10 h-10 rounded-full flex items-center justify-center border-2 transition-all",
                  isCompleted &&
                    "bg-ferox-green border-ferox-green text-dark-900",
                  isCurrent &&
                    "bg-ferox-green/20 border-ferox-green text-ferox-green",
                  isUpcoming && "bg-dark-700 border-dark-600 text-text-muted",
                )}
              >
                {isCompleted ? <Check size={18} /> : step.icon}
              </div>
              <span
                className={clsx(
                  "text-xs mt-2 font-medium",
                  isCurrent ? "text-ferox-green" : "text-text-muted",
                )}
              >
                {step.label}
              </span>
            </div>
            {index < steps.length - 1 && (
              <div
                className={clsx(
                  "flex-1 h-0.5 mx-3 transition-colors",
                  isCompleted ? "bg-ferox-green" : "bg-dark-600",
                )}
              />
            )}
          </div>
        );
      })}
    </div>
  );
}

// Progress bar component
function ProgressBar({
  progress,
  label,
  status,
}: {
  progress: number;
  label: string;
  status?: "running" | "completed" | "failed";
}) {
  return (
    <div className="space-y-1">
      <div className="flex justify-between text-xs">
        <span className="text-text-secondary">{label}</span>
        <span
          className={clsx(
            status === "completed" && "text-ferox-green",
            status === "failed" && "text-red-400",
            status === "running" && "text-blue-400",
            !status && "text-text-muted",
          )}
        >
          {Math.round(progress)}%
        </span>
      </div>
      <div className="h-2 bg-dark-700 rounded-full overflow-hidden">
        <div
          className={clsx(
            "h-full transition-all duration-300 rounded-full",
            status === "completed" && "bg-ferox-green",
            status === "failed" && "bg-red-400",
            status === "running" && "bg-blue-400 animate-pulse",
            !status && "bg-dark-500",
          )}
          style={{ width: `${Math.min(100, Math.max(0, progress))}%` }}
        />
      </div>
    </div>
  );
}

// Template card component
function TemplateCard({
  template,
  selected,
  onClick,
}: {
  template: {
    id: string;
    name: string;
    description: string;
    icon: string;
    tags: string[];
  };
  selected: boolean;
  onClick: () => void;
}) {
  const iconMap: Record<string, React.ReactNode> = {
    zap: <Zap size={24} />,
    globe: <Globe size={24} />,
    layout: <Layout size={24} />,
    network: <Network size={24} />,
  };

  return (
    <button
      onClick={onClick}
      className={clsx(
        "p-4 rounded-lg border text-left transition-all",
        selected
          ? "border-ferox-green bg-ferox-green/10"
          : "border-dark-600 bg-dark-800 hover:border-dark-500",
      )}
    >
      <div className="flex items-start gap-3">
        <div
          className={clsx(
            "p-2 rounded-lg",
            selected ? "bg-ferox-green/20 text-ferox-green" : "bg-dark-700 text-text-secondary",
          )}
        >
          {iconMap[template.icon] || <Radar size={24} />}
        </div>
        <div className="flex-1 min-w-0">
          <h4 className="font-medium text-text-primary">{template.name}</h4>
          <p className="text-xs text-text-muted mt-1">{template.description}</p>
          <div className="flex flex-wrap gap-1 mt-2">
            {template.tags.map((tag) => (
              <span
                key={tag}
                className="px-2 py-0.5 text-xs rounded bg-dark-700 text-text-muted"
              >
                {tag}
              </span>
            ))}
          </div>
        </div>
        {selected && (
          <CheckCircle size={20} className="text-ferox-green flex-shrink-0" />
        )}
      </div>
    </button>
  );
}

// Discovery item component
function DiscoveryItem({ discovery }: { discovery: Discovery }) {
  return (
    <div className="flex items-center gap-3 p-2 rounded bg-dark-800 border border-dark-700">
      <div
        className={clsx(
          "w-2 h-2 rounded-full",
          discovery.importance >= 7
            ? "bg-red-400"
            : discovery.importance >= 5
              ? "bg-yellow-400"
              : "bg-blue-400",
        )}
      />
      <span
        className={clsx(
          "text-xs font-medium",
          discoveryTypeColors[discovery.discovery_type],
        )}
      >
        {discoveryTypeLabels[discovery.discovery_type]}
      </span>
      <span className="text-sm text-text-primary flex-1 truncate">
        {discovery.value}
      </span>
    </div>
  );
}

// Step 1: Target Selection
function TargetStep() {
  const {
    targetType,
    target,
    authorized,
    authorizationRef,
    setTargetType,
    setTarget,
    setAuthorized,
    setAuthorizationRef,
    templates,
    selectedTemplate,
    selectTemplate,
  } = useWorkflowStore();

  const targetTypes: AssessmentTargetType[] = [
    "ip_address",
    "domain",
    "url",
    "cidr_range",
  ];

  return (
    <div className="space-y-6">
      {/* Templates */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <Zap size={16} className="text-yellow-400" />
          Quick Start Templates
        </h3>
        <div className="grid grid-cols-2 gap-3">
          {templates.map((template) => (
            <TemplateCard
              key={template.id}
              template={template}
              selected={selectedTemplate?.id === template.id}
              onClick={() => selectTemplate(template.id)}
            />
          ))}
        </div>
      </div>

      <div className="h-px bg-dark-600" />

      {/* Target Type */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <Target size={16} className="text-blue-400" />
          Target Type
        </h3>
        <div className="grid grid-cols-4 gap-2">
          {targetTypes.map((type) => (
            <button
              key={type}
              onClick={() => setTargetType(type)}
              className={clsx(
                "px-3 py-2 rounded text-sm transition-all",
                targetType === type
                  ? "bg-ferox-green text-dark-900 font-medium"
                  : "bg-dark-700 text-text-secondary hover:bg-dark-600",
              )}
            >
              {targetTypeLabels[type]}
            </button>
          ))}
        </div>
      </div>

      {/* Target Input */}
      <div>
        <label className="block text-sm font-medium text-text-primary mb-2">
          Target
        </label>
        <input
          type="text"
          value={target}
          onChange={(e) => setTarget(e.target.value)}
          placeholder={
            targetType === "ip_address"
              ? "192.168.1.1"
              : targetType === "domain"
                ? "example.com"
                : targetType === "url"
                  ? "https://example.com"
                  : "192.168.1.0/24"
          }
          className="w-full px-4 py-3 bg-dark-700 border border-dark-600 rounded-lg text-text-primary placeholder:text-text-muted focus:outline-none focus:border-ferox-green/50"
        />
      </div>

      {/* Authorization */}
      <div className="p-4 rounded-lg bg-amber-500/10 border border-amber-500/30">
        <div className="flex items-start gap-3">
          <Shield size={20} className="text-amber-400 flex-shrink-0 mt-0.5" />
          <div className="flex-1">
            <h4 className="font-medium text-amber-400 mb-2">
              Authorization Required
            </h4>
            <p className="text-xs text-amber-200/70 mb-3">
              You must have explicit written authorization to perform security
              assessments. Unauthorized scanning is illegal.
            </p>
            <label className="flex items-center gap-3 cursor-pointer">
              <input
                type="checkbox"
                checked={authorized}
                onChange={(e) => setAuthorized(e.target.checked)}
                className="w-4 h-4 rounded border-amber-500/50 bg-dark-800 text-ferox-green focus:ring-ferox-green"
              />
              <span className="text-sm text-text-primary">
                I confirm I have authorization to assess this target
              </span>
            </label>
            {authorized && (
              <input
                type="text"
                value={authorizationRef}
                onChange={(e) => setAuthorizationRef(e.target.value)}
                placeholder="Authorization reference (optional)"
                className="mt-3 w-full px-3 py-2 bg-dark-800 border border-dark-600 rounded text-sm text-text-primary placeholder:text-text-muted focus:outline-none focus:border-ferox-green/50"
              />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// Step 2: Scope Selection
function ScopeStep() {
  const { scope, intensity, setScope, setIntensity } = useWorkflowStore();

  const scopes: AssessmentScope[] = [
    "passive_recon",
    "active_recon",
    "discovery",
    "comprehensive",
  ];
  const intensities: ScanIntensity[] = ["quiet", "normal", "aggressive"];

  return (
    <div className="space-y-6">
      {/* Scope Selection */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <Radar size={16} className="text-cyan-400" />
          Assessment Scope
        </h3>
        <div className="space-y-2">
          {scopes.map((s) => (
            <button
              key={s}
              onClick={() => setScope(s)}
              className={clsx(
                "w-full p-4 rounded-lg border text-left transition-all",
                scope === s
                  ? "border-ferox-green bg-ferox-green/10"
                  : "border-dark-600 bg-dark-800 hover:border-dark-500",
              )}
            >
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="font-medium text-text-primary">
                    {scopeLabels[s]}
                  </h4>
                  <p className="text-xs text-text-muted mt-1">
                    {scopeDescriptions[s]}
                  </p>
                </div>
                {scope === s && (
                  <CheckCircle size={20} className="text-ferox-green" />
                )}
              </div>
            </button>
          ))}
        </div>
      </div>

      {/* Intensity Selection */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <Clock size={16} className="text-orange-400" />
          Scan Intensity
        </h3>
        <div className="grid grid-cols-3 gap-3">
          {intensities.map((i) => (
            <button
              key={i}
              onClick={() => setIntensity(i)}
              className={clsx(
                "p-4 rounded-lg border text-center transition-all",
                intensity === i
                  ? "border-ferox-green bg-ferox-green/10"
                  : "border-dark-600 bg-dark-800 hover:border-dark-500",
              )}
            >
              <h4
                className={clsx(
                  "font-medium",
                  intensity === i ? "text-ferox-green" : "text-text-primary",
                )}
              >
                {intensityLabels[i]}
              </h4>
              <p className="text-xs text-text-muted mt-1">
                {intensityDescriptions[i]}
              </p>
            </button>
          ))}
        </div>
      </div>

      {/* Info box */}
      <div className="p-4 rounded-lg bg-blue-500/10 border border-blue-500/30">
        <div className="flex items-start gap-3">
          <Info size={20} className="text-blue-400 flex-shrink-0 mt-0.5" />
          <div>
            <h4 className="font-medium text-blue-400 mb-1">
              Reconnaissance & Discovery Only
            </h4>
            <p className="text-xs text-blue-200/70">
              This wizard focuses on Phase 1 (Reconnaissance) and Phase 2
              (Discovery). No exploitation or post-exploitation modules are
              included.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

// Step 3: Module Selection
function ModulesStep() {
  const { selectedModules, toggleModule, updateModuleOption, target } =
    useWorkflowStore();

  const phase1Modules = selectedModules.filter((m) => m.phase === 1);
  const phase2Modules = selectedModules.filter((m) => m.phase === 2);

  const renderModuleCard = (module: WorkflowModule) => (
    <div
      key={module.id}
      className={clsx(
        "p-3 rounded-lg border transition-all",
        module.enabled
          ? "border-ferox-green/50 bg-ferox-green/5"
          : "border-dark-600 bg-dark-800 opacity-60",
      )}
    >
      <div className="flex items-start gap-3">
        <button
          onClick={() => toggleModule(module.id)}
          className={clsx(
            "w-5 h-5 rounded border flex items-center justify-center flex-shrink-0 mt-0.5",
            module.enabled
              ? "bg-ferox-green border-ferox-green text-dark-900"
              : "border-dark-500 bg-dark-700",
          )}
        >
          {module.enabled && <Check size={12} />}
        </button>
        <div className="flex-1 min-w-0">
          <h4 className="font-medium text-text-primary text-sm">
            {module.name}
          </h4>
          <p className="text-xs text-text-muted mt-0.5">{module.description}</p>
          <div className="flex items-center gap-2 mt-2 text-xs text-text-muted">
            <Clock size={12} />
            <span>~{module.estimated_duration_secs}s</span>
          </div>
          {/* Module options */}
          {module.enabled && Object.keys(module.options).length > 0 && (
            <div className="mt-3 space-y-2">
              {Object.entries(module.options).map(([key, value]) => (
                <div key={key} className="flex items-center gap-2">
                  <label className="text-xs text-text-muted min-w-20">
                    {key}:
                  </label>
                  <input
                    type="text"
                    value={
                      key === "RHOSTS" || key === "TARGET" ? target : value
                    }
                    onChange={(e) =>
                      updateModuleOption(module.id, key, e.target.value)
                    }
                    disabled={key === "RHOSTS" || key === "TARGET"}
                    className="flex-1 px-2 py-1 bg-dark-700 border border-dark-600 rounded text-xs text-text-primary focus:outline-none focus:border-ferox-green/50 disabled:opacity-50"
                  />
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );

  return (
    <div className="space-y-6">
      {/* Phase 1: Reconnaissance */}
      {phase1Modules.length > 0 && (
        <div>
          <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
            <span className="w-6 h-6 rounded-full bg-blue-500/20 text-blue-400 flex items-center justify-center text-xs font-bold">
              1
            </span>
            Reconnaissance
          </h3>
          <div className="space-y-2">{phase1Modules.map(renderModuleCard)}</div>
        </div>
      )}

      {/* Phase 2: Discovery */}
      {phase2Modules.length > 0 && (
        <div>
          <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
            <span className="w-6 h-6 rounded-full bg-cyan-500/20 text-cyan-400 flex items-center justify-center text-xs font-bold">
              2
            </span>
            Discovery
          </h3>
          <div className="space-y-2">{phase2Modules.map(renderModuleCard)}</div>
        </div>
      )}

      {/* Empty state */}
      {selectedModules.length === 0 && (
        <div className="text-center py-12 text-text-muted">
          <Radar size={48} className="mx-auto mb-4 opacity-30" />
          <p>No modules selected</p>
          <p className="text-sm mt-1">
            Go back and select a template to get started
          </p>
        </div>
      )}
    </div>
  );
}

// Step 4: Review
function ReviewStep() {
  const {
    target,
    targetType,
    scope,
    intensity,
    selectedModules,
    authorized,
    authorizationRef,
  } = useWorkflowStore();

  const enabledModules = selectedModules.filter((m) => m.enabled);
  const totalDuration = enabledModules.reduce(
    (sum, m) => sum + m.estimated_duration_secs,
    0,
  );

  return (
    <div className="space-y-6">
      {/* Summary Card */}
      <div className="p-4 rounded-lg bg-dark-800 border border-dark-600">
        <h3 className="text-sm font-medium text-text-primary mb-4">
          Assessment Summary
        </h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-text-muted">Target:</span>
            <p className="text-text-primary font-medium mt-1">{target}</p>
          </div>
          <div>
            <span className="text-text-muted">Type:</span>
            <p className="text-text-primary font-medium mt-1">
              {targetTypeLabels[targetType]}
            </p>
          </div>
          <div>
            <span className="text-text-muted">Scope:</span>
            <p className="text-text-primary font-medium mt-1">
              {scopeLabels[scope]}
            </p>
          </div>
          <div>
            <span className="text-text-muted">Intensity:</span>
            <p className="text-text-primary font-medium mt-1">
              {intensityLabels[intensity]}
            </p>
          </div>
          <div>
            <span className="text-text-muted">Modules:</span>
            <p className="text-text-primary font-medium mt-1">
              {enabledModules.length} selected
            </p>
          </div>
          <div>
            <span className="text-text-muted">Est. Duration:</span>
            <p className="text-text-primary font-medium mt-1">
              ~{Math.ceil(totalDuration / 60)} min
            </p>
          </div>
        </div>
      </div>

      {/* Module list */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3">
          Modules to Execute
        </h3>
        <div className="space-y-1">
          {enabledModules.map((module, index) => (
            <div
              key={module.id}
              className="flex items-center gap-3 p-2 rounded bg-dark-800 text-sm"
            >
              <span className="text-text-muted w-5 text-center">
                {index + 1}
              </span>
              <span
                className={clsx(
                  "px-1.5 py-0.5 rounded text-xs",
                  module.phase === 1
                    ? "bg-blue-500/20 text-blue-400"
                    : "bg-cyan-500/20 text-cyan-400",
                )}
              >
                P{module.phase}
              </span>
              <span className="text-text-primary">{module.name}</span>
              <span className="text-text-muted ml-auto">
                ~{module.estimated_duration_secs}s
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Authorization confirmation */}
      <div
        className={clsx(
          "p-4 rounded-lg border",
          authorized
            ? "bg-green-500/10 border-green-500/30"
            : "bg-red-500/10 border-red-500/30",
        )}
      >
        <div className="flex items-center gap-3">
          {authorized ? (
            <CheckCircle size={20} className="text-green-400" />
          ) : (
            <AlertTriangle size={20} className="text-red-400" />
          )}
          <div>
            <h4
              className={clsx(
                "font-medium",
                authorized ? "text-green-400" : "text-red-400",
              )}
            >
              {authorized ? "Authorization Confirmed" : "Authorization Required"}
            </h4>
            {authorizationRef && (
              <p className="text-xs text-text-muted mt-1">
                Reference: {authorizationRef}
              </p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// Step 5: Execute
function ExecuteStep() {
  const {
    workflowProgress,
    discoveries,
    isExecuting,
    isPaused,
    setIsExecuting,
    setIsPaused,
    setWorkflowProgress,
    addDiscovery,
    buildWorkflowConfig,
    events,
    addEvent,
    clearEvents,
    clearDiscoveries,
  } = useWorkflowStore();

  // Simulated execution
  const startExecution = useCallback(() => {
    const config = buildWorkflowConfig();
    const enabledModules = config.modules.filter((m) => m.enabled);
    const totalModules = enabledModules.length;

    clearEvents();
    clearDiscoveries();

    const progress: WorkflowProgress = {
      workflow_id: config.id,
      status: "running",
      current_phase: 1,
      current_module_index: 0,
      total_modules: totalModules,
      completed_modules: 0,
      progress_percent: 0,
      phase1_result: null,
      phase2_result: null,
      all_discoveries: [],
      started_at: new Date().toISOString(),
      estimated_completion: null,
      error: null,
    };

    setWorkflowProgress(progress);
    setIsExecuting(true);

    addEvent({
      type: "started",
      workflow_id: config.id,
      timestamp: new Date().toISOString(),
      total_modules: totalModules,
    });

    // Simulate module execution
    let moduleIndex = 0;
    const executeNextModule = () => {
      if (moduleIndex >= enabledModules.length) {
        // Completed
        setWorkflowProgress({
          ...progress,
          status: "completed",
          completed_modules: totalModules,
          progress_percent: 100,
          estimated_completion: new Date().toISOString(),
        });
        setIsExecuting(false);
        addEvent({
          type: "completed",
          workflow_id: config.id,
          timestamp: new Date().toISOString(),
          success: true,
          total_discoveries: discoveries.length,
        });
        return;
      }

      const module = enabledModules[moduleIndex];

      addEvent({
        type: "module_started",
        workflow_id: config.id,
        timestamp: new Date().toISOString(),
        module_id: module.id,
        module_name: module.name,
      });

      // Simulate discoveries
      const simulatedDiscoveries = generateSimulatedDiscoveries(module);
      simulatedDiscoveries.forEach((d) => addDiscovery(d));

      // Simulate completion after delay
      setTimeout(() => {
        addEvent({
          type: "module_completed",
          workflow_id: config.id,
          timestamp: new Date().toISOString(),
          module_id: module.id,
          module_name: module.name,
          success: true,
          discoveries_count: simulatedDiscoveries.length,
        });

        moduleIndex++;
        const newProgress: WorkflowProgress = {
          ...progress,
          current_module_index: moduleIndex,
          completed_modules: moduleIndex,
          progress_percent: (moduleIndex / totalModules) * 100,
          current_phase: moduleIndex < enabledModules.filter((m) => m.phase === 1).length ? 1 : 2,
        };
        setWorkflowProgress(newProgress);

        executeNextModule();
      }, 2000); // 2 second simulation per module
    };

    executeNextModule();
  }, [
    buildWorkflowConfig,
    setWorkflowProgress,
    setIsExecuting,
    addEvent,
    addDiscovery,
    clearEvents,
    clearDiscoveries,
    discoveries.length,
  ]);

  const pauseExecution = () => {
    setIsPaused(true);
    addEvent({
      type: "paused",
      workflow_id: workflowProgress?.workflow_id || "",
      timestamp: new Date().toISOString(),
    });
  };

  const resumeExecution = () => {
    setIsPaused(false);
    addEvent({
      type: "resumed",
      workflow_id: workflowProgress?.workflow_id || "",
      timestamp: new Date().toISOString(),
    });
  };

  const cancelExecution = () => {
    setIsExecuting(false);
    setIsPaused(false);
    if (workflowProgress) {
      setWorkflowProgress({
        ...workflowProgress,
        status: "cancelled",
      });
    }
  };

  const progress = workflowProgress;

  return (
    <div className="space-y-6">
      {/* Control buttons */}
      <div className="flex items-center gap-3">
        {!isExecuting && progress?.status !== "completed" && (
          <button
            onClick={startExecution}
            className="flex items-center gap-2 px-4 py-2 bg-ferox-green text-dark-900 rounded-lg font-medium hover:bg-ferox-green/90 transition-colors"
          >
            <PlayCircle size={18} />
            Start Assessment
          </button>
        )}
        {isExecuting && !isPaused && (
          <button
            onClick={pauseExecution}
            className="flex items-center gap-2 px-4 py-2 bg-yellow-500 text-dark-900 rounded-lg font-medium hover:bg-yellow-400 transition-colors"
          >
            <Pause size={18} />
            Pause
          </button>
        )}
        {isExecuting && isPaused && (
          <button
            onClick={resumeExecution}
            className="flex items-center gap-2 px-4 py-2 bg-ferox-green text-dark-900 rounded-lg font-medium hover:bg-ferox-green/90 transition-colors"
          >
            <PlayCircle size={18} />
            Resume
          </button>
        )}
        {isExecuting && (
          <button
            onClick={cancelExecution}
            className="flex items-center gap-2 px-4 py-2 bg-red-500 text-white rounded-lg font-medium hover:bg-red-400 transition-colors"
          >
            <Square size={18} />
            Cancel
          </button>
        )}
        {progress?.status === "completed" && (
          <>
            <button
              onClick={startExecution}
              className="flex items-center gap-2 px-4 py-2 bg-dark-700 text-text-primary rounded-lg font-medium hover:bg-dark-600 transition-colors"
            >
              <RotateCcw size={18} />
              Run Again
            </button>
            <button className="flex items-center gap-2 px-4 py-2 bg-dark-700 text-text-primary rounded-lg font-medium hover:bg-dark-600 transition-colors">
              <Download size={18} />
              Export Report
            </button>
          </>
        )}
      </div>

      {/* Overall progress */}
      {progress && (
        <div className="p-4 rounded-lg bg-dark-800 border border-dark-600">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-medium text-text-primary">
              Overall Progress
            </h3>
            <span
              className={clsx(
                "px-2 py-0.5 rounded text-xs font-medium",
                progress.status === "running" && "bg-blue-500/20 text-blue-400",
                progress.status === "completed" &&
                  "bg-green-500/20 text-green-400",
                progress.status === "failed" && "bg-red-500/20 text-red-400",
                progress.status === "paused" &&
                  "bg-yellow-500/20 text-yellow-400",
                progress.status === "cancelled" &&
                  "bg-gray-500/20 text-gray-400",
              )}
            >
              {progress.status.toUpperCase()}
            </span>
          </div>
          <ProgressBar
            progress={progress.progress_percent}
            label={`${progress.completed_modules} / ${progress.total_modules} modules`}
            status={
              progress.status === "completed"
                ? "completed"
                : progress.status === "failed"
                  ? "failed"
                  : "running"
            }
          />
        </div>
      )}

      {/* Discoveries */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <Target size={16} className="text-purple-400" />
          Discoveries ({discoveries.length})
        </h3>
        <div className="max-h-48 overflow-y-auto space-y-1">
          {discoveries.length > 0 ? (
            discoveries.map((d, i) => <DiscoveryItem key={i} discovery={d} />)
          ) : (
            <div className="text-center py-6 text-text-muted text-sm">
              {isExecuting
                ? "Scanning for discoveries..."
                : "No discoveries yet"}
            </div>
          )}
        </div>
      </div>

      {/* Event log */}
      <div>
        <h3 className="text-sm font-medium text-text-primary mb-3 flex items-center gap-2">
          <FileText size={16} className="text-gray-400" />
          Event Log
        </h3>
        <div className="max-h-32 overflow-y-auto space-y-1 font-mono text-xs">
          {events.map((event, i) => (
            <div key={i} className="flex items-center gap-2 text-text-muted">
              <span className="text-text-muted/50">
                {new Date(event.timestamp || "").toLocaleTimeString()}
              </span>
              <span
                className={clsx(
                  event.type === "completed" && event.success && "text-green-400",
                  event.type === "error" && "text-red-400",
                  event.type === "module_started" && "text-blue-400",
                  event.type === "module_completed" && "text-cyan-400",
                )}
              >
                [{event.type}]
              </span>
              <span className="text-text-secondary">
                {event.module_name || event.message || ""}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

// Helper function to generate simulated discoveries
function generateSimulatedDiscoveries(module: WorkflowModule): Discovery[] {
  const discoveries: Discovery[] = [];

  switch (module.path) {
    case "scanner/port_scanner":
      [22, 80, 443].forEach((port) => {
        discoveries.push({
          discovery_type: "open_port",
          value: `Port ${port}/tcp open`,
          details: { protocol: "tcp", state: "open" },
          importance: port === 22 ? 7 : 5,
        });
      });
      break;
    case "scanner/http_scanner":
      discoveries.push({
        discovery_type: "http_service",
        value: "HTTP/1.1 200 OK - nginx/1.18.0",
        details: { server: "nginx", version: "1.18.0" },
        importance: 5,
      });
      discoveries.push({
        discovery_type: "technology",
        value: "nginx web server detected",
        details: { version: "1.18.0" },
        importance: 4,
      });
      break;
    case "recon/dns_enum":
      discoveries.push({
        discovery_type: "dns_record",
        value: "A: 93.184.216.34",
        details: { record_type: "A" },
        importance: 5,
      });
      discoveries.push({
        discovery_type: "dns_record",
        value: "MX: mail.example.com (priority 10)",
        details: { record_type: "MX", priority: "10" },
        importance: 4,
      });
      break;
    case "recon/subdomain_enum":
      ["www", "mail", "api", "dev"].forEach((sub) => {
        discoveries.push({
          discovery_type: "subdomain",
          value: `${sub}.example.com`,
          details: {},
          importance: 6,
        });
      });
      break;
    case "recon/whois_lookup":
      discoveries.push({
        discovery_type: "whois_info",
        value: "Registrar: Example Registrar Inc.",
        details: { created: "2000-01-01", expires: "2025-01-01" },
        importance: 3,
      });
      break;
    case "recon/asn_discovery":
      discoveries.push({
        discovery_type: "asn_info",
        value: "AS15169 - Google LLC",
        details: { asn: "15169", org: "Google LLC" },
        importance: 4,
      });
      break;
  }

  return discoveries;
}

// Main Workflow Wizard component
export function WorkflowWizard() {
  const { wizardStep, setWizardStep, target, authorized, selectedModules, resetWizard } =
    useWorkflowStore();

  const steps = [
    { id: "target", label: "Target", icon: <Target size={16} /> },
    { id: "scope", label: "Scope", icon: <Radar size={16} /> },
    { id: "modules", label: "Modules", icon: <Settings size={16} /> },
    { id: "review", label: "Review", icon: <FileText size={16} /> },
    { id: "execute", label: "Execute", icon: <PlayCircle size={16} /> },
  ];

  const canProceed = () => {
    switch (wizardStep) {
      case "target":
        return target.length > 0 && authorized;
      case "scope":
        return true;
      case "modules":
        return selectedModules.some((m) => m.enabled);
      case "review":
        return true;
      default:
        return true;
    }
  };

  const goNext = () => {
    const currentIndex = steps.findIndex((s) => s.id === wizardStep);
    if (currentIndex < steps.length - 1) {
      setWizardStep(steps[currentIndex + 1].id as typeof wizardStep);
    }
  };

  const goBack = () => {
    const currentIndex = steps.findIndex((s) => s.id === wizardStep);
    if (currentIndex > 0) {
      setWizardStep(steps[currentIndex - 1].id as typeof wizardStep);
    }
  };

  return (
    <div className="h-full flex flex-col bg-dark-900">
      {/* Header */}
      <div className="p-4 border-b border-dark-600">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h2 className="text-lg font-semibold text-text-primary flex items-center gap-2">
              <Shield size={20} className="text-ferox-green" />
              Security Assessment Wizard
            </h2>
            <p className="text-xs text-text-muted mt-1">
              Guided workflow for authorized reconnaissance and discovery
            </p>
          </div>
          <button
            onClick={resetWizard}
            className="px-3 py-1.5 text-xs text-text-muted hover:text-text-primary transition-colors"
          >
            Reset
          </button>
        </div>
        <StepIndicator steps={steps} currentStep={wizardStep} />
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {wizardStep === "target" && <TargetStep />}
        {wizardStep === "scope" && <ScopeStep />}
        {wizardStep === "modules" && <ModulesStep />}
        {wizardStep === "review" && <ReviewStep />}
        {wizardStep === "execute" && <ExecuteStep />}
      </div>

      {/* Footer */}
      {wizardStep !== "execute" && (
        <div className="p-4 border-t border-dark-600 flex items-center justify-between">
          <button
            onClick={goBack}
            disabled={wizardStep === "target"}
            className={clsx(
              "flex items-center gap-2 px-4 py-2 rounded-lg transition-colors",
              wizardStep === "target"
                ? "text-text-muted cursor-not-allowed"
                : "text-text-secondary hover:text-text-primary hover:bg-dark-700",
            )}
          >
            <ChevronLeft size={18} />
            Back
          </button>
          <button
            onClick={goNext}
            disabled={!canProceed()}
            className={clsx(
              "flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors",
              canProceed()
                ? "bg-ferox-green text-dark-900 hover:bg-ferox-green/90"
                : "bg-dark-700 text-text-muted cursor-not-allowed",
            )}
          >
            {wizardStep === "review" ? "Start Assessment" : "Continue"}
            <ChevronRight size={18} />
          </button>
        </div>
      )}
    </div>
  );
}
