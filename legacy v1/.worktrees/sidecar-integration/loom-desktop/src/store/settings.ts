import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export type AppView = 'terminals' | 'conversation' | 'graph' | 'arena' | 'automation' | 'settings' | 'extensions';

export interface AppSettings {
  appearance: {
    theme: 'dark' | 'light' | 'system';
  };
  workspace: {
    currentFolder: string | null;
  };
  agents: {
    customPaths: Record<string, string>;
  };
  rooms: {
    defaultMaxTurns: number;
    defaultTimeoutSeconds: number;
    defaultBudgetCents: number;
    defaultTurnPolicy: 'round-robin' | 'directed' | 'mention';
  };
  extensions: {
    installed: ExtensionEntry[];
  };
}

export interface ExtensionEntry {
  id: string;
  name: string;
  type: 'mcp-server' | 'skill' | 'agent-cli' | 'plugin' | 'unknown';
  source: string;
  version: string;
  enabled: boolean;
  entryPoint: string;
}

const DEFAULT_SETTINGS: AppSettings = {
  appearance: { theme: 'dark' },
  workspace: { currentFolder: null },
  agents: { customPaths: {} },
  rooms: {
    defaultMaxTurns: 6,
    defaultTimeoutSeconds: 300,
    defaultBudgetCents: 100,
    defaultTurnPolicy: 'round-robin',
  },
  extensions: { installed: [] },
};

interface SettingsStore {
  settings: AppSettings;
  activeView: AppView;
  settingsOpen: boolean;
  loading: boolean;
  setView: (view: AppView) => void;
  toggleSettings: () => void;
  loadSettings: () => Promise<void>;
  updateSettings: (patch: Partial<AppSettings>) => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: DEFAULT_SETTINGS,
  activeView: 'terminals',
  settingsOpen: false,
  loading: false,

  setView: (view) => set({ activeView: view }),

  toggleSettings: () => set((state) => ({ settingsOpen: !state.settingsOpen })),

  loadSettings: async () => {
    set({ loading: true });
    try {
      const result = await invoke<AppSettings>('get_settings');
      set({ settings: { ...DEFAULT_SETTINGS, ...result } });
    } catch {
      // Settings file doesn't exist yet, use defaults
    } finally {
      set({ loading: false });
    }
  },

  updateSettings: async (patch) => {
    const merged = { ...get().settings, ...patch };
    set({ settings: merged });
    try {
      await invoke('update_settings', { settings: merged });
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  },
}));
