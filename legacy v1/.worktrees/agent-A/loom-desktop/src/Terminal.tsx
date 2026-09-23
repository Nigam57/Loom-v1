import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { WebglAddon } from '@xterm/addon-webgl';
import { invoke, Channel } from '@tauri-apps/api/core';
import '@xterm/xterm/css/xterm.css';

interface TerminalComponentProps {
  id: string;
}

interface PtyDataEvent {
  type: 'data';
  pid: number;
  data: string;
}

interface PtyExitEvent {
  type: 'exit';
  pid: number;
  code: number | null;
}

interface PtyErrorEvent {
  type: 'error';
  pid: number;
  message: string;
}

type PtyEvent = PtyDataEvent | PtyExitEvent | PtyErrorEvent;

export function TerminalComponent({ id, agentId, cwd }: TerminalComponentProps & { agentId?: string; cwd?: string }) {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<Terminal | null>(null);
  const pidRef = useRef<number | null>(null);

  useEffect(() => {
    if (!terminalRef.current) return;

    const term = new Terminal({
      theme: {
        background: '#0a0a0b',
        foreground: '#fafafa',
        cursor: '#fafafa',
        selectionBackground: '#27272a',
        black: '#121214',
        red: '#dc2626',
        green: '#16a34a',
        yellow: '#d97706',
        blue: '#2563eb',
        magenta: '#9333ea',
        cyan: '#0891b2',
        white: '#fafafa',
      },
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace",
      fontSize: 13,
      fontWeight: '500',
      cursorBlink: true,
      scrollback: 5000,
    });

    xtermRef.current = term;
    term.open(terminalRef.current);

    try {
      const webglAddon = new WebglAddon();
      term.loadAddon(webglAddon);
      webglAddon.onContextLoss(() => {
        webglAddon.dispose();
      });
    } catch (e) {
      console.warn('WebGL addon failed, using canvas renderer', e);
    }

    term.write(`\x1b[1;35m> Loom Terminal [${id}]\x1b[0m\r\n`);

    // Use Tauri Channel for streaming PTY output (not events)
    const onEvent = new Channel<PtyEvent>();
    onEvent.onmessage = (event: PtyEvent) => {
      switch (event.type) {
        case 'data':
          term.write(event.data);
          break;
        case 'exit':
          term.write(`\r\n\x1b[1;33m> Process exited (code: ${event.code})\x1b[0m\r\n`);
          break;
        case 'error':
          term.write(`\r\n\x1b[1;31m> Error: ${event.message}\x1b[0m\r\n`);
          break;
      }
    };

    async function startPty() {
      try {
        // On Windows, if agentId is provided, we use cmd.exe /c to ensure .cmd shims resolve correctly
        const cmd = agentId ? 'cmd.exe' : 'cmd.exe';
        const args = agentId ? ['/c', agentId] : undefined;

        const pid = await invoke<number>('spawn_pty', {
          cmd,
          args,
          cwd,
          onEvent,
        });
        pidRef.current = pid;

        // Wire terminal input to Rust
        term.onData((data) => {
          if (pidRef.current !== null) {
            invoke('write_pty', { pid: pidRef.current, data }).catch(console.error);
          }
        });

        // Wire terminal resize to Rust
        term.onResize(({ cols, rows }) => {
          if (pidRef.current !== null) {
            invoke('resize_pty', { pid: pidRef.current, cols, rows }).catch(console.error);
          }
        });
      } catch (e) {
        term.write(`\r\n\x1b[1;31m> Failed to spawn PTY: ${e}\x1b[0m\r\n`);
      }
    }

    startPty();

    return () => {
      if (pidRef.current !== null) {
        invoke('kill_pty', { pid: pidRef.current }).catch(console.error);
      }
      term.dispose();
    };
  }, [id, agentId, cwd]);

  return <div ref={terminalRef} className="w-full h-full overflow-hidden" />;
}
