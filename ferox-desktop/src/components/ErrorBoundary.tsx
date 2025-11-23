import { Component, type ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';

interface ErrorBoundaryProps {
  children: ReactNode;
  /** Name of the component/section being wrapped (for error messages) */
  name?: string;
  /** Optional fallback component */
  fallback?: ReactNode;
  /** Called when an error is caught */
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

/**
 * Error Boundary component for graceful error handling
 *
 * Catches JavaScript errors in child component tree and displays
 * a fallback UI instead of crashing the entire app.
 */
export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    console.error(`[ErrorBoundary${this.props.name ? `: ${this.props.name}` : ''}]`, error, errorInfo);
    this.props.onError?.(error, errorInfo);
  }

  handleRetry = (): void => {
    this.setState({ hasError: false, error: null });
  };

  render(): ReactNode {
    if (this.state.hasError) {
      // Custom fallback provided
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default fallback UI
      return (
        <div className="flex flex-col items-center justify-center h-full p-4 bg-dark-800">
          <AlertTriangle size={32} className="text-danger mb-3" />
          <h3 className="text-sm font-medium text-text-primary mb-1">
            {this.props.name ? `${this.props.name} Error` : 'Something went wrong'}
          </h3>
          <p className="text-xs text-text-muted text-center mb-4 max-w-[200px]">
            {this.state.error?.message || 'An unexpected error occurred'}
          </p>
          <button
            onClick={this.handleRetry}
            className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium
                       bg-dark-600 hover:bg-dark-500 text-text-primary rounded
                       transition-colors"
          >
            <RefreshCw size={12} />
            Retry
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

/**
 * Minimal error fallback for small components
 */
export function MinimalErrorFallback({
  message = 'Error',
  onRetry
}: {
  message?: string;
  onRetry?: () => void;
}) {
  return (
    <div className="flex items-center gap-2 p-2 text-xs text-danger">
      <AlertTriangle size={14} />
      <span>{message}</span>
      {onRetry && (
        <button
          onClick={onRetry}
          className="text-text-muted hover:text-text-primary transition-colors"
        >
          <RefreshCw size={12} />
        </button>
      )}
    </div>
  );
}

/**
 * Panel-specific error fallback with more context
 */
export function PanelErrorFallback({
  panelName,
  error,
  onRetry
}: {
  panelName: string;
  error?: Error | null;
  onRetry: () => void;
}) {
  return (
    <div className="flex flex-col items-center justify-center h-full min-h-[100px] p-4 bg-dark-900/50">
      <AlertTriangle size={24} className="text-warning mb-2" />
      <p className="text-sm text-text-primary mb-1">{panelName} unavailable</p>
      {error && (
        <p className="text-xs text-text-muted mb-3 text-center max-w-[250px] truncate">
          {error.message}
        </p>
      )}
      <button
        onClick={onRetry}
        className="flex items-center gap-1 px-2 py-1 text-xs
                   bg-dark-600 hover:bg-dark-500 rounded transition-colors"
      >
        <RefreshCw size={10} />
        Reload Panel
      </button>
    </div>
  );
}
