// src/hooks/useAsyncCommand.ts
// Standardized async command execution with loading, error, and retry support

import { useState, useCallback, useRef, useEffect } from "react";
import { TauriTimeoutError, TauriInvokeError } from "@/lib/tauri";

export interface UseAsyncCommandOptions<T> {
  onSuccess?: (data: T) => void;
  onError?: (error: Error) => void;
  retryCount?: number;
  retryDelay?: number;
  initialData?: T;
}

export interface UseAsyncCommandReturn<T, A extends unknown[] = []> {
  data: T | null;
  loading: boolean;
  error: Error | null;
  execute: (...args: A) => Promise<T>;
  reset: () => void;
  retry: () => Promise<T | null>;
  isTimeout: boolean;
}

/**
 * Hook for executing async Tauri commands with standardized loading/error states
 *
 * @param commandFn - The async function to execute (typically a Tauri command)
 * @param options - Configuration options
 * @returns Object with data, loading, error states and control functions
 *
 * @example
 * const { data, loading, error, execute, retry } = useAsyncCommand(
 *   (id: string) => getSession(id),
 *   { onError: (e) => toast.error(e.message) }
 * );
 *
 * // Execute the command
 * await execute(sessionId);
 *
 * // Retry last execution
 * await retry();
 */
export function useAsyncCommand<T, A extends unknown[] = []>(
  commandFn: (...args: A) => Promise<T>,
  options: UseAsyncCommandOptions<T> = {}
): UseAsyncCommandReturn<T, A> {
  const {
    onSuccess,
    onError,
    retryCount = 0,
    retryDelay = 1000,
    initialData,
  } = options;

  const [data, setData] = useState<T | null>(initialData ?? null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [isTimeout, setIsTimeout] = useState(false);

  // Store last args for retry functionality
  const lastArgsRef = useRef<A | null>(null);
  const isMountedRef = useRef(true);

  useEffect(() => {
    isMountedRef.current = true;
    return () => {
      isMountedRef.current = false;
    };
  }, []);

  const execute = useCallback(
    async (...args: A): Promise<T> => {
      lastArgsRef.current = args;
      setLoading(true);
      setError(null);
      setIsTimeout(false);

      let lastError: Error | null = null;
      let attempts = 0;

      while (attempts <= retryCount) {
        try {
          const result = await commandFn(...args);

          if (isMountedRef.current) {
            setData(result);
            setLoading(false);
            onSuccess?.(result);
          }

          return result;
        } catch (err) {
          lastError = err instanceof Error ? err : new Error(String(err));

          if (err instanceof TauriTimeoutError) {
            if (isMountedRef.current) {
              setIsTimeout(true);
            }
            // Don't retry on timeouts
            break;
          }

          attempts++;

          if (attempts <= retryCount) {
            await new Promise((resolve) => setTimeout(resolve, retryDelay));
          }
        }
      }

      if (isMountedRef.current) {
        setError(lastError);
        setLoading(false);
        if (lastError) {
          onError?.(lastError);
        }
      }

      throw lastError;
    },
    [commandFn, onSuccess, onError, retryCount, retryDelay]
  );

  const reset = useCallback(() => {
    setData(initialData ?? null);
    setLoading(false);
    setError(null);
    setIsTimeout(false);
    lastArgsRef.current = null;
  }, [initialData]);

  const retry = useCallback(async (): Promise<T | null> => {
    if (lastArgsRef.current === null) {
      console.warn("useAsyncCommand: No previous execution to retry");
      return null;
    }

    try {
      return await execute(...lastArgsRef.current);
    } catch {
      return null;
    }
  }, [execute]);

  return {
    data,
    loading,
    error,
    execute,
    reset,
    retry,
    isTimeout,
  };
}

/**
 * Hook for lazy-loaded data that fetches on mount
 *
 * @example
 * const { data, loading, error, refetch } = useAsyncData(
 *   () => getSessions(),
 *   { deps: [refreshTrigger] }
 * );
 */
export interface UseAsyncDataOptions<T> extends UseAsyncCommandOptions<T> {
  deps?: unknown[];
  enabled?: boolean;
}

export function useAsyncData<T>(
  fetchFn: () => Promise<T>,
  options: UseAsyncDataOptions<T> = {}
): UseAsyncCommandReturn<T, []> & { refetch: () => Promise<T | null> } {
  const { deps = [], enabled = true, ...commandOptions } = options;
  const command = useAsyncCommand(fetchFn, commandOptions);
  const hasExecutedRef = useRef(false);

  useEffect(() => {
    if (enabled) {
      hasExecutedRef.current = true;
      command.execute().catch(() => {
        // Error is already handled by the hook
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [enabled, ...deps]);

  const refetch = useCallback(async (): Promise<T | null> => {
    try {
      return await command.execute();
    } catch {
      return null;
    }
  }, [command]);

  return {
    ...command,
    refetch,
  };
}

/**
 * Hook for polling data at regular intervals
 *
 * @example
 * const { data, loading, stop, start } = usePolling(
 *   () => getSessionStatus(id),
 *   { interval: 5000 }
 * );
 */
export interface UsePollingOptions<T> extends UseAsyncCommandOptions<T> {
  interval: number;
  enabled?: boolean;
  stopOnError?: boolean;
}

export function usePolling<T>(
  fetchFn: () => Promise<T>,
  options: UsePollingOptions<T>
): UseAsyncCommandReturn<T, []> & {
  start: () => void;
  stop: () => void;
  isPolling: boolean;
} {
  const { interval, enabled = true, stopOnError = false, ...commandOptions } = options;
  const command = useAsyncCommand(fetchFn, commandOptions);
  const [isPolling, setIsPolling] = useState(enabled);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const stop = useCallback(() => {
    setIsPolling(false);
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const start = useCallback(() => {
    setIsPolling(true);
  }, []);

  useEffect(() => {
    if (!isPolling) {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      return;
    }

    // Initial fetch
    command.execute().catch(() => {
      if (stopOnError) {
        stop();
      }
    });

    // Set up interval
    intervalRef.current = setInterval(() => {
      command.execute().catch(() => {
        if (stopOnError) {
          stop();
        }
      });
    }, interval);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isPolling, interval, stopOnError]);

  return {
    ...command,
    start,
    stop,
    isPolling,
  };
}

// Re-export error types for convenience
export { TauriTimeoutError, TauriInvokeError };
