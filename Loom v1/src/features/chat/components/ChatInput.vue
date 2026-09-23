<template>
  <div class="px-5 pb-4 pt-2">
    <div v-if="mentionBarItems.length" class="mb-3 flex flex-wrap items-center gap-2">
      <span class="inline-flex items-center gap-1 rounded-full border border-border/10 bg-overlay/5 px-2.5 py-1 text-2xs font-semibold text-text/60">
        <span class="material-symbols-outlined text-xs">alternate_email</span>
        {{ t('chat.input.mentions') }}
      </span>
      <button
        v-for="item in mentionBarItems"
        :key="item.key"
        type="button"
        class="group inline-flex items-center gap-1.5 rounded-full border border-primary/30 bg-primary/15 px-2.5 py-1 text-2xs font-semibold text-primary hover:bg-primary/25 transition-colors"
        :aria-label="t('chat.input.removeMention', { name: item.label })"
        @click="removeMention(item)"
      >
        <span>@{{ item.label }}</span>
        <span class="material-symbols-outlined text-sm text-primary/70 group-hover:text-primary">close</span>
      </button>
    </div>
    <form
      ref="formRef"
      class="group relative flex items-end gap-2 rounded-lg border border-line bg-surface py-1.5 pl-3 pr-1.5 transition-colors focus-within:border-line-strong"
      @submit.prevent="emitSend"
    >
      <div ref="scrollRef" class="max-h-40 min-h-[32px] flex-1 overflow-y-auto py-1.5" @scroll="scheduleMentionAnchorUpdate">
        <textarea
          ref="inputRef"
          :value="modelValue"
          class="min-h-[20px] w-full resize-none border-none bg-transparent p-0 text-sm text-text outline-none placeholder:text-faint focus:outline-none focus:ring-0 focus-visible:outline-none"
          :placeholder="placeholder"
          :maxlength="maxLength"
          spellcheck="false"
          rows="1"
          @input="handleInput"
          @compositionstart="isComposing = true"
          @compositionend="isComposing = false"
          @click="updateCursor"
          @keydown="handleKeydown"
          @keyup="updateCursor"
        ></textarea>
      </div>
      <div
        v-if="showMentionSuggestions"
        ref="mentionDropdownRef"
        class="custom-scrollbar absolute z-dropdown max-h-64 w-64 overflow-y-auto rounded-lg glass-modal py-1 animate-in fade-in zoom-in-95 duration-150"
        :style="mentionDropdownStyle"
      >
        <button
          v-for="(member, index) in mentionSuggestions"
          :key="member.id"
          type="button"
          :class="[
            'flex w-full items-center gap-2.5 px-2.5 py-1.5 text-left transition-colors',
            index === activeMentionIndex
              ? 'bg-hover'
              : 'hover:bg-hover'
          ]"
          @click="applyMention(member)"
        >
          <AvatarBadge
            :avatar="member.avatar"
            :label="member.name"
            class="h-6 w-6 rounded"
          />
          <div class="min-w-0">
            <div class="truncate text-xs font-medium text-text">@{{ member.name }}</div>
            <div class="truncate font-mono text-2xs text-faint">{{ getMemberRole(member) }}</div>
          </div>
        </button>
      </div>
      <div class="flex shrink-0 items-center gap-1">
        <button
          ref="emojiButtonRef"
          type="button"
          :class="[
            'flex h-8 w-8 items-center justify-center rounded-md transition-colors',
            isEmojiPanelOpen ? 'bg-hover text-text' : 'text-faint hover:bg-hover hover:text-text'
          ]"
          @click="toggleEmojiPanel"
        >
          <span class="material-symbols-outlined text-lg">sentiment_satisfied</span>
        </button>
       <button
          v-if="isGenerating"
          type="button"
          class="flex h-8 items-center gap-1.5 rounded-md border border-danger/40 bg-danger/10 px-3 text-sm font-medium text-danger transition-colors hover:bg-danger/20"
          @click="emit('stop')"
        >
          <span class="material-symbols-outlined text-lg">stop_circle</span>
          {{ t('chat.input.stop') }}
        </button>
        <button
          v-else
          type="submit"
          :disabled="!modelValue.trim()"
          :class="[
            'flex h-8 items-center gap-1.5 rounded-md px-3 text-sm font-medium transition-colors',
            !modelValue.trim() ? 'cursor-not-allowed bg-overlay/[0.06] text-faint' : 'bg-primary text-on-primary hover:bg-primary-hover'
          ]"
        >
          {{ t('chat.input.send') }}
        </button>
      </div>
      <div
        v-if="isEmojiPanelOpen"
        ref="emojiPanelRef"
        class="absolute bottom-[calc(100%+8px)] right-0 z-dropdown w-[320px] overflow-hidden rounded-lg glass-modal animate-in fade-in zoom-in-95 duration-150"
      >
        <div class="p-3 border-b border-border/10">
          <input
            ref="emojiSearchRef"
            v-model="emojiSearchInput"
            type="text"
            class="w-full bg-overlay/5 border border-border/10 rounded-xl px-3 py-2 text-xs text-text placeholder-text/30 focus:outline-none focus:border-primary/40 focus:ring-1 focus:ring-primary/20"
            :placeholder="t('chat.input.emoji.search')"
            @keydown.stop="handleEmojiSearchKeydown"
          />
        </div>
        <div class="flex items-center gap-1 px-3 py-2 border-b border-border/5 overflow-x-auto no-scrollbar">
          <button
            v-for="group in emojiGroups"
            :key="group.id"
            type="button"
            :title="group.label"
            :class="[
              'w-8 h-8 rounded-lg text-lg flex items-center justify-center transition-colors',
              activeEmojiGroup === group.id
                ? 'bg-overlay/[0.12] text-text'
                : 'text-text/60 hover:text-text hover:bg-overlay/5'
            ]"
            @click="activeEmojiGroup = group.id"
          >
            {{ group.icon }}
          </button>
        </div>
        <div ref="emojiGridRef" class="max-h-56 overflow-y-auto custom-scrollbar px-3 py-3" @scroll="handleEmojiScroll">
          <div v-if="filteredEmojis.length" class="grid grid-cols-8 gap-1" :style="emojiGridStyle">
            <button
              v-for="entry in visibleEmojis"
              :key="`${entry.emoji}-${entry.order}`"
              type="button"
              class="w-8 h-8 rounded-lg text-lg flex items-center justify-center transition-colors hover:bg-overlay/10"
              @click="insertEmoji(entry.emoji)"
            >
              {{ entry.emoji }}
            </button>
          </div>
          <div v-else class="text-xs text-text/40 text-center py-6">
            {{ emojiEmptyMessage }}
          </div>
        </div>
      </div>
    </form>
    <div class="mt-2 flex items-center justify-between px-2 text-2xs text-text/30 font-medium ">
      <span>{{ t('chat.input.hint') }}</span>
      <span>{{ modelValue.length }}/{{ maxLength }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
// 聊天输入组件：处理输入、提及选择与发送触发。
import { computed, nextTick, onBeforeUnmount, onMounted, ref, toRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Member, MessageMentionsPayload } from '../types';
import { EMOJI_DATA, EMOJI_GROUPS, type EmojiEntry } from '../emoji-data';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';

type EmojiSearchEntry = EmojiEntry & { searchText: string };

type EmojiIndexResult = {
  grouped: Record<number, EmojiSearchEntry[]>;
  flat: EmojiSearchEntry[];
  byEmoji: Map<string, EmojiSearchEntry>;
};

const buildEmojiIndex = (entries: EmojiEntry[]): EmojiIndexResult => {
  const grouped: Record<number, EmojiSearchEntry[]> = {};
  const flat: EmojiSearchEntry[] = [];
  const byEmoji = new Map<string, EmojiSearchEntry>();
  for (const entry of entries) {
    const searchText = `${entry.emoji} ${entry.label} ${entry.tags.join(' ')}`.toLowerCase();
    const indexed = { ...entry, searchText };
    const bucket = grouped[entry.group] ?? [];
    bucket.push(indexed);
    grouped[entry.group] = bucket;
    flat.push(indexed);
    if (!byEmoji.has(entry.emoji)) {
      byEmoji.set(entry.emoji, indexed);
    }
  }
  return { grouped, flat, byEmoji };
};

const EMOJI_INDEX = buildEmojiIndex(EMOJI_DATA);
const EMOJI_GROUP_INDEX = EMOJI_INDEX.grouped;
const EMOJI_FLAT_LIST = EMOJI_INDEX.flat;
const EMOJI_BY_EMOJI = EMOJI_INDEX.byEmoji;

const RECENT_GROUP_ID = -1;
const RECENT_STORAGE_KEY = 'loom.emoji.recents.v1';
const RECENT_MAX_COUNT = 24;
const DEFAULT_EMOJI_GROUP_ID = 0;
const EMOJI_GROUP_LABEL_KEYS: Record<number, string> = {
  0: 'chat.input.emoji.groups.smileys',
  1: 'chat.input.emoji.groups.people',
  2: 'chat.input.emoji.groups.component',
  3: 'chat.input.emoji.groups.animals',
  4: 'chat.input.emoji.groups.food',
  5: 'chat.input.emoji.groups.travel',
  6: 'chat.input.emoji.groups.activities',
  7: 'chat.input.emoji.groups.objects',
  8: 'chat.input.emoji.groups.symbols'
};

const props = defineProps<{
  modelValue: string;
  maxLength?: number;
  isGenerating?: boolean;
  placeholder?: string;
  members?: Member[];
}>();
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'send', payload: MessageMentionsPayload): void;
  (e: 'stop'): void;
}>();

