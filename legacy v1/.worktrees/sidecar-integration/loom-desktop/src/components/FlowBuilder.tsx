import { Workflow } from 'lucide-react';

export function FlowBuilder() {
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
      <Workflow size={32} strokeWidth={1.5} />
      <span style={{ fontSize: 14 }}>Visual Automation</span>
      <span style={{ fontSize: 12, color: 'var(--color-text-dim)' }}>Not implemented yet</span>
      <span style={{ fontSize: 11, color: 'var(--color-text-dim)', fontFamily: 'var(--font-mono)' }}>
        Node-based flow builder with triggers
      </span>
    </div>
  );
}
