<template>
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/70 animate-in fade-in duration-200">
    <div class="relative w-full max-w-[480px] max-h-[64vh] bg-panel/90 border border-border/10 rounded-2xl shadow-float flex flex-col overflow-hidden ring-1 ring-border/5">
      <div class="px-5 py-4 border-b border-border/5 flex items-center gap-4">
        <div class="w-9 h-9 rounded-[10px] bg-danger/10 text-danger flex items-center justify-center shadow-red-500/10 shrink-0 border border-danger/10">
          <span class="material-symbols-outlined text-lg">admin_panel_settings</span>
        </div>
        <div class="flex-1">
          <h3 class="text-text font-semibold text-lg tracking-tight leading-tight">{{ t('invite.admin.title') }}</h3>
          <p class="text-text/40 text-2xs font-medium mt-0.5">{{ t('invite.admin.subtitle') }}</p>
        </div>
        <button type="button" @click="emit('close')" class="text-text/20 hover:text-text transition-colors rounded-lg p-1 hover:bg-overlay/5">
          <span class="material-symbols-outlined text-xl">close</span>
        </button>
      </div>

      <div class="p-5 space-y-4 max-h-[64vh] overflow-y-auto custom-scrollbar">
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <label class="text-2xs font-medium text-faint">{{ t('invite.admin.uniqueLink') }}</label>
            <span class="text-2xs text-primary cursor-pointer hover:underline">{{ t('invite.admin.regenerate') }}</span>
          </div>
          <div class="flex gap-2 group focus-within:ring-2 ring-primary/20 rounded-xl transition">
            <div class="flex-1 bg-surface border border-border/10 rounded-xl px-3 py-2 flex items-center gap-2 transition-colors group-focus-within:border-primary/50 group-focus-within:bg-panel/70">
              <span class="material-symbols-outlined text-text/20 text-lg">link</span>
              <input class="bg-transparent border-none p-0 text-text/70 text-xs font-mono w-full focus:ring-0 truncate select-all" readonly value="https://sky.chat/invite/adm_9x82m..." />
            </div>
            <button type="button" class="px-3 bg-overlay/5 hover:bg-overlay/10 border border-border/10 rounded-xl text-text/70 hover:text-text transition font-medium text-xs flex items-center gap-2 active:scale-95">
              <span class="material-symbols-outlined text-lg">content_copy</span>
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <label class="text-2xs font-medium text-faint">{{ t('invite.admin.userIdentifier') }}</label>
          <div class="relative group">
            <div class="absolute left-3 top-2 text-text/30 group-focus-within:text-primary transition-colors">
              <span class="material-symbols-outlined text-lg">alternate_email</span>
            </div>
            <input
              v-model="identifier"
              class="w-full bg-surface border border-border/10 rounded-xl py-2 pl-10 pr-4 text-text placeholder-text/20 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition text-sm"
              :placeholder="t('invite.admin.userPlaceholder')"
              type="text"
            />
          </div>
        </div>

        <div class="space-y-2">
          <label class="text-2xs font-medium text-faint">{{ t('invite.admin.permissions') }}</label>
          <div class="bg-surface/50 border border-border/10 rounded-xl overflow-hidden">
            <label v-for="perm in permissions" :key="perm.title" class="flex items-start gap-3 p-3 hover:bg-overlay/5 cursor-pointer transition-colors group border-b border-border/5 last:border-0 relative">
              <div class="relative flex items-center mt-0.5">
                <input type="checkbox" :checked="perm.checked" class="peer sr-only" />
                <div class="w-4 h-4 border-2 border-border/20 rounded-[4px] peer-checked:bg-primary peer-checked:border-primary transition flex items-center justify-center">
                  <span class="material-symbols-outlined text-xs text-on-primary font-semibold opacity-0 peer-checked:opacity-100">check</span>
                </div>
              </div>
              <div class="flex flex-col">
                <span class="text-xs font-medium text-text/90 group-hover:text-text transition-colors">{{ perm.title }}</span>
                <span class="text-2xs text-text/30 mt-0.5 font-normal">{{ perm.desc }}</span>
              </div>
            </label>
          </div>
        </div>
      </div>

      <div class="p-5 pt-2">
        <button
          type="button"
          @click="handleInvite"
          class="w-full py-2.5 rounded-xl bg-primary hover:bg-primary-hover text-on-primary font-semibold text-sm flex items-center justify-center gap-2 transition active:scale-[0.98] group"
        >
          <span>{{ t('invite.admin.send') }}</span>
          <span class="material-symbols-outlined text-lg group-hover:translate-x-0.5 transition-transform">send</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 邀请管理员弹窗：输入标识并发送管理员邀请。
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const emit = defineEmits<{ (e: 'close'): void; (e: 'invite', payload: { identifier: string }): void }>();

const { t } = useI18n();
const identifier = ref('');

const handleInvite = () => {
  emit('invite', { identifier: identifier.value.trim() });
};

const permissions = computed(() => [
  {
    title: t('invite.admin.permissionsList.fullAccess.title'),
    desc: t('invite.admin.permissionsList.fullAccess.desc'),
    checked: true
  },
  {
    title: t('invite.admin.permissionsList.billing.title'),
    desc: t('invite.admin.permissionsList.billing.desc'),
    checked: false
  },
  {
    title: t('invite.admin.permissionsList.memberManagement.title'),
    desc: t('invite.admin.permissionsList.memberManagement.desc'),
    checked: true
  }
]);
</script>