const { t } = useI18n();
const formRef = ref<HTMLFormElement | null>(null);
const scrollRef = ref<HTMLDivElement | null>(null);
const inputRef = ref<HTMLTextAreaElement | null>(null);
const emojiPanelRef = ref<HTMLDivElement | null>(null);
const emojiButtonRef = ref<HTMLButtonElement | null>(null);
const emojiSearchRef = ref<HTMLInputElement | null>(null);
const emojiGridRef = ref<HTMLDivElement | null>(null);
const mentionDropdownRef = ref<HTMLDivElement | null>(null);
const maxLength = computed(() => props.maxLength ?? 1200);
const isGenerating = computed(() => props.isGenerating ?? false);
const placeholder = computed(() => props.placeholder ?? t('chat.input.placeholder', { channel: '' }));
const isComposing = ref(false);
const members = computed(() => props.members ?? []);
const cursorIndex = ref(0);
const activeMentionIndex = ref(0);
const mentionAnchor = ref({ left: 0, top: 0 });
const mentionTokens = ref<Array<{ id: string; name: string }>>([]);
const mentionAllPattern = /(^|\s)@all(\s|$)/i;
const isEmojiPanelOpen = ref(false);
const emojiSearchInput = ref('');
const emojiSearch = ref('');
const emojiGroups = computed(() => {
  const localizedGroups = EMOJI_GROUPS.map((group) => {
    const labelKey = EMOJI_GROUP_LABEL_KEYS[group.id];
    return {
      ...group,
      label: labelKey ? t(labelKey) : group.label
    };
  });
  return [{ id: RECENT_GROUP_ID, label: t('chat.input.emoji.recent'), icon: '🕘' }, ...localizedGroups];
});
const activeEmojiGroup = ref(DEFAULT_EMOJI_GROUP_ID);
const emojiEntries = EMOJI_GROUP_INDEX;
const emojiScrollTop = ref(0);
const emojiViewportHeight = ref(224);
const EMOJI_COLUMNS = 8;
const EMOJI_ROW_GAP = 4;
const EMOJI_ROW_HEIGHT = 32 + EMOJI_ROW_GAP;
const EMOJI_OVERSCAN_ROWS = 2;
const EMOJI_SEARCH_DEBOUNCE_MS = 120;
let emojiSearchTimer: number | null = null;
const recentEmojiIds = ref<string[]>([]);
const recentEmojiEntries = ref<EmojiSearchEntry[]>([]);

