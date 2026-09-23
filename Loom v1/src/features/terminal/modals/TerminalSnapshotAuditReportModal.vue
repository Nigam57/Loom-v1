<!-- [2026-01-23 11:56] 目的: 终端快照审计结果弹窗展示轮次对比与调用链; 边界: 仅负责展示与交互关闭，不驱动审计流程; 设计: 细化轮次对比摘要并保留错误细节。 -->
<template>
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/70 animate-in fade-in duration-200">
    <div class="w-full max-w-3xl bg-panel/90 border border-border/10 rounded-3xl shadow-float flex flex-col relative overflow-hidden">
      <button
        type="button"
        @click="emit('close')"
        class="absolute top-4 right-4 w-8 h-8 rounded-full bg-overlay/5 text-text/40 hover:text-text flex items-center justify-center transition-colors hover:bg-overlay/10"
      >
        <span class="material-symbols-outlined text-lg">close</span>
      </button>

      <div class="px-6 pt-6 pb-4 border-b border-border/10">
        <div class="text-text font-semibold text-lg">{{ t('settings.terminalSnapshotAuditTitle') }}</div>
        <div class="text-xs text-text/40 mt-1">{{ snapshotAuditStatusLabel }}</div>
      </div>

      <div class="px-6 py-5 space-y-5 max-h-[70vh] overflow-y-auto custom-scrollbar">
        <div class="rounded-2xl border border-border/10 bg-overlay/5 p-4 space-y-3">
          <div class="text-xs font-medium text-faint">
            {{ t('settings.terminalSnapshotAuditTitle') }}
          </div>
          <div class="text-2xs text-text/40">{{ t('settings.terminalSnapshotAuditLegend') }}</div>

          <div v-if="terminalSnapshotAuditResults.length" class="space-y-3">
            <div
              v-for="result in terminalSnapshotAuditResults"
              :key="result.memberId"
              class="rounded-xl border border-border/10 bg-overlay/5 px-3 py-3 space-y-2"
            >
              <div class="flex items-center gap-2 text-xs text-text/80">
                <span :class="['w-2 h-2 rounded-full', checkStatusClass(result.status)]"></span>
                <span class="font-semibold">{{ result.name }}</span>
                <span v-if="result.created" class="text-2xs text-text/40">
                  {{ t('settings.terminalSnapshotAuditCreated') }}
                </span>
              </div>
              <div class="space-y-2">
                <div
                  v-for="round in result.rounds"
                  :key="round.round"
                  class="rounded-lg border border-border/10 bg-black/20 px-3 py-2 space-y-1"
                >
                  <div class="flex flex-wrap items-center gap-2 text-2xs text-text/60">
                    <span :class="['w-2 h-2 rounded-full', checkStatusClass(round.status)]"></span>
                    <span>{{ t('settings.terminalSnapshotAuditRound', { round: round.round }) }}</span>
                    <span class="text-text/40">
                      FB: {{ round.comparisons.frontBackend ? t('settings.terminalSnapshotAuditCheck.ok') : t('settings.terminalSnapshotAuditCheck.ng') }}
                      · FR: {{ round.comparisons.frontReopen ? t('settings.terminalSnapshotAuditCheck.ok') : t('settings.terminalSnapshotAuditCheck.ng') }}
                      · BR: {{ round.comparisons.backendReopen ? t('settings.terminalSnapshotAuditCheck.ok') : t('settings.terminalSnapshotAuditCheck.ng') }}
                    </span>
                    <span v-if="round.error" class="text-danger ml-auto">{{ round.error }}</span>
                  </div>
                  <div class="grid grid-cols-1 md:grid-cols-3 gap-2 text-2xs text-text/40 font-mono">
                    <div>FRONT: {{ formatSignature(round.front) }}</div>
                    <div>BACK: {{ formatSignature(round.backend) }}</div>
                    <div>REOPEN: {{ formatSignature(round.reopen) }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-2xs text-text/40">{{ t('settings.terminalSnapshotAuditNoMembers') }}</div>
          <div v-if="terminalSnapshotAuditError" class="text-2xs text-danger">{{ terminalSnapshotAuditError }}</div>
        </div>

        <div class="space-y-3">
          <div class="text-xs font-medium text-faint">
            {{ t('settings.terminalCallChainsTitle') }}
          </div>
          <TerminalCallChainList :call-chains="terminalSnapshotAuditCallChains" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useTerminalSnapshotAuditStore } from '@/stores/terminalSnapshotAuditStore';
import TerminalCallChainList from '@/features/terminal/components/TerminalCallChainList.vue';

type SnapshotSignature = { hash: string; lineCount: number };

const emit = defineEmits<{ (e: 'close'): void }>();
const { t } = useI18n();
const terminalSnapshotAuditStore = useTerminalSnapshotAuditStore();
const {
  status: terminalSnapshotAuditStatus,
  results: terminalSnapshotAuditResults,
  lastError: terminalSnapshotAuditError,
  callChains: terminalSnapshotAuditCallChains
} = storeToRefs(terminalSnapshotAuditStore);

const snapshotAuditStatusLabel = computed(() => {
  const status = terminalSnapshotAuditStatus.value;
  if (status === 'running') return t('settings.terminalSnapshotAuditStatus.running');
  if (status === 'passed') return t('settings.terminalSnapshotAuditStatus.passed');
  if (status === 'failed') return t('settings.terminalSnapshotAuditStatus.failed');
  return t('settings.terminalSnapshotAuditStatus.idle');
});

const checkStatusClass = (status: string) => {
  if (status === 'passed') return 'bg-success';
  if (status === 'failed') return 'bg-danger';
  if (status === 'running') return 'bg-warning';
  return 'bg-overlay/20';
};

const formatSignature = (signature?: SnapshotSignature) => {
  if (!signature) return '--';
  return `${signature.hash} (${signature.lineCount})`;
};
</script>
