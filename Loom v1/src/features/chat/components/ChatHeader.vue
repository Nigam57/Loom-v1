<template>
  <header class="h-[52px] px-5 flex items-center gap-3 border-b border-line bg-background sticky top-0 z-sticky">
    <div class="flex min-w-0 flex-1 items-baseline gap-2">
      <span class="font-mono text-sm text-faint" aria-hidden="true">#</span>
      <h1 class="truncate text-base font-semibold text-text">{{ headerTitle }}</h1>
      <p class="hidden truncate text-xs text-faint lg:block">{{ headerDescription }}</p>
    </div>
    <div class="flex items-center gap-1">
      <span v-if="memberCount !== undefined" class="md:hidden">
        <button type="button" class="header-action" @click="emit('open-members')">
          <span class="material-symbols-outlined text-lg">group</span>
          <span class="font-mono text-2xs tabular">{{ memberCount }}</span>
        </button>
      </span>
      <button type="button" class="header-action" :title="t('chat.header.todo')" @click="emit('open-roadmap')">
        <span class="material-symbols-outlined text-lg">checklist</span>
      </button>
      <button type="button" class="header-action" :title="t('chat.header.inventory')" @click="emit('open-skills')">
        <span class="material-symbols-outlined text-lg">backpack</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
// 会话头部组件：展示标题、描述与成员信息。
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{ title?: string; description?: string; memberCount?: number }>();
const emit = defineEmits<{ (e: 'open-roadmap'): void; (e: 'open-skills'): void; (e: 'open-members'): void }>();

const { t } = useI18n();

const headerTitle = computed(() => props.title ?? '');
const headerDescription = computed(() => props.description ?? t('chat.channelDescription'));
</script>

<style scoped>
.header-action {
  display: inline-flex;
  height: 30px;
  min-width: 30px;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 0 6px;
  border-radius: 5px;
  color: oklch(var(--color-text-muted));
  transition: background-color 120ms var(--ease-out), color 120ms var(--ease-out);
}

.header-action:hover {
  background-color: oklch(var(--color-hover));
  color: oklch(var(--color-text));
}
</style>