const syncRecentEmojiEntries = () => {
  recentEmojiEntries.value = recentEmojiIds.value
    .map((emoji) => EMOJI_BY_EMOJI.get(emoji))
    .filter((entry): entry is EmojiSearchEntry => Boolean(entry));
};

const loadRecentEmojiIds = () => {
  try {
    const raw = window.localStorage.getItem(RECENT_STORAGE_KEY);
    if (!raw) return;
    const parsed = JSON.parse(raw);
    if (Array.isArray(parsed)) {
      recentEmojiIds.value = parsed.filter((item) => typeof item === 'string').slice(0, RECENT_MAX_COUNT);
      syncRecentEmojiEntries();
    }
  } catch {
    // 忽略本地缓存损坏，避免影响输入流程。
  }
};

const persistRecentEmojiIds = () => {
  try {
    window.localStorage.setItem(RECENT_STORAGE_KEY, JSON.stringify(recentEmojiIds.value));
  } catch {
    // 忽略本地缓存写入失败，避免阻塞输入。
  }
};

const dropTrailingSpace = (value: string) => value.replace(/[ \t]$/, '');

const removeMentionAtCursor = () => {
  const input = inputRef.value;
  if (!input) return false;
  const start = input.selectionStart ?? 0;
  const end = input.selectionEnd ?? 0;
  if (start !== end) return false;
  const value = input.value;
  const before = value.slice(0, start);
  if (!before) return false;
  const after = value.slice(end);
  const mentions = mentionTokens.value
    .map((token) => `@${token.name}`)
    .sort((a, b) => b.length - a.length);
  for (const mention of mentions) {
    const withSpace = `${mention} `;
    if (before.endsWith(withSpace)) {
      const nextBefore = dropTrailingSpace(before.slice(0, -withSpace.length));
      const nextValue = `${nextBefore}${after}`;
      emit('update:modelValue', nextValue);
      nextTick(() => {
        if (!inputRef.value) return;
        const pos = nextBefore.length;
        inputRef.value.focus();
        inputRef.value.setSelectionRange(pos, pos);
        cursorIndex.value = pos;
        scheduleMentionAnchorUpdate();
      });
      return true;
    }
    if (before.endsWith(mention)) {
      const nextBefore = dropTrailingSpace(before.slice(0, -mention.length));
      const nextValue = `${nextBefore}${after}`;
      emit('update:modelValue', nextValue);
      nextTick(() => {
        if (!inputRef.value) return;
        const pos = nextBefore.length;
        inputRef.value.focus();
        inputRef.value.setSelectionRange(pos, pos);
        cursorIndex.value = pos;
        scheduleMentionAnchorUpdate();
      });
      return true;
    }
  }
  return false;
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && isEmojiPanelOpen.value) {
    event.preventDefault();
    closeEmojiPanel();
    return;
  }
  if (showMentionSuggestions.value) {
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      if (!mentionSuggestions.value.length) return;
      const direction = event.key === 'ArrowDown' ? 1 : -1;
      activeMentionIndex.value =
        (activeMentionIndex.value + direction + mentionSuggestions.value.length) % mentionSuggestions.value.length;
      return;
    }
    if (isEnterKey(event) || event.key === 'Tab') {
      event.preventDefault();
      const selected = mentionSuggestions.value[activeMentionIndex.value];
      if (selected) {
        applyMention(selected);
      }
      return;
    }
  }
  if (event.key === 'Backspace' && !isComposing.value) {
    if (removeMentionAtCursor()) {
      event.preventDefault();
      return;
    }
  }
  if (isEnterKey(event) && !event.shiftKey && !isComposing.value) {
    event.preventDefault();
    if (isGenerating.value) return;
    emitSend();
  }
};

