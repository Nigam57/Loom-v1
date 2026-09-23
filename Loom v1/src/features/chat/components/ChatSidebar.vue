<template>
  <div ref="containerRef" class="flex w-12 shrink-0 flex-col border-r border-line bg-background lg:w-56">
    <div class="flex h-[52px] items-center justify-center border-b border-line px-2 lg:justify-start lg:px-4">
      <h2 class="flex min-w-0 items-center gap-2 text-sm font-medium text-text">
        <span class="material-symbols-outlined text-lg text-faint">folder</span>
        <span class="hidden truncate lg:inline">{{ workspaceName || t('chat.sidebar.workspaceName') }}</span>
      </h2>
    </div>

    <div class="custom-scrollbar flex-1 space-y-4 overflow-y-auto px-1 py-3 lg:px-2">
      <div>
        <div class="mb-1 hidden items-center px-2 lg:flex">
          <h3 class="text-2xs font-medium text-faint">{{ t('chat.sidebar.channels') }}</h3>
        </div>
        <div class="space-y-1">
          <div
            v-for="item in channelItems"
            :key="item.conversation.id"
            :class="[
              'group flex w-full cursor-pointer items-center justify-center gap-2 rounded-md px-2 py-1.5 transition-colors lg:justify-start',
              item.conversation.id === activeConversationId ? 'bg-hover text-text' : 'text-muted hover:bg-hover hover:text-text'
            ]"
            @click="selectConversation(item.conversation.id)"
          >
            <div class="relative">
              <span
                :class="[
                  'w-4 text-center font-mono text-sm leading-none',
                  item.conversation.id === activeConversationId ? 'text-text' : 'text-faint'
                ]"
              >
                #
              </span>
              <span
                v-if="hasUnread(item.conversation)"
                class="absolute -top-0.5 -right-1.5 w-2 h-2 rounded-full bg-danger lg:hidden"
              ></span>
            </div>
            <div class="hidden lg:flex items-start gap-3 min-w-0 flex-1">
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2 min-w-0">
                  <span class="truncate text-sm">{{ item.title }}</span>
                  <span v-if="item.conversation.pinned" class="material-symbols-outlined text-xs text-text/40">push_pin</span>
                  <span v-if="item.conversation.muted" class="material-symbols-outlined text-xs text-text/40">notifications_off</span>
                </div>
                <div class="truncate text-2xs text-faint">{{ item.preview }}</div>
              </div>
            </div>
            <div class="relative shrink-0 hidden lg:flex items-center gap-2">
              <span
                v-if="hasUnread(item.conversation)"
                class="h-4 min-w-4 rounded-full bg-danger px-1 text-center font-mono text-2xs font-medium leading-4 text-background"
              >
                {{ formatUnreadCount(item.conversation.unreadCount) }}
              </span>
              <button
                type="button"
                @click.stop="toggleMenu(item.conversation.id)"
                :class="[
                  'flex h-6 w-6 items-center justify-center rounded opacity-0 transition-[opacity,background-color] group-hover:opacity-100',
                  openMenuId === item.conversation.id ? 'bg-hover text-text opacity-100' : 'text-faint hover:bg-hover hover:text-text'
                ]"
              >
                <span class="material-symbols-outlined text-base">more_horiz</span>
              </button>
              <div
                v-if="openMenuId === item.conversation.id"
                class="absolute right-0 top-full mt-2 w-56 max-h-64 rounded-lg glass-modal flex flex-col py-1 overflow-y-auto custom-scrollbar z-menu animate-in fade-in zoom-in-95 duration-150"
                @click.stop
              >
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, item.conversation.pinned ? 'unpin' : 'pin')"
                >
                  <span class="material-symbols-outlined text-base text-muted">push_pin</span>
                  {{ item.conversation.pinned ? t('chat.conversation.actions.unpin') : t('chat.conversation.actions.pin') }}
                </button>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  v-if="!isDefaultChannel(item.conversation)"
                  @click="handleAction(item.conversation, 'rename')"
                >
                  <span class="material-symbols-outlined text-base text-muted">edit</span>
                  {{ t('chat.conversation.actions.rename') }}
                </button>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, item.conversation.muted ? 'unmute' : 'mute')"
                >
                  <span class="material-symbols-outlined text-base text-muted">notifications_off</span>
                  {{ item.conversation.muted ? t('chat.conversation.actions.unmute') : t('chat.conversation.actions.mute') }}
                </button>
                <div class="h-px bg-line my-1"></div>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, 'clear')"
                >
                  <span class="material-symbols-outlined text-base text-muted">delete_sweep</span>
                  {{ t('chat.conversation.actions.clear') }}
                </button>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-danger transition-colors hover:bg-danger/10"
                  v-if="!isDefaultChannel(item.conversation)"
                  @click="handleAction(item.conversation, 'delete')"
                >
                  <span class="material-symbols-outlined text-base text-muted">delete</span>
                  {{ t('chat.conversation.actions.deleteChannel') }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div>
        <div class="mb-1 hidden items-center px-2 lg:flex">
          <h3 class="text-2xs font-medium text-faint">{{ t('chat.sidebar.directMessages') }}</h3>
        </div>
        <div class="space-y-1">
          <div
            v-for="item in directMessageItems"
            :key="item.conversation.id"
            :class="[
              'group flex w-full cursor-pointer items-center justify-center gap-2 rounded-md px-2 py-1.5 transition-colors lg:justify-start',
              item.conversation.id === activeConversationId ? 'bg-hover text-text' : 'text-muted hover:bg-hover hover:text-text'
            ]"
            @click="selectConversation(item.conversation.id)"
          >
            <div class="relative">
              <AvatarBadge
                :avatar="item.member.avatar"
                :label="item.member.name"
                class="w-9 h-9 rounded-full "
              />
              <span
                v-if="hasUnread(item.conversation)"
                class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full bg-danger border border-panel lg:hidden"
              ></span>
              <MemberStatusDots
                class="absolute -bottom-0.5 -right-0.5"
                :status="resolveManualStatus(item.member)"
                :terminal-status="item.member.terminalStatus"
                :show-terminal-status="hasTerminalConfig(item.member.terminalType, item.member.terminalCommand)"
              />
            </div>
            <div class="hidden lg:flex items-start gap-3 min-w-0 flex-1">
              <div class="min-w-0 flex-1">
                <div class="flex items-center gap-2 min-w-0">
                  <span class="truncate text-sm">{{ item.title }}</span>
                  <span v-if="item.conversation.pinned" class="material-symbols-outlined text-xs text-text/40">push_pin</span>
                  <span v-if="item.conversation.muted" class="material-symbols-outlined text-xs text-text/40">notifications_off</span>
                </div>
                <div class="truncate text-2xs text-faint">{{ item.preview }}</div>
              </div>
            </div>
            <div class="relative shrink-0 hidden lg:flex items-center gap-2">
              <span
                v-if="hasUnread(item.conversation)"
                class="h-4 min-w-4 rounded-full bg-danger px-1 text-center font-mono text-2xs font-medium leading-4 text-background"
              >
                {{ formatUnreadCount(item.conversation.unreadCount) }}
              </span>
              <button
                type="button"
                @click.stop="toggleMenu(item.conversation.id)"
                :class="[
                  'flex h-6 w-6 items-center justify-center rounded opacity-0 transition-[opacity,background-color] group-hover:opacity-100',
                  openMenuId === item.conversation.id ? 'bg-hover text-text opacity-100' : 'text-faint hover:bg-hover hover:text-text'
                ]"
              >
                <span class="material-symbols-outlined text-base">more_horiz</span>
              </button>
              <div
                v-if="openMenuId === item.conversation.id"
                class="absolute right-0 top-full mt-2 w-56 max-h-64 rounded-lg glass-modal flex flex-col py-1 overflow-y-auto custom-scrollbar z-menu animate-in fade-in zoom-in-95 duration-150"
                @click.stop
              >
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, item.conversation.pinned ? 'unpin' : 'pin')"
                >
                  <span class="material-symbols-outlined text-base text-muted">push_pin</span>
                  {{ item.conversation.pinned ? t('chat.conversation.actions.unpin') : t('chat.conversation.actions.pin') }}
                </button>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, item.conversation.muted ? 'unmute' : 'mute')"
                >
                  <span class="material-symbols-outlined text-base text-muted">notifications_off</span>
                  {{ item.conversation.muted ? t('chat.conversation.actions.unmute') : t('chat.conversation.actions.mute') }}
                </button>
                <div class="h-px bg-line my-1"></div>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
                  @click="handleAction(item.conversation, 'clear')"
                >
                  <span class="material-symbols-outlined text-base text-muted">delete_sweep</span>
                  {{ t('chat.conversation.actions.clear') }}
                </button>
                <button
                  type="button"
                  class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-danger transition-colors hover:bg-danger/10"
                  @click="handleAction(item.conversation, 'delete')"
                >
                  <span class="material-symbols-outlined text-base text-muted">delete</span>
                  {{ t('chat.conversation.actions.deleteDirect') }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 会话侧栏组件：展示频道/私聊列表与会话操作。
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Conversation, ConversationAction, Member } from '../types';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';
import MemberStatusDots from './MemberStatusDots.vue';
import { buildGroupConversationTitle } from '../utils';
import { CURRENT_USER_ID } from '../data';
import { hasTerminalConfig } from '@/shared/utils/terminal';
import { resolveMemberDisplayName } from '@/shared/utils/memberDisplay';

