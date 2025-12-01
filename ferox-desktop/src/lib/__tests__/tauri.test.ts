// src/lib/__tests__/tauri.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import {
  invokeWithTimeout,
  TauriTimeoutError,
  TauriInvokeError,
  TIMEOUTS,
  getConnectionState,
  onConnectionStateChange,
} from "../tauri";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("TIMEOUTS", () => {
  it("has expected timeout values", () => {
    expect(TIMEOUTS.QUICK).toBe(5000);
    expect(TIMEOUTS.STANDARD).toBe(30000);
    expect(TIMEOUTS.LONG).toBe(60000);
    expect(TIMEOUTS.VERY_LONG).toBe(120000);
  });
});

describe("TauriTimeoutError", () => {
  it("creates error with correct message", () => {
    const error = new TauriTimeoutError("test_command", 5000);

    expect(error.name).toBe("TauriTimeoutError");
    expect(error.message).toBe('Command "test_command" timed out after 5000ms');
    expect(error).toBeInstanceOf(Error);
  });
});

describe("TauriInvokeError", () => {
  it("creates error with Error as cause", () => {
    const originalError = new Error("Original error");
    const error = new TauriInvokeError("test_command", originalError);

    expect(error.name).toBe("TauriInvokeError");
    expect(error.message).toBe('Command "test_command" failed: Original error');
    expect(error.command).toBe("test_command");
    expect(error.originalError).toBe(originalError);
  });

  it("creates error with string as cause", () => {
    const error = new TauriInvokeError("test_command", "String error");

    expect(error.message).toBe('Command "test_command" failed: String error');
    expect(error.originalError).toBe("String error");
  });
});

describe("invokeWithTimeout", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("resolves with result on success", async () => {
    vi.mocked(invoke).mockResolvedValue({ data: "success" });

    const result = await invokeWithTimeout("test_command", { arg: "value" });

    expect(result).toEqual({ data: "success" });
    expect(invoke).toHaveBeenCalledWith("test_command", { arg: "value" });
  });

  it("calls invoke with correct arguments", async () => {
    vi.mocked(invoke).mockResolvedValue("result");

    await invokeWithTimeout("my_command", { foo: "bar", num: 42 });

    expect(invoke).toHaveBeenCalledWith("my_command", { foo: "bar", num: 42 });
  });

  it("calls invoke without args when not provided", async () => {
    vi.mocked(invoke).mockResolvedValue("result");

    await invokeWithTimeout("simple_command");

    expect(invoke).toHaveBeenCalledWith("simple_command", undefined);
  });

  it("propagates invoke errors as-is when they are Error instances", async () => {
    const originalError = new Error("Invoke failed");
    vi.mocked(invoke).mockRejectedValue(originalError);

    await expect(invokeWithTimeout("failing_command")).rejects.toThrow(
      "Invoke failed"
    );
  });

  it("wraps non-Error rejections in TauriInvokeError", async () => {
    vi.mocked(invoke).mockRejectedValue("String rejection");

    try {
      await invokeWithTimeout("test_command");
      expect.fail("Should have thrown");
    } catch (error) {
      expect(error).toBeInstanceOf(TauriInvokeError);
      expect((error as Error).message).toBe(
        'Command "test_command" failed: String rejection'
      );
    }
  });

  it("retries on failure and succeeds on final attempt", async () => {
    vi.mocked(invoke)
      .mockRejectedValueOnce(new Error("First failure"))
      .mockRejectedValueOnce(new Error("Second failure"))
      .mockResolvedValue("success");

    const onRetry = vi.fn();

    const result = await invokeWithTimeout("test_command", undefined, {
      retries: 2,
      retryDelay: 0, // No delay for test speed
      onRetry,
    });

    expect(result).toBe("success");
    expect(invoke).toHaveBeenCalledTimes(3);
    expect(onRetry).toHaveBeenCalledTimes(2);
    expect(onRetry).toHaveBeenNthCalledWith(1, 1, expect.any(Error));
    expect(onRetry).toHaveBeenNthCalledWith(2, 2, expect.any(Error));
  });

  it("throws after exhausting all retries", async () => {
    vi.mocked(invoke).mockRejectedValue(new Error("Persistent failure"));

    try {
      await invokeWithTimeout("test_command", undefined, {
        retries: 2,
        retryDelay: 0,
      });
      expect.fail("Should have thrown");
    } catch (error) {
      expect((error as Error).message).toBe("Persistent failure");
    }
    expect(invoke).toHaveBeenCalledTimes(3);
  });
});

describe("Connection state management", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("getConnectionState returns current state", () => {
    // Initial state should be connected
    const state = getConnectionState();
    expect(["connected", "disconnected", "connecting"]).toContain(state);
  });

  it("onConnectionStateChange registers listener and returns unsubscribe", () => {
    const listener = vi.fn();
    const unsubscribe = onConnectionStateChange(listener);

    expect(typeof unsubscribe).toBe("function");

    // Unsubscribe
    unsubscribe();

    // Listener should not be called after unsubscribe
    // (This is harder to test without triggering state change)
  });
});
