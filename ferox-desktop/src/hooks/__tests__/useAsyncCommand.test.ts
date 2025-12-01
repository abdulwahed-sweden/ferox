// src/hooks/__tests__/useAsyncCommand.test.ts
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import {
  useAsyncCommand,
  useAsyncData,
  usePolling,
  TauriTimeoutError,
} from "../useAsyncCommand";

describe("useAsyncCommand", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.clearAllMocks();
  });

  describe("basic functionality", () => {
    it("initializes with default state", () => {
      const commandFn = vi.fn().mockResolvedValue("result");
      const { result } = renderHook(() => useAsyncCommand(commandFn));

      expect(result.current.data).toBeNull();
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.isTimeout).toBe(false);
    });

    it("initializes with initialData when provided", () => {
      const commandFn = vi.fn().mockResolvedValue("result");
      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, { initialData: "initial" })
      );

      expect(result.current.data).toBe("initial");
    });

    it("executes command and updates state on success", async () => {
      const commandFn = vi.fn().mockResolvedValue("success result");
      const onSuccess = vi.fn();
      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, { onSuccess })
      );

      let executePromise: Promise<string>;
      act(() => {
        executePromise = result.current.execute();
      });

      // Loading state
      expect(result.current.loading).toBe(true);

      await act(async () => {
        await executePromise;
      });

      // Success state
      expect(result.current.loading).toBe(false);
      expect(result.current.data).toBe("success result");
      expect(result.current.error).toBeNull();
      expect(onSuccess).toHaveBeenCalledWith("success result");
    });

    it("handles errors correctly", async () => {
      const error = new Error("Command failed");
      const commandFn = vi.fn().mockRejectedValue(error);
      const onError = vi.fn();
      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, { onError })
      );

      await act(async () => {
        try {
          await result.current.execute();
        } catch {
          // Expected
        }
      });

      expect(result.current.loading).toBe(false);
      expect(result.current.error).toEqual(error);
      expect(result.current.data).toBeNull();
      expect(onError).toHaveBeenCalledWith(error);
    });

    it("handles timeout errors and sets isTimeout flag", async () => {
      const timeoutError = new TauriTimeoutError("test_command", 5000);
      const commandFn = vi.fn().mockRejectedValue(timeoutError);
      const { result } = renderHook(() => useAsyncCommand(commandFn));

      await act(async () => {
        try {
          await result.current.execute();
        } catch {
          // Expected
        }
      });

      expect(result.current.isTimeout).toBe(true);
      expect(result.current.error).toEqual(timeoutError);
    });
  });

  describe("with arguments", () => {
    it("passes arguments to command function", async () => {
      const commandFn = vi.fn().mockResolvedValue("result");
      const { result } = renderHook(() =>
        useAsyncCommand<string, [string, number]>(commandFn)
      );

      await act(async () => {
        await result.current.execute("arg1", 42);
      });

      expect(commandFn).toHaveBeenCalledWith("arg1", 42);
    });
  });

  describe("retry functionality", () => {
    it("retries failed commands", async () => {
      const commandFn = vi
        .fn()
        .mockRejectedValueOnce(new Error("First failure"))
        .mockRejectedValueOnce(new Error("Second failure"))
        .mockResolvedValue("success");

      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, {
          retryCount: 2,
          retryDelay: 100,
        })
      );

      await act(async () => {
        const promise = result.current.execute();
        await vi.advanceTimersByTimeAsync(200);
        await promise;
      });

      expect(commandFn).toHaveBeenCalledTimes(3);
      expect(result.current.data).toBe("success");
    });

    it("does not retry on timeout errors", async () => {
      const timeoutError = new TauriTimeoutError("cmd", 1000);
      const commandFn = vi.fn().mockRejectedValue(timeoutError);
      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, { retryCount: 3 })
      );

      await act(async () => {
        try {
          await result.current.execute();
        } catch {
          // Expected
        }
      });

      // Should only be called once - no retries for timeouts
      expect(commandFn).toHaveBeenCalledTimes(1);
    });

    it("retry() re-executes with last arguments", async () => {
      const commandFn = vi
        .fn()
        .mockRejectedValueOnce(new Error("First failure"))
        .mockResolvedValue("retry success");

      const { result } = renderHook(() =>
        useAsyncCommand<string, [string]>(commandFn)
      );

      // First execution fails
      await act(async () => {
        try {
          await result.current.execute("test-arg");
        } catch {
          // Expected
        }
      });

      expect(result.current.error).toBeTruthy();

      // Retry
      await act(async () => {
        await result.current.retry();
      });

      expect(commandFn).toHaveBeenLastCalledWith("test-arg");
      expect(result.current.data).toBe("retry success");
    });

    it("retry() returns null when no previous execution", async () => {
      const commandFn = vi.fn().mockResolvedValue("result");
      const { result } = renderHook(() => useAsyncCommand(commandFn));

      const consoleWarn = vi.spyOn(console, "warn").mockImplementation(() => {});

      let retryResult: unknown = undefined;
      await act(async () => {
        retryResult = await result.current.retry();
      });

      expect(retryResult).toBeNull();
      expect(consoleWarn).toHaveBeenCalled();
      consoleWarn.mockRestore();
    });
  });

  describe("reset functionality", () => {
    it("resets state to initial values", async () => {
      const commandFn = vi.fn().mockResolvedValue("result");
      const { result } = renderHook(() =>
        useAsyncCommand(commandFn, { initialData: "initial" })
      );

      // Execute first
      await act(async () => {
        await result.current.execute();
      });

      expect(result.current.data).toBe("result");

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.data).toBe("initial");
      expect(result.current.loading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.isTimeout).toBe(false);
    });
  });

  describe("unmount handling", () => {
    it("does not update state after unmount", async () => {
      let resolvePromise: (value: string) => void;
      const commandFn = vi.fn().mockImplementation(
        () =>
          new Promise((resolve) => {
            resolvePromise = resolve;
          })
      );

      const { result, unmount } = renderHook(() => useAsyncCommand(commandFn));

      act(() => {
        result.current.execute();
      });

      // Unmount before promise resolves
      unmount();

      // Resolve promise after unmount
      await act(async () => {
        resolvePromise!("result");
      });

      // No errors should occur - state updates are ignored after unmount
    });
  });
});

