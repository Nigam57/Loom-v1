<template>
  <nav class="fixed bottom-0 left-0 right-0 z-overlay flex h-14 shrink-0 flex-row items-center gap-1 border-t border-line bg-background px-2 md:static md:h-full md:w-12 md:flex-col md:gap-1 md:border-r md:border-t-0 md:px-0 md:py-2">
    <div ref="statusMenuRef" class="relative mb-1 hidden md:block">
      <button
        type="button"
        class="group relative block rounded-md p-1 transition-colors hover:bg-hover"
        :aria-expanded="statusMenuOpen"
        :title="t('settings.status')"
        @click="toggleStatusMenu"
      >
        <AvatarBadge :avatar="accountAvatar" :label="accountName" class="h-7 w-7 rounded" />
        <span :class="['absolute bottom-0.5 right-0.5 h-2.5 w-2.5 rounded-full border-2 border-background', statusDotClass]"></span>
      </button>
      <div
        v-if="statusMenuOpen"
        class="absolute left-full top-0 z-menu ml-2 flex w-48 flex-col overflow-hidden rounded-lg border border-line-strong bg-surface-2 py-1 shadow-float animate-in fade-in zoom-in-95 duration-150"
      >
        <div class="px-3 py-1.5 text-2xs font-medium text-faint">{{ t('settings.status') }}</div>
        <button
          v-for="option in statusOptions"
          :key="option.id"
          type="button"
          class="flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
          @click="selectStatus(option.id)"
        >
          <span :class="['h-2 w-2 rounded-full', option.dotClass]"></span>
          {{ t(option.labelKey) }}
          <span v-if="accountStatus === option.id" class="material-symbols-outlined ml-auto text-base text-muted">check</span>
        </button>
      </div>
    </div>

    <div class="mx-auto mb-1 hidden h-px w-6 bg-line md:block"></div>

    <div class="flex w-full flex-row justify-around gap-1 md:flex-col md:justify-start">
      <div v-for="item in navItems" :key="item.id" class="relative flex w-full justify-center">
        <span
          v-if="activeTab === item.id"
          class="absolute left-0 top-1/2 hidden h-5 w-0.5 -translate-y-1/2 rounded-r bg-primary md:block"
        ></span>
        <button
          type="button"
          :title="t(item.tooltipKey)"
          :aria-current="activeTab === item.id ? 'page' : undefined"
          :class="[
            'relative flex h-9 w-9 items-center justify-center rounded-md transition-colors',
            activeTab === item.id ? 'bg-hover text-text' : 'text-faint hover:bg-hover hover:text-text'
          ]"
          @click="handleNavClick(item.id)"
        >
          <span class="material-symbols-outlined text-xl">{{ item.icon }}</span>
          <span
            v-if="shouldShowUnread(item.id)"
            class="absolute -right-0.5 -top-0.5 flex h-4 min-w-4 items-center justify-center rounded-full bg-danger px-1 font-mono text-2xs font-medium leading-none text-background"
          >
            {{ formatUnreadCount(totalUnreadCount) }}
          </span>
        </button>
      </div>

      <div class="relative flex w-full justify-center md:hidden">
        <button
          type="button"
          :title="t('nav.settings')"
          :class="[
            'flex h-9 w-9 items-center justify-center rounded-md transition-colors',
            activeTab === 'settings' ? 'bg-hover text-text' : 'text-faint hover:bg-hover hover:text-text'
          ]"
          @click="emitChange('settings')"
        >
          <span class="material-symbols-outlined text-xl">settings</span>
        </button>
      </div>
    </div>

    <div class="relative mt-auto hidden w-full justify-center md:flex">
      <span v-if="activeTab === 'settings'" class="absolute left-0 top-1/2 h-5 w-0.5 -translate-y-1/2 rounded-r bg-primary"></span>
      <button
        type="button"
        :title="t('nav.settings')"
        :aria-current="activeTab === 'settings' ? 'page' : undefined"
        :class="[
          'flex h-9 w-9 items-center justify-center rounded-md transition-colors',
          activeTab === 'settings' ? 'bg-hover text-text' : 'text-faint hover:bg-hover hover:text-text'
        ]"
        @click="emitChange('settings')"
      >
        <span class="material-symbols-outlined text-xl">settings</span>
      </button>
    </div>
  </nav>
