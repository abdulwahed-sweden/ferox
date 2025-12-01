// src/components/cli/CLICommandDisplay.tsx
// CLI command display component with syntax highlighting and copy functionality

import { useState, ReactNode } from "react";
import { Check, Copy, ChevronDown, ChevronUp, Terminal } from "lucide-react";
import { CLICommand, CommandExample } from "@/types/cli-commands";

interface CLICommandDisplayProps {
  command: CLICommand;
  showExamples?: boolean;
  className?: string;
}

export function CLICommandDisplay({
  command,
  showExamples = true,
  className = "",
}: CLICommandDisplayProps) {
  const [copied, setCopied] = useState<string | null>(null);
  const [showAllExamples, setShowAllExamples] = useState(false);

  const copyToClipboard = async (text: string, id: string) => {
    await navigator.clipboard.writeText(text);
    setCopied(id);
    setTimeout(() => setCopied(null), 2000);
  };

  const displayedExamples = showAllExamples
    ? command.examples
    : command.examples.slice(0, 1);

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Main Command */}
      <div className="code-block">
        {/* Header */}
        <div className="code-block-header">
          <div className="flex items-center gap-3">
            <div className="flex gap-1.5">
              <div className="w-3 h-3 rounded-full bg-[hsl(354,70%,54%)]" />
              <div className="w-3 h-3 rounded-full bg-[hsl(45,93%,47%)]" />
              <div className="w-3 h-3 rounded-full bg-[hsl(140,60%,45%)]" />
            </div>
            <Terminal className="w-4 h-4 opacity-60" />
            <span className="code-block-title">ferox-cli</span>
          </div>
          <span className="font-mono text-xs px-2 py-1 rounded bg-primary-soft text-primary-text">
            {command.category}
          </span>
        </div>

        {/* Command Syntax */}
        <div className="code-block-content">
          <div className="flex items-start justify-between gap-4">
            <div className="flex-1">
              <code className="font-mono text-lg font-semibold text-code-keyword">
                {command.command}
              </code>
              <p className="mt-2 text-sm text-content-secondary">
                {command.description}
              </p>
            </div>
          </div>

          {/* Parameters */}
          <div className="mt-4 pt-4 border-t border-border-subtle">
            <h4 className="font-mono text-xs uppercase tracking-wider text-content-tertiary mb-3">
              Parameters
            </h4>
            <div className="grid gap-2">
              {command.parameters.map((param) => (
                <div
                  key={param.name}
                  className="flex items-start gap-3 text-sm"
                >
                  <code className="font-mono text-code-param whitespace-nowrap">
                    {param.flag}
                  </code>
                  <span className="text-content-secondary flex-1">
                    {param.description}
                    {param.required && (
                      <span className="text-danger-text ml-1">*</span>
                    )}
                    {param.default !== undefined && (
                      <span className="text-code-value ml-1">
                        (default: {String(param.default)})
                      </span>
                    )}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Examples */}
      {showExamples && command.examples.length > 0 && (
        <div className="space-y-3">
          <h4 className="font-mono text-sm font-semibold text-content-primary flex items-center gap-2">
            <span>Examples</span>
            {command.examples.length > 1 && (
              <button
                onClick={() => setShowAllExamples(!showAllExamples)}
                className="text-xs text-primary-text hover:underline flex items-center gap-1"
              >
                {showAllExamples ? (
                  <>
                    Show less <ChevronUp className="w-3 h-3" />
                  </>
                ) : (
                  <>
                    Show all ({command.examples.length}){" "}
                    <ChevronDown className="w-3 h-3" />
                  </>
                )}
              </button>
            )}
          </h4>

          {displayedExamples.map((example, index) => (
            <ExampleBlock
              key={index}
              example={example}
              copied={copied === `example-${index}`}
              onCopy={() =>
                copyToClipboard(example.command, `example-${index}`)
              }
            />
          ))}
        </div>
      )}
    </div>
  );
}

// Example Block Sub-component
interface ExampleBlockProps {
  example: CommandExample;
  copied: boolean;
  onCopy: () => void;
}

function ExampleBlock({ example, copied, onCopy }: ExampleBlockProps) {
  return (
    <div className="code-block">
      {/* Example Header */}
      <div className="code-block-header">
        <span className="font-mono text-sm font-medium">{example.title}</span>
        <button
          onClick={onCopy}
          className="code-block-copy"
          title={copied ? "Copied!" : "Copy code"}
        >
          {copied ? (
            <>
              <Check className="w-3.5 h-3.5 text-success-text" />
              <span className="text-success-text">Copied!</span>
            </>
          ) : (
            <>
              <Copy className="w-3.5 h-3.5" />
              <span>Copy</span>
            </>
          )}
        </button>
      </div>

      {/* Command */}
      <div className="code-block-content">
        <pre className="code-block-pre">
          <code>
            <SyntaxHighlight code={example.command} />
          </code>
        </pre>

        {/* Output */}
        {example.output && (
          <div className="mt-4 pt-4 border-t border-border-subtle">
            <span className="font-mono text-xs text-content-tertiary block mb-2">
              # Output:
            </span>
            <pre className="font-mono text-sm text-code-value whitespace-pre-wrap">
              {example.output}
            </pre>
          </div>
        )}
      </div>

      {/* Description */}
      {example.description && (
        <div className="px-4 py-2 bg-surface-hover border-t border-border-subtle">
          <p className="text-xs text-content-secondary">{example.description}</p>
        </div>
      )}
    </div>
  );
}

// Syntax Highlighting Helper
function SyntaxHighlight({ code }: { code: string }) {
  // Simple syntax highlighting for ferox commands
  const lines = code.split("\n");

  return (
    <>
      {lines.map((line, lineIndex) => (
        <div key={lineIndex}>
          <HighlightLine line={line} />
        </div>
      ))}
    </>
  );
}

function HighlightLine({ line }: { line: string }) {
  const parts = line.split(/(\s+)/);
  const result: ReactNode[] = [];

  parts.forEach((part, i) => {
    // Command (ferox)
    if (part === "ferox") {
      result.push(
        <span key={i} className="text-code-cmd">
          {part}
        </span>
      );
    }
    // Subcommands
    else if (
      [
        "build",
        "scan",
        "sessions",
        "exploit",
        "post",
        "persist",
        "evasion",
        "listener",
        "lateral",
        "enum",
        "hashdump",
        "wmi",
        "amsi",
        "psexec",
        "ad",
      ].includes(part)
    ) {
      result.push(
        <span key={i} className="text-code-keyword font-semibold">
          {part}
        </span>
      );
    }
    // Flags (--something or -x)
    else if (part.startsWith("--") || (part.startsWith("-") && part.length <= 3)) {
      result.push(
        <span key={i} className="text-code-param">
          {part}
        </span>
      );
    }
    // Line continuation
    else if (part === "\\") {
      result.push(
        <span key={i} className="text-code-comment">
          {part}
        </span>
      );
    }
    // Values with slashes, dots, or numbers
    else if (
      part.includes("/") ||
      part.includes(".") ||
      /^\d+$/.test(part) ||
      part === "true" ||
      part === "false"
    ) {
      result.push(
        <span key={i} className="text-code-value">
          {part}
        </span>
      );
    }
    // Quoted strings
    else if (part.startsWith('"') || part.startsWith("'")) {
      result.push(
        <span key={i} className="text-code-string">
          {part}
        </span>
      );
    }
    // Default
    else {
      result.push(<span key={i}>{part}</span>);
    }
  });

  return <>{result}</>;
}

export default CLICommandDisplay;
