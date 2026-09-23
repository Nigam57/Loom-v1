<template>
  <div class="flex h-full w-full flex-col">
    <header class="shrink-0 border-b border-line px-8 pb-4 pt-8">
      <div class="flex items-center gap-2">
        <h1 class="text-xl font-semibold text-text">{{ t('marketplace.title') }}</h1>
        <span class="rounded-sm border border-line px-1 font-mono text-2xs text-faint">{{ t('common.notBuilt') }}</span>
      </div>
      <p class="mt-1 text-sm text-muted">{{ t('marketplace.subtitle') }}</p>
    </header>

    <div class="custom-scrollbar flex-1 overflow-y-auto px-8 py-6">
      <section class="max-w-[64ch]">
        <p class="text-sm text-text">{{ t('marketplace.notBuiltBody') }}</p>

        <h2 class="mt-8 text-sm font-medium text-text">{{ t('marketplace.plannedTitle') }}</h2>
        <ol class="mt-3 space-y-2.5">
          <li v-for="(key, index) in plannedKeys" :key="key" class="flex gap-3 text-sm text-muted">
            <span class="font-mono text-xs text-faint tabular">{{ String(index + 1).padStart(2, '0') }}</span>
            <span>{{ t(key) }}</span>
          </li>
        </ol>

        <h2 class="mt-8 text-sm font-medium text-text">{{ t('marketplace.todayTitle') }}</h2>
        <p class="mt-2 text-sm text-muted">{{ t('marketplace.todayBody') }}</p>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
// Plugins: not implemented yet. Shown honestly (vision rule: nothing unbuilt is faked) instead of an empty store.
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

const plannedKeys = ['marketplace.planned.github', 'marketplace.planned.shared', 'marketplace.planned.isolated'];
</script>