const modelValue = toRef(props, 'modelValue');

const resizeInput = () => {
  if (!inputRef.value) return;
  inputRef.value.style.height = 'auto';
  inputRef.value.style.height = `${Math.min(inputRef.value.scrollHeight, 160)}px`;
};

const focusInput = () => {
  if (!inputRef.value) return;
  inputRef.value.focus();
  const length = inputRef.value.value.length;
  inputRef.value.setSelectionRange(length, length);
};


const updateCursor = () => {
  if (!inputRef.value) return;
  cursorIndex.value = inputRef.value.selectionStart ?? inputRef.value.value.length;
  scheduleMentionAnchorUpdate();
};

const buildSendPayload = (): MessageMentionsPayload => ({
  mentionIds: mentionTokens.value.map((token) => token.id),
  mentionAll: mentionAllPattern.test(modelValue.value)
});

const emitSend = () => {
  emit('send', buildSendPayload());
};

const syncMentionTokens = (value: string) => {
  if (!mentionTokens.value.length) {
    return;
  }
  const lower = value.toLowerCase();
  mentionTokens.value = mentionTokens.value.filter((token) => lower.includes(`@${token.name.toLowerCase()}`));
};

const handleInput = (event: Event) => {
  const target = event.target as HTMLTextAreaElement;
  emit('update:modelValue', target.value);
  cursorIndex.value = target.selectionStart ?? target.value.length;
  scheduleMentionAnchorUpdate();
  syncMentionTokens(target.value);
};

