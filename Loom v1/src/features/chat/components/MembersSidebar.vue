<template>
  <aside
    :class="[
      'h-full shrink-0 flex-col border-l border-line bg-background px-2 py-3',
      variant === 'drawer' ? 'flex w-72' : 'hidden w-[248px] md:flex'
    ]"
  >
    <div class="mb-3 flex h-7 items-center justify-between px-2">
      <h2 class="text-sm font-medium text-text">{{ t('members.title') }}</h2>
      <!-- 允许外部替换头部操作按钮，未提供时回退到默认邀请入口。 -->
      <slot name="header-action">
        <button
          type="button"
          class="flex h-7 w-7 items-center justify-center rounded-md text-muted transition-colors hover:bg-hover hover:text-text"
          :title="t('friends.invite')"
          @click="emit('open-invite')"
        >
          <span class="material-symbols-outlined text-lg">group_add</span>
        </button>
      </slot>
    </div>

    <div class="custom-scrollbar flex-1 overflow-y-auto">
      <div v-if="owners.length" class="mb-4">
        <h3 class="mb-1 px-2 text-2xs font-medium text-faint">{{ t('members.sections.owner', { count: owners.length }) }}</h3>
        <div class="space-y-px">
          <MemberRow
            v-for="member in owners"
            :key="member.id"
            :member="member"
            :current-user-id="currentUserId"
            :menu-open="openMenuId === member.id"
            @toggle-menu="toggleMenu"
            @action="handleAction"
          />
        </div>
      </div>

      <div v-if="admins.length" class="mb-4">
        <h3 class="mb-1 px-2 text-2xs font-medium text-faint">{{ t('members.sections.admin', { count: admins.length }) }}</h3>
        <div class="space-y-px">
          <MemberRow
            v-for="member in admins"
            :key="member.id"
            :member="member"
            :current-user-id="currentUserId"
            :menu-open="openMenuId === member.id"
            @toggle-menu="toggleMenu"
            @action="handleAction"
          />
        </div>
      </div>

      <div v-if="assistants.length" class="mb-4">
        <h3 class="mb-1 px-2 text-2xs font-medium text-faint">{{ t('members.sections.assistant', { count: assistants.length }) }}</h3>
        <div class="space-y-px">
          <MemberRow
            v-for="member in assistants"
            :key="member.id"
            :member="member"
            :current-user-id="currentUserId"
            :menu-open="openMenuId === member.id"
            @toggle-menu="toggleMenu"
            @action="handleAction"
          />
        </div>
      </div>

      <div v-if="membersGroup.length" class="mb-4">
        <h3 class="mb-1 px-2 text-2xs font-medium text-faint">{{ t('members.sections.member', { count: membersGroup.length }) }}</h3>
        <div class="space-y-px">
          <MemberRow
            v-for="member in membersGroup"
            :key="member.id"
            :member="member"
            :current-user-id="currentUserId"
            :menu-open="openMenuId === member.id"
            @toggle-menu="toggleMenu"
            @action="handleAction"
          />
        </div>
      </div>
    </div>
  </aside>
</template>

<script setup lang="ts">
// 成员侧栏组件：展示成员列表与操作菜单。
import { computed, onBeforeUnmount, onMounted, ref, toRef } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Member, MemberActionPayload } from '../types';
import MemberRow from './MemberRow.vue';

const props = defineProps<{ members: Member[]; currentUserId?: string; variant?: 'sidebar' | 'drawer' }>();
const emit = defineEmits<{ (e: 'member-action', payload: MemberActionPayload): void; (e: 'open-invite'): void }>();

const owners = computed(() => props.members.filter((member) => member.roleType === 'owner'));
const admins = computed(() => props.members.filter((member) => member.roleType === 'admin'));
const assistants = computed(() => props.members.filter((member) => member.roleType === 'assistant'));
const membersGroup = computed(() => props.members.filter((member) => member.roleType === 'member'));

const currentUserId = toRef(props, 'currentUserId');
const variant = computed(() => props.variant ?? 'sidebar');
const openMenuId = ref<string | null>(null);

const { t } = useI18n();

const toggleMenu = (member: Member) => {
  openMenuId.value = openMenuId.value === member.id ? null : member.id;
};

const handleAction = (payload: MemberActionPayload) => {
  openMenuId.value = null;
  emit('member-action', payload);
};

const handleClickOutside = (event: MouseEvent) => {
  if (!openMenuId.value) return;
  const target = event.target as HTMLElement | null;
  if (target?.closest('[data-member-menu]') || target?.closest('[data-member-menu-toggle]')) return;
  openMenuId.value = null;
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside, true);
});

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside, true);
});
</script>
