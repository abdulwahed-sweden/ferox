import { useState, useCallback, useEffect, useRef } from "react";

interface UseResizableOptions {
  initialSize: number;
  minSize?: number;
  maxSize?: number;
  direction?: "horizontal" | "vertical";
  onResize?: (size: number) => void;
}

/**
 * Hook for creating resizable panels
 */
export function useResizable({
  initialSize,
  minSize = 150,
  maxSize = 500,
  direction = "horizontal",
  onResize,
}: UseResizableOptions) {
  const [size, setSize] = useState(initialSize);
  const [isResizing, setIsResizing] = useState(false);
  const startPosRef = useRef(0);
  const startSizeRef = useRef(0);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      setIsResizing(true);
      startPosRef.current = direction === "horizontal" ? e.clientX : e.clientY;
      startSizeRef.current = size;
    },
    [direction, size],
  );

  useEffect(() => {
    if (!isResizing) return;

    const handleMouseMove = (e: MouseEvent) => {
      const currentPos = direction === "horizontal" ? e.clientX : e.clientY;
      const delta = currentPos - startPosRef.current;
      const newSize = Math.min(
        maxSize,
        Math.max(minSize, startSizeRef.current + delta),
      );

      setSize(newSize);
      onResize?.(newSize);
    };

    const handleMouseUp = () => {
      setIsResizing(false);
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);

    // Add cursor style to body during resize
    document.body.style.cursor =
      direction === "horizontal" ? "col-resize" : "row-resize";
    document.body.style.userSelect = "none";

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };
  }, [isResizing, direction, minSize, maxSize, onResize]);

  return {
    size,
    isResizing,
    handleMouseDown,
    setSize,
  };
}