const mentionState = computed(() => {
  if (isComposing.value) return null;
  const text = modelValue.value;
  const cursor = cursorIndex.value;
  const prefix = text.slice(0, cursor);
  const match = prefix.match(/@([^\s]*)$/);
  if (!match) return null;
  const query = match[1] ?? '';
  const startIndex = prefix.length - query.length - 1;
  return { query, startIndex };
});

const mentionSuggestions = computed(() => {
  if (!mentionState.value) return [];
  const query = mentionState.value.query.trim().toLowerCase();
  const options = members.value.filter((member) => member.name.toLowerCase().startsWith(query));
  return options.slice(0, 6);
});

const showMentionSuggestions = computed(() => mentionState.value !== null && mentionSuggestions.value.length > 0);

const isEnterKey = (event: KeyboardEvent) => event.key === 'Enter' || event.code === 'NumpadEnter';

type MentionBarItem = {
  key: string;
  id?: string;
  label: string;
  kind: 'member' | 'all';
};

const mentionBarItems = computed<MentionBarItem[]>(() => {
  const items = mentionTokens.value
    .filter((token) => token.name.trim())
    .map((token) => ({
      key: token.id,
      id: token.id,
      label: token.name.trim(),
      kind: 'member' as const
    }));
  if (mentionAllPattern.test(modelValue.value)) {
    items.unshift({ key: '__mention_all__', label: 'all', kind: 'all' });
  }
  return items;
});

const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

const removeMentionText = (value: string, label: string) => {
  if (!value) {
    return '';
  }
  const escaped = escapeRegExp(label);
  const pattern = new RegExp(`(^|\\s)${escaped}(?=\\s|$|[\\.,!?;:，。！？；：])`, 'gi');
  let next = value.replace(pattern, (match, leading) => {
    if (!leading) {
      return '';
    }
    if (leading.includes('\n') || leading.includes('\r')) {
      return leading;
    }
    return '';
  });
  next = next.replace(/[ \t]{2,}/g, ' ');
  next = next.replace(/[ \t]+\n/g, '\n');
  next = next.replace(/\n[ \t]+/g, '\n');
  return next;
};

