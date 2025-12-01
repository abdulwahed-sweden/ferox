// src/components/ErrorBoundary.tsx
// Reusable error boundary with retry functionality

import { Component, ReactNode, useState } from "react";
import { AlertTriangle, RefreshCw, Bug, Copy, Check } from "lucide-react";

interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
  onRetry?: () => void;
  name?: string;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
  errorInfo: React.ErrorInfo | null;
}

export class ErrorBoundary extends Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<ErrorBoundaryState> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.setState({ errorInfo });
    const boundaryName = this.props.name ? `: ${this.props.name}` : "";
    console.error(`[ErrorBoundary${boundaryName}]`, error, errorInfo);
    this.props.onError?.(error, errorInfo);
  }

  handleRetry = () => {
    this.setState({ hasError: false, error: null, errorInfo: null });
    this.props.onRetry?.();
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }
      return (
        <ErrorFallback
          error={this.state.error}
          errorInfo={this.state.errorInfo}
          componentName={this.props.name}
          onRetry={this.handleRetry}
        />
      );
    }
    return this.props.children;
  }
}

interface ErrorFallbackProps {
  error: Error | null;
  errorInfo: React.ErrorInfo | null;
  componentName?: string;
  onRetry: () => void;
}

function ErrorFallback({
  error,
  errorInfo,
  componentName,
  onRetry,
}: ErrorFallbackProps) {
  const [showDetails, setShowDetails] = useState(false);
  const [copied, setCopied] = useState(false);

  const copyErrorDetails = async () => {
    const details = [
      "Error: " + (error?.message || "Unknown error"),
      "Component: " + (componentName || "Unknown"),
      "Stack: " + (error?.stack || "No stack trace"),
      "Component Stack: " + (errorInfo?.componentStack || "No component stack"),
    ].join("\n");
    await navigator.clipboard.writeText(details);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-[200px] p-6 bg-surface-default rounded-xl border border-border-subtle">
      <div className="w-16 h-16 rounded-full bg-danger-soft flex items-center justify-center mb-4">
        <AlertTriangle className="w-8 h-8 text-danger-text" />
      </div>
      <h3 className="text-lg font-semibold text-content-primary mb-2">
        Something went wrong
      </h3>
      <p className="text-sm text-content-secondary text-center max-w-md mb-4">
        {componentName
          ? `The ${componentName} component encountered an error.`
          : "An unexpected error occurred in this component."}
      </p>
      <div className="flex items-center gap-3 mb-4">
        <button
          onClick={onRetry}
          className="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary-new text-white font-medium text-sm hover:bg-primary-hover transition-colors"
        >
          <RefreshCw className="w-4 h-4" />
          Try Again
        </button>
        <button
          onClick={() => setShowDetails(!showDetails)}
          className="flex items-center gap-2 px-4 py-2 rounded-lg bg-surface-hover text-content-secondary font-medium text-sm hover:bg-surface-active transition-colors"
        >
          <Bug className="w-4 h-4" />
          {showDetails ? "Hide Details" : "Show Details"}
        </button>
      </div>
      {showDetails && (
        <div className="w-full max-w-lg mt-2">
          <div className="flex items-center justify-between mb-2">
            <span className="text-xs font-mono text-content-tertiary uppercase tracking-wider">
              Error Details
            </span>
            <button
              onClick={copyErrorDetails}
              className="flex items-center gap-1.5 px-2 py-1 rounded text-xs text-content-secondary hover:bg-surface-hover transition-colors"
            >
              {copied ? (
                <>
                  <Check className="w-3 h-3 text-success-text" />
                  Copied
                </>
              ) : (
                <>
                  <Copy className="w-3 h-3" />
                  Copy
                </>
              )}
            </button>
          </div>
          <div className="bg-surface-elevated rounded-lg p-4 border border-border-subtle overflow-auto max-h-48">
            <pre className="text-xs font-mono text-danger-text whitespace-pre-wrap break-words">
              {error?.message || "Unknown error"}
            </pre>
            {error?.stack && (
              <pre className="text-xs font-mono text-content-tertiary whitespace-pre-wrap break-words mt-2 pt-2 border-t border-border-subtle">
                {error.stack.split("\n").slice(1, 6).join("\n")}
              </pre>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export function InlineErrorFallback({
  error,
  onRetry,
}: {
  error?: Error | null;
  onRetry: () => void;
}) {
  return (
    <div className="flex items-center gap-3 p-3 bg-danger-soft rounded-lg border border-danger-border">
      <AlertTriangle className="w-5 h-5 text-danger-text flex-shrink-0" />
      <div className="flex-1 min-w-0">
        <p className="text-sm text-danger-text font-medium">Error loading content</p>
        {error?.message && (
          <p className="text-xs text-danger-text/70 truncate">{error.message}</p>
        )}
      </div>
      <button
        onClick={onRetry}
        className="flex items-center gap-1.5 px-3 py-1.5 rounded bg-danger-new/20 text-danger-text text-xs font-medium hover:bg-danger-new/30 transition-colors"
      >
        <RefreshCw className="w-3 h-3" />
        Retry
      </button>
    </div>
  );
}

export function withErrorBoundary<P extends object>(
  WrappedComponent: React.ComponentType<P>,
  name?: string
) {
  return function WithErrorBoundary(props: P) {
    return (
      <ErrorBoundary name={name || WrappedComponent.displayName}>
        <WrappedComponent {...props} />
      </ErrorBoundary>
    );
  };
}

export default ErrorBoundary;
