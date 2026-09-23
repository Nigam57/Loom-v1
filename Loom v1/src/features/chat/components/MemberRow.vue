<template>
  <div
    :class="[
      'group relative flex items-center gap-2.5 rounded-md px-2 py-1.5 transition-colors hover:bg-hover',
      menuOpen ? 'z-40' : 'z-0 group-hover:z-30'
    ]"
  >
    <button
      type="button"
      @mousedown.prevent
      @click.stop="handleAvatarClick"
      :aria-disabled="!canOpenTerminal"
      :class="[
        'member-avatar-button relative rounded transition-opacity',
        resolvedStatus === 'offline' ? 'grayscale opacity-60 group-hover:grayscale-0 group-hover:opacity-100' : '',
        canOpenTerminal ? 'cursor-pointer' : 'cursor-default'
      ]"
    >
      <AvatarBadge :avatar="member.avatar" :label="displayName" class="h-7 w-7 rounded" />
      <MemberStatusDots
        class="absolute -bottom-0.5 -right-0.5"
        :status="resolvedStatus"
        :terminal-status="member.terminalStatus"
        :show-terminal-status="canOpenTerminal"
      />
    </button>
    <div class="flex items-center justify-between gap-2 min-w-0 flex-1">
      <div class="flex flex-col min-w-0">
        <div class="flex items-center gap-2">
          <span class="truncate text-sm font-medium leading-5 text-text">{{ displayName }}</span>
          <span v-if="member.roleType === 'owner'" class="rounded-sm border border-line px-1 font-mono text-2xs leading-4 text-faint">{{ t('members.roles.owner') }}</span>
          <span v-if="member.roleType === 'admin'" class="rounded-sm border border-line px-1 font-mono text-2xs leading-4 text-faint">{{ t('members.roles.admin') }}</span>
        </div>
        <span v-if="displayRole" class="truncate font-mono text-2xs leading-4 text-faint">{{ displayRole }}</span>
      </div>
      <div class="relative shrink-0">
        <button
          type="button"
          @click.stop="emit('toggle-menu', member)"
          data-member-menu-toggle
          :class="[
            'flex h-7 w-7 items-center justify-center rounded-md opacity-0 transition-[opacity,background-color] group-hover:opacity-100 focus-visible:opacity-100',
            menuOpen ? 'bg-hover text-text opacity-100' : 'text-faint hover:bg-hover hover:text-text'
          ]"
        >
          <span class="material-symbols-outlined text-lg">more_horiz</span>
        </button>
        <div
          v-if="menuOpen"
          data-member-menu
          class="absolute right-0 top-full mt-2 w-52 rounded-lg glass-modal flex flex-col py-1 overflow-hidden z-menu animate-in fade-in zoom-in-95 duration-150"
          @click.stop
        >
          <button
            v-if="canSendMessage"
            type="button"
            class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
            @click="emit('action', { action: 'send-message', member })"
          >
            <span class="material-symbols-outlined text-base text-muted">chat_bubble</span>
            {{ t('members.actions.sendMessage') }}
          </button>
          <button
            v-if="canMention"
            type="button"
            class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
            @click="emit('action', { action: 'mention', member })"
          >
            <span class="material-symbols-outlined text-base text-muted">alternate_email</span>
            @{{ member.name }}
          </button>
          <button
            v-if="canRename"
            type="button"
            class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
            @click="emit('action', { action: 'rename', member })"
          >
            <span class="material-symbols-outlined text-base text-muted">edit</span>
            {{ t('members.actions.rename') }}
          </button>
          <div class="h-px bg-line my-1"></div>
          <div class="px-4 py-1 text-2xs font-semibold text-text/40">
            {{ t('settings.status') }}
          </div>
          <button
            v-for="option in menuStatusOptions"
            :key="option.id"
            type="button"
            class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-text transition-colors hover:bg-hover"
            @click="emit('action', { action: 'set-status', member, status: option.id })"
          >
            <span :class="['w-2.5 h-2.5 rounded-full', option.dotClass]"></span>
            {{ t(option.labelKey) }}
            <span v-if="resolvedStatus === option.id" class="material-symbols-outlined text-lg ml-auto text-text/60">check</span>
          </button>
          <template v-if="canRemove">
            <div class="h-px bg-line my-1"></div>
            <button
              type="button"
              class="relative flex w-full items-center gap-2.5 px-3 py-1.5 text-left text-xs text-danger transition-colors hover:bg-danger/10"
              @click="emit('action', { action: 'remove', member })"
            >
              <span class="material-symbols-outlined text-base text-muted">person_remove</span>
              {{ t('members.manage.remove') }}
            </button>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 成员行组件：展示单个成员信息与操作入口。
