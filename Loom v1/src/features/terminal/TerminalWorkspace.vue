<template>
  <section class="flex h-full w-full flex-col overflow-hidden">
    <header class="relative z-sticky flex h-11 items-center justify-between gap-4 border-b border-line bg-background px-4">
      <div class="flex min-w-0 items-baseline gap-2">
        <h1 class="text-sm font-semibold text-text">{{ t('terminal.title') }}</h1>
        <p class="hidden truncate text-2xs text-faint md:block">{{ t('terminal.subtitle') }}</p>
      </div>
      <div class="flex items-center gap-3">
        <div ref="tabSearchRef" class="relative">
          <div class="absolute inset-y-0 left-3 flex items-center pointer-events-none">
            <span class="material-symbols-outlined text-base text-faint">search</span>
          </div>
          <input
            v-model="tabSearchQuery"
            type="text"
            class="w-52 rounded-md border border-line bg-surface py-1 pl-8 pr-7 text-xs text-text placeholder:text-faint transition-colors focus:border-line-strong focus:outline-none focus:ring-0"
            :placeholder="t('terminal.tabSearchPlaceholder')"
            @focus="openTabSearch"
            @keydown="handleTabSearchKeydown"
          />
          <button
            v-if="tabSearchQuery"
            type="button"
            class="absolute inset-y-0 right-2 flex items-center text-text/40 hover:text-text transition-colors"
            @click="clearTabSearch"
          >
            <span class="material-symbols-outlined text-base">close</span>
          </button>
          <div
            v-if="tabSearchOpen && tabSearchHasQuery"
            class="absolute left-0 right-0 z-dropdown mt-1.5 flex flex-col overflow-hidden rounded-lg glass-modal py-1 animate-in fade-in zoom-in-95 duration-150"
          >
            <button
              type="button"
              class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
              @click="handleTabSearchCreate"
            >
              <span class="material-symbols-outlined text-base text-muted">add</span>
              {{ t('terminal.tabSearchCreate') }}
            </button>
            <div class="h-px bg-line my-1"></div>
            <div v-if="tabSearchHasResults" class="max-h-56 overflow-y-auto custom-scrollbar">
              <button
                v-for="tab in tabSearchResults"
                :key="tab.id"
                type="button"
                class="relative flex w-full items-center justify-between gap-3 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                @click="handleTabSearchSelect(tab.id)"
              >
                <span class="truncate">{{ tab.title }}</span>
                <span v-if="tab.id === activeId" class="w-1.5 h-1.5 rounded-full bg-overlay/40"></span>
              </button>
              <div v-if="tabSearchResults.length && tabSearchMemberResults.length" class="h-px bg-line my-1"></div>
              <button
                v-for="member in tabSearchMemberResults"
                :key="member.id"
                type="button"
                class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                @click="handleTabSearchMemberSelect(member.id)"
              >
                <span class="material-symbols-outlined text-base text-muted">person</span>
                <span class="truncate">{{ member.name }}</span>
              </button>
            </div>
            <div v-else class="px-3 py-1.5 text-2xs text-faint">
              {{ t('terminal.tabSearchEmpty') }}
            </div>
          </div>
        </div>
        <button
          type="button"
          class="inline-flex h-7 items-center gap-1.5 rounded-md px-2 text-xs font-medium text-muted transition-colors hover:bg-hover hover:text-text"
          @click="handleNewTab"
        >
          <span class="material-symbols-outlined text-base">add</span>
          {{ t('terminal.newTab') }}
        </button>
      </div>
    </header>

    <div v-show="poolTabs.length || showRecentClosedButton || isDragging || tabs.length === 0" ref="tabBarRef" class="relative flex h-9 items-stretch gap-0 overflow-x-auto border-b border-line bg-background px-2">
      <button
        v-if="showRecentClosedButton"
        type="button"
        class="group flex items-center gap-2 px-3 py-1 rounded-lg border text-2xs font-semibold whitespace-nowrap transition"
        :class="[
          recentClosedDisabled
            ? 'bg-overlay/5 border-border/10 text-text/30 cursor-not-allowed'
            : 'bg-overlay/10 border-border/20 text-text/70 hover:text-text hover:bg-overlay/20 hover:border-border/40'
        ]"
        :disabled="recentClosedDisabled"
        @click="handleOpenRecentClosedTab"
      >
        <span class="material-symbols-outlined text-lg">history</span>
        <span>{{ t('terminal.recentClosedTabs', { count: recentClosedCount }) }}</span>
        <span
          class="ml-1 inline-flex items-center justify-center w-4 h-4 rounded-full text-text/30 hover:text-text hover:bg-overlay/20 transition"
          @pointerdown.stop
          @click.stop="handleDismissRecentClosedTab"
        >
          <span class="material-symbols-outlined text-xs">close</span>
        </span>
      </button>
      <TransitionGroup name="terminal-tab" tag="div" class="flex items-stretch">
        <button
          v-for="tab in poolTabs"
          :key="tab.id"
          type="button"
          :data-tab-id="tab.id"
          data-context-scope="terminal-tab"
          @pointerdown="onPointerDown(tab.id, $event, 'tab-bar')"
          @click="handleTabClick(tab.id, $event)"
          :class="[
            'group -mb-px flex cursor-default items-center gap-1.5 whitespace-nowrap border-b-2 px-3 text-xs transition-colors',
            tab.id === activeId
              ? 'border-primary text-text'
              : 'border-transparent text-muted hover:text-text',
            isDragging && dragId === tab.id ? 'terminal-tab--placeholder' : '',
            dragOverId === tab.id && dragId !== tab.id ? 'ring-1 ring-primary/60' : ''
          ]"
        >
          <span class="material-symbols-outlined text-base text-faint">terminal</span>
          <span class="font-medium">{{ tab.title }}</span>
          <span v-if="tab.pinned" class="material-symbols-outlined text-xs text-text/40">push_pin</span>
          <span
            v-if="tab.hasActivity || tab.isBlinking"
            :class="[
              'ml-1 w-2 h-2 rounded-full bg-primary ',
              tab.isBlinking ? 'terminal-tab__activity--blink' : ''
            ]"
          ></span>
          <span
            class="terminal-tab__close-button inline-flex h-5 w-5 cursor-pointer items-center justify-center rounded text-faint transition-colors hover:bg-hover hover:text-text"
            @pointerdown.stop
            @click.stop="handleCloseTab(tab.id)"
          >
            <span class="material-symbols-outlined terminal-tab__close-icon">close</span>
          </span>
        </button>
      </TransitionGroup>
      <div
        v-if="isDragging && dragGhostTab"
        class="terminal-tab-ghost group flex items-center gap-2 px-3 py-1 rounded-lg border whitespace-nowrap"
        :class="[
          dragGhostTab.id === activeId
            ? 'border-primary text-text'
            : 'bg-overlay/5 border-border/10 text-text/60'
        ]"
        :style="dragGhostStyle"
      >
        <span class="material-symbols-outlined text-base text-faint">terminal</span>
        <span class="text-xs font-semibold">{{ dragGhostTab.title }}</span>
        <span
          v-if="dragGhostTab.hasActivity || dragGhostTab.isBlinking"
          :class="[
            'ml-1 w-2 h-2 rounded-full bg-primary ',
            dragGhostTab.isBlinking ? 'terminal-tab__activity--blink' : ''
          ]"
        ></span>
        <span class="material-symbols-outlined text-base text-text/40">close</span>
      </div>
      <span v-if="tabs.length === 0" class="text-xs text-text/40">
        {{ t('terminal.emptyTabs') }}
      </span>
    </div>

    <div class="flex-1 min-h-0 relative">
      <div v-if="tabs.length === 0" class="h-full flex flex-col items-center justify-center text-center text-text/50">
        <div class="w-14 h-14 rounded-2xl bg-overlay/5 border border-border/10 flex items-center justify-center mb-4">
          <span class="material-symbols-outlined text-2xl">terminal</span>
        </div>
        <p class="text-sm font-semibold text-text/70">{{ t('terminal.emptyTitle') }}</p>
        <p class="text-xs text-text/40 mt-1">{{ t('terminal.emptySubtitle') }}</p>
        <button
          type="button"
          class="mt-4 inline-flex items-center gap-2 px-4 py-2 rounded-lg text-xs font-semibold text-on-primary bg-primary hover:bg-primary-hover transition"
          @click="handleNewTab"
        >
          <span class="material-symbols-outlined text-lg">add</span>
          {{ t('terminal.newTab') }}
        </button>
      </div>
      <div v-else class="h-full w-full">
        <div class="grid h-full w-full gap-px bg-line" :class="layoutClass">
          <div
            v-for="pane in paneSlots"
            :key="pane.id"
            :data-pane-id="pane.id"
            class="relative flex min-h-0 min-w-0 flex-col bg-background"
            :class="[
              pane.id === focusedPaneId
                ? ''
                : '',
              isDragging && dragOverPaneId === pane.id ? 'ring-1 ring-inset ring-primary/60' : ''
            ]"
            @pointerdown.capture="handlePanePointerDown(pane.id)"
          >
            <div
              v-if="pane.tab"
              class="flex h-8 items-stretch overflow-x-auto border-b border-line bg-background px-1"
            >
              <button
                type="button"
                :data-tab-id="pane.tab.id"
                data-context-scope="terminal-tab"
                :class="['group -mb-px flex cursor-default items-center gap-1.5 whitespace-nowrap border-b-2 px-3 text-xs transition-colors', pane.id === focusedPaneId ? 'border-primary text-text' : 'border-transparent text-muted hover:text-text']"
                @pointerdown="onPointerDown(pane.tab.id, $event, 'pane')"
                @click.stop="handlePaneTabClick(pane.tab.id, pane.id, $event)"
              >
                <span class="material-symbols-outlined text-base text-faint">terminal</span>
                <span class="max-w-[160px] truncate font-medium">{{ pane.tab.title }}</span>
                <span
                  v-if="pane.tab.hasActivity || pane.tab.isBlinking"
                  :class="[
                    'ml-1 w-1.5 h-1.5 rounded-full bg-primary ',
                    pane.tab.isBlinking ? 'terminal-tab__activity--blink' : ''
                  ]"
                ></span>
                <span
                  class="terminal-tab__close-button inline-flex h-5 w-5 cursor-pointer items-center justify-center rounded text-faint transition-colors hover:bg-hover hover:text-text"
                  @pointerdown.stop
                  @click.stop="handleCloseTab(pane.tab.id)"
                >
                  <span class="material-symbols-outlined terminal-tab__close-icon">close</span>
                </span>
              </button>
            </div>
            <div v-if="pane.tab" class="flex-1 min-h-0">
              <TerminalPane
                :key="pane.tab.id"
                :terminal-id="pane.tab.id"
                :member-id="pane.tab.memberId"
                :terminal-type="pane.tab.terminalType"
                :use-webgl="layoutMode === 'single' || pane.id === focusedPaneId"
                :active="true"
                :focused="pane.id === focusedPaneId"
              />
            </div>
            <div v-else class="flex h-full w-full items-center justify-center text-center text-text/50">
              <div class="flex flex-col items-center gap-3">
                <span class="material-symbols-outlined text-xl opacity-70">drag_indicator</span>
                <p class="text-xs text-text/40">{{ t('terminal.splitEmpty') }}</p>
                <button
                  type="button"
                  class="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold text-text/80 border border-border/10 bg-overlay/5 hover:bg-overlay/10 hover:text-text transition"
                  @click.stop="handleNewTab(pane.id)"
                >
                  <span class="material-symbols-outlined text-lg">add</span>
                  {{ t('terminal.newTab') }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
// 终端工作区：管理终端标签页、拖拽排序与窗口间事件通信。
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { emit, listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useI18n } from 'vue-i18n';
import { storeToRefs } from 'pinia';
import TerminalPane from './TerminalPane.vue';
import { createSession, onActivity, onStatusChange, trackSession, untrackSession } from './terminalBridge';
import { useTerminalStore, type TerminalPaneId } from './terminalStore';
import { registerTerminalTabContextMenu } from './terminalContextMenu';
import {
  TERMINAL_OPEN_TAB_EVENT,
  TERMINAL_TAB_OPENED_EVENT,
  TERMINAL_WINDOW_READY_EVENT,
  TERMINAL_WINDOW_READY_REQUEST_EVENT,
  type TerminalOpenTabPayload,
  type TerminalTabOpenedPayload
} from './terminalEvents';
import { useToastStore } from '@/stores/toastStore';
import { useTerminalOrchestratorStore } from '@/stores/terminalOrchestratorStore';
import { useProjectStore } from '@/features/workspace/projectStore';
import { useWorkspaceStore } from '@/features/workspace/workspaceStore';
import { logDiagnosticsEvent } from '@/shared/monitoring/diagnostics/logger';
import { hasTerminalConfig } from '@/shared/utils/terminal';
import { readAppData, readWorkspaceData, writeAppData, writeWorkspaceData } from '@/shared/tauri/storage';
import type { Member } from '@/features/chat/types';
import { CURRENT_USER_ID } from '@/features/chat/data';
import { resolveMemberDisplayName } from '@/shared/utils/memberDisplay';

const { t } = useI18n();
const toastStore = useToastStore();
const { pushToast } = toastStore;
const terminalOrchestratorStore = useTerminalOrchestratorStore();
const { openMemberTerminal } = terminalOrchestratorStore;
const terminalStore = useTerminalStore();
const { tabs, activeId, layoutMode, paneAssignments, focusedPaneId, paneIds } = storeToRefs(terminalStore);
const {
  createTab,
  setActive,
  closeTab,
  moveTabToIndex,
  swapTabOrder,
  markActivity,
  clearActivity,
  openTab,
  assignTabToPane,
  unassignTab,
  syncPaneAssignments
} = terminalStore;
const projectStore = useProjectStore();
const { members } = storeToRefs(projectStore);
const workspaceStore = useWorkspaceStore();
const { currentWorkspace } = storeToRefs(workspaceStore);

const tabBarRef = ref<HTMLDivElement | null>(null);
const tabSearchRef = ref<HTMLDivElement | null>(null);
const tabSearchQuery = ref('');
const tabSearchOpen = ref(false);
const backendMembers = ref<Member[]>([]);
const normalizedTabSearch = computed(() => tabSearchQuery.value.trim().toLowerCase());
const tabSearchHasQuery = computed(() => normalizedTabSearch.value.length > 0);
const terminalFriendCandidates = computed(() => {
  const merged = new Map<string, Member>();
  for (const member of backendMembers.value) {
    if (!member || !member.id) {
      continue;
    }
    merged.set(member.id, member);
  }
  for (const member of members.value) {
    if (!member || !member.id) {
      continue;
    }
    merged.set(member.id, member);
  }
  return Array.from(merged.values()).filter((member) => member.id !== CURRENT_USER_ID);
});
const resolveDisplayName = (member: Member) => resolveMemberDisplayName(member).trim();
const tabSearchResults = computed(() => {
  const query = normalizedTabSearch.value;
  if (!query) {
    return [];
  }
  return tabs.value.filter((tab) => tab.title.toLowerCase().includes(query));
});
const tabSearchMemberResults = computed(() => {
  const query = normalizedTabSearch.value;
  if (!query) {
    return [];
  }
  const tabTitleSet = new Set(tabs.value.map((tab) => tab.title.trim().toLowerCase()));
  return terminalFriendCandidates.value
    .filter((member) => hasTerminalConfig(member.terminalType, member.terminalCommand))
    .filter((member) => {
      const name = resolveDisplayName(member);
      const nameLower = name.toLowerCase();
      const idLower = member.id.toLowerCase();
      if (!nameLower.includes(query) && !idLower.includes(query)) {
        return false;
      }
      if (name && tabTitleSet.has(nameLower)) {
        return false;
      }
      return true;
    })
    .map((member) => ({
      id: member.id,
      name: resolveDisplayName(member) || member.id
    }));
});
const tabSearchHasResults = computed(
  () => tabSearchResults.value.length > 0 || tabSearchMemberResults.value.length > 0
);
const recentClosedTabs = ref<RecentClosedTab[]>([]);
const recentClosedDismissed = ref(false);
const recentClosedCandidateTabs = computed(() => {
  if (recentClosedTabs.value.length === 0) {
    return [];
  }
  const memberMap = new Map(terminalFriendCandidates.value.map((member) => [member.id, member]));
  return recentClosedTabs.value.filter((entry) => {
    const member = memberMap.get(entry.memberId);
    if (!member) {
      return false;
    }
    return hasTerminalConfig(member.terminalType, member.terminalCommand);
  });
});
const recentClosedCount = computed(() => recentClosedCandidateTabs.value.length);
const recentClosedDisabled = computed(() => recentClosedCount.value === 0);
const showRecentClosedButton = computed(() => !recentClosedDismissed.value && recentClosedCount.value > 0);
const layoutClass = computed(() => {
  switch (layoutMode.value) {
    case 'split-vertical':
      return 'grid-cols-2 grid-rows-1';
    case 'split-horizontal':
      return 'grid-cols-1 grid-rows-2';
    case 'grid-2x2':
      return 'grid-cols-2 grid-rows-2';
    default:
      return 'grid-cols-1 grid-rows-1';
  }
});
const isTabAssigned = (terminalId: string) =>
  paneIds.value.some((paneId) => paneAssignments.value[paneId] === terminalId);
const poolTabs = computed(() => tabs.value.filter((tab) => !isTabAssigned(tab.id)));
const paneSlots = computed(() =>
  paneIds.value.map((paneId) => {
    const terminalId = paneAssignments.value[paneId] ?? null;
    const tab = terminalId ? tabs.value.find((item) => item.id === terminalId) ?? null : null;
    return { id: paneId, terminalId, tab };
  })
);
const BULK_OPEN_EXPIRE_MS = 8000;
const bulkOpenState = ref<{
  activeMemberId: string;
  remaining: Set<string>;
  expiresAt: number;
} | null>(null);
let bulkOpenTimer: number | null = null;
const dragId = ref<string | null>(null);
const dragOverId = ref<string | null>(null);
const dragOverPaneId = ref<TerminalPaneId | null>(null);
const dragPointerId = ref<number | null>(null);
const isDragging = ref(false);
const dragSource = ref<'tab-bar' | 'pane'>('tab-bar');
const isCreating = ref(false);
const suppressClick = ref(false);
const dragInsertIndex = ref<number | null>(null);
const dragGhostLeft = ref(0);
const dragGhostTop = ref(0);
const dragGhostWidth = ref(0);
const dragGhostHeight = ref(0);
const dragGrabOffsetX = ref(0);
const dragGrabOffsetY = ref(0);
const lastPointerX = ref<number | null>(null);
const lastPointerY = ref<number | null>(null);
const dragCaptureTarget = ref<HTMLElement | null>(null);
// 拖拽阈值与自动滚动参数用于平衡手感与误触。
const pointerMoveThreshold = 4;
const snapThreshold = 10;
const autoScrollEdge = 28;
const autoScrollMaxSpeed = 16;
let autoScrollRaf: number | null = null;
let autoScrollVelocity = 0;
let dragStartX = 0;
let dragStartY = 0;
const statusBySession = new Map<string, string>();

type WorkspaceWindowContext = { id?: string; name?: string; path?: string };
type TerminalWindowFlags = { __LOOM_TERMINAL_AUTO_TAB__?: boolean };
type RecentClosedTab = { memberId: string; closedAt: number };

const MAX_RECENT_CLOSED_TABS = 12;

const resolveTabTitle = (memberId: string | undefined, payloadTitle: string) => {
  if (memberId) {
    const member = members.value.find((item) => item.id === memberId);
    const displayName = member ? resolveMemberDisplayName(member).trim() : '';
    if (displayName) {
      return displayName;
    }
  }
  const fallbackTitle = payloadTitle.trim();
  return fallbackTitle || payloadTitle;
};
const buildMemberTerminalId = (workspaceId: string, memberId: string) => `member-${workspaceId}-${memberId}`;

// 支持从窗口初始化脚本读取工作区上下文，兼容独立终端窗口。
const resolveWorkspaceContext = () => {
  const workspace = currentWorkspace.value;
  if (workspace?.path) {
    return { cwd: workspace.path, workspaceId: workspace.id };
  }
  if (typeof window === 'undefined') {
    return { cwd: undefined, workspaceId: undefined };
  }
  const meta = (window as typeof window & { __LOOM_WORKSPACE__?: WorkspaceWindowContext }).__LOOM_WORKSPACE__;
  const path = typeof meta?.path === 'string' ? meta.path.trim() : '';
  const id = typeof meta?.id === 'string' ? meta.id.trim() : '';
  return { cwd: path || undefined, workspaceId: id || undefined };
};

const projectDataAppPath = (workspaceId: string) => `${workspaceId}/project.json`;
const projectDataPath = '.loom/workspace.json';

const normalizeRecentClosedTabs = (candidate: unknown): RecentClosedTab[] => {
  if (!Array.isArray(candidate)) {
    return [];
  }
  const normalized: RecentClosedTab[] = [];
  const memberSet = new Set<string>();
  for (const entry of candidate) {
    let memberId = '';
    let closedAt = 0;
    if (typeof entry === 'string') {
      memberId = entry.trim();
    } else if (entry && typeof entry === 'object') {
      const rawMemberId = (entry as { memberId?: unknown }).memberId;
      const rawClosedAt = (entry as { closedAt?: unknown }).closedAt;
      memberId = typeof rawMemberId === 'string' ? rawMemberId.trim() : '';
      closedAt = typeof rawClosedAt === 'number' && Number.isFinite(rawClosedAt) ? rawClosedAt : 0;
    }
    if (!memberId || memberSet.has(memberId)) {
      continue;
    }
    memberSet.add(memberId);
    normalized.push({ memberId, closedAt });
    if (normalized.length >= MAX_RECENT_CLOSED_TABS) {
      break;
    }
  }
  return normalized;
};

const loadRecentClosedTabs = async () => {
  const { cwd, workspaceId } = resolveWorkspaceContext();
  if (!workspaceId) {
    recentClosedTabs.value = [];
    return;
  }
  let stored: Record<string, unknown> | null = null;
  if (cwd) {
    try {
      stored = await readWorkspaceData<Record<string, unknown>>(cwd, projectDataPath);
    } catch (error) {
      console.error('Failed to read workspace recent closed tabs.', error);
    }
  }
  if (!stored) {
    try {
      stored = await readAppData<Record<string, unknown>>(projectDataAppPath(workspaceId));
    } catch (error) {
      console.error('Failed to read cached recent closed tabs.', error);
    }
  }
  const terminal = stored?.terminal;
  const recentClosed = terminal && typeof terminal === 'object' ? (terminal as { recentClosedTabs?: unknown }).recentClosedTabs : null;
  recentClosedTabs.value = normalizeRecentClosedTabs(recentClosed);
};

// 最近关闭列表写入项目数据，工作区不可写时回退到 app data。
const persistRecentClosedTabs = async (nextTabs: RecentClosedTab[]) => {
  const { cwd, workspaceId } = resolveWorkspaceContext();
  if (!workspaceId) {
    return;
  }
  const normalized = normalizeRecentClosedTabs(nextTabs);
  let stored: Record<string, unknown> | null = null;
  if (cwd) {
    try {
      stored = await readWorkspaceData<Record<string, unknown>>(cwd, projectDataPath);
    } catch (error) {
      console.error('Failed to read workspace project data.', error);
    }
  }
  if (!stored) {
    try {
      stored = await readAppData<Record<string, unknown>>(projectDataAppPath(workspaceId));
    } catch (error) {
      console.error('Failed to read cached project data.', error);
    }
  }
  const basePayload =
    stored && typeof stored === 'object' && !Array.isArray(stored)
      ? stored
      : {};
  const terminal =
    basePayload.terminal && typeof basePayload.terminal === 'object' && !Array.isArray(basePayload.terminal)
      ? basePayload.terminal
      : {};
  const nextPayload = {
    ...basePayload,
    terminal: {
      ...terminal,
      recentClosedTabs: normalized
    }
  };
  let wroteWorkspace = false;
  if (cwd) {
    try {
      await writeWorkspaceData(cwd, projectDataPath, nextPayload);
      wroteWorkspace = true;
    } catch (error) {
      console.error('Failed to write recent closed tabs to workspace data.', error);
    }
  }
  if (!wroteWorkspace) {
    try {
      await writeAppData(projectDataAppPath(workspaceId), nextPayload);
    } catch (error) {
      console.error('Failed to write recent closed tabs to app data.', error);
    }
  }
};

const updateRecentClosedTabs = async (nextTabs: RecentClosedTab[]) => {
  const normalized = normalizeRecentClosedTabs(nextTabs);
  recentClosedTabs.value = normalized;
  await persistRecentClosedTabs(normalized);
};

const clearBulkOpenState = () => {
  bulkOpenState.value = null;
  if (bulkOpenTimer !== null) {
    window.clearTimeout(bulkOpenTimer);
    bulkOpenTimer = null;
  }
};

const beginBulkOpen = (memberIds: string[]) => {
  const uniqueIds = Array.from(new Set(memberIds.map((id) => id.trim()).filter((id) => id)));
  if (uniqueIds.length === 0) {
    clearBulkOpenState();
    return;
  }
  bulkOpenState.value = {
    activeMemberId: uniqueIds[0],
    remaining: new Set(uniqueIds),
    expiresAt: Date.now() + BULK_OPEN_EXPIRE_MS
  };
  if (bulkOpenTimer !== null) {
    window.clearTimeout(bulkOpenTimer);
  }
  bulkOpenTimer = window.setTimeout(() => {
    clearBulkOpenState();
  }, BULK_OPEN_EXPIRE_MS + 300);
};

const resolveBulkActivation = (memberId?: string) => {
  const state = bulkOpenState.value;
  if (!state || !memberId) {
    return true;
  }
  if (Date.now() > state.expiresAt) {
    clearBulkOpenState();
    return true;
  }
  if (!state.remaining.has(memberId)) {
    return true;
  }
  state.remaining.delete(memberId);
  const shouldActivate = memberId === state.activeMemberId;
  if (state.remaining.size === 0) {
    clearBulkOpenState();
  }
  return shouldActivate;
};

const loadBackendMembers = async () => {
  const { cwd, workspaceId } = resolveWorkspaceContext();
  if (!workspaceId) {
    backendMembers.value = [];
    return;
  }
  let stored: { members?: Member[] } | null = null;
  if (cwd) {
    try {
      stored = await readWorkspaceData<{ members?: Member[] }>(cwd, projectDataPath);
    } catch (error) {
      console.error('Failed to read workspace member data.', error);
    }
  }
  if (!stored) {
    try {
      stored = await readAppData<{ members?: Member[] }>(projectDataAppPath(workspaceId));
    } catch (error) {
      console.error('Failed to read cached member data.', error);
    }
  }
  backendMembers.value = Array.isArray(stored?.members) ? stored.members : [];
};

const isAutoTabEnabled = () => {
  if (typeof window === 'undefined') {
    return true;
  }
  const flag = (window as typeof window & TerminalWindowFlags).__LOOM_TERMINAL_AUTO_TAB__;
  return flag !== false;
};

const handleNewTab = async (paneId?: TerminalPaneId) => {
  if (isCreating.value) {
    return;
  }
  isCreating.value = true;
  try {
    const { cwd, workspaceId } = resolveWorkspaceContext();
    const terminalId = await createTab({ cwd, workspaceId });
    trackSession(terminalId);
    if (paneId) {
      assignTabToPane(terminalId, paneId, { focus: true, activate: true });
      setActive(terminalId);
    }
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    if (message.includes('terminal buffer limit reached')) {
      pushToast(t('terminal.resourceLimit'), { tone: 'error' });
    } else {
      console.error('Failed to start terminal.', error);
    }
  } finally {
    isCreating.value = false;
  }
};


const openTabSearch = () => {
  tabSearchOpen.value = true;
  void loadBackendMembers();
};

const closeTabSearch = () => {
  tabSearchOpen.value = false;
};

const resetTabSearch = () => {
  tabSearchQuery.value = '';
  tabSearchOpen.value = false;
};

const clearTabSearch = () => {
  resetTabSearch();
};

const scrollTabIntoView = (terminalId: string) => {
  const container = tabBarRef.value;
  if (!container) {
    return;
  }
  const element = container.querySelector<HTMLElement>(`[data-tab-id="${terminalId}"]`);
  if (!element) {
    return;
  }
  const targetLeft = element.offsetLeft;
  const targetRight = targetLeft + element.offsetWidth;
  const viewLeft = container.scrollLeft;
  const viewRight = viewLeft + container.clientWidth;
  if (targetLeft < viewLeft) {
    container.scrollLeft = Math.max(0, targetLeft - 12);
  } else if (targetRight > viewRight) {
    container.scrollLeft = targetRight - container.clientWidth + 12;
  }
};

const resolvePaneForTab = (terminalId: string) => {
  for (const paneId of paneIds.value) {
    if (paneAssignments.value[paneId] === terminalId) {
      return paneId;
    }
  }
  return null;
};

const focusTabInPane = (terminalId: string, options?: { paneId?: TerminalPaneId; scroll?: boolean }) => {
  const existingPane = resolvePaneForTab(terminalId);
  const targetPane = options?.paneId ?? existingPane ?? focusedPaneId.value ?? paneIds.value[0];
  if (targetPane) {
    assignTabToPane(terminalId, targetPane, { focus: true, activate: true });
  }
  setActive(terminalId);
  if (options?.scroll) {
    scrollTabIntoView(terminalId);
  }
};

const handlePanePointerDown = (paneId: TerminalPaneId) => {
  if (focusedPaneId.value === paneId) {
    return;
  }
  focusedPaneId.value = paneId;
  const terminalId = paneAssignments.value[paneId] ?? null;
  if (terminalId) {
    setActive(terminalId);
  }
};

const handleTabSearchSelect = (terminalId: string) => {
  focusTabInPane(terminalId, { scroll: true });
  resetTabSearch();
};

const openMemberTerminalById = async (
  memberId: string,
  options?: { activate?: boolean; scroll?: boolean; resetSearch?: boolean }
) => {
  const shouldActivate = options?.activate ?? true;
  const shouldScroll = options?.scroll ?? true;
  const existingTab = tabs.value.find((tab) => tab.memberId === memberId);
  if (existingTab) {
    if (shouldActivate) {
      focusTabInPane(existingTab.id, { scroll: shouldScroll });
    }
    if (options?.resetSearch) {
      resetTabSearch();
    }
    return true;
  }
  const member = terminalFriendCandidates.value.find((entry) => entry.id === memberId);
  if (options?.resetSearch) {
    resetTabSearch();
  }
  if (!member) {
    return false;
  }
  let terminalId: string | null = null;
  if (currentWorkspace.value?.id) {
    const entry = await openMemberTerminal(member);
    terminalId = entry?.terminalId ?? null;
  } else {
    const context = resolveWorkspaceContext();
    if (!context.workspaceId) {
      return;
    }
    const stableTerminalId = buildMemberTerminalId(context.workspaceId, member.id);
    try {
      terminalId = await createSession({
        cwd: context.cwd,
        workspaceId: context.workspaceId,
        memberId: member.id,
        memberName: member.name,
        keepAlive: true,
        terminalType: member.terminalType,
        terminalCommand: member.terminalCommand,
        terminalPath: member.terminalPath,
        postReadyMode: 'none'
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      if (!message.includes('terminal session already exists')) {
        console.error('Failed to open member terminal.', error);
        return false;
      }
      terminalId = stableTerminalId;
    }
    trackSession(terminalId);
    openTab(terminalId, {
      title: resolveTabTitle(member.id, member.name),
      memberId: member.id,
      terminalType: member.terminalType,
      keepAlive: true,
      activate: shouldActivate
    });
  }
  if (!terminalId) {
    return false;
  }
  if (shouldActivate) {
    focusTabInPane(terminalId, { scroll: shouldScroll });
  }
  return true;
};

const handleTabSearchMemberSelect = async (memberId: string) => {
  clearBulkOpenState();
  return openMemberTerminalById(memberId, { activate: true, scroll: true, resetSearch: true });
};

const handleOpenRecentClosedTab = async () => {
  if (recentClosedDisabled.value) {
    return;
  }
  const candidates = recentClosedCandidateTabs.value;
  if (candidates.length === 0) {
    await updateRecentClosedTabs([]);
    recentClosedDismissed.value = true;
    return;
  }
  recentClosedDismissed.value = true;
  void updateRecentClosedTabs([]);
  const { workspaceId } = resolveWorkspaceContext();
  if (workspaceId) {
    const memberMap = new Map(terminalFriendCandidates.value.map((member) => [member.id, member]));
    for (const entry of candidates) {
      const member = memberMap.get(entry.memberId);
      if (!member) {
        continue;
      }
      const terminalId = buildMemberTerminalId(workspaceId, entry.memberId);
      openTab(terminalId, {
        title: resolveTabTitle(member.id, member.name),
        memberId: member.id,
        terminalType: member.terminalType,
        keepAlive: true,
        activate: false
      });
    }
  }
  beginBulkOpen(candidates.map((entry) => entry.memberId));
  const tasks = candidates.map((entry, index) => {
    const shouldActivate = index === 0;
    return openMemberTerminalById(entry.memberId, { activate: shouldActivate, scroll: shouldActivate });
  });
  await Promise.allSettled(tasks);
  clearBulkOpenState();
};

const handleDismissRecentClosedTab = async () => {
  await updateRecentClosedTabs([]);
  recentClosedDismissed.value = true;
  clearBulkOpenState();
};

const handleTabSearchCreate = async () => {
  resetTabSearch();
  await handleNewTab();
};

const isEnterKey = (event: KeyboardEvent) => event.key === 'Enter' || event.code === 'NumpadEnter';

const handleTabSearchKeydown = async (event: KeyboardEvent) => {
  if (isEnterKey(event)) {
    if (!tabSearchHasQuery.value) {
      return;
    }
    event.preventDefault();
    const firstTab = tabSearchResults.value[0];
    if (firstTab) {
      handleTabSearchSelect(firstTab.id);
      return;
    }
    const firstMember = tabSearchMemberResults.value[0];
    if (firstMember) {
      await handleTabSearchMemberSelect(firstMember.id);
      return;
    }
    await handleTabSearchCreate();
    return;
  }
  if (event.key === 'Escape') {
    event.preventDefault();
    resetTabSearch();
  }
};

const handleTabSearchOutside = (event: PointerEvent) => {
  const target = event.target;
  if (!(target instanceof Node)) {
    return;
  }
  if (tabSearchRef.value && tabSearchRef.value.contains(target)) {
    return;
  }
  closeTabSearch();
};

const cleanupDragListeners = () => {
  window.removeEventListener('pointermove', onPointerMove);
  window.removeEventListener('pointerup', onPointerUp);
  window.removeEventListener('pointercancel', onPointerUp);
};

const cleanupDragState = () => {
  dragOverId.value = null;
  dragId.value = null;
  dragOverPaneId.value = null;
  dragPointerId.value = null;
  isDragging.value = false;
  dragSource.value = 'tab-bar';
  dragInsertIndex.value = null;
  dragGhostLeft.value = 0;
  dragGhostTop.value = 0;
  dragGhostWidth.value = 0;
  dragGhostHeight.value = 0;
  dragGrabOffsetX.value = 0;
  dragGrabOffsetY.value = 0;
  lastPointerX.value = null;
  lastPointerY.value = null;
  dragCaptureTarget.value = null;
};

const stopAutoScroll = () => {
  if (autoScrollRaf !== null) {
    window.cancelAnimationFrame(autoScrollRaf);
    autoScrollRaf = null;
  }
  autoScrollVelocity = 0;
};

const startAutoScroll = () => {
  if (autoScrollRaf !== null) {
    return;
  }
  const step = () => {
    if (!isDragging.value || autoScrollVelocity === 0) {
      autoScrollRaf = null;
      return;
    }
    const container = tabBarRef.value;
    if (container) {
      const prevScroll = container.scrollLeft;
      container.scrollLeft += autoScrollVelocity;
      if (container.scrollLeft !== prevScroll && lastPointerX.value !== null && lastPointerY.value !== null) {
        updateGhostPosition(lastPointerX.value, lastPointerY.value);
        updateDragTarget(lastPointerX.value);
        if (lastPointerY.value !== null) {
          updateDragPaneTarget(lastPointerX.value, lastPointerY.value);
        }
      }
    }
    autoScrollRaf = window.requestAnimationFrame(step);
  };
  autoScrollRaf = window.requestAnimationFrame(step);
};

const onPointerDown = (id: string, event: PointerEvent, source: 'tab-bar' | 'pane' = 'tab-bar') => {
  if (event.button !== 0) {
    return;
  }
  dragId.value = id;
  dragOverId.value = null;
  dragOverPaneId.value = null;
  dragInsertIndex.value = null;
  dragPointerId.value = event.pointerId;
  dragSource.value = source;
  dragStartX = event.clientX;
  dragStartY = event.clientY;
  lastPointerX.value = event.clientX;
  lastPointerY.value = event.clientY;
  isDragging.value = false;
  suppressClick.value = false;
  const target = event.currentTarget as HTMLElement | null;
  const rect = target?.getBoundingClientRect();
  if (rect) {
    dragGhostWidth.value = rect.width;
    dragGhostHeight.value = rect.height;
    dragGrabOffsetX.value = event.clientX - rect.left;
    dragGrabOffsetY.value = event.clientY - rect.top;
  } else {
    dragGhostWidth.value = 0;
    dragGhostHeight.value = 0;
    dragGrabOffsetX.value = 0;
    dragGrabOffsetY.value = 0;
  }
  dragCaptureTarget.value = target;
  if (target && target.setPointerCapture) {
    target.setPointerCapture(event.pointerId);
  }
  updateGhostPosition(event.clientX, event.clientY);
  event.preventDefault();
  window.addEventListener('pointermove', onPointerMove);
  window.addEventListener('pointerup', onPointerUp);
  window.addEventListener('pointercancel', onPointerUp);
};

const resolveInsertTarget = (clientX: number) => {
  const container = tabBarRef.value;
  if (!container || !dragId.value) {
    return null;
  }
  const elements = Array.from(container.querySelectorAll<HTMLElement>('[data-tab-id]')).filter(
    (element) => element.dataset.tabId && element.dataset.tabId !== dragId.value
  );
  if (elements.length === 0) {
    return { insertIndex: 0, overId: null, midpoint: null };
  }
  for (let index = 0; index < elements.length; index += 1) {
    const rect = elements[index].getBoundingClientRect();
    const midpoint = rect.left + rect.width / 2;
    if (clientX < midpoint) {
      return { insertIndex: index, overId: elements[index].dataset.tabId ?? null, midpoint };
    }
  }
  const last = elements[elements.length - 1];
  const lastRect = last.getBoundingClientRect();
  const lastMidpoint = lastRect.left + lastRect.width / 2;
  return { insertIndex: elements.length, overId: last?.dataset.tabId ?? null, midpoint: lastMidpoint };
};

const updateAutoScroll = (clientX: number) => {
  if (dragSource.value !== 'tab-bar') {
    stopAutoScroll();
    return;
  }
  const container = tabBarRef.value;
  if (!container) {
    stopAutoScroll();
    return;
  }
  const rect = container.getBoundingClientRect();
  const distanceLeft = clientX - rect.left;
  const distanceRight = rect.right - clientX;
  let nextVelocity = 0;
  if (distanceLeft < autoScrollEdge) {
    const clamped = Math.max(0, distanceLeft);
    nextVelocity = -autoScrollMaxSpeed * (1 - clamped / autoScrollEdge);
  } else if (distanceRight < autoScrollEdge) {
    const clamped = Math.max(0, distanceRight);
    nextVelocity = autoScrollMaxSpeed * (1 - clamped / autoScrollEdge);
  }
  autoScrollVelocity = nextVelocity;
  if (autoScrollVelocity !== 0) {
    startAutoScroll();
  } else {
    stopAutoScroll();
  }
};

const updateGhostPosition = (clientX: number, clientY: number) => {
  dragGhostLeft.value = clientX - dragGrabOffsetX.value;
  dragGhostTop.value = clientY - dragGrabOffsetY.value;
};

const updateDragTarget = (clientX: number) => {
  if (!dragId.value || dragSource.value !== 'tab-bar') {
    dragOverId.value = null;
    return;
  }
  const target = resolveInsertTarget(clientX);
  let nextIndex = target?.insertIndex ?? null;
  dragOverId.value = target?.overId ?? null;
  if (target?.midpoint !== null && dragInsertIndex.value !== null && nextIndex !== null) {
    const distance = Math.abs(clientX - target.midpoint);
    if (distance <= snapThreshold && Math.abs(nextIndex - dragInsertIndex.value) <= 1) {
      nextIndex = dragInsertIndex.value;
    }
  }
  if (nextIndex !== null && nextIndex !== dragInsertIndex.value) {
    dragInsertIndex.value = nextIndex;
    moveTabToIndex(dragId.value, nextIndex);
  } else if (nextIndex !== null) {
    dragInsertIndex.value = nextIndex;
  }
};

const resolvePaneDropTarget = (clientX: number, clientY: number) => {
  const element = document.elementFromPoint(clientX, clientY) as HTMLElement | null;
  if (!element) {
    return null;
  }
  const pane = element.closest('[data-pane-id]') as HTMLElement | null;
  const paneId = pane?.dataset.paneId as TerminalPaneId | undefined;
  if (!paneId || !paneIds.value.includes(paneId)) {
    return null;
  }
  return paneId;
};

const updateDragPaneTarget = (clientX: number, clientY: number) => {
  if (!isDragging.value || !dragId.value) {
    dragOverPaneId.value = null;
    return;
  }
  dragOverPaneId.value = resolvePaneDropTarget(clientX, clientY);
};

const isPointInTabBar = (clientX: number, clientY: number) => {
  const bar = tabBarRef.value;
  if (!bar) {
    return false;
  }
  const rect = bar.getBoundingClientRect();
  return clientX >= rect.left && clientX <= rect.right && clientY >= rect.top && clientY <= rect.bottom;
};

const onPointerMove = (event: PointerEvent) => {
  if (!dragId.value || dragPointerId.value !== event.pointerId) {
    return;
  }
  if (!isDragging.value) {
    const distance = Math.max(Math.abs(event.clientX - dragStartX), Math.abs(event.clientY - dragStartY));
    if (distance < pointerMoveThreshold) {
      return;
    }
    isDragging.value = true;
  }

  event.preventDefault();
  updateGhostPosition(event.clientX, event.clientY);
  lastPointerX.value = event.clientX;
  lastPointerY.value = event.clientY;
  updateAutoScroll(event.clientX);
  updateDragTarget(event.clientX);
  updateDragPaneTarget(event.clientX, event.clientY);
};

const onPointerUp = (event: PointerEvent) => {
  if (dragPointerId.value !== null && event.pointerId !== dragPointerId.value) {
    return;
  }
  if (dragCaptureTarget.value?.hasPointerCapture(event.pointerId)) {
    dragCaptureTarget.value.releasePointerCapture(event.pointerId);
  }
  if (isDragging.value) {
    suppressClick.value = true;
    window.setTimeout(() => {
      suppressClick.value = false;
    }, 0);
  }
  const dropTabId = dragId.value;
  const pointerX = event.clientX;
  const pointerY = event.clientY;
  const dropPaneId = isDragging.value
    ? resolvePaneDropTarget(pointerX, pointerY) ?? dragOverPaneId.value
    : null;
  if (isDragging.value && dropPaneId && dropTabId) {
    focusTabInPane(dropTabId, { paneId: dropPaneId });
  } else if (isDragging.value && dragSource.value === 'pane' && dropTabId) {
    if (isPointInTabBar(pointerX, pointerY)) {
      unassignTab(dropTabId);
    }
  }
  stopAutoScroll();
  cleanupDragListeners();
  cleanupDragState();
};

const dragGhostTab = computed(() => {
  if (!dragId.value) {
    return null;
  }
  return tabs.value.find((tab) => tab.id === dragId.value) ?? null;
});

const dragGhostStyle = computed(() => {
  if (!isDragging.value || !dragGhostTab.value) {
    return undefined;
  }
  return {
    left: `${dragGhostLeft.value}px`,
    top: `${dragGhostTop.value}px`,
    width: `${dragGhostWidth.value}px`,
    height: `${dragGhostHeight.value}px`
  };
});

const handleTabClick = (id: string, event: MouseEvent) => {
  if (suppressClick.value) {
    suppressClick.value = false;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
  const targetPane = focusedPaneId.value ?? paneIds.value[0];
  const currentTabId = targetPane ? paneAssignments.value[targetPane] ?? null : null;
  if (currentTabId && currentTabId !== id && !isTabAssigned(id)) {
    swapTabOrder(currentTabId, id);
  }
  focusTabInPane(id);
};

const handlePaneTabClick = (id: string, paneId: TerminalPaneId, event: MouseEvent) => {
  if (suppressClick.value) {
    suppressClick.value = false;
    event.preventDefault();
    event.stopPropagation();
    return;
  }
  focusedPaneId.value = paneId;
  setActive(id);
};

const handleCloseTab = async (terminalId: string) => {
  untrackSession(terminalId);
  await closeTab(terminalId);
};

const stopActivity = onActivity((terminalId) => {
  markActivity(terminalId);
});
const stopStatus = onStatusChange((payload) => {
  const previous = statusBySession.get(payload.terminalId);
  statusBySession.set(payload.terminalId, payload.status);
  if (payload.status === 'working') {
    const tab = tabs.value.find((item) => item.id === payload.terminalId);
    if (tab) {
      tab.isBlinking = false;
    }
    return;
  }
  if (payload.status === 'online' && previous === 'working') {
    if (activeId.value === payload.terminalId) {
      return;
    }
    const tab = tabs.value.find((item) => item.id === payload.terminalId);
    if (!tab) {
      return;
    }
    tab.hasActivity = true;
    tab.isBlinking = true;
  }
});
let stopOpenTab: (() => void) | null = null;
let stopReadyRequest: (() => void) | null = null;
let stopTerminalTabContextMenu: (() => void) | null = null;
let stopWindowClose: (() => void) | null = null;
let autoTabTimer: number | null = null;
let isClosingWindow = false;

const resolveCurrentWindow = () => {
  try {
    return getCurrentWebviewWindow();
  } catch {
    return null;
  }
};

const emitTabOpened = (payload: TerminalTabOpenedPayload) => {
  void emit(TERMINAL_TAB_OPENED_EVENT, payload);
};

const emitWindowReady = () => {
  const currentWindow = resolveCurrentWindow();
  if (!currentWindow) {
    return;
  }
  void emit(TERMINAL_WINDOW_READY_EVENT, { windowLabel: currentWindow.label });
};

const buildRecentClosedSnapshot = () => {
  const memberMap = new Map(terminalFriendCandidates.value.map((member) => [member.id, member]));
  const now = Date.now();
  const entries: RecentClosedTab[] = [];
  for (let index = 0; index < tabs.value.length; index += 1) {
    const tab = tabs.value[index];
    if (!tab.memberId) {
      continue;
    }
    const member = memberMap.get(tab.memberId);
    if (!member || !hasTerminalConfig(member.terminalType, member.terminalCommand)) {
      continue;
    }
    entries.push({ memberId: tab.memberId, closedAt: now - index });
  }
  return entries;
};

watch(activeId, (next) => {
  if (next) {
    clearActivity(next);
  }
});

watch(tabSearchQuery, (next) => {
  if (next.trim()) {
    tabSearchOpen.value = true;
  }
});

watch(
  members,
  (nextMembers) => {
    const memberMap = new Map(
      nextMembers.map((member) => [member.id, resolveMemberDisplayName(member)])
    );
    for (const tab of tabs.value) {
      if (!tab.memberId) {
        continue;
      }
      const memberName = memberMap.get(tab.memberId)?.trim();
      if (memberName && tab.title !== memberName) {
        tab.title = memberName;
      }
    }
  },
  { deep: true, immediate: true }
);

onMounted(async () => {
  window.addEventListener('pointerdown', handleTabSearchOutside);
  stopTerminalTabContextMenu = registerTerminalTabContextMenu();
  void loadBackendMembers();
  void loadRecentClosedTabs();
  const currentWindow = resolveCurrentWindow();
  if (currentWindow) {
    try {
      stopWindowClose = await currentWindow.onCloseRequested(async (event) => {
        if (isClosingWindow) {
          return;
        }
        isClosingWindow = true;
        let prevented = false;
        if (event && 'preventDefault' in event && typeof event.preventDefault === 'function') {
          event.preventDefault();
          prevented = true;
        }
        await updateRecentClosedTabs(buildRecentClosedSnapshot());
        if (prevented) {
          await currentWindow.close();
        }
      });
    } catch (error) {
      console.error('Failed to register terminal window close handler.', error);
    }
  }
  const init = async () => {
    const currentWindow = resolveCurrentWindow();
    try {
      stopOpenTab = currentWindow
        ? await currentWindow.listen<TerminalOpenTabPayload>(TERMINAL_OPEN_TAB_EVENT, (event) => {
            const { terminalId, title, memberId, terminalType, keepAlive } = event.payload;
            const resolvedTitle = resolveTabTitle(memberId, title);
            trackSession(terminalId);
            openTab(terminalId, {
              title: resolvedTitle,
              memberId,
              terminalType,
              keepAlive,
              activate: resolveBulkActivation(memberId)
            });
            void logDiagnosticsEvent('terminal-open-tab', {
              terminalId,
              memberId,
              terminalType,
              title: resolvedTitle,
              windowLabel: currentWindow?.label
            });
            emitTabOpened({
              windowLabel: currentWindow?.label ?? '',
              terminalId,
              memberId,
              terminalType
            });
            void logDiagnosticsEvent('terminal-tab-opened', {
              terminalId,
              memberId,
              terminalType,
              windowLabel: currentWindow?.label
            });
            if (autoTabTimer !== null) {
              window.clearTimeout(autoTabTimer);
              autoTabTimer = null;
            }
          })
        : await listen<TerminalOpenTabPayload>(TERMINAL_OPEN_TAB_EVENT, (event) => {
            const { terminalId, title, memberId, terminalType, keepAlive } = event.payload;
            const resolvedTitle = resolveTabTitle(memberId, title);
            trackSession(terminalId);
            openTab(terminalId, {
              title: resolvedTitle,
              memberId,
              terminalType,
              keepAlive,
              activate: resolveBulkActivation(memberId)
            });
            void logDiagnosticsEvent('terminal-open-tab', {
              terminalId,
              memberId,
              terminalType,
              title: resolvedTitle,
              windowLabel: currentWindow?.label
            });
            const resolvedWindow = resolveCurrentWindow();
            emitTabOpened({
              windowLabel: resolvedWindow?.label ?? '',
              terminalId,
              memberId,
              terminalType
            });
            void logDiagnosticsEvent('terminal-tab-opened', {
              terminalId,
              memberId,
              terminalType,
              windowLabel: resolvedWindow?.label
            });
            if (autoTabTimer !== null) {
              window.clearTimeout(autoTabTimer);
              autoTabTimer = null;
            }
          });
    } catch (error) {
      console.error('Failed to listen for terminal tabs.', error);
    }

    try {
      stopReadyRequest = currentWindow
        ? await currentWindow.listen(TERMINAL_WINDOW_READY_REQUEST_EVENT, () => {
            emitWindowReady();
          })
        : await listen(TERMINAL_WINDOW_READY_REQUEST_EVENT, () => {
            emitWindowReady();
          });
    } catch (error) {
      console.error('Failed to listen for terminal ready requests.', error);
    }

    emitWindowReady();

    for (const tab of tabs.value) {
      trackSession(tab.id);
    }
    syncPaneAssignments();

    if (tabs.value.length === 0 && isAutoTabEnabled()) {
      // 600ms 延迟自动创建首个标签，避免窗口刚打开时频繁创建。
      autoTabTimer = window.setTimeout(() => {
        if (tabs.value.length === 0 && isAutoTabEnabled()) {
          void handleNewTab();
        }
      }, 600);
    }
  };

  await init();
});

onBeforeUnmount(() => {
  window.removeEventListener('pointerdown', handleTabSearchOutside);
  stopTerminalTabContextMenu?.();
  stopTerminalTabContextMenu = null;
  stopActivity();
  stopStatus();
  cleanupDragListeners();
  cleanupDragState();
  stopAutoScroll();
  stopOpenTab?.();
  stopOpenTab = null;
  stopReadyRequest?.();
  stopReadyRequest = null;
  stopWindowClose?.();
  stopWindowClose = null;
  clearBulkOpenState();
  if (autoTabTimer !== null) {
    window.clearTimeout(autoTabTimer);
    autoTabTimer = null;
  }
});
</script>

<style scoped>
.terminal-tab-move {
  transition: transform 220ms cubic-bezier(0.22, 0.61, 0.36, 1);
}

.terminal-tab--placeholder {
  opacity: 0;
  pointer-events: none;
}

.terminal-tab-ghost {
  position: fixed;
  z-index: 20;
  pointer-events: none;
  box-shadow: 0 10px 22px rgb(0 0 0 / 0.25);
  transform: scale(1.02);
  transition: none;
  will-change: transform;
}

@keyframes terminal-tab-blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.25;
  }
}

.terminal-tab__activity--blink {
  animation: terminal-tab-blink 0.9s ease-in-out infinite;
}

.terminal-tab__close-icon {
  font-size: 13px;
  line-height: 1;
  font-variation-settings: 'FILL' 0, 'wght' 500, 'GRAD' 0, 'opsz' 16;
  display: block;
}
</style>

