import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useDebounce } from "./useDebounce";

describe("useDebounce", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("should return initial value immediately", () => {
    const { result } = renderHook(() => useDebounce("initial", 500));
    expect(result.current).toBe("initial");
  });

  it("should debounce value changes", () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebounce(value, delay),
      { initialProps: { value: "initial", delay: 500 } },
    );

    expect(result.current).toBe("initial");

    // Update value
    rerender({ value: "updated", delay: 500 });

    // Value should not have changed yet
    expect(result.current).toBe("initial");

    // Fast forward time
    act(() => {
      vi.advanceTimersByTime(500);
    });

    // Now value should be updated
    expect(result.current).toBe("updated");
  });

  it("should reset timer on rapid changes", () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebounce(value, delay),
      { initialProps: { value: "a", delay: 300 } },
    );

    // Rapid changes
    rerender({ value: "b", delay: 300 });
    act(() => vi.advanceTimersByTime(100));

    rerender({ value: "c", delay: 300 });
    act(() => vi.advanceTimersByTime(100));

    rerender({ value: "d", delay: 300 });
    act(() => vi.advanceTimersByTime(100));

    // Should still be initial value
    expect(result.current).toBe("a");

    // Wait for full debounce period
    act(() => vi.advanceTimersByTime(300));

    // Should now be the last value
    expect(result.current).toBe("d");
  });

  it("should handle different delay values", () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebounce(value, delay),
      { initialProps: { value: "test", delay: 1000 } },
    );

    rerender({ value: "changed", delay: 1000 });

    act(() => vi.advanceTimersByTime(500));
    expect(result.current).toBe("test");

    act(() => vi.advanceTimersByTime(500));
    expect(result.current).toBe("changed");
  });
});
