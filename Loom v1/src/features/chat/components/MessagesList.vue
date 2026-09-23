<!-- 消息列表：按日分组、连续消息合并、Markdown 渲染与贴底滚动。边界：仅负责展示与本地滚动状态，不负责数据获取或持久化。 -->
<template>
  <div ref="listRef" class="custom-scrollbar flex flex-1 flex-col overflow-y-auto px-5 pb-4 pt-3">
    <div v-if="hasMore" class="flex justify-start py-2 pl-11">
      <button
        type="button"
        class="rounded px-2 py-1 font-mono text-2xs text-faint transition-colors hover:bg-hover hover:text-text disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="isLoadingMore"
        @click="emit('load-more')"
      >
        {{ isLoadingMore ? t('chat.messages.loadingHistory') : t('chat.messages.loadHistory') }}
      </button>
    </div>

    <div v-if="!rows.length && !hasMore" class="flex flex-1 flex-col justify-end pb-6 pl-11">
      <p class="text-sm text-muted">{{ t('chat.messages.empty', 'No messages yet.') }}</p>
      <p class="mt-1 max-w-[52ch] text-xs text-faint">
        {{ t('chat.messages.emptyHint', 'Mention an agent with @ to start. Agents in this room can ask each other for help.') }}
      </p>
    </div>

    <template v-for="row in rows" :key="row.id">
      <div v-if="row.type === 'separator'" class="my-3 flex items-center gap-3 pl-11">
        <span class="font-mono text-2xs text-faint">{{ row.label }}</span>
        <div class="h-px flex-1 bg-line"></div>
      </div>

      <article
        v-else
        :class="['message-row group', row.continued ? 'mt-0.5' : 'mt-3']"
        :data-me="isMe(row.message) || undefined"
      >
        <div class="w-8 shrink-0">
          <AvatarBadge
            v-if="!row.continued"
            :avatar="row.message.avatar"
            :label="resolveMessageAuthor(row.message)"
            :class="['h-8 w-8 rounded', canOpenAvatar(row.message) ? 'cursor-pointer' : 'cursor-default']"
            @click="handleAvatarClick(row.message)"
          />
          <time
            v-else
            class="block pt-[3px] text-right font-mono text-2xs leading-5 text-faint opacity-0 transition-opacity group-hover:opacity-100"
          >{{ getShortTime(row.message) }}</time>
        </div>

        <div class="min-w-0 flex-1">
          <header v-if="!row.continued" class="flex items-baseline gap-2">
            <span class="text-sm font-semibold text-text">{{ resolveMessageAuthor(row.message) }}</span>
            <span v-if="memberKind(row.message)" class="rounded-sm border border-line px-1 font-mono text-2xs leading-4 text-faint">
              {{ memberKind(row.message) }}
            </span>
            <time class="font-mono text-2xs text-faint">{{ getMessageTime(row.message) }}</time>
            <span v-if="isStreamMessage(row.message)" class="streaming-dot" :title="t('chat.messages.status.sending')"></span>
          </header>

          <!-- eslint-disable-next-line vue/no-v-html -- renderMarkdown disables raw HTML and escapes all text -->
          <div
            v-if="row.message.content.type === 'text'"
            class="md selectable text-sm text-text/90"
            v-html="renderMarkdown(row.message.content.text)"
          ></div>
          <p v-else class="text-sm italic text-muted">{{ resolveMessageText(row.message) }}</p>

          <p v-if="isMe(row.message) && row.message.status === 'failed'" class="mt-0.5 text-2xs text-danger">
            {{ t('chat.messages.status.failed') }}
          </p>

          <div
            v-if="row.message.attachment && row.message.attachment.type === 'image'"
            class="group/image mt-2 max-w-sm cursor-pointer overflow-hidden rounded-lg border border-line bg-surface transition-colors hover:border-line-strong"
          >
            <div
              class="h-44 w-full bg-cover bg-center"
              :style="{ backgroundImage: `url('${row.message.attachment.thumbnailPath ?? row.message.attachment.filePath}')` }"
            ></div>
            <div class="flex items-center justify-between gap-3 border-t border-line px-3 py-2">
              <span class="truncate text-xs text-text">{{ row.message.attachment.fileName }}</span>
              <span class="shrink-0 font-mono text-2xs text-faint">{{ formatFileSize(row.message.attachment.fileSize) }}</span>
            </div>
          </div>

          <button
            v-if="row.message.attachment && row.message.attachment.type === 'roadmap'"
            type="button"
            class="mt-2 inline-flex items-center gap-3 rounded-lg border border-line bg-surface px-3 py-2 text-left transition-colors hover:border-line-strong"
            @click="emit('open-roadmap')"
          >
            <span class="material-symbols-outlined text-lg text-success">map</span>
            <span>
              <span class="block text-sm font-medium text-text">{{ row.message.attachment.title }}</span>
              <span class="block text-2xs text-faint">{{ t('chat.messages.roadmapHint') }}</span>
            </span>
          </button>
        </div>
      </article>
    </template>

    <div v-if="isTyping" class="mt-3 flex items-center gap-3 pl-11 text-xs text-faint">
      <span class="typing-dots" aria-hidden="true"><i></i><i></i><i></i></span>
      <span>{{ t('chat.messages.typing', { name: typingName }) }}</span>
    </div>

    <button
      v-if="showJumpButton"
      type="button"
      class="sticky bottom-3 ml-auto mt-2 inline-flex items-center gap-1 rounded-full border border-line bg-surface-2 px-3 py-1 text-xs text-muted shadow-float transition-colors hover:text-text"
      @click="handleJumpToLatest"
    >
      <span class="material-symbols-outlined text-base">south</span>
      {{ t('chat.messages.jumpToLatest') }}
    </button>
  </div>
