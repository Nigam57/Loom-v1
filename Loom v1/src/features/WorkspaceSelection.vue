<template>
  <div ref="containerRef" class="relative z-10 mx-auto flex min-h-full w-full max-w-[1100px] flex-grow px-10 py-14">
    <div class="grid w-full grid-cols-1 gap-12 md:grid-cols-[minmax(0,5fr)_minmax(0,7fr)] md:gap-16">
      <section class="flex flex-col md:pt-10">
        <span class="font-mono text-sm text-faint">loom</span>
        <h1 class="mt-3 text-2xl font-semibold text-text">{{ t('workspace.openTitle') }}</h1>
        <p class="mt-2 max-w-[38ch] text-sm text-muted">{{ t('workspace.openSubtitle') }}</p>

        <button
          type="button"
          class="mt-8 inline-flex w-fit items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-on-primary transition-colors hover:bg-primary-hover"
          @click="handleOpenFolder"
        >
          <span class="material-symbols-outlined text-lg">folder_open</span>
          {{ t('workspace.openTitle') }}
        </button>

        <ul class="mt-10 space-y-2 text-xs text-faint">
          <li class="flex gap-2"><span class="font-mono text-primary">01</span>{{ t('workspace.pitch.rooms', 'Put your CLI agents in one room.') }}</li>
          <li class="flex gap-2"><span class="font-mono text-primary">02</span>{{ t('workspace.pitch.ask', 'They ask each other for reviews and help.') }}</li>
          <li class="flex gap-2"><span class="font-mono text-primary">03</span>{{ t('workspace.pitch.local', 'Everything runs on this machine.') }}</li>
        </ul>
      </section>

      <section class="flex min-w-0 flex-col">
        <div
          v-if="workspaceError"
          role="alert"
          class="mb-4 flex items-start gap-3 rounded-lg border border-danger/40 bg-danger/10 px-3 py-2.5"
        >
          <span class="material-symbols-outlined mt-0.5 text-lg text-danger">error</span>
          <div class="min-w-0 flex-1">
            <p class="text-sm font-medium text-text">{{ t('workspace.openErrorTitle') }}</p>
            <p class="selectable mt-0.5 break-words text-xs text-muted">{{ workspaceError }}</p>
          </div>
          <button
            type="button"
            class="rounded p-0.5 text-faint transition-colors hover:bg-hover hover:text-text"
            :aria-label="t('common.dismiss', 'Dismiss')"
            @click="clearWorkspaceError"
          >
            <span class="material-symbols-outlined text-base">close</span>
          </button>
        </div>

        <div class="flex items-center gap-3 border-b border-line pb-3">
          <h2 class="text-sm font-medium text-text">{{ t('workspace.recentTitle') }}</h2>
          <span class="font-mono text-2xs text-faint tabular">{{ allRecent.length }}</span>
          <label v-if="allRecent.length > 4" class="relative ml-auto w-56">
            <span class="material-symbols-outlined pointer-events-none absolute left-2 top-1/2 -translate-y-1/2 text-base text-faint">search</span>
            <input
              v-model="searchQuery"
              type="text"
              class="w-full rounded-md border border-line bg-surface py-1.5 pl-8 pr-2 text-xs text-text placeholder:text-faint focus:border-line-strong focus:outline-none"
              :placeholder="t('workspace.searchPlaceholder')"
            />
          </label>
        </div>

        <div v-if="!allRecent.length" class="py-10">
          <p class="text-sm text-muted">{{ t('workspace.emptyTitle') }}</p>
          <p class="mt-1 text-xs text-faint">{{ t('workspace.emptySubtitle') }}</p>
        </div>

        <ul v-else class="divide-y divide-line">
          <li v-for="workspace in filteredRecent" :key="workspace.id">
            <button
              type="button"
              class="group -mx-2 flex w-[calc(100%+1rem)] items-center gap-3 rounded-md px-2 py-3 text-left transition-colors hover:bg-hover"
              @click="handleOpenRecent(workspace.path)"
            >
              <span class="material-symbols-outlined text-lg text-faint transition-colors group-hover:text-primary">folder</span>
              <span class="min-w-0 flex-1">
                <span class="block truncate text-sm font-medium text-text">{{ workspace.name }}</span>
                <span class="block truncate font-mono text-2xs text-faint">{{ formatWorkspacePath(workspace.path) }}</span>
              </span>
              <time class="shrink-0 text-2xs text-faint tabular">{{ formatRelativeTime(workspace.lastOpenedAt) }}</time>
              <span class="material-symbols-outlined text-base text-faint opacity-0 transition-opacity group-hover:opacity-100">arrow_forward</span>
            </button>
          </li>
          <li v-if="!filteredRecent.length" class="py-4 text-xs text-faint">{{ t('workspace.noResults') }}</li>
        </ul>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