const props = defineProps<{
  conversations: Conversation[];
  members: Member[];
  currentUserId?: string;
  activeConversationId: string;
  workspaceName?: string;
  defaultChannelId: string | null;
}>();
const emit = defineEmits<{
  (e: 'select-conversation', conversationId: string): void;
  (e: 'conversation-action', payload: { conversationId: string; action: ConversationAction }): void;
}>();

const { t } = useI18n();
const DEFAULT_MEMBER_NAME = 'Member';
const MAX_UNREAD_DISPLAY = 99;

const openMenuId = ref<string | null>(null);
const containerRef = ref<HTMLElement | null>(null);

const displayMembers = computed(() =>
  props.members.map((member) => ({ ...member, name: resolveMemberDisplayName(member) }))
);
const memberById = computed(() => new Map(displayMembers.value.map((member) => [member.id, member])));
const resolveManualStatus = (member: Member) => member.manualStatus ?? member.status;

// Previews are plain text: drop markdown markers so agent replies read cleanly in the list.
const normalizePreview = (text: string) =>
  text
    .replace(/```[\s\S]*?(```|$)/g, ' ')
    .replace(/\*\*|__|`/g, '')
    .replace(/(^|\s)[#>]+\s/g, '$1')
    .replace(/\[([^\]]+)\]\([^)]*\)/g, '$1')
    .replace(/\s+/g, ' ')
    .trim();
const hasUnread = (conversation: Conversation) => (conversation.unreadCount ?? 0) > 0;
const formatUnreadCount = (value?: number | null) => {
  const count = Number.isFinite(value) ? Math.max(0, Math.floor(value ?? 0)) : 0;
  if (count > MAX_UNREAD_DISPLAY) {
    return `${MAX_UNREAD_DISPLAY}+`;
  }
  return `${count}`;
};

const getConversationTitle = (conversation: Conversation) => {
  if (conversation.type === 'dm') {
    const targetId = conversation.targetId ?? '';
    return memberById.value.get(targetId)?.name ?? DEFAULT_MEMBER_NAME;
  }
  const workspaceLabel = props.workspaceName?.trim();
  if ((conversation.isDefault || conversation.id === props.defaultChannelId) && workspaceLabel) {
    return workspaceLabel;
  }
  if (conversation.customName) {
    return conversation.customName;
  }
  if (conversation.nameKey) {
    return t(conversation.nameKey);
  }
  const groupTitle = buildGroupConversationTitle(
    conversation.memberIds,
    displayMembers.value,
    props.currentUserId ?? CURRENT_USER_ID,
    25
  );
  return groupTitle || conversation.id;
};

const isDefaultChannel = (conversation: Conversation) =>
  conversation.type === 'channel' && conversation.id === props.defaultChannelId;

const getLastMessagePreview = (conversation: Conversation) =>
  conversation.lastMessagePreview ? normalizePreview(conversation.lastMessagePreview) : '';


const channelItems = computed(() =>
  props.conversations.filter((conversation) => conversation.type === 'channel').map((conversation) => ({
    conversation,
    title: getConversationTitle(conversation),
    preview: getLastMessagePreview(conversation)
  }))
);

const directMessageItems = computed(() =>
  props.conversations
    .filter((conversation) => conversation.type === 'dm')
    .map((conversation) => {
      const targetId = conversation.targetId ?? '';
      const member = memberById.value.get(targetId);
      if (!member) return null;
      return {
        conversation,
        member,
        title: getConversationTitle(conversation),
        preview: getLastMessagePreview(conversation)
      };
    })
    .filter((item): item is { conversation: Conversation; member: Member; title: string; preview: string } => Boolean(item))
);

const toggleMenu = (conversationId: string) => {
  openMenuId.value = openMenuId.value === conversationId ? null : conversationId;
};

const selectConversation = (conversationId: string) => {
  openMenuId.value = null;
  emit('select-conversation', conversationId);
};

const handleAction = (conversation: Conversation, action: ConversationAction) => {
  openMenuId.value = null;
  emit('conversation-action', { conversationId: conversation.id, action });
};

const handleClickOutside = (event: MouseEvent) => {
  if (containerRef.value && !containerRef.value.contains(event.target as Node)) {
    openMenuId.value = null;
  }
};

onMounted(() => {
  document.addEventListener('mousedown', handleClickOutside);
});

onBeforeUnmount(() => {
  document.removeEventListener('mousedown', handleClickOutside);
});
</script>