</template>

<script setup lang="ts">
// 消息列表组件：负责渲染分组消息、时间分隔与滚动管理。
import { computed, nextTick, onBeforeUnmount, onMounted, ref, toRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Message } from '../types';
import { formatMessageTime, groupMessagesByDay } from '../utils';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';
import { renderMarkdown } from '@/shared/utils/markdown';

const props = defineProps<{
  messages: Message[];
  currentUserId: string;
  currentUserName: string;
  isTyping?: boolean;
  typingName?: string;
  typingAvatar?: string;
  hasMore?: boolean;
  isLoadingMore?: boolean;
  terminalMemberIds?: string[];
  /** Member id -> agent kind label ("Claude Code"), shown next to agent names. */
  memberKinds?: Record<string, string>;
}>();
const emit = defineEmits<{
  (e: 'open-roadmap'): void;
  (e: 'load-more'): void;
  (e: 'open-terminal', memberId: string): void;
}>();

const messages = toRef(props, 'messages');
const listRef = ref<HTMLDivElement | null>(null);
const isPinnedToBottom = ref(true);
const terminalMemberIdSet = computed(() => new Set(props.terminalMemberIds ?? []));
let scrollRaf: number | null = null;

const { t, locale } = useI18n();

const STREAM_MESSAGE_PREFIX = 'terminal-stream:';
// Consecutive messages from one sender within this window share a header.
const GROUP_WINDOW_MS = 5 * 60 * 1000;

type Row =
  | { type: 'separator'; id: string; label: string }
  | { type: 'message'; id: string; message: Message; continued: boolean };

const rows = computed<Row[]>(() => {
  const result: Row[] = [];
  let previous: Message | null = null;
  for (const item of groupMessagesByDay(messages.value, locale.value)) {
    if (item.type === 'separator') {
      result.push({ type: 'separator', id: item.id, label: item.label });
      previous = null;
      continue;
    }
    const message = item.message;
    const continued = Boolean(
      previous &&
        previous.senderId === message.senderId &&
        previous.user === message.user &&
        message.createdAt - previous.createdAt < GROUP_WINDOW_MS
    );
    result.push({ type: 'message', id: item.id, message, continued });
    previous = message;
  }
  return result;
});

const isTyping = computed(() => props.isTyping ?? false);
const typingName = computed(() => props.typingName ?? t('members.roles.aiAssistant'));
const hasMore = computed(() => props.hasMore ?? false);
const isLoadingMore = computed(() => props.isLoadingMore ?? false);
const showJumpButton = computed(() => !isPinnedToBottom.value);
const lastMessageSignature = computed(() => {
  const last = messages.value[messages.value.length - 1];
  if (!last) return '';
  return last.content.type === 'text' ? last.content.text : last.content.key;
});

const isMe = (msg: Message) => {
  if (msg.senderId) {
    return msg.senderId === props.currentUserId;
  }
  return msg.user === props.currentUserName;
};