// 工作区选择页：左侧打开入口，右侧可搜索的最近列表；错误内联展示并自动消退。
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useWorkspaceStore } from './workspace/workspaceStore';

const { t, locale } = useI18n();
const workspaceStore = useWorkspaceStore();
const { recentPrimary, recentMore, workspaceError } = storeToRefs(workspaceStore);
const { loadRecent, openWorkspaceDialog, openWorkspaceByPath, clearWorkspaceError } = workspaceStore;

const containerRef = ref<HTMLElement | null>(null);
const searchQuery = ref('');
const errorTimer = ref<number | null>(null);

const allRecent = computed(() => [...recentPrimary.value, ...recentMore.value]);

const handleOpenFolder = async () => {
  await openWorkspaceDialog();
};

const handleOpenRecent = async (path: string) => {
  await openWorkspaceByPath(path);
};

// 兼容 Windows 扩展路径与 UNC 格式，保证展示层可读性。
const formatWorkspacePath = (path: string) => {
  if (!path) return path;
  if (!path.startsWith('\\\\?\\')) return path;
  const trimmed = path.slice(4);
  if (trimmed.toLowerCase().startsWith('unc\\')) {
    return `\\\\${trimmed.slice(4)}`;
  }
  return trimmed;
};

const formatRelativeTime = (timestamp: number) => {
  if (!timestamp) return '';
  const diff = Date.now() - timestamp;
  const rtf = new Intl.RelativeTimeFormat(locale.value, { numeric: 'auto' });
  const seconds = Math.round(diff / 1000);
  if (Math.abs(seconds) < 60) return rtf.format(-seconds, 'second');
  const minutes = Math.round(seconds / 60);
  if (Math.abs(minutes) < 60) return rtf.format(-minutes, 'minute');
  const hours = Math.round(minutes / 60);
  if (Math.abs(hours) < 24) return rtf.format(-hours, 'hour');
  const days = Math.round(hours / 24);
  if (Math.abs(days) < 30) return rtf.format(-days, 'day');
  const months = Math.round(days / 30);
  return rtf.format(-months, 'month');
};

const filteredRecent = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  if (!query) return allRecent.value;
  return allRecent.value.filter(
    (item) => item.name.toLowerCase().includes(query) || formatWorkspacePath(item.path).toLowerCase().includes(query)
  );
});

onMounted(() => {
  void loadRecent();
});

watch(
  () => workspaceError.value,
  (message) => {
    if (errorTimer.value !== null) {
      window.clearTimeout(errorTimer.value);
      errorTimer.value = null;
    }
    if (message) {
      // 错误提示自动关闭，避免长期占用视觉焦点。
      errorTimer.value = window.setTimeout(() => {
        clearWorkspaceError();
        errorTimer.value = null;
      }, 5000);
    }
  }
);

onBeforeUnmount(() => {
  if (errorTimer.value !== null) {
    window.clearTimeout(errorTimer.value);
  }
});
</script>