const removeMention = (item: MentionBarItem) => {
  const label = item.kind === 'all' ? '@all' : `@${item.label}`;
  if (item.kind === 'member' && item.id) {
    mentionTokens.value = mentionTokens.value.filter((token) => token.id !== item.id);
  }
  emit('update:modelValue', removeMentionText(modelValue.value, label));
  nextTick(() => {
    focusInput();
    scheduleMentionAnchorUpdate();
  });
};

const isEmojiMatch = (entry: EmojiSearchEntry, query: string) => {
  if (!query) return true;
  return entry.searchText.includes(query);
};

const filteredEmojis = computed(() => {
  const query = emojiSearch.value.trim().toLowerCase();
  if (query) {
    return EMOJI_FLAT_LIST.filter((entry) => isEmojiMatch(entry, query));
  }
  const groupId = activeEmojiGroup.value;
  if (groupId === RECENT_GROUP_ID) {
    return recentEmojiEntries.value;
  }
  return emojiEntries[groupId] ?? [];
});

const emojiVisibleRange = computed(() => {
  const totalRows = Math.ceil(filteredEmojis.value.length / EMOJI_COLUMNS);
  if (totalRows <= 0) {
    return { totalRows: 0, startRow: 0, endRow: -1 };
  }
  // 仅渲染可视区域的行，避免一次性创建上千 DOM 节点导致卡顿。
  const startRow = Math.max(0, Math.floor(emojiScrollTop.value / EMOJI_ROW_HEIGHT) - EMOJI_OVERSCAN_ROWS);
  const endRow = Math.min(
    totalRows - 1,
    Math.floor((emojiScrollTop.value + emojiViewportHeight.value) / EMOJI_ROW_HEIGHT) + EMOJI_OVERSCAN_ROWS
  );
  return { totalRows, startRow, endRow };
});

const visibleEmojis = computed(() => {
  const { startRow, endRow } = emojiVisibleRange.value;
  if (endRow < startRow) return [];
  return filteredEmojis.value.slice(startRow * EMOJI_COLUMNS, (endRow + 1) * EMOJI_COLUMNS);
});

const emojiEmptyMessage = computed(() => {
  if (emojiSearch.value.trim()) {
    return t('chat.input.emoji.empty');
  }
  if (activeEmojiGroup.value === RECENT_GROUP_ID) {
    return t('chat.input.emoji.emptyRecent');
  }
  return t('chat.input.emoji.empty');
});

const emojiGridStyle = computed(() => {
  const { totalRows, startRow, endRow } = emojiVisibleRange.value;
  const totalHeight = totalRows > 0 ? totalRows * EMOJI_ROW_HEIGHT - EMOJI_ROW_GAP : 0;
  const paddingTop = startRow * EMOJI_ROW_HEIGHT;
  const paddingBottom = Math.max(0, totalHeight - paddingTop - (endRow - startRow + 1) * EMOJI_ROW_HEIGHT);
  return {
    paddingTop: `${paddingTop}px`,
    paddingBottom: `${paddingBottom}px`,
    minHeight: `${Math.max(totalHeight, 0)}px`
  };
});

const mentionDropdownStyle = computed(() => ({
  left: `${mentionAnchor.value.left}px`,
  top: `${mentionAnchor.value.top}px`
}));

const getMemberRole = (member: Member) => {
  if (member.roleType === 'assistant') {
    return t('members.roles.member');
  }
  if (member.roleKey) return t(member.roleKey);
  if (member.role) return member.role;
  return t(`members.roles.${member.roleType}`);
};

const applyMention = (member: Member) => {
  if (!mentionState.value) return;
  if (!mentionTokens.value.some((token) => token.id === member.id)) {
    mentionTokens.value.push({ id: member.id, name: member.name });
  }
  const mention = `@${member.name}`;
  const before = modelValue.value.slice(0, mentionState.value.startIndex);
  const joiner = before && !/\s$/.test(before) ? ' ' : '';
  const after = modelValue.value.slice(cursorIndex.value);
  const trimmedAfter = after.replace(/^[\s\n]+/, '');
  const nextValue = `${before}${joiner}${mention} ${trimmedAfter}`;
  emit('update:modelValue', nextValue);
  nextTick(() => {
    if (!inputRef.value) return;
    const nextCursor = before.length + joiner.length + mention.length + 1;
    inputRef.value.focus();
    inputRef.value.setSelectionRange(nextCursor, nextCursor);
    cursorIndex.value = nextCursor;
    updateMentionAnchor();
  });
};

