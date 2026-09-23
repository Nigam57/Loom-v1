import { Network } from 'lucide-react';

export function GraphView() {
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
      <Network size={32} strokeWidth={1.5} />
      <span style={{ fontSize: 14 }}>Knowledge Graph</span>
      <span style={{ fontSize: 12, color: 'var(--color-text-dim)' }}>Not implemented yet</span>
      <span style={{ fontSize: 11, color: 'var(--color-text-dim)', fontFamily: 'var(--font-mono)' }}>
        Graphiphy — SQLite + petgraph + react-force-graph-2d
      </span>
    </div>
  );
}
