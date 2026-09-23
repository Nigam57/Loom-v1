import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export type BaseTerminal = {
  id: string;
  nameKey?: string;
  label?: string;
  command: string;
  terminalType: string;
  icon: string;
  /** Flat identity swatch (Tailwind class); low chroma so the accent stays the only loud colour. */
  swatch: string;
};

export interface AgentManifest {
  id: string;
  name: string;
  command: string;
  args: string[];
  adapter?: string;
  parsing: {
    promptMarker: string;
    bulletMarker: string;
    readySignalMs: number;
  };
}

export const BASE_TERMINALS = ref<BaseTerminal[]>([]);

export const CUSTOM_TERMINAL_ICON = 'settings_suggest';
export const CUSTOM_TERMINAL_SWATCH = 'bg-[oklch(0.62_0.01_255)]';

// Keyed by manifest id (== terminal type).
const agentStyleMap: Record<string, { icon: string; swatch: string; nameKey?: string }> = {
  gemini: { icon: 'token', swatch: 'bg-[oklch(0.70_0.08_250)]', nameKey: 'settings.memberOptions.gemini' },
  codex: { icon: 'code', swatch: 'bg-[oklch(0.70_0.07_160)]', nameKey: 'settings.memberOptions.codex' },
  claude: { icon: 'psychology', swatch: 'bg-[oklch(0.72_0.09_50)]', nameKey: 'settings.memberOptions.claude' },
  opencode: { icon: 'code', swatch: 'bg-[oklch(0.70_0.05_200)]', nameKey: 'settings.memberOptions.opencode' },
  qwen: { icon: 'model_training', swatch: 'bg-[oklch(0.70_0.08_290)]', nameKey: 'settings.memberOptions.qwen' },
  shell: { icon: 'terminal', swatch: 'bg-[oklch(0.62_0.01_255)]' },
  'antigravity-cli': { icon: 'rocket_launch', swatch: 'bg-[oklch(0.72_0.08_230)]', nameKey: 'settings.memberOptions.antigravity' }
};

export const fetchAvailableAgents = async (): Promise<void> => {
  try {
    const manifests = await invoke<AgentManifest[]>('get_available_agents');
    BASE_TERMINALS.value = manifests.map((m) => {
      const style = agentStyleMap[m.id];
      return {
        id: m.id,
        nameKey: style?.nameKey,
        label: style?.nameKey ? undefined : m.name,
        command: m.command,
        terminalType: m.id,
        icon: style?.icon || CUSTOM_TERMINAL_ICON,
        swatch: style?.swatch || CUSTOM_TERMINAL_SWATCH
      };
    });
  } catch (error) {
    console.error('Failed to fetch agents:', error);
  }
};

export const resolveBaseTerminalLabel = (terminal: BaseTerminal, translate: (key: string) => string) => {
  if (terminal.label) {
    return terminal.label;
  }
  if (terminal.nameKey) {
    return translate(terminal.nameKey);
  }
  return terminal.id;
};
