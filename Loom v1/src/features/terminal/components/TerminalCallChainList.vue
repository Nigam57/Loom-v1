<!-- [2026-01-23 11:56] 目的: 统一渲染终端调用链步骤与通过状态; 边界: 仅负责展示与样式，不参与测试逻辑; 设计: 根据步骤状态映射图标与颜色，保持诊断与审计一致呈现。 -->
<template>
  <div class="space-y-3">
    <div
      v-for="chain in callChains"
      :key="chain.id"
      class="rounded-xl border border-border/10 bg-overlay/5 px-3 py-3"
    >
      <div class="flex items-center justify-between">
        <div class="text-xs font-semibold text-text/80">{{ chain.title }}</div>
        <span
          class="material-symbols-outlined text-lg"
          :class="chainStatusClass(chain)"
        >
          {{ chainStatusIcon(chain) }}
        </span>
      </div>
      <div class="mt-2 space-y-1.5">
        <div
          v-for="step in chain.steps"
          :key="step.id"
          class="flex items-start gap-2 text-xs text-text/70"
        >
          <span class="material-symbols-outlined text-lg" :class="stepStatusClass(step.status)">
            {{ stepStatusIcon(step.status) }}
          </span>
          <div class="flex-1 space-y-0.5">
            <div class="leading-relaxed">{{ step.label }}</div>
            <div v-if="step.detail" :class="['text-2xs leading-relaxed break-words', stepDetailClass(step.status)]">
              {{ step.detail }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
type StepStatus = 'pending' | 'passed' | 'failed' | 'skipped';
type CallChainStep = { id: string; label: string; status: StepStatus; detail?: string };
type CallChain = { id: string; title: string; steps: CallChainStep[] };

defineProps<{ callChains: CallChain[] }>();

const stepStatusIcon = (status: StepStatus) => {
  if (status === 'passed') return 'check_circle';
  if (status === 'failed') return 'cancel';
  if (status === 'skipped') return 'remove_circle';
  return 'radio_button_unchecked';
};

const stepStatusClass = (status: StepStatus) => {
  if (status === 'passed') return 'text-success';
  if (status === 'failed') return 'text-danger';
  if (status === 'skipped') return 'text-text/30';
  return 'text-text/30';
};

const stepDetailClass = (status: StepStatus) => {
  if (status === 'failed') return 'text-danger';
  if (status === 'passed') return 'text-success/80';
  if (status === 'skipped') return 'text-text/40';
  return 'text-text/40';
};

const chainStatusIcon = (chain: CallChain) => {
  if (chain.steps.some((step) => step.status === 'failed')) return 'cancel';
  if (chain.steps.every((step) => step.status === 'passed' || step.status === 'skipped')) {
    return 'check_circle';
  }
  return 'radio_button_unchecked';
};

const chainStatusClass = (chain: CallChain) => {
  if (chain.steps.some((step) => step.status === 'failed')) return 'text-danger';
  if (chain.steps.every((step) => step.status === 'passed' || step.status === 'skipped')) {
    return 'text-success';
  }
  return 'text-text/30';
};
</script>
