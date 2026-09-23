<template>
  <Teleport to="body">
    <div v-if="state.open" class="context-menu-overlay">
      <div class="absolute inset-0" @pointerdown="closeMenu" @contextmenu.prevent></div>
      <div
        ref="menuRef"
        class="context-menu-panel rounded-xl glass-modal bg-panel-strong/95 flex flex-col py-1.5 shadow-float overflow-hidden animate-in fade-in zoom-in-95 duration-200 ring-1 ring-border/10 min-w-[180px]"
        :style="menuStyle"
        role="menu"
        @pointerdown.stop
        @contextmenu.prevent
      >
        <template v-for="(entry, index) in state.entries" :key="entry.id ?? `${entry.kind}-${index}`">
          <div v-if="entry.kind === 'separator'" class="h-px bg-line my-1"></div>
          <button
            v-else
            type="button"
            class="relative w-full text-left px-4 py-2.5 text-xs font-semibold flex items-center gap-3 transition-colors"
            :class="entryClass(entry)"
            :disabled="entry.enabled === false"
            role="menuitem"
            @click="handleEntry(entry)"
          >
            <span v-if="entry.icon" class="material-symbols-outlined text-base text-muted">{{ entry.icon }}</span>
            <span class="flex-1">{{ entryLabel(entry) }}</span>
            <span v-if="entry.shortcut" class="text-2xs text-text/40 ">
              {{ entry.shortcut }}
            </span>
          </button>
        </template>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
// 右键菜单渲染层：负责定位与关闭交互。
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useContextMenu } from './useContextMenu';
import type { ContextMenuItem } from './types';

const { t } = useI18n();
const { state, closeMenu, runAction } = useContextMenu();
const menuRef = ref<HTMLDivElement | null>(null);
const menuStyle = ref<Record<string, string>>({});

const entryLabel = (entry: ContextMenuItem) => (entry.labelKey ? t(entry.labelKey) : entry.label ?? '');

const entryClass = (entry: ContextMenuItem) => {
  if (entry.enabled === false) {
    return 'text-text/30 cursor-default';
  }
  if (entry.danger) {
    return 'text-danger hover:bg-danger/20 hover:text-danger hover:ring-1 hover:ring-danger/40';
  }
  return 'text-text hover:bg-overlay/15 hover:text-text hover:ring-1 hover:ring-border/10';
};

const updatePosition = async () => {
  if (!state.open) {
    return;
  }
  await nextTick();
  const menu = menuRef.value;
  if (!menu) {
    return;
  }
  const rect = menu.getBoundingClientRect();
  const padding = 8;
  let left = state.x;
  let top = state.y;
  const maxLeft = window.innerWidth - rect.width - padding;
  const maxTop = window.innerHeight - rect.height - padding;
  if (left > maxLeft) {
    left = Math.max(padding, maxLeft);
  }
  if (top > maxTop) {
    top = Math.max(padding, maxTop);
  }
  menuStyle.value = {
    left: `${left}px`,
    top: `${top}px`
  };
};

const handleEntry = async (entry: ContextMenuItem) => {
  if (entry.enabled === false) {
    return;
  }
  await runAction(entry);
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    event.preventDefault();
    closeMenu();
  }
};

watch(
  () => [state.open, state.x, state.y],
  () => {
    void updatePosition();
  },
  { immediate: true }
);

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>
.context-menu-overlay {
  position: fixed;
  inset: 0;
  z-index: var(--z-context-menu);
}

.context-menu-panel {
  position: absolute;
  z-index: calc(var(--z-context-menu) + 1);
}
</style>