const openEmojiPanel = () => {
  isEmojiPanelOpen.value = true;
  if (activeEmojiGroup.value === DEFAULT_EMOJI_GROUP_ID && recentEmojiEntries.value.length > 0) {
    activeEmojiGroup.value = RECENT_GROUP_ID;
  }
  nextTick(() => {
    updateEmojiViewport();
    emojiSearchRef.value?.focus();
  });
};

const closeEmojiPanel = () => {
  isEmojiPanelOpen.value = false;
  emojiSearch.value = '';
  emojiSearchInput.value = '';
  if (emojiSearchTimer !== null) {
    window.clearTimeout(emojiSearchTimer);
    emojiSearchTimer = null;
  }
};

const toggleEmojiPanel = () => {
  if (isEmojiPanelOpen.value) {
    closeEmojiPanel();
    return;
  }
  openEmojiPanel();
};

const insertEmoji = (emoji: string) => {
  const input = inputRef.value;
  if (!input) return;
  const start = input.selectionStart ?? modelValue.value.length;
  const end = input.selectionEnd ?? start;
  const value = modelValue.value;
  const nextValue = `${value.slice(0, start)}${emoji}${value.slice(end)}`;
  emit('update:modelValue', nextValue);
  recordRecentEmoji(emoji);
  nextTick(() => {
    if (!inputRef.value) return;
    const nextCursor = start + emoji.length;
    inputRef.value.focus();
    inputRef.value.setSelectionRange(nextCursor, nextCursor);
    cursorIndex.value = nextCursor;
    scheduleMentionAnchorUpdate();
  });
};

const recordRecentEmoji = (emoji: string) => {
  if (!emoji) return;
  const nextIds = recentEmojiIds.value.filter((item) => item !== emoji);
  nextIds.unshift(emoji);
  recentEmojiIds.value = nextIds.slice(0, RECENT_MAX_COUNT);
  syncRecentEmojiEntries();
  persistRecentEmojiIds();
};

const handleEmojiScroll = () => {
  if (!emojiGridRef.value) return;
  emojiScrollTop.value = emojiGridRef.value.scrollTop;
};

const updateEmojiViewport = () => {
  if (!emojiGridRef.value) return;
  emojiViewportHeight.value = emojiGridRef.value.clientHeight || 224;
};

const handleEmojiSearchKeydown = (event: KeyboardEvent) => {
  if (isEnterKey(event)) {
    event.preventDefault();
  }
  if (event.key === 'Escape') {
    event.preventDefault();
    closeEmojiPanel();
  }
};

const handleGlobalPointerDown = (event: MouseEvent) => {
  if (!isEmojiPanelOpen.value) return;
  const target = event.target as Node;
  if (emojiPanelRef.value?.contains(target) || emojiButtonRef.value?.contains(target)) {
    return;
  }
  closeEmojiPanel();
};

const registerMention = (member: Member) => {
  if (!mentionTokens.value.some((token) => token.id === member.id)) {
    mentionTokens.value.push({ id: member.id, name: member.name });
  }
};

const scheduleMentionAnchorUpdate = () => {
  if (!showMentionSuggestions.value) return;
  nextTick(() => {
    updateMentionAnchor();
  });
};

