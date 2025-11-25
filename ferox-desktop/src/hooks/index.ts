// Custom hooks for Ferox Desktop

// Existing hooks
export { useTauriEvents } from './useTauriEvents';
export { useTheme } from './useTheme';
export type { Theme } from './useTheme';
export { useDebounce } from './useDebounce';
export { useKeyboardShortcuts } from './useKeyboardShortcuts';
export { useResizable } from './useResizable';

// New service-connected hooks (Phase 1)
export { useScan } from './useScan';
export { useRecon } from './useRecon';
export { useOpsec } from './useOpsec';
export { useNetworkMap } from './useNetworkMap';
