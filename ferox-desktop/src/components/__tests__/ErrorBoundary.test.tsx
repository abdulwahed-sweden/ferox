// src/components/__tests__/ErrorBoundary.test.tsx
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import {
  ErrorBoundary,
  InlineErrorFallback,
  withErrorBoundary,
} from "../ErrorBoundary";

// Component that throws an error
function ThrowingComponent({ shouldThrow = true }: { shouldThrow?: boolean }) {
  if (shouldThrow) {
    throw new Error("Test error");
  }
  return <div>No error</div>;
}


describe("ErrorBoundary", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("error catching", () => {
    it("renders children when there is no error", () => {
      render(
        <ErrorBoundary>
          <div>Test content</div>
        </ErrorBoundary>
      );

      expect(screen.getByText("Test content")).toBeInTheDocument();
    });

    it("catches errors and displays default fallback UI", () => {
      render(
        <ErrorBoundary>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      expect(screen.getByText("Something went wrong")).toBeInTheDocument();
      expect(screen.getByText("Try Again")).toBeInTheDocument();
    });

    it("displays custom fallback when provided", () => {
      render(
        <ErrorBoundary fallback={<div>Custom error UI</div>}>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      expect(screen.getByText("Custom error UI")).toBeInTheDocument();
      expect(screen.queryByText("Something went wrong")).not.toBeInTheDocument();
    });

    it("displays component name in error message when provided", () => {
      render(
        <ErrorBoundary name="TestComponent">
          <ThrowingComponent />
        </ErrorBoundary>
      );

      expect(
        screen.getByText("The TestComponent component encountered an error.")
      ).toBeInTheDocument();
    });

    it("calls onError callback when error occurs", () => {
      const onError = vi.fn();

      render(
        <ErrorBoundary onError={onError}>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      expect(onError).toHaveBeenCalledTimes(1);
      expect(onError).toHaveBeenCalledWith(
        expect.any(Error),
        expect.objectContaining({
          componentStack: expect.any(String),
        })
      );
    });
  });

  describe("retry functionality", () => {
    it("resets error state when Try Again is clicked", async () => {
      let shouldThrow = true;

      function ConditionalThrow() {
        if (shouldThrow) {
          throw new Error("Test error");
        }
        return <div>Recovered</div>;
      }

      const { rerender } = render(
        <ErrorBoundary>
          <ConditionalThrow />
        </ErrorBoundary>
      );

      // Initial error state
      expect(screen.getByText("Something went wrong")).toBeInTheDocument();

      // Fix the error condition
      shouldThrow = false;

      // Click retry
      fireEvent.click(screen.getByText("Try Again"));

      // Force rerender
      rerender(
        <ErrorBoundary>
          <ConditionalThrow />
        </ErrorBoundary>
      );

      // Should show recovered content
      await waitFor(() => {
        expect(screen.getByText("Recovered")).toBeInTheDocument();
      });
    });

    it("calls onRetry callback when retry button is clicked", () => {
      const onRetry = vi.fn();

      render(
        <ErrorBoundary onRetry={onRetry}>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      fireEvent.click(screen.getByText("Try Again"));

      expect(onRetry).toHaveBeenCalledTimes(1);
    });
  });

  describe("error details", () => {
    it("shows error details when Show Details is clicked", () => {
      render(
        <ErrorBoundary>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      // Initially details are hidden
      expect(screen.queryByText("Error Details")).not.toBeInTheDocument();

      // Click show details
      fireEvent.click(screen.getByText("Show Details"));

      // Details should be visible
      expect(screen.getByText("Error Details")).toBeInTheDocument();
      expect(screen.getByText("Test error")).toBeInTheDocument();
    });

    it("hides error details when Hide Details is clicked", () => {
      render(
        <ErrorBoundary>
          <ThrowingComponent />
        </ErrorBoundary>
      );

      // Show details first
      fireEvent.click(screen.getByText("Show Details"));
      expect(screen.getByText("Error Details")).toBeInTheDocument();

      // Hide details
      fireEvent.click(screen.getByText("Hide Details"));
      expect(screen.queryByText("Error Details")).not.toBeInTheDocument();
    });

    it("copies error details to clipboard when Copy is clicked", async () => {
      const writeText = vi.fn().mockResolvedValue(undefined);
      Object.assign(navigator, {
        clipboard: { writeText },
      });

      render(
        <ErrorBoundary name="TestComponent">
          <ThrowingComponent />
        </ErrorBoundary>
      );

      // Show details
      fireEvent.click(screen.getByText("Show Details"));

      // Click copy button
      fireEvent.click(screen.getByText("Copy"));

      await waitFor(() => {
        expect(writeText).toHaveBeenCalledWith(
          expect.stringContaining("Error: Test error")
        );
        expect(writeText).toHaveBeenCalledWith(
          expect.stringContaining("Component: TestComponent")
        );
      });

      // Should show "Copied" temporarily
      expect(screen.getByText("Copied")).toBeInTheDocument();
    });
  });
});

describe("InlineErrorFallback", () => {
  it("renders error message", () => {
    const error = new Error("Test inline error");
    const onRetry = vi.fn();

    render(<InlineErrorFallback error={error} onRetry={onRetry} />);

    expect(screen.getByText("Error loading content")).toBeInTheDocument();
    expect(screen.getByText("Test inline error")).toBeInTheDocument();
  });

  it("renders without error message", () => {
    const onRetry = vi.fn();

    render(<InlineErrorFallback onRetry={onRetry} />);

    expect(screen.getByText("Error loading content")).toBeInTheDocument();
    expect(screen.getByText("Retry")).toBeInTheDocument();
  });

  it("calls onRetry when Retry button is clicked", () => {
    const onRetry = vi.fn();

    render(<InlineErrorFallback onRetry={onRetry} />);

    fireEvent.click(screen.getByText("Retry"));

    expect(onRetry).toHaveBeenCalledTimes(1);
  });
});

describe("withErrorBoundary HOC", () => {
  it("wraps component with error boundary", () => {
    function MyComponent() {
      return <div>My component content</div>;
    }

    const WrappedComponent = withErrorBoundary(MyComponent, "MyComponent");

    render(<WrappedComponent />);

    expect(screen.getByText("My component content")).toBeInTheDocument();
  });

  it("catches errors in wrapped component", () => {
    const WrappedThrowing = withErrorBoundary(ThrowingComponent, "Throwing");

    render(<WrappedThrowing />);

    expect(screen.getByText("Something went wrong")).toBeInTheDocument();
    expect(
      screen.getByText("The Throwing component encountered an error.")
    ).toBeInTheDocument();
  });

  it("uses displayName when name is not provided", () => {
    function MyNamedComponent() {
      return <div>Named component</div>;
    }
    MyNamedComponent.displayName = "CustomDisplayName";

    const WrappedComponent = withErrorBoundary(MyNamedComponent);

    render(<WrappedComponent />);

    expect(screen.getByText("Named component")).toBeInTheDocument();
  });
});