describe("useAsyncData", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("fetches data on mount", async () => {
    const fetchFn = vi.fn().mockResolvedValue("initial data");
    const { result } = renderHook(() => useAsyncData(fetchFn));

    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(fetchFn).toHaveBeenCalledTimes(1);
    expect(result.current.data).toBe("initial data");
  });

  it("does not fetch when disabled", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    renderHook(() => useAsyncData(fetchFn, { enabled: false }));

    await act(async () => {
      await vi.advanceTimersByTimeAsync(100);
    });

    expect(fetchFn).not.toHaveBeenCalled();
  });

  it("refetches when deps change", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    let dep = 1;

    const { rerender } = renderHook(() =>
      useAsyncData(fetchFn, { deps: [dep] })
    );

    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(fetchFn).toHaveBeenCalledTimes(1);

    dep = 2;
    rerender();

    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(fetchFn).toHaveBeenCalledTimes(2);
  });

  it("provides refetch function", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    const { result } = renderHook(() => useAsyncData(fetchFn));

    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(fetchFn).toHaveBeenCalledTimes(1);

    await act(async () => {
      await result.current.refetch();
    });

    expect(fetchFn).toHaveBeenCalledTimes(2);
  });
});

describe("usePolling", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("polls at specified interval", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    renderHook(() => usePolling(fetchFn, { interval: 1000 }));

    // Initial fetch
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(fetchFn).toHaveBeenCalledTimes(1);

    // After 1 second
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1000);
    });
    expect(fetchFn).toHaveBeenCalledTimes(2);

    // After another second
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1000);
    });
    expect(fetchFn).toHaveBeenCalledTimes(3);
  });

  it("can stop and start polling", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    const { result } = renderHook(() =>
      usePolling(fetchFn, { interval: 1000 })
    );

    // Initial fetch
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(fetchFn).toHaveBeenCalledTimes(1);

    // Stop polling
    act(() => {
      result.current.stop();
    });
    expect(result.current.isPolling).toBe(false);

    // Time passes but no more calls
    await act(async () => {
      await vi.advanceTimersByTimeAsync(3000);
    });
    expect(fetchFn).toHaveBeenCalledTimes(1);

    // Start polling again
    act(() => {
      result.current.start();
    });
    expect(result.current.isPolling).toBe(true);

    // Should fetch again
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(fetchFn).toHaveBeenCalledTimes(2);
  });

  it("stops on error when stopOnError is true", async () => {
    const fetchFn = vi
      .fn()
      .mockResolvedValueOnce("data")
      .mockRejectedValue(new Error("Poll failed"));

    const { result } = renderHook(() =>
      usePolling(fetchFn, { interval: 1000, stopOnError: true })
    );

    // Initial fetch succeeds
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });
    expect(fetchFn).toHaveBeenCalledTimes(1);
    expect(result.current.isPolling).toBe(true);

    // Second fetch fails
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1000);
    });
    expect(fetchFn).toHaveBeenCalledTimes(2);
    expect(result.current.isPolling).toBe(false);
  });

  it("does not poll when initially disabled", async () => {
    const fetchFn = vi.fn().mockResolvedValue("data");
    const { result } = renderHook(() =>
      usePolling(fetchFn, { interval: 1000, enabled: false })
    );

    await act(async () => {
      await vi.advanceTimersByTimeAsync(3000);
    });

    expect(fetchFn).not.toHaveBeenCalled();
    expect(result.current.isPolling).toBe(false);
  });
});
