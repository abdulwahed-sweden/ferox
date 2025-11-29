import { create } from "zustand";
import type {
  WorkflowConfig,
  WorkflowProgress,
  WorkflowTemplate,
  WizardStep,
  AssessmentTargetType,
  AssessmentScope,
  ScanIntensity,
  WorkflowModule,
  Discovery,
  AssessmentReport,
  WorkflowEvent,
} from "../types/workflow";

// Default templates
const defaultTemplates: WorkflowTemplate[] = [
  {
    id: "quick-scan",
    name: "Quick Scan",
    description: "Fast port scan and HTTP fingerprinting",
    recommended_target_type: "ip_address",
    default_scope: "discovery",
    default_intensity: "normal",
    icon: "zap",
    tags: ["fast", "ports", "http"],
    modules: [
      {
        id: "qs-port",
        path: "scanner/port_scanner",
        name: "Port Scanner",
        description: "Scan for open TCP ports",
        options: {
          PORTS:
            "21,22,23,25,53,80,110,143,443,445,993,995,3306,3389,5432,8080,8443",
        },
        enabled: true,
        phase: 2,
        estimated_duration_secs: 60,
      },
      {
        id: "qs-http",
        path: "scanner/http_scanner",
        name: "HTTP Scanner",
        description: "Fingerprint HTTP/HTTPS services",
        options: { FOLLOW_REDIRECTS: "true" },
        enabled: true,
        phase: 2,
        estimated_duration_secs: 45,
      },
    ],
  },
  {
    id: "domain-recon",
    name: "Domain Reconnaissance",
    description: "Comprehensive domain information gathering",
    recommended_target_type: "domain",
    default_scope: "active_recon",
    default_intensity: "quiet",
    icon: "globe",
    tags: ["domain", "dns", "recon"],
    modules: [
      {
        id: "dr-dns",
        path: "recon/dns_enum",
        name: "DNS Enumeration",
        description: "Query DNS records (A, AAAA, MX, NS, TXT, SOA)",
        options: { RECORD_TYPES: "A,AAAA,MX,NS,TXT,SOA,CNAME" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 30,
      },
      {
        id: "dr-whois",
        path: "recon/whois_lookup",
        name: "WHOIS Lookup",
        description: "Domain registration and ownership info",
        options: { FOLLOW_REFERRAL: "true" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 15,
      },
      {
        id: "dr-subdomain",
        path: "recon/subdomain_enum",
        name: "Subdomain Enumeration",
        description: "Discover subdomains via passive sources",
        options: { PROBE_HTTP: "false" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 90,
      },
    ],
  },
  {
    id: "web-assessment",
    name: "Web Application Assessment",
    description: "Web server reconnaissance and discovery",
    recommended_target_type: "url",
    default_scope: "comprehensive",
    default_intensity: "normal",
    icon: "layout",
    tags: ["web", "http", "comprehensive"],
    modules: [
      {
        id: "wa-dns",
        path: "recon/dns_enum",
        name: "DNS Enumeration",
        description: "Query DNS records for the domain",
        options: { RECORD_TYPES: "A,AAAA,MX,NS,TXT,CNAME" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 30,
      },
      {
        id: "wa-subdomain",
        path: "recon/subdomain_enum",
        name: "Subdomain Discovery",
        description: "Find related subdomains",
        options: { PROBE_HTTP: "true" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 90,
      },
      {
        id: "wa-port",
        path: "scanner/port_scanner",
        name: "Port Scanner",
        description: "Scan common web ports",
        options: { PORTS: "80,443,8080,8443,8000,3000,5000,9000" },
        enabled: true,
        phase: 2,
        estimated_duration_secs: 45,
      },
      {
        id: "wa-http",
        path: "scanner/http_scanner",
        name: "HTTP Fingerprinting",
        description: "Detect web technologies and frameworks",
        options: {
          FOLLOW_REDIRECTS: "true",
          PATHS: "/,/robots.txt,/sitemap.xml,/.well-known/",
        },
        enabled: true,
        phase: 2,
        estimated_duration_secs: 60,
      },
    ],
  },
  {
    id: "network-infra",
    name: "Network Infrastructure",
    description: "Network range reconnaissance and service discovery",
    recommended_target_type: "cidr_range",
    default_scope: "discovery",
    default_intensity: "quiet",
    icon: "network",
    tags: ["network", "infrastructure", "ports"],
    modules: [
      {
        id: "ni-asn",
        path: "recon/asn_discovery",
        name: "ASN Discovery",
        description: "Identify network ownership and prefixes",
        options: { LOOKUP_PREFIXES: "true" },
        enabled: true,
        phase: 1,
        estimated_duration_secs: 20,
      },
      {
        id: "ni-whois",
        path: "recon/whois_lookup",
        name: "WHOIS Lookup",
        description: "IP/network registration info",
        options: {},
        enabled: true,
        phase: 1,
        estimated_duration_secs: 15,
      },
      {
        id: "ni-port",
        path: "scanner/port_scanner",
        name: "Port Scanner",
        description: "Comprehensive port scan",
        options: { PORTS: "1-1000" },
        enabled: true,
        phase: 2,
        estimated_duration_secs: 180,
      },
      {
        id: "ni-http",
        path: "scanner/http_scanner",
        name: "HTTP Scanner",
        description: "Detect HTTP services on open ports",
        options: {},
        enabled: true,
        phase: 2,
        estimated_duration_secs: 90,
      },
    ],
  },
];

interface WorkflowState {
  // Wizard state
  wizardStep: WizardStep;
  setWizardStep: (step: WizardStep) => void;

  // Target configuration
  targetType: AssessmentTargetType;
  target: string;
  authorized: boolean;
  authorizationRef: string;
  targetNotes: string;
  setTargetType: (type: AssessmentTargetType) => void;
  setTarget: (target: string) => void;
  setAuthorized: (authorized: boolean) => void;
  setAuthorizationRef: (ref: string) => void;
  setTargetNotes: (notes: string) => void;

  // Scope configuration
  scope: AssessmentScope;
  intensity: ScanIntensity;
  setScope: (scope: AssessmentScope) => void;
  setIntensity: (intensity: ScanIntensity) => void;

  // Module selection
  selectedModules: WorkflowModule[];
  setSelectedModules: (modules: WorkflowModule[]) => void;
  toggleModule: (moduleId: string) => void;
  updateModuleOption: (
    moduleId: string,
    optionKey: string,
    value: string,
  ) => void;

  // Templates
  templates: WorkflowTemplate[];
  selectedTemplate: WorkflowTemplate | null;
  selectTemplate: (templateId: string) => void;

  // Workflow execution
  workflowConfig: WorkflowConfig | null;
  workflowProgress: WorkflowProgress | null;
  isExecuting: boolean;
  isPaused: boolean;
  setWorkflowConfig: (config: WorkflowConfig | null) => void;
  setWorkflowProgress: (progress: WorkflowProgress | null) => void;
  setIsExecuting: (executing: boolean) => void;
  setIsPaused: (paused: boolean) => void;

  // Discoveries
  discoveries: Discovery[];
  addDiscovery: (discovery: Discovery) => void;
  clearDiscoveries: () => void;

  // Report
  report: AssessmentReport | null;
  setReport: (report: AssessmentReport | null) => void;

  // Event log
  events: WorkflowEvent[];
  addEvent: (event: WorkflowEvent) => void;
  clearEvents: () => void;

  // Build workflow config from current state
  buildWorkflowConfig: () => WorkflowConfig;

  // Reset wizard
  resetWizard: () => void;
}

export const useWorkflowStore = create<WorkflowState>((set, get) => ({
  // Wizard state
  wizardStep: "target",
  setWizardStep: (wizardStep) => set({ wizardStep }),

  // Target configuration
  targetType: "domain",
  target: "",
  authorized: false,
  authorizationRef: "",
  targetNotes: "",
  setTargetType: (targetType) => set({ targetType }),
  setTarget: (target) => set({ target }),
  setAuthorized: (authorized) => set({ authorized }),
  setAuthorizationRef: (authorizationRef) => set({ authorizationRef }),
  setTargetNotes: (targetNotes) => set({ targetNotes }),

  // Scope configuration
  scope: "discovery",
  intensity: "normal",
  setScope: (scope) => set({ scope }),
  setIntensity: (intensity) => set({ intensity }),

  // Module selection
  selectedModules: [],
  setSelectedModules: (selectedModules) => set({ selectedModules }),
  toggleModule: (moduleId) =>
    set((state) => ({
      selectedModules: state.selectedModules.map((m) =>
        m.id === moduleId ? { ...m, enabled: !m.enabled } : m,
      ),
    })),
  updateModuleOption: (moduleId, optionKey, value) =>
    set((state) => ({
      selectedModules: state.selectedModules.map((m) =>
        m.id === moduleId
          ? { ...m, options: { ...m.options, [optionKey]: value } }
          : m,
      ),
    })),

  // Templates
  templates: defaultTemplates,
  selectedTemplate: null,
  selectTemplate: (templateId) => {
    const state = get();
    const template = state.templates.find((t) => t.id === templateId);
    if (template) {
      // Apply template settings
      const modulesWithTarget = template.modules.map((m) => ({
        ...m,
        id: `${m.id}-${Date.now()}`,
        options: {
          ...m.options,
          ...(m.path.startsWith("scanner/")
            ? { RHOSTS: state.target }
            : { TARGET: state.target }),
        },
      }));

      set({
        selectedTemplate: template,
        targetType: template.recommended_target_type,
        scope: template.default_scope,
        intensity: template.default_intensity,
        selectedModules: modulesWithTarget,
      });
    }
  },

  // Workflow execution
  workflowConfig: null,
  workflowProgress: null,
  isExecuting: false,
  isPaused: false,
  setWorkflowConfig: (workflowConfig) => set({ workflowConfig }),
  setWorkflowProgress: (workflowProgress) => set({ workflowProgress }),
  setIsExecuting: (isExecuting) => set({ isExecuting }),
  setIsPaused: (isPaused) => set({ isPaused }),

  // Discoveries
  discoveries: [],
  addDiscovery: (discovery) =>
    set((state) => ({
      discoveries: [...state.discoveries, discovery],
    })),
  clearDiscoveries: () => set({ discoveries: [] }),

  // Report
  report: null,
  setReport: (report) => set({ report }),

  // Event log
  events: [],
  addEvent: (event) =>
    set((state) => ({
      events: [...state.events, event],
    })),
  clearEvents: () => set({ events: [] }),

  // Build workflow config
  buildWorkflowConfig: () => {
    const state = get();
    const now = new Date().toISOString();
    return {
      id: `workflow-${Date.now()}`,
      name: `Assessment - ${state.target}`,
      target: {
        target_type: state.targetType,
        target: state.target,
        resolved_targets: [],
        authorized: state.authorized,
        authorization_ref: state.authorizationRef,
        notes: state.targetNotes,
      },
      scope: state.scope,
      intensity: state.intensity,
      modules: state.selectedModules.filter((m) => m.enabled),
      created_at: now,
      modified_at: now,
    };
  },

  // Reset wizard
  resetWizard: () =>
    set({
      wizardStep: "target",
      targetType: "domain",
      target: "",
      authorized: false,
      authorizationRef: "",
      targetNotes: "",
      scope: "discovery",
      intensity: "normal",
      selectedModules: [],
      selectedTemplate: null,
      workflowConfig: null,
      workflowProgress: null,
      isExecuting: false,
      isPaused: false,
      discoveries: [],
      report: null,
      events: [],
    }),
}));