const resolveMessageAuthor = (message: Message) => {
  if (message.senderId && message.senderId === props.currentUserId) {
    return props.currentUserName || message.user;
  }
  if (message.userKey) {
    return t(message.userKey, message.userArgs ?? {});
  }
  return message.user;
};

const memberKind = (message: Message) => (message.senderId ? props.memberKinds?.[message.senderId] : undefined);

const resolveMessageText = (message: Message) => {
  if (message.content.type === 'system') {
    return t(message.content.key, message.content.args ?? {});
  }
  return message.content.text;
};

const canOpenAvatar = (message: Message) => Boolean(message.senderId && terminalMemberIdSet.value.has(message.senderId));

const handleAvatarClick = (message: Message) => {
  if (!message.senderId || !terminalMemberIdSet.value.has(message.senderId)) {
    return;
  }
  emit('open-terminal', message.senderId);
};

const isStreamMessage = (message: Message) => message.content.type === 'text' && message.id.startsWith(STREAM_MESSAGE_PREFIX);

const getMessageTime = (message: Message) => formatMessageTime(message.createdAt, locale.value);

const getShortTime = (message: Message) =>
  new Date(message.createdAt).toLocaleTimeString(locale.value, { hour: '2-digit', minute: '2-digit' });

const formatFileSize = (bytes: number) => {
  if (!Number.isFinite(bytes) || bytes <= 0) {
    return '0 B';
  }
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex += 1;
  }
  const precision = size < 10 && unitIndex > 0 ? 1 : 0;
  return `${size.toFixed(precision)} ${units[unitIndex]}`;
};

const updatePinnedState = () => {
  if (!listRef.value) return;
  const threshold = 120;
  const distanceFromBottom = listRef.value.scrollHeight - listRef.value.scrollTop - listRef.value.clientHeight;
  isPinnedToBottom.value = distanceFromBottom < threshold;
};

const scrollToBottom = () => {
  if (!listRef.value) return;
  listRef.value.scrollTop = Math.max(0, listRef.value.scrollHeight - listRef.value.clientHeight);
};

const scheduleScrollToBottom = () => {
  if (!listRef.value || !isPinnedToBottom.value || scrollRaf !== null) return;
  // Streaming updates arrive often; merge scrolls into one frame.
  scrollRaf = window.requestAnimationFrame(() => {
    scrollRaf = null;
    scrollToBottom();
  });
};

const handleJumpToLatest = () => {
  scrollToBottom();
  isPinnedToBottom.value = true;
};

defineExpose({
  jumpToLatest: handleJumpToLatest
});

watch(
  [() => messages.value.length, () => lastMessageSignature.value, isTyping],
  async () => {
    await nextTick();
    scheduleScrollToBottom();
  }
);

onMounted(() => {
  updatePinnedState();
  listRef.value?.addEventListener('scroll', updatePinnedState, { passive: true });
  scrollToBottom();
});

onBeforeUnmount(() => {
  listRef.value?.removeEventListener('scroll', updatePinnedState);
  if (scrollRaf !== null) {
    window.cancelAnimationFrame(scrollRaf);
    scrollRaf = null;
  }
});
</script>

<style scoped>
.message-row {
  display: flex;
  gap: 12px;
  margin-left: -8px;
  margin-right: -8px;
  padding: 2px 8px;
  border-radius: 5px;
  transition: background-color 120ms var(--ease-out);
}

.message-row:hover {
  background-color: oklch(var(--color-overlay) / 0.025);
}

.message-row[data-me] .md {
  color: oklch(var(--color-text));
}

.streaming-dot {
  width: 6px;
  height: 6px;
  align-self: center;
  border-radius: 999px;
  background: oklch(var(--color-primary));
  animation: pulse-dot 1.2s var(--ease-out) infinite;
}

.typing-dots {
  display: inline-flex;
  gap: 3px;
}

.typing-dots i {
  width: 4px;
  height: 4px;
  border-radius: 999px;
  background: oklch(var(--color-text-faint));
  animation: pulse-dot 1.2s var(--ease-out) infinite;
}

.typing-dots i:nth-child(2) {
  animation-delay: 150ms;
}

.typing-dots i:nth-child(3) {
  animation-delay: 300ms;
}

@keyframes pulse-dot {
  0%,
  100% {
    opacity: 0.35;
    transform: scale(0.85);
  }
  50% {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
