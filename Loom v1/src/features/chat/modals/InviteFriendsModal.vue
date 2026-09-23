<template>
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/70 animate-in fade-in duration-200">
    <div class="w-full max-w-[520px] max-h-[70vh] bg-panel/90 border border-border/10 rounded-2xl shadow-float flex flex-col overflow-hidden ring-1 ring-border/5">
      <div class="px-5 py-4 border-b border-border/5 flex items-center justify-between">
        <div class="flex flex-col">
          <h3 class="text-text font-semibold text-lg tracking-tight leading-tight">{{ title }}</h3>
          <p class="text-text/40 text-xs font-medium mt-0.5">{{ selectedCount }}</p>
        </div>
        <button type="button" @click="emit('close')" class="text-text/20 hover:text-text transition-colors rounded-lg p-1 hover:bg-overlay/5">
          <span class="material-symbols-outlined text-xl">close</span>
        </button>
      </div>

      <div class="px-5 py-3 border-b border-border/5">
        <div class="relative">
          <span class="material-symbols-outlined text-lg text-text/20 absolute left-3 top-2.5">search</span>
          <input
            v-model="query"
            type="text"
            :placeholder="t('friends.inviteModal.search')"
            class="w-full bg-surface border border-border/10 rounded-xl py-2 pl-10 pr-3 text-text placeholder-text/20 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition text-sm"
          />
        </div>
      </div>

      <div class="flex-1 min-h-0 overflow-y-auto px-4 py-3 space-y-1.5 custom-scrollbar">
        <div
          v-for="friend in filteredFriends"
          :key="friend.id"
          class="flex items-center gap-3 px-3 py-1.5 rounded-xl hover:bg-overlay/5 transition-colors"
          @click="toggleFriend(friend.id)"
        >
          <label class="relative flex items-center">
            <input type="checkbox" class="sr-only" :checked="selectedIds.has(friend.id)" />
            <span
              :class="[
                'w-4 h-4 rounded-[4px] border-2 flex items-center justify-center transition',
                selectedIds.has(friend.id) ? 'bg-primary border-primary text-on-primary' : 'border-border/20 text-transparent'
              ]"
            >
              <span class="material-symbols-outlined text-xs">check</span>
            </span>
          </label>
          <AvatarBadge :avatar="friend.avatar" :label="friend.name" class="w-8 h-8 rounded-full " />
          <div class="min-w-0 flex-1">
            <div class="text-xs font-semibold text-text truncate">{{ friend.name }}</div>
            <div class="text-2xs text-text/40">{{ getFriendRole(friend) }}</div>
          </div>
          <div :class="['w-2.5 h-2.5 rounded-full border-2 border-panel', statusDotClass(resolveFriendStatus(friend))]"></div>
        </div>
        <div v-if="filteredFriends.length === 0" class="text-center text-text/30 text-sm py-8">
          {{ t('friends.empty') }}
        </div>
      </div>

      <div class="px-5 py-3 border-t border-border/5 flex items-center justify-between">
        <span class="text-xs text-text/40">{{ selectedCount }}</span>
        <button
          type="button"
          :disabled="selectedIds.size === 0"
          @click="confirmInvite"
          :class="[
            'h-9 px-4 rounded-xl font-semibold text-xs transition',
            selectedIds.size === 0
              ? 'bg-overlay/10 text-text/30 cursor-not-allowed'
              : 'bg-primary text-on-primary hover:bg-primary-hover active:scale-[0.98]'
          ]"
        >
          {{ actionLabel }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 邀请好友弹窗：从联系人中选择并发起邀请。
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';
import type { FriendEntry } from '../types';

const props = defineProps<{ friends: FriendEntry[]; title: string; actionLabel: string }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'invite', ids: string[]): void }>();

const { t } = useI18n();
const query = ref('');
const selectedIds = ref<Set<string>>(new Set());

const filteredFriends = computed(() => {
  const keyword = query.value.trim().toLowerCase();
  if (!keyword) {
    return props.friends;
  }
  return props.friends.filter((friend) => friend.name.toLowerCase().includes(keyword));
});

const selectedCount = computed(() => `${selectedIds.value.size}/${props.friends.length}`);

const statusDotClass = (status: FriendEntry['status']) => {
  if (status === 'online') return 'bg-success';
  if (status === 'working') return 'bg-warning';
  if (status === 'dnd') return 'bg-danger';
  return 'bg-faint';
};

const resolveFriendStatus = (friend: FriendEntry) => friend.manualStatus ?? friend.status;

const getFriendRole = (friend: FriendEntry) => {
  if (friend.roleType === 'assistant') {
    return t('members.roles.member');
  }
  return t(`members.roles.${friend.roleType}`);
};

const toggleFriend = (id: string) => {
  const next = new Set(selectedIds.value);
  if (next.has(id)) {
    next.delete(id);
  } else {
    next.add(id);
  }
  selectedIds.value = next;
};

const confirmInvite = () => {
  if (selectedIds.value.size === 0) return;
  emit('invite', Array.from(selectedIds.value));
};
</script>
