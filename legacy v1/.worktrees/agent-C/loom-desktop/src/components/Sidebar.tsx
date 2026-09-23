import { useEffect, useState } from 'react';
import { useAgentStore } from '../store/agents';
import type { AgentInfo } from '../store/agents';
import { useRoomStore } from '../store/rooms';
import { useSettingsStore } from '../store/settings';
import type { AppView } from '../store/settings';
import {
  Monitor,
  MessageSquare,
  GitBranch,
  Workflow,
  Settings,
  Puzzle,
  Plus,
  ChevronDown,
  Network,
} from 'lucide-react';

const NAV_ITEMS: { id: AppView; label: string; icon: typeof Monitor }[] = [
  { id: 'terminals', label: 'Terminals', icon: Monitor },
  { id: 'conversation', label: 'Conversations', icon: MessageSquare },
  { id: 'graph', label: 'Knowledge', icon: Network },
  { id: 'arena', label: 'Arena', icon: GitBranch },
  { id: 'automation', label: 'Automation', icon: Workflow },
  { id: 'extensions', label: 'Extensions', icon: Puzzle },
];

function StatusDot({ status }: { status: AgentInfo['status'] }) {
  const colorMap = {
    running: 'var(--color-status-running)',
    installed: 'var(--color-status-installed)',
    'not-found': 'var(--color-status-notfound)',
  };
  return (
    <span
      style={{
        width: 8,
        height: 8,
        borderRadius: '50%',
        backgroundColor: colorMap[status],
        flexShrink: 0,
      }}
    />
  );
}

interface SidebarProps {
  onSpawnAgent: (agentId: string) => void;
  onNewRoom: () => void;
}

