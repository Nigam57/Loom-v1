import { useState, useCallback, useEffect } from 'react';
import { TerminalComponent } from '../Terminal';
import { X, MessageSquarePlus, FolderOpen } from 'lucide-react';
import { useSettingsStore } from '../store/settings';
import { open } from '@tauri-apps/plugin-dialog';

interface Pane {
  agentId: string;
  label: string;
  instanceId: string;
}

interface TerminalGridProps {
  onAddToConversation: (agentId: string) => void;
}

export function TerminalGrid({ onAddToConversation }: TerminalGridProps) {
  const [panes, setPanes] = useState<Pane[]>([]);
  const { settings, updateSettings } = useSettingsStore();
  const currentFolder = settings.workspace.currentFolder;

  const [editingPaneId, setEditingPaneId] = useState<string | null>(null);
  const [editTitle, setEditTitle] = useState('');

  const handleSelectWorkspace = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected && typeof selected === 'string') {
      updateSettings({ workspace: { currentFolder: selected } });
    }
  };

  const addPane = useCallback((agentId: string, label: string) => {
    const instanceId = `${agentId}-${Date.now()}`;
    setPanes((prev) => [...prev, { agentId, label, instanceId }]);
  }, []);

  const removePane = useCallback((instanceId: string) => {
    setPanes((prev) => prev.filter((p) => p.instanceId !== instanceId));
  }, []);

  useEffect(() => {
    const handleSpawn = (e: any) => {
      addPane(e.detail.agentId, e.detail.label);
    };
    window.addEventListener('loom:spawn-agent', handleSpawn);
    return () => window.removeEventListener('loom:spawn-agent', handleSpawn);
  }, [addPane]);

  if (panes.length === 0) {
    return (
      <div
        style={{
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'var(--color-text-dim)',
          gap: 16,
          backgroundColor: 'var(--color-bg-0)',
        }}
      >
        <div style={{
          padding: 16,
          borderRadius: '50%',
          backgroundColor: 'var(--color-bg-1)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          border: '1px solid var(--color-border)',
        }}>
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
            <polyline points="4 17 10 11 4 5" />
            <line x1="12" y1="19" x2="20" y2="19" />
          </svg>
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12 }}>
          <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 4 }}>
            <span style={{ fontSize: 16, fontWeight: 500, color: 'var(--color-text-primary)' }}>No active terminals</span>
            <span style={{ fontSize: 13, color: 'var(--color-text-dim)', maxWidth: 280, textAlign: 'center' }}>
              Select an agent from the sidebar to launch a new terminal session.
            </span>
          </div>
          <button
            onClick={handleSelectWorkspace}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 8,
              padding: '6px 12px',
              backgroundColor: 'var(--color-bg-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-text-primary)',
              cursor: 'pointer',
              fontSize: 13,
            }}
          >
            <FolderOpen size={14} />
            {currentFolder ? 'Change Workspace' : 'Select Workspace'}
          </button>
          {currentFolder && (
            <span style={{ fontSize: 11, color: 'var(--color-text-muted)', fontFamily: 'var(--font-mono)' }}>
              {currentFolder}
            </span>
          )}
        </div>
      </div>
    );
  }

  const columns = panes.length <= 2 ? panes.length : 2;
  const rows = Math.ceil(panes.length / columns);

  return (
    <div
      style={{
        height: '100%',
        display: 'grid',
        gridTemplateColumns: `repeat(${columns}, 1fr)`,
        gridTemplateRows: `repeat(${rows}, 1fr)`,
        gap: 0,
      }}
    >
      {panes.map((pane) => (
        <div
          key={pane.instanceId}
          style={{
            display: 'flex',
            flexDirection: 'column',
            borderRight: '1px solid var(--color-border)',
            borderBottom: '1px solid var(--color-border)',
            overflow: 'hidden',
          }}
        >
          {/* Header */}
          <div
            onContextMenu={(e) => {
              e.preventDefault();
              setEditingPaneId(pane.instanceId);
              setEditTitle(pane.label);
            }}
            title="Right-click to rename"
            style={{
              height: 36,
              display: 'flex',
              alignItems: 'center',
              padding: '0 12px',
              backgroundColor: 'var(--color-bg-1)',
              borderBottom: '1px solid var(--color-border)',
              gap: 8,
              flexShrink: 0,
            }}
          >
            {editingPaneId === pane.instanceId ? (
              <input
                autoFocus
                value={editTitle}
                onChange={(e) => setEditTitle(e.target.value)}
                onBlur={() => {
                  setPanes((prev) => prev.map((p) => p.instanceId === pane.instanceId ? { ...p, label: editTitle } : p));
                  setEditingPaneId(null);
                }}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    setPanes((prev) => prev.map((p) => p.instanceId === pane.instanceId ? { ...p, label: editTitle } : p));
                    setEditingPaneId(null);
                  } else if (e.key === 'Escape') {
                    setEditingPaneId(null);
                  }
                }}
                style={{
                  flex: 1,
                  background: 'var(--color-bg-2)',
                  border: '1px solid var(--color-accent-primary)',
                  color: 'var(--color-text-primary)',
                  fontSize: 12,
                  fontFamily: 'var(--font-mono)',
                  padding: '2px 4px',
                  outline: 'none',
                  borderRadius: 4,
                }}
              />
            ) : (
              <span
                style={{
                  fontSize: 12,
                  fontFamily: 'var(--font-mono)',
                  color: 'var(--color-text-primary)',
                  fontWeight: 500,
                  flex: 1,
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                }}
              >
                {pane.label}
              </span>
            )}
            <button
              onClick={() => onAddToConversation(pane.agentId)}
              title="Add to Conversation"
              style={{
                border: 'none',
                background: 'transparent',
                color: 'var(--color-text-muted)',
                cursor: 'pointer',
                padding: 4,
                display: 'flex',
                alignItems: 'center',
                borderRadius: 4,
              }}
            >
              <MessageSquarePlus size={14} />
            </button>
            <button
              onClick={() => removePane(pane.instanceId)}
              title="Close"
              style={{
                border: 'none',
                background: 'transparent',
                color: 'var(--color-text-muted)',
                cursor: 'pointer',
                padding: 4,
                display: 'flex',
                alignItems: 'center',
                borderRadius: 4,
              }}
            >
              <X size={14} />
            </button>
          </div>
          {/* Terminal body */}
          <div style={{ flex: 1, backgroundColor: 'var(--color-bg-0)' }}>
            <TerminalComponent id={pane.instanceId} agentId={pane.agentId} cwd={currentFolder || undefined} />
          </div>
        </div>
      ))}
    </div>
  );
}

// Export addPane ref for external use
export type TerminalGridHandle = {
  addPane: (agentId: string, label: string) => void;
};
