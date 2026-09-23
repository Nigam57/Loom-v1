<!-- [2026-01-23 02:05] 目的: 邀请助手或终端成员并配置实例与权限选项; 边界: 只负责收集配置并触发邀请事件，不处理会话创建或持久化; 设计: 合并内置与自定义模型并遵循默认成员索引。 -->
<template>
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/70 animate-in fade-in duration-200">
    <div class="w-full max-w-[340px] max-h-[64vh] bg-panel/80 border border-border/10 rounded-3xl shadow-float flex flex-col relative ring-1 ring-border/5 overflow-hidden">
      <button type="button" @click="emit('close')" class="absolute top-3 right-3 w-6 h-6 rounded-full bg-overlay/5 text-text/40 hover:text-text flex items-center justify-center transition-colors z-10 hover:bg-overlay/10">
        <span class="material-symbols-outlined text-lg">close</span>
      </button>
      <div class="px-5 pt-6 pb-2 text-center">
        <h2 class="text-text font-semibold text-lg tracking-tight">{{ title }}</h2>
        <p class="text-text/40 text-2xs font-medium mt-1">{{ t('invite.assistant.subtitle') }}</p>
      </div>

      <div class="flex-1 min-h-0 overflow-y-auto custom-scrollbar">
        <div class="px-3 py-2 space-y-1">
          <div
            v-for="model in models"
            :key="model.id"
            @click="selectedModel = model.id"
            :class="[
              'flex items-center gap-3 p-2 rounded-xl border cursor-pointer group transition',
              selectedModel === model.id ? 'bg-primary/10 border-primary/20 relative overflow-hidden' : 'border-transparent hover:bg-overlay/5'
            ]"
          >
            <div v-if="selectedModel === model.id" class="absolute left-0 top-0 bottom-0 w-0.5 bg-primary"></div>
            <div
              :class="[
                'w-8 h-8 rounded-lg flex items-center justify-center text-text border border-border/10 transition-transform',
                model.accentClass,
                selectedModel === model.id ? 'ring-2 ring-border/30' : 'opacity-80 group-hover:opacity-100'
              ]"
            >
              <div v-if="model.iconKind === 'opencode'" class="member-icon-opencode" aria-hidden="true">
                <span>open</span>
                <span>code</span>
              </div>
              <div v-else-if="model.iconKind === 'qwen'" class="member-icon-qwen" aria-hidden="true">
                <svg viewBox="0 0 200 200" role="presentation">
                  <defs>
                    <linearGradient id="qwen-invite-grad-top" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stop-color="#ffffff" />
                      <stop offset="100%" stop-color="#a29bfe" />
                    </linearGradient>
                    <linearGradient id="qwen-invite-grad-side" x1="0%" y1="0%" x2="0%" y2="100%">
                      <stop offset="0%" stop-color="#6c5ce7" />
                      <stop offset="100%" stop-color="#4834d4" />
                    </linearGradient>
                    <g id="qwen-invite-arm">
                      <path d="M100 100 L100 40 L155 65 L125 115 Z" fill="url(#qwen-invite-grad-side)" />
                      <path
                        d="M100 100 L100 30 L165 60 L135 110 Z"
                        fill="url(#qwen-invite-grad-top)"
                        transform="translate(-5, -5)"
                      />
                    </g>
                  </defs>
                  <use href="#qwen-invite-arm" transform="rotate(0, 100, 100)" />
                  <use href="#qwen-invite-arm" transform="rotate(120, 100, 100)" />
                  <use href="#qwen-invite-arm" transform="rotate(240, 100, 100)" />
                </svg>
              </div>
              <span v-else class="material-symbols-outlined text-lg">{{ model.icon }}</span>
            </div>
            <div class="flex flex-col z-10">
              <span :class="['text-xs font-medium', selectedModel === model.id ? 'text-text font-semibold' : 'text-text/80 group-hover:text-text']">{{ model.label }}</span>
            </div>
            <div v-if="selectedModel === model.id" class="ml-auto text-primary">
              <span class="material-symbols-outlined text-lg">check_circle</span>
            </div>
          </div>
        </div>

        <div class="px-4 py-4 space-y-3 mt-1 border-t border-border/5 bg-overlay/[0.02]">
          <div class="flex items-center justify-between">
            <span class="text-xs text-text/90 font-medium tracking-tight">{{ t('invite.assistant.instances') }}</span>
            <div class="flex items-center bg-panel/60 rounded-lg p-0.5 border border-border/10 shadow-inner">
              <button
                type="button"
                :disabled="!canDecreaseInstances"
                :class="[
                  'w-6 h-5 rounded-md flex items-center justify-center transition-colors',
                  canDecreaseInstances ? 'text-text/60 hover:text-text hover:bg-overlay/10' : 'text-text/20 cursor-not-allowed'
                ]"
                @click="decreaseInstances"
              >
                <span class="material-symbols-outlined text-base">remove</span>
              </button>
              <input
                :value="instanceText"
                type="text"
                inputmode="numeric"
                class="w-8 bg-transparent text-center text-xs font-semibold text-text tabular-nums outline-none border-none focus:ring-0"
                aria-label="Instance count"
                @input="handleInstanceInput"
                @blur="commitInstanceInput"
                @keydown.enter.prevent="commitInstanceInput"
              />
              <button
                type="button"
                :class="[
                  'w-6 h-5 rounded-md flex items-center justify-center transition-colors',
                  canIncreaseInstances
                    ? 'text-primary hover:bg-primary/10'
                    : 'text-text/20 cursor-not-allowed'
                ]"
                @click="increaseInstances"
              >
                <span class="material-symbols-outlined text-base">add</span>
              </button>
            </div>
          </div>
          <p v-if="showLimitWarning" class="text-2xs text-danger font-medium">
            {{ t('invite.assistant.instanceLimit', { count: maxInstances }) }}
          </p>

          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-xs text-text/90 font-medium tracking-tight">{{ t('invite.assistant.unlimitedAccess') }}</span>
              <span class="text-2xs text-text/30 font-medium">{{ t('invite.assistant.unlimitedAccessDesc') }}</span>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input v-model="unlimitedAccess" type="checkbox" class="sr-only peer" />
              <div class="w-10 h-5 bg-panel-strong/80 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-text after:border-border/40 after:border after:rounded-full after:h-4 after:w-4 after:transition peer-checked:bg-primary shadow-inner"></div>
            </label>
          </div>
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span class="text-xs text-text/90 font-medium tracking-tight">{{ t('invite.assistant.sandboxed') }}</span>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input v-model="sandboxed" type="checkbox" class="sr-only peer" />
              <div class="w-10 h-5 bg-panel-strong/80 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-text after:border-border/40 after:border after:rounded-full after:h-4 after:w-4 after:transition peer-checked:bg-primary shadow-inner"></div>
            </label>
          </div>
        </div>
      </div>

      <div class="p-4 pt-2">
        <button
          type="button"
          @click="emitInvite"
          class="w-full py-2.5 bg-primary hover:bg-primary-hover text-on-primary text-xs font-semibold rounded-xl transition active:scale-[0.98] border-t border-border/20"
        >
          {{ t('invite.assistant.send') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useSettingsStore } from '@/features/global/settingsStore';
import {
  BASE_TERMINALS,
  CUSTOM_TERMINAL_SWATCH,
  CUSTOM_TERMINAL_ICON,
  resolveBaseTerminalLabel
} from '@/shared/constants/terminalCatalog';
import type { TerminalType } from '@/shared/types/terminal';
import { resolveMemberIdFromSelectionIndex } from '@/shared/utils/memberSelection';

const props = defineProps<{ title?: string; inviteRole?: 'assistant' | 'member' }>();
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'invite', model: {
    id: string;
    label: string;
    command: string;
    terminalType: TerminalType;
    instances: number;
    unlimitedAccess: boolean;
    sandboxed: boolean;
  }): void;
}>();

