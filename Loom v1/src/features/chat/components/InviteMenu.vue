<template>
  <div :class="[positionClass, baseClass]">
    <div class="px-3 py-2 mb-1">
      <h3 class="text-text font-semibold text-sm tracking-tight">{{ t('invite.menu.title') }}</h3>
      <p class="text-text/40 text-2xs font-medium mt-0.5">{{ t('invite.menu.subtitle') }}</p>
    </div>
    <div class="h-[1px] bg-overlay/5 mx-2 mb-1.5"></div>
    <button
      type="button"
      @click="emit('select', 'admin')"
      class="flex items-center gap-3 px-3 py-2 rounded-xl hover:bg-overlay/10 group transition duration-200 text-left w-full"
    >
      <div class="w-8 h-8 rounded-[10px] bg-danger/10 text-danger flex items-center justify-center group-hover:bg-danger group-hover:text-on-primary group-hover:shadow-red-500/20 transition duration-200">
        <span class="material-symbols-outlined text-lg">admin_panel_settings</span>
      </div>
      <div class="flex flex-col">
        <span class="text-text/90 text-sm font-medium group-hover:text-text tracking-tight">{{ t('invite.menu.admin') }}</span>
        <span class="text-text/40 text-2xs group-hover:text-text/60">{{ t('invite.menu.adminDesc') }}</span>
      </div>
    </button>
    <button
      type="button"
      @click="emit('select', 'assistant')"
      class="flex items-center gap-3 px-3 py-2 rounded-xl hover:bg-overlay/10 group transition duration-200 text-left w-full"
    >
      <div class="w-8 h-8 rounded-[10px] bg-warning/10 text-warning flex items-center justify-center group-hover:bg-warning group-hover:text-on-primary group-hover:shadow-yellow-500/20 transition duration-200">
        <span class="material-symbols-outlined text-lg">manage_accounts</span>
      </div>
      <div class="flex flex-col">
        <span class="text-text/90 text-sm font-medium group-hover:text-text tracking-tight">{{ t('invite.menu.assistant') }}</span>
        <span class="text-text/40 text-2xs group-hover:text-text/60">{{ t('invite.menu.assistantDesc') }}</span>
      </div>
    </button>
    <button
      type="button"
      @click="emit('select', 'member')"
      class="flex items-center gap-3 px-3 py-2 rounded-xl hover:bg-overlay/10 group transition duration-200 text-left w-full"
    >
      <div class="w-8 h-8 rounded-[10px] bg-primary/10 text-primary flex items-center justify-center group-hover:bg-primary group-hover:text-on-primary group-hover:shadow-primary/20 transition duration-200">
        <span class="material-symbols-outlined text-lg">person</span>
      </div>
      <div class="flex flex-col">
        <span class="text-text/90 text-sm font-medium group-hover:text-text tracking-tight">{{ t('invite.menu.member') }}</span>
        <span class="text-text/40 text-2xs group-hover:text-text/60">{{ t('invite.menu.memberDesc') }}</span>
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
// 邀请菜单组件：提供不同邀请入口的快捷触发。
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{ positionClass?: string }>();
const emit = defineEmits<{ (e: 'select', type: 'admin' | 'assistant' | 'member'): void }>();

const { t } = useI18n();

const baseClass =
  'z-50 w-[260px] bg-panel/80 border border-border/10 rounded-2xl shadow-float flex flex-col p-1.5 animate-in fade-in zoom-in-95 duration-200 origin-top-right ring-1 ring-border/5';
const positionClass = computed(() => props.positionClass ?? 'fixed top-[74px] right-6');
</script>