const updateMentionAnchor = () => {
  if (!inputRef.value || !formRef.value || !mentionState.value) return;
  const anchorIndex = Math.min(Math.max(mentionState.value.startIndex, 0), inputRef.value.value.length);
  const caret = getCaretCoordinates(inputRef.value, anchorIndex);
  if (!caret) return;
  const inputRect = inputRef.value.getBoundingClientRect();
  const formRect = formRef.value.getBoundingClientRect();
  const scrollOffsetY = scrollRef.value?.scrollTop ?? 0;
  const scrollOffsetX = inputRef.value.scrollLeft ?? 0;
  const absLeft = inputRect.left + caret.left - scrollOffsetX;
  const absTop = inputRect.top + caret.top - scrollOffsetY;
  const dropdownWidth = mentionDropdownRef.value?.offsetWidth ?? 256;
  const dropdownHeight = mentionDropdownRef.value?.offsetHeight ?? 256;
  const viewportPadding = 8;
  const clampedLeft = Math.max(viewportPadding, Math.min(absLeft, window.innerWidth - dropdownWidth - viewportPadding));
  const clampedTop = Math.max(viewportPadding, Math.min(absTop - dropdownHeight, window.innerHeight - dropdownHeight - viewportPadding));
  mentionAnchor.value = {
    left: clampedLeft - formRect.left,
    top: clampedTop - formRect.top
  };
};

const getCaretCoordinates = (input: HTMLTextAreaElement, position: number) => {
  const style = window.getComputedStyle(input);
  const mirror = document.createElement('div');
  mirror.style.position = 'absolute';
  mirror.style.visibility = 'hidden';
  mirror.style.whiteSpace = 'pre-wrap';
  mirror.style.wordWrap = 'break-word';
  mirror.style.boxSizing = 'border-box';
  mirror.style.padding = style.padding;
  mirror.style.border = style.border;
  mirror.style.fontFamily = style.fontFamily;
  mirror.style.fontSize = style.fontSize;
  mirror.style.lineHeight = style.lineHeight;
  mirror.style.letterSpacing = style.letterSpacing;
  mirror.style.width = `${input.clientWidth}px`;
  mirror.textContent = input.value.slice(0, position);
  const marker = document.createElement('span');
  marker.textContent = '\u200b';
  mirror.appendChild(marker);
  document.body.appendChild(mirror);
  const coords = { left: marker.offsetLeft, top: marker.offsetTop, height: marker.offsetHeight };
  document.body.removeChild(mirror);
  return coords;
};

watch(modelValue, async () => {
  if (!modelValue.value.trim()) {
    mentionTokens.value = [];
  } else {
    syncMentionTokens(modelValue.value);
  }
  await nextTick();
  resizeInput();
  scheduleMentionAnchorUpdate();
});

watch(mentionSuggestions, () => {
  activeMentionIndex.value = 0;
  scheduleMentionAnchorUpdate();
});

watch(showMentionSuggestions, (visible) => {
  if (visible) {
    scheduleMentionAnchorUpdate();
  }
});

watch(emojiSearchInput, (value) => {
  if (emojiSearchTimer !== null) {
    window.clearTimeout(emojiSearchTimer);
  }
  emojiSearchTimer = window.setTimeout(() => {
    emojiSearch.value = value;
    emojiSearchTimer = null;
  }, EMOJI_SEARCH_DEBOUNCE_MS);
});

watch([activeEmojiGroup, emojiSearch], () => {
  if (!emojiGridRef.value) return;
  emojiGridRef.value.scrollTop = 0;
  emojiScrollTop.value = 0;
});

watch(isEmojiPanelOpen, async (visible) => {
  if (!visible) return;
  await nextTick();
  updateEmojiViewport();
});

onMounted(() => {
  resizeInput();
  loadRecentEmojiIds();
  document.addEventListener('mousedown', handleGlobalPointerDown);
  window.addEventListener('resize', updateEmojiViewport);
});

onBeforeUnmount(() => {
  if (emojiSearchTimer !== null) {
    window.clearTimeout(emojiSearchTimer);
  }
  document.removeEventListener('mousedown', handleGlobalPointerDown);
  window.removeEventListener('resize', updateEmojiViewport);
});

defineExpose({ focus: focusInput, registerMention });
</script>