const { t } = useI18n();
const settingsStore = useSettingsStore();
const { settings } = storeToRefs(settingsStore);

type TerminalModel = {
  id: string;
  label: string;
  icon: string;
  iconKind: 'material' | 'opencode' | 'qwen';
  accentClass: string;
  command: string;
  terminalType: TerminalType;
};

const inviteRole = computed(() => props.inviteRole ?? 'assistant');

const models = computed<TerminalModel[]>(() => {
  // 邀请助手时隐藏内置终端助手，避免与终端专属入口重复。
  const baseModels = BASE_TERMINALS.value.filter(
    (member) => inviteRole.value === 'member' || member.id !== 'terminal'
  ).map((member) => ({
    id: member.id,
    label: resolveBaseTerminalLabel(member, t),
    icon: member.icon,
    iconKind: member.id === 'opencode' ? 'opencode' : member.id === 'qwen-code' ? 'qwen' : 'material',
    accentClass: member.swatch,
    command: member.command,
    terminalType: member.terminalType
  }));

  const customModels = settings.value.members.customMembers.map((member) => ({
    id: member.id,
    label: member.name || member.command || t('settings.memberOptions.custom'),
    icon: CUSTOM_TERMINAL_ICON,
    iconKind: 'material' as const,
    accentClass: CUSTOM_TERMINAL_SWATCH,
    command: member.command,
    terminalType: 'shell' as const
  }));

  return [...baseModels, ...customModels];
});