export function Sidebar({ onSpawnAgent, onNewRoom }: SidebarProps) {
  const { agents, detecting, detectAgents } = useAgentStore();
  const { rooms, activeRoomId, setActiveRoom } = useRoomStore();
  const { activeView, setView, toggleSettings } = useSettingsStore();
  const [agentsExpanded, setAgentsExpanded] = useState(true);

  useEffect(() => {
    detectAgents();
  }, [detectAgents]);

  const grouped = {
    running: agents.filter((a) => a.status === 'running'),
    installed: agents.filter((a) => a.status === 'installed'),
    notFound: agents.filter((a) => a.status === 'not-found'),
  };

  return (
    <aside
      style={{
        width: 220,
        height: '100%',
        backgroundColor: 'var(--color-bg-1)',
        borderRight: '1px solid var(--color-border)',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
        flexShrink: 0,
      }}
    >
      {/* Logo */}
      <div
        style={{
          padding: '16px 20px',
          display: 'flex',
          alignItems: 'center',
          gap: 8,
          backgroundColor: 'var(--color-bg-0)',
          borderBottom: '1px solid var(--color-border)',
        }}
      >
        <div style={{ width: 12, height: 12, backgroundColor: 'var(--color-text-primary)' }} />
        <span style={{ fontSize: 16, fontWeight: 700, letterSpacing: '-0.03em', textTransform: 'lowercase' }}>loom</span>
        <span style={{ fontSize: 10, color: 'var(--color-text-muted)', fontFamily: 'var(--font-mono)', paddingLeft: 4 }}>
          0.1
        </span>
      </div>

      {/* Navigation */}
      <nav style={{ padding: '12px 8px' }}>
        {NAV_ITEMS.map((item) => {
          const Icon = item.icon;
          const active = activeView === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setView(item.id)}
              style={{
                width: '100%',
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                padding: '8px 12px',
                borderRadius: 6,
                border: 'none',
                background: active ? 'var(--color-bg-3)' : 'transparent',
                color: active ? 'var(--color-text-primary)' : 'var(--color-text-muted)',
                cursor: 'pointer',
                fontSize: 13,
                fontWeight: active ? 500 : 400,
                fontFamily: 'var(--font-sans)',
                textAlign: 'left',
                transition: 'all 0.15s ease',
              }}
            >
              <Icon size={14} />
              {item.label}
            </button>
          );
        })}
      </nav>

      {/* Agents */}
      <div style={{ flex: 1, overflow: 'auto', padding: '8px 0' }}>
        <button
          onClick={() => setAgentsExpanded(!agentsExpanded)}
          style={{
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            gap: 6,
            padding: '4px 20px',
            border: 'none',
            background: 'transparent',
            color: 'var(--color-text-dim)',
            cursor: 'pointer',
            fontSize: 10,
            fontFamily: 'var(--font-sans)',
            textTransform: 'uppercase',
            letterSpacing: '0.06em',
            fontWeight: 600,
          }}
        >
          <ChevronDown
            size={12}
            style={{ transform: agentsExpanded ? 'rotate(0)' : 'rotate(-90deg)', transition: 'transform 100ms' }}
          />
          Agents {detecting && '...'}
        </button>

        {agentsExpanded && (
          <div>
            {grouped.running.length > 0 && (
              <AgentGroup label="Running" agents={grouped.running} onSpawn={onSpawnAgent} />
            )}
            {grouped.installed.length > 0 && (
              <AgentGroup label="Installed" agents={grouped.installed} onSpawn={onSpawnAgent} />
            )}
            {grouped.notFound.length > 0 && (
              <AgentGroup label="Not Found" agents={grouped.notFound} onSpawn={onSpawnAgent} />
            )}
            {agents.length === 0 && !detecting && (
              <div style={{ padding: '8px 16px', color: 'var(--color-text-dim)', fontSize: 12 }}>
                No agents detected
              </div>
            )}
          </div>
        )}

        {/* Rooms */}
        {rooms.length > 0 && (
          <>
            <div
              style={{
                padding: '12px 16px 4px',
                fontSize: 11,
                color: 'var(--color-text-muted)',
                textTransform: 'uppercase',
                letterSpacing: '0.05em',
                fontWeight: 600,
              }}
            >
              Rooms
            </div>
            {rooms.map((room) => (
              <button
                key={room.id}
                onClick={() => {
                  setActiveRoom(room.id);
                  setView('conversation');
                }}
                style={{
                  width: '100%',
                  display: 'flex',
                  alignItems: 'center',
                  gap: 8,
                  padding: '6px 20px',
                  border: 'none',
                  background: activeRoomId === room.id ? 'var(--color-bg-3)' : 'transparent',
                  color: activeRoomId === room.id ? 'var(--color-text-primary)' : 'var(--color-text-muted)',
                  cursor: 'pointer',
                  fontSize: 12,
                  fontFamily: 'var(--font-sans)',
                  textAlign: 'left',
                  borderLeft: activeRoomId === room.id ? '2px solid var(--color-text-primary)' : '2px solid transparent',
                }}
              >
                <MessageSquare size={12} />
                <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                  {room.goal.slice(0, 30)}
                </span>
              </button>
            ))}
          </>
        )}
      </div>

      {/* Bottom Actions */}
      <div style={{ borderTop: '1px solid var(--color-border)', padding: '8px' }}>
        <button
          onClick={onNewRoom}
          style={{
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 6,
            padding: '7px 0',
            border: '1px solid var(--color-border)',
            borderRadius: 4,
            background: 'var(--color-bg-2)',
            color: 'var(--color-text-primary)',
            cursor: 'pointer',
            fontSize: 12,
            fontFamily: 'var(--font-sans)',
          }}
        >
          <Plus size={12} />
          New Room
        </button>
        <button
          onClick={toggleSettings}
          style={{
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 6,
            padding: '7px 0',
            marginTop: 4,
            border: 'none',
            borderRadius: 4,
            background: 'var(--color-bg-2)',
            color: 'var(--color-text-primary)',
            cursor: 'pointer',
            fontSize: 12,
            fontWeight: 500,
            fontFamily: 'var(--font-sans)',
          }}
        >
          <Settings size={14} />
          Settings
        </button>
      </div>
    </aside>
  );
}

function AgentGroup({
  label,
  agents,
  onSpawn,
}: {
  label: string;
  agents: AgentInfo[];
  onSpawn: (id: string) => void;
}) {
  return (
    <div>
      <div
        style={{
          padding: '4px 28px',
          fontSize: 10,
          color: 'var(--color-text-dim)',
          textTransform: 'uppercase',
          letterSpacing: '0.04em',
        }}
      >
        {label}
      </div>
      {agents.map((agent) => (
        <button
          key={agent.id}
          onClick={() => agent.installed && onSpawn(agent.id)}
          disabled={!agent.installed}
          style={{
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            padding: '6px 20px 6px 32px',
            border: 'none',
            background: 'transparent',
            color: agent.installed ? 'var(--color-text-primary)' : 'var(--color-text-dim)',
            cursor: agent.installed ? 'pointer' : 'default',
            fontSize: 13,
            fontFamily: 'var(--font-sans)',
            textAlign: 'left',
            opacity: agent.installed ? 1 : 0.5,
          }}
        >
          <StatusDot status={agent.status} />
          <span>{agent.name}</span>
        </button>
      ))}
    </div>
  );
}
