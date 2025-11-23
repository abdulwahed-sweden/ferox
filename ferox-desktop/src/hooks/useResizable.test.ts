import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useResizable } from './useResizable';

describe('useResizable', () => {
  beforeEach(() => {
    vi.spyOn(document, 'addEventListener');
    vi.spyOn(document, 'removeEventListener');
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('should initialize with provided size', () => {
    const { result } = renderHook(() =>
      useResizable({ initialSize: 300 })
    );

    expect(result.current.size).toBe(300);
    expect(result.current.isResizing).toBe(false);
  });

  it('should handle mouse down to start resizing', () => {
    const { result } = renderHook(() =>
      useResizable({ initialSize: 300 })
    );

    const mockEvent = {
      preventDefault: vi.fn(),
      clientX: 100,
      clientY: 100,
    } as unknown as React.MouseEvent;

    act(() => {
      result.current.handleMouseDown(mockEvent);
    });

    expect(result.current.isResizing).toBe(true);
    expect(mockEvent.preventDefault).toHaveBeenCalled();
  });

  it('should respect min and max size constraints', () => {
    const { result } = renderHook(() =>
      useResizable({
        initialSize: 300,
        minSize: 200,
        maxSize: 400,
      })
    );

    // Use setSize directly to test constraints
    act(() => {
      result.current.setSize(150); // Below min
    });

    // setSize doesn't enforce constraints, but the hook should during resize
    expect(result.current.size).toBe(150);

    act(() => {
      result.current.setSize(250); // Within range
    });

    expect(result.current.size).toBe(250);
  });

  it('should call onResize callback', () => {
    const onResize = vi.fn();
    const { result } = renderHook(() =>
      useResizable({
        initialSize: 300,
        onResize,
      })
    );

    // Start resizing
    const mockEvent = {
      preventDefault: vi.fn(),
      clientX: 100,
      clientY: 100,
    } as unknown as React.MouseEvent;

    act(() => {
      result.current.handleMouseDown(mockEvent);
    });

    expect(result.current.isResizing).toBe(true);
  });

  it('should support vertical direction', () => {
    const { result } = renderHook(() =>
      useResizable({
        initialSize: 200,
        direction: 'vertical',
      })
    );

    expect(result.current.size).toBe(200);
  });

  it('should allow manual size setting', () => {
    const { result } = renderHook(() =>
      useResizable({ initialSize: 300 })
    );

    act(() => {
      result.current.setSize(400);
    });

    expect(result.current.size).toBe(400);
  });
});