const selectedModel = ref('');
// 实例数量限制用于控制资源占用与 UI 复杂度，过大时提示风险。
const minInstances = 1;
const maxInstances = 20;
const instances = ref(minInstances);
const instanceText = ref(String(minInstances));
const unlimitedAccess = ref(true);
const sandboxed = ref(false);
const showLimitWarning = ref(false);

const title = computed(() => props.title ?? t('invite.assistant.title'));
const canDecreaseInstances = computed(() => instances.value > minInstances);
const canIncreaseInstances = computed(() => instances.value < maxInstances);

watch(
  () => ({ options: models.value, defaultIndex: settings.value.members.defaultMemberIndex }),
  ({ options, defaultIndex }) => {
    if (!options.length) {
      selectedModel.value = '';
      return;
    }
    if (options.some((item) => item.id === selectedModel.value)) {
      return;
    }
    // 根据设置中的默认索引恢复首选项，缺失时回退到首个可用模型。
    const resolvedId = resolveMemberIdFromSelectionIndex(defaultIndex, settings.value.members.customMembers);
    const preferred = options.find((item) => item.id === resolvedId)?.id ?? options[0].id;
    selectedModel.value = preferred;
  },
  { immediate: true }
);

const clampInstances = (value: number) => Math.min(maxInstances, Math.max(minInstances, value));

const handleInstanceInput = (event: Event) => {
  const nextValue = (event.target as HTMLInputElement).value;
  instanceText.value = nextValue;
  if (!nextValue.trim()) return;
  const parsed = Number(nextValue);
  if (!Number.isFinite(parsed)) return;
  showLimitWarning.value = parsed > maxInstances;
  instances.value = clampInstances(Math.round(parsed));
};

const commitInstanceInput = () => {
  const parsed = Number(instanceText.value);
  if (!Number.isFinite(parsed)) {
    instanceText.value = String(instances.value);
    return;
  }
  showLimitWarning.value = parsed > maxInstances;
  instances.value = clampInstances(Math.round(parsed));
  instanceText.value = String(instances.value);
};

const decreaseInstances = () => {
  if (instances.value > minInstances) {
    instances.value -= 1;
    instanceText.value = String(instances.value);
    showLimitWarning.value = false;
  }
};

const increaseInstances = () => {
  if (instances.value < maxInstances) {
    instances.value += 1;
    instanceText.value = String(instances.value);
    showLimitWarning.value = false;
  } else {
    showLimitWarning.value = true;
  }
};

const emitInvite = () => {
  const model = models.value.find((item) => item.id === selectedModel.value);
  if (model) {
    emit('invite', {
      id: model.id,
      label: model.label,
      command: model.command,
      terminalType: model.terminalType,
      instances: instances.value,
      unlimitedAccess: unlimitedAccess.value,
      sandboxed: sandboxed.value
    });
  }
};
</script>

<style scoped>
.member-icon-opencode {
  font-family: 'Rajdhani', 'Orbitron', 'Share Tech Mono', 'Fira Mono', 'Consolas', monospace;
  font-size: 7.5px;
  font-weight: 700;
  letter-spacing: 0.4px;
  line-height: 1;
  text-transform: lowercase;
  color: rgba(255, 255, 255, 0.92);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.35);
  user-select: none;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1px;
  transform: translateY(1px);
}

.member-icon-qwen {
  position: relative;
  width: 20px;
  height: 20px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.35));
}

.member-icon-qwen svg {
  width: 100%;
  height: 100%;
  display: block;
}
</style>
