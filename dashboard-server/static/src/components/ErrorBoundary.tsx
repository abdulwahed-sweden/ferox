import { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo);
  }

  private handleRetry = () => {
    this.setState({ hasError: false, error: null });
  };

  public render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="flex flex-col items-center justify-center min-h-[400px] p-8">
          <div className="bg-dark-700 border border-danger/30 rounded-lg p-8 max-w-md text-center">
            <AlertTriangle size={48} className="text-danger mx-auto mb-4" />
            <h2 className="text-xl font-semibold text-text-primary mb-2">
              Something went wrong
            </h2>
            <p className="text-text-secondary mb-4">
              An unexpected error occurred. Please try refreshing the page.
            </p>
            {this.state.error && (
              <details className="mb-4 text-left">
                <summary className="text-sm text-text-muted cursor-pointer hover:text-text-secondary">
                  Error details
                </summary>
                <pre className="mt-2 p-2 bg-dark-900 rounded text-xs text-danger overflow-auto max-h-32">
                  {this.state.error.message}
                </pre>
              </details>
            )}
            <div className="flex gap-3 justify-center">
              <button
                onClick={this.handleRetry}
                className="btn-primary"
              >
                <RefreshCw size={16} />
                Try Again
              </button>
              <button
                onClick={() => window.location.reload()}
                className="btn-outline"
              >
                Refresh Page
              </button>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

// Simple error page for full page errors
export function ErrorPage({ message }: { message?: string }) {
  return (
    <div className="min-h-screen bg-dark-900 flex items-center justify-center p-8">
      <div className="bg-dark-700 border border-dark-500 rounded-lg p-8 max-w-md text-center">
        <AlertTriangle size={64} className="text-danger mx-auto mb-4" />
        <h1 className="text-2xl font-bold text-text-primary mb-2">
          Oops! Something went wrong
        </h1>
        <p className="text-text-secondary mb-6">
          {message || 'We encountered an unexpected error. Please refresh the page to try again.'}
        </p>
        <button
          onClick={() => window.location.reload()}
          className="btn-primary"
        >
          <RefreshCw size={16} />
          Refresh Page
        </button>
      </div>
    </div>
  );
}

// 404 Not Found page
export function NotFoundPage() {
  return (
    <div className="min-h-screen bg-dark-900 flex items-center justify-center p-8">
      <div className="text-center">
        <h1 className="text-8xl font-bold text-ferox-green mb-4">404</h1>
        <h2 className="text-2xl font-semibold text-text-primary mb-2">
          Page Not Found
        </h2>
        <p className="text-text-secondary mb-6">
          The page you're looking for doesn't exist or has been moved.
        </p>
        <button
          onClick={() => window.location.href = '/'}
          className="btn-primary"
        >
          Go to Dashboard
        </button>
      </div>
    </div>
  );
}
