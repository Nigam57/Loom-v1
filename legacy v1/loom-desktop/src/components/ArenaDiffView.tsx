import { GitBranch } from 'lucide-react';

export function ArenaDiffView() {
  return (
    <div
      style={{
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        color: 'var(--color-text-dim)',
        gap: 12,
      }}
    >
      <GitBranch size={32} strokeWidth={1.5} />
      <span style={{ fontSize: 14 }}>Arena Mode</span>
      <span style={{ fontSize: 12, color: 'var(--color-text-dim)' }}>Not implemented yet</span>
      <span style={{ fontSize: 11, color: 'var(--color-text-dim)', fontFamily: 'var(--font-mono)' }}>
        Git worktrees + side-by-side diff view
      </span>
    </div>
  );
}