</template>
<script setup lang="ts">
// 侧边导航栏：负责主导航切换与账户状态菜单交互。
import { computed, onBeforeUnmount, onMounted, ref, toRef } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { invoke, isTauri } from '@tauri-apps/api/core';
import AvatarBadge from './AvatarBadge.vue';
import { useSettingsStore, type AccountStatus } from '@/features/global/settingsStore';
import { useProjectStore } from '@/features/workspace/projectStore';
import { useWorkspaceStore } from '@/features/workspace/workspaceStore';
import { CURRENT_USER_ID } from '@/features/chat/data';
import { ensureAvatar } from '@/shared/utils/avatar';
import { useChatStore } from '@/features/chat/chatStore';
type TabId = 'chat' | 'friends' | 'workspaces' | 'store' | 'plugins' | 'settings';

type NavItem = {
  id: TabId;
  icon: string;
  tooltipKey: string;
};

const props = defineProps<{ activeTab: TabId }>();
const emit = defineEmits<{ (e: 'change', tab: TabId): void }>();

const navItems: NavItem[] = [
  { id: 'chat', icon: 'chat_bubble', tooltipKey: 'nav.chat' },
  { id: 'friends', icon: 'group', tooltipKey: 'nav.friends' },
  { id: 'workspaces', icon: 'folder_open', tooltipKey: 'nav.workspaces' },
  { id: 'store', icon: 'storefront', tooltipKey: 'nav.store' },
  { id: 'plugins', icon: 'extension', tooltipKey: 'nav.plugins' }
];

const { t } = useI18n();
const settingsStore = useSettingsStore();
const projectStore = useProjectStore();
const workspaceStore = useWorkspaceStore();
const chatStore = useChatStore();
const { settings } = storeToRefs(settingsStore);
const { currentWorkspace } = storeToRefs(workspaceStore);
const { setAccountStatus } = settingsStore;
const { updateMember } = projectStore;
const { totalUnreadCount } = storeToRefs(chatStore);

const statusMenuRef = ref<HTMLElement | null>(null);
const statusMenuOpen = ref(false);
const accountStatus = computed(() => settings.value.account.status);
const accountAvatar = computed(() => ensureAvatar(settings.value.account.avatar));
const accountName = computed(() => settings.value.account.displayName || t('common.userAvatarAlt'));
const statusOptions: Array<{ id: AccountStatus; labelKey: string; dotClass: string }> = [
  { id: 'online', labelKey: 'settings.statusOptions.online', dotClass: 'bg-success' },
  { id: 'working', labelKey: 'settings.statusOptions.working', dotClass: 'bg-warning' },
  { id: 'dnd', labelKey: 'settings.statusOptions.dnd', dotClass: 'bg-danger' },
  { id: 'offline', labelKey: 'settings.statusOptions.offline', dotClass: 'bg-faint' }
];
const statusDotClass = computed(() => {
  if (accountStatus.value === 'online') return 'bg-success';
  if (accountStatus.value === 'working') return 'bg-warning';
  if (accountStatus.value === 'dnd') return 'bg-danger';
  return 'bg-faint';
});

const toggleStatusMenu = () => {
  statusMenuOpen.value = !statusMenuOpen.value;
};

const selectStatus = (status: AccountStatus) => {
  if (accountStatus.value !== status) {
    setAccountStatus(status);
    void updateMember(CURRENT_USER_ID, { status });
  }
  statusMenuOpen.value = false;
};

const handleClickOutside = (event: MouseEvent) => {
  if (statusMenuRef.value && !statusMenuRef.value.contains(event.target as Node)) {
    statusMenuOpen.value = false;
  }
};

const emitChange = (tab: TabId) => {
  emit('change', tab);
};

const openWorkspaceFolder = async () => {
  const path = currentWorkspace.value?.path?.trim();
  if (!path) {
    emitChange('workspaces');
    return;
  }
  if (!isTauri()) {
    emitChange('workspaces');
    return;
  }
  try {
    await invoke('workspace_open_folder', { path });
  } catch (error) {
    console.error('Failed to open workspace folder.', error);
    emitChange('workspaces');
  }
};

const handleNavClick = (id: TabId) => {
  if (id === 'workspaces') {
    void openWorkspaceFolder();
    return;
  }
  emitChange(id);
};

const shouldShowUnread = (id: TabId) => {
  if (id !== 'chat') return false;
  return totalUnreadCount.value > 0;
};

// 未读数上限展示为 99+，避免徽标过宽影响布局。
const formatUnreadCount = (value: number) => (value > 99 ? '99+' : String(value));

const activeTab = toRef(props, 'activeTab');

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('mousedown', handleClickOutside);
});
</script>
