import { Puzzle, Plus } from 'lucide-react';
import { useState } from 'react';
import { useSettingsStore } from '../store/settings';
import type { ExtensionEntry } from '../store/settings';

export function ExtensionPanel() {
  const { settings } = useSettingsStore();
  const [installUrl, setInstallUrl] = useState('');
  const extensions = settings.extensions.installed;

  return (
    <div style={{ height: '100%', display: 'flex', flexDirection: 'column', backgroundColor: 'var(--color-bg-0)' }}>
      {/* Header */}
      <div
        style={{
          padding: '16px 20px',
          borderBottom: '1px solid var(--color-border)',
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          backgroundColor: 'var(--color-bg-0)',
        }}
      >
        <Puzzle size={14} />
        <span style={{ fontSize: 14, fontWeight: 600 }}>Extensions</span>
      </div>

      {/* Install from GitHub */}
      <div style={{ padding: '12px 16px', borderBottom: '1px solid var(--color-border)' }}>
        <div style={{ fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 8 }}>
          Install from GitHub URL
        </div>
        <div style={{ display: 'flex', gap: 8 }}>
          <input
            type="text"
            value={installUrl}
            onChange={(e) => setInstallUrl(e.target.value)}
            placeholder="https://github.com/owner/repo"
            style={{
              flex: 1,
              padding: '10px 14px',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              backgroundColor: 'var(--color-bg-1)',
              color: 'var(--color-text-primary)',
              fontSize: 12,
              fontFamily: 'var(--font-mono)',
              outline: 'none',
              transition: 'border-color 0.15s',
            }}
            onFocus={(e) => (e.target.style.borderColor = 'var(--color-text-dim)')}
            onBlur={(e) => (e.target.style.borderColor = 'var(--color-border)')}
          />
          <button
            disabled={!installUrl.trim()}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 4,
              padding: '8px 16px',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              background: installUrl.trim() ? 'var(--color-text-primary)' : 'var(--color-bg-2)',
              color: installUrl.trim() ? 'var(--color-bg-0)' : 'var(--color-text-primary)',
              cursor: installUrl.trim() ? 'pointer' : 'default',
              fontSize: 12,
              fontWeight: 500,
              fontFamily: 'var(--font-sans)',
              opacity: installUrl.trim() ? 1 : 0.5,
            }}
          >
            <Plus size={12} /> Install
          </button>
        </div>
      </div>

      {/* Extension List */}
      <div style={{ flex: 1, overflow: 'auto', padding: '8px 0' }}>
        {extensions.length === 0 ? (
          <div style={{ padding: '40px 16px', textAlign: 'center', color: 'var(--color-text-dim)', fontSize: 12 }}>
            No extensions installed
          </div>
        ) : (
          extensions.map((ext) => <ExtensionRow key={ext.id} extension={ext} />)
        )}
      </div>
    </div>
  );
}

function ExtensionRow({ extension }: { extension: ExtensionEntry }) {
  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        padding: '8px 16px',
        gap: 12,
        borderBottom: '1px solid var(--color-border)',
      }}
    >
      <div style={{ flex: 1 }}>
        <div style={{ fontSize: 13, fontWeight: 500 }}>{extension.name}</div>
        <div style={{ fontSize: 11, color: 'var(--color-text-muted)', fontFamily: 'var(--font-mono)' }}>
          {extension.type} · {extension.version}
        </div>
      </div>
      <label style={{ display: 'flex', alignItems: 'center', gap: 6, cursor: 'pointer' }}>
        <input type="checkbox" checked={extension.enabled} onChange={() => {}} />
        <span style={{ fontSize: 11, color: 'var(--color-text-muted)' }}>
          {extension.enabled ? 'Enabled' : 'Disabled'}
        </span>
      </label>
    </div>
  );
}
