import { useEffect, useRef } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { WebLinksAddon } from '@xterm/addon-web-links';
import '@xterm/xterm/css/xterm.css';
import { executeCommand } from '../lib/tauri';
import { useAppStore } from '../store';

interface TerminalProps {
  tabId: string;
  sessionId: string;
}

export function Terminal({ tabId, sessionId }: TerminalProps) {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const { sessions, updateTab } = useAppStore();

  const session = sessions.find((s) => s.id === sessionId);

  useEffect(() => {
    if (!terminalRef.current) return;

    // Create terminal instance
    const xterm = new XTerm({
      theme: {
        background: '#0f1525',
        foreground: '#ffffff',
        cursor: '#00ff88',
        cursorAccent: '#0f1525',
        selectionBackground: 'rgba(0, 255, 136, 0.3)',
        black: '#0a0e17',
        red: '#ff3366',
        green: '#00ff88',
        yellow: '#ffaa00',
        blue: '#00ccff',
        magenta: '#cc66ff',
        cyan: '#00ccff',
        white: '#ffffff',
        brightBlack: '#6b7a90',
        brightRed: '#ff6699',
        brightGreen: '#33ffaa',
        brightYellow: '#ffcc33',
        brightBlue: '#33ddff',
        brightMagenta: '#dd99ff',
        brightCyan: '#33ddff',
        brightWhite: '#ffffff',
      },
      fontFamily: 'JetBrains Mono, Fira Code, Consolas, monospace',
      fontSize: 13,
      lineHeight: 1.2,
      cursorBlink: true,
      cursorStyle: 'block',
      scrollback: 10000,
      allowProposedApi: true,
    });

    // Add addons
    const fitAddon = new FitAddon();
    const searchAddon = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();

    xterm.loadAddon(fitAddon);
    xterm.loadAddon(searchAddon);
    xterm.loadAddon(webLinksAddon);

    // Open terminal
    xterm.open(terminalRef.current);
    fitAddon.fit();

    // Store refs
    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    // Write welcome message
    const hostname = session?.hostname || 'unknown';
    const username = session?.username || 'user';
    xterm.writeln('\x1b[32m╔════════════════════════════════════════════════════════════╗\x1b[0m');
    xterm.writeln('\x1b[32m║\x1b[0m  \x1b[1;32mFerox C2 Terminal\x1b[0m                                         \x1b[32m║\x1b[0m');
    xterm.writeln('\x1b[32m║\x1b[0m  Session: \x1b[33m' + hostname.padEnd(47) + '\x1b[0m \x1b[32m║\x1b[0m');
    xterm.writeln('\x1b[32m╚════════════════════════════════════════════════════════════╝\x1b[0m');
    xterm.writeln('');
    xterm.write(`\x1b[32m${username}@${hostname}\x1b[0m:\x1b[34m~\x1b[0m$ `);

    // Handle resize
    const handleResize = () => {
      fitAddon.fit();
    };
    window.addEventListener('resize', handleResize);

    // Handle input
    let currentLine = '';
    xterm.onData((data) => {
      // Enter key
      if (data === '\r') {
        xterm.writeln('');
        if (currentLine.trim()) {
          handleCommand(currentLine.trim());
        }
        currentLine = '';
        xterm.write(`\x1b[32m${username}@${hostname}\x1b[0m:\x1b[34m~\x1b[0m$ `);
      }
      // Backspace
      else if (data === '\x7f') {
        if (currentLine.length > 0) {
          currentLine = currentLine.slice(0, -1);
          xterm.write('\b \b');
        }
      }
      // Ctrl+C
      else if (data === '\x03') {
        xterm.writeln('^C');
        currentLine = '';
        xterm.write(`\x1b[32m${username}@${hostname}\x1b[0m:\x1b[34m~\x1b[0m$ `);
      }
      // Ctrl+L (clear)
      else if (data === '\x0c') {
        xterm.clear();
        xterm.write(`\x1b[32m${username}@${hostname}\x1b[0m:\x1b[34m~\x1b[0m$ `);
      }
      // Regular input
      else {
        currentLine += data;
        xterm.write(data);
      }
    });

    const handleCommand = async (cmd: string) => {
      try {
        xterm.writeln('\x1b[33mExecuting...\x1b[0m');
        const result = await executeCommand(sessionId, cmd);

        if (result.success) {
          xterm.writeln(result.output);
        } else {
          xterm.writeln(`\x1b[31mError: ${result.output}\x1b[0m`);
        }
        xterm.writeln(`\x1b[90m[${result.execution_time_ms}ms]\x1b[0m`);
      } catch (error) {
        xterm.writeln(`\x1b[31mFailed to execute command: ${error}\x1b[0m`);
      }
    };

    // Update tab title with hostname
    if (session) {
      updateTab(tabId, { title: session.hostname });
    }

    // Cleanup
    return () => {
      window.removeEventListener('resize', handleResize);
      xterm.dispose();
    };
  }, [sessionId, tabId, session, updateTab]);

  // Resize on container resize
  useEffect(() => {
    const observer = new ResizeObserver(() => {
      fitAddonRef.current?.fit();
    });

    if (terminalRef.current) {
      observer.observe(terminalRef.current);
    }

    return () => observer.disconnect();
  }, []);

  return (
    <div className="h-full w-full bg-dark-900">
      <div ref={terminalRef} className="h-full w-full" />
    </div>
  );
}
