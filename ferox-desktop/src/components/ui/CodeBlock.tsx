// ferox-desktop/src/components/ui/CodeBlock.tsx
// Code block component with syntax highlighting and copy functionality

import { useState, useCallback, ReactNode } from "react";
import { Copy, Check, Terminal } from "lucide-react";

interface CodeBlockProps {
  children: ReactNode;
  language?: string;
  title?: string;
  showLineNumbers?: boolean;
  className?: string;
  copyable?: boolean;
}

export function CodeBlock({
  children,
  language = "bash",
  title,
  showLineNumbers = false,
  className = "",
  copyable = true,
}: CodeBlockProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = useCallback(async () => {
    const text = typeof children === "string" ? children : String(children);

    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  }, [children]);

  const codeContent =
    typeof children === "string" ? children : String(children);
  const lines = codeContent.split("\n");

  return (
    <div className={`code-block ${className}`}>
      {/* Header */}
      <div className="code-block-header">
        <div className="flex items-center gap-2">
          <Terminal size={14} className="opacity-60" />
          <span className="code-block-title">{title || language}</span>
        </div>
        {copyable && (
          <button
            onClick={handleCopy}
            className="code-block-copy"
            title={copied ? "Copied!" : "Copy code"}
          >
            {copied ? (
              <Check size={14} className="text-success-text" />
            ) : (
              <Copy size={14} />
            )}
          </button>
        )}
      </div>

      {/* Code content */}
      <div className="code-block-content">
        <pre className="code-block-pre">
          <code className={`language-${language}`}>
            {showLineNumbers ? (
              <table className="code-table">
                <tbody>
                  {lines.map((line, index) => (
                    <tr key={index}>
                      <td className="code-line-number">{index + 1}</td>
                      <td className="code-line-content">
                        <HighlightedLine line={line} language={language} />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <HighlightedCode code={codeContent} language={language} />
            )}
          </code>
        </pre>
      </div>
    </div>
  );
}

// Simple syntax highlighting for common patterns
interface HighlightedCodeProps {
  code: string;
  language: string;
}

function HighlightedCode({ code, language }: HighlightedCodeProps) {
  return (
    <>
      {code.split("\n").map((line, index) => (
        <div key={index}>
          <HighlightedLine line={line} language={language} />
        </div>
      ))}
    </>
  );
}

interface HighlightedLineProps {
  line: string;
  language: string;
}

function HighlightedLine({ line, language }: HighlightedLineProps) {
  // Simple regex-based highlighting for common patterns
  const highlightPatterns = getHighlightPatterns(language);

  let result: ReactNode[] = [];
  let remainingLine = line;
  let keyIndex = 0;

  while (remainingLine.length > 0) {
    let matched = false;

    for (const { pattern, className } of highlightPatterns) {
      const match = remainingLine.match(pattern);
      if (match && match.index === 0) {
        result.push(
          <span key={keyIndex++} className={className}>
            {match[0]}
          </span>
        );
        remainingLine = remainingLine.slice(match[0].length);
        matched = true;
        break;
      }
    }

    if (!matched) {
      // Find the next potential match position
      let nextMatchPos = remainingLine.length;
      for (const { pattern } of highlightPatterns) {
        const match = remainingLine.match(pattern);
        if (match && match.index !== undefined && match.index < nextMatchPos) {
          nextMatchPos = match.index;
        }
      }

      if (nextMatchPos > 0) {
        result.push(
          <span key={keyIndex++}>{remainingLine.slice(0, nextMatchPos)}</span>
        );
        remainingLine = remainingLine.slice(nextMatchPos);
      }
    }
  }

  return <>{result}</>;
}

interface HighlightPattern {
  pattern: RegExp;
  className: string;
}

function getHighlightPatterns(language: string): HighlightPattern[] {
  const commonPatterns: HighlightPattern[] = [
    // Comments
    { pattern: /^#.*$/, className: "code-comment" },
    { pattern: /^\/\/.*$/, className: "code-comment" },
    // Strings
    { pattern: /^"[^"]*"/, className: "code-string" },
    { pattern: /^'[^']*'/, className: "code-string" },
    // Numbers
    { pattern: /^\b\d+\.?\d*\b/, className: "code-number" },
  ];

  const languagePatterns: Record<string, HighlightPattern[]> = {
    bash: [
      // Commands (at start of line or after pipe/semicolon)
      {
        pattern:
          /^(sudo|cd|ls|cat|grep|find|chmod|chown|curl|wget|echo|export|source|pip|npm|cargo|git|docker|kubectl)\b/,
        className: "code-keyword",
      },
      // Flags
      { pattern: /^\s*-{1,2}[\w-]+/, className: "code-flag" },
      // Variables
      { pattern: /^\$[\w_]+/, className: "code-variable" },
      { pattern: /^\$\{[^}]+\}/, className: "code-variable" },
      // Paths
      { pattern: /^[.~]?\/[\w./-]+/, className: "code-path" },
      ...commonPatterns,
    ],
    rust: [
      // Keywords
      {
        pattern:
          /^(fn|let|mut|const|pub|mod|use|struct|enum|impl|trait|where|async|await|match|if|else|for|while|loop|return|break|continue|self|Self|super|crate)\b/,
        className: "code-keyword",
      },
      // Types
      {
        pattern:
          /^(String|Vec|Option|Result|Box|Rc|Arc|RefCell|Cell|HashMap|HashSet|BTreeMap|BTreeSet|i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str)\b/,
        className: "code-type",
      },
      // Macros
      { pattern: /^\w+!/, className: "code-macro" },
      // Attributes
      { pattern: /^#\[[\w(,="\s)]+\]/, className: "code-attribute" },
      ...commonPatterns,
    ],
    typescript: [
      // Keywords
      {
        pattern:
          /^(const|let|var|function|class|interface|type|enum|import|export|from|as|default|if|else|for|while|do|switch|case|break|continue|return|try|catch|finally|throw|new|this|super|extends|implements|async|await|yield)\b/,
        className: "code-keyword",
      },
      // Types
      {
        pattern:
          /^(string|number|boolean|object|any|unknown|never|void|null|undefined|Array|Promise|Record|Partial|Required|Pick|Omit)\b/,
        className: "code-type",
      },
      // JSX tags
      { pattern: /^<\/?[\w.]+>?/, className: "code-tag" },
      ...commonPatterns,
    ],
    javascript: [
      // Keywords
      {
        pattern:
          /^(const|let|var|function|class|import|export|from|as|default|if|else|for|while|do|switch|case|break|continue|return|try|catch|finally|throw|new|this|super|extends|async|await|yield)\b/,
        className: "code-keyword",
      },
      ...commonPatterns,
    ],
    json: [
      // Property names
      { pattern: /^"[^"]+"\s*:/, className: "code-property" },
      // Boolean and null
      { pattern: /^(true|false|null)\b/, className: "code-keyword" },
      ...commonPatterns,
    ],
    python: [
      // Keywords
      {
        pattern:
          /^(def|class|if|elif|else|for|while|try|except|finally|with|as|import|from|return|yield|raise|pass|break|continue|and|or|not|in|is|lambda|global|nonlocal|assert|True|False|None)\b/,
        className: "code-keyword",
      },
      // Decorators
      { pattern: /^@\w+/, className: "code-attribute" },
      ...commonPatterns,
    ],
  };

  return languagePatterns[language] || commonPatterns;
}

// Inline code component
interface InlineCodeProps {
  children: ReactNode;
  className?: string;
}

export function InlineCode({ children, className = "" }: InlineCodeProps) {
  return <code className={`inline-code ${className}`}>{children}</code>;
}

// Command display with prompt
interface CommandProps {
  children: string;
  prompt?: string;
  className?: string;
}

export function Command({
  children,
  prompt = "$",
  className = "",
}: CommandProps) {
  return (
    <div className={`command-line ${className}`}>
      <span className="command-prompt">{prompt}</span>
      <span className="command-text">{children}</span>
    </div>
  );
}