import { computed, toRef } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Member, MemberActionPayload, MemberStatus } from '../types';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';
import MemberStatusDots from './MemberStatusDots.vue';
import { hasTerminalConfig } from '@/shared/utils/terminal';
import { logDiagnosticsEvent } from '@/shared/monitoring/diagnostics/logger';
import { resolveMemberDisplayName } from '@/shared/utils/memberDisplay';
import { BASE_TERMINALS, resolveBaseTerminalLabel } from '@/shared/constants/terminalCatalog';

const props = defineProps<{ member: Member; menuOpen?: boolean; currentUserId?: string }>();
const emit = defineEmits<{
  (e: 'toggle-menu', member: Member): void;
  (e: 'action', payload: MemberActionPayload): void;
}>();
const member = toRef(props, 'member');
const menuOpen = computed(() => props.menuOpen ?? false);
const canRemove = computed(() => (props.currentUserId ? member.value.id !== props.currentUserId : true));
const canSendMessage = computed(() => (props.currentUserId ? member.value.id !== props.currentUserId : true));
const isCurrentUser = computed(() => (props.currentUserId ? member.value.id === props.currentUserId : false));
const canMention = computed(() => !isCurrentUser.value);
const canRename = computed(() => !isCurrentUser.value);
const canOpenTerminal = computed(() => hasTerminalConfig(member.value.terminalType, member.value.terminalCommand));
const displayName = computed(() => resolveMemberDisplayName(member.value));
const resolvedStatus = computed(() => member.value.manualStatus ?? member.value.status);

const { t } = useI18n();

const baseStatusOptions: Array<{ id: MemberStatus; labelKey: string; dotClass: string }> = [
  { id: 'online', labelKey: 'settings.statusOptions.online', dotClass: 'bg-success' },
  { id: 'working', labelKey: 'settings.statusOptions.working', dotClass: 'bg-warning' },
  { id: 'dnd', labelKey: 'settings.statusOptions.dnd', dotClass: 'bg-danger' },
  { id: 'offline', labelKey: 'settings.statusOptions.offline', dotClass: 'bg-faint' }
];
const menuStatusOptions = computed(() =>
  canOpenTerminal.value ? baseStatusOptions.filter((option) => option.id !== 'working') : baseStatusOptions
);

const displayRole = computed(() => {
  if (isCurrentUser.value) {
    const match = baseStatusOptions.find((option) => option.id === member.value.status);
    return match ? t(match.labelKey) : '';
  }
  if (member.value.terminalType) {
    const base = BASE_TERMINALS.value.find((item) => item.terminalType === member.value.terminalType);
    return base ? resolveBaseTerminalLabel(base, t) : member.value.terminalType;
  }
  if (member.value.roleKey) {
    return t(member.value.roleKey);
  }
  return member.value.role;
});

const handleAvatarClick = (event?: MouseEvent) => {
  const target = event?.currentTarget as HTMLElement | null;
  target?.blur();
  if (!canOpenTerminal.value) {
    return;
  }
  void logDiagnosticsEvent('avatar-click', {
    memberId: member.value.id,
    terminalType: member.value.terminalType,
    terminalCommand: member.value.terminalCommand
  });
  emit('action', { action: 'open-terminal', member: member.value });
};
</script>

