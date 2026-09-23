import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface AgentInfo {
  id: string;
  name: string;
  installed: boolean;
  path: string | null;
  requires_wsl: boolean;
  reason: string | null;
  status: 'running' | 'installed' | 'not-found';
  color: string;
}

const AGENT_COLORS: Record<string, string> = {
  claude: '#9333ea',
  coder: '#2563eb',
  opencode: '#16a34a',
  agy: '#d97706',
  hermes: '#dc2626',
  odysseus: '#7c3aed',
};

interface AgentStore {
  agents: AgentInfo[];
  detecting: boolean;
  detectAgents: () => Promise<void>;
  getAgent: (id: string) => AgentInfo | undefined;
}

export const useAgentStore = create<AgentStore>((set, get) => ({
  agents: [],
  detecting: false,

  detectAgents: async () => {
    set({ detecting: true });
    try {
      const result = await invoke<any[]>('detect_agents');
      set({
        agents: result.map((a) => ({
          ...a,
          status: a.installed ? 'installed' : 'not-found',
          color: AGENT_COLORS[a.id] || '#71717a',
        })),
      });
    } catch (e) {
      console.error('Agent detection failed:', e);
    } finally {
      set({ detecting: false });
    }
  },

  getAgent: (id: string) => get().agents.find((a) => a.id === id),
}));
