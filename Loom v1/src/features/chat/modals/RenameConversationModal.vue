<template>
  <div class="fixed inset-0 z-[150] flex items-center justify-center bg-black/60 animate-in fade-in duration-200">
    <div class="w-full max-w-sm bg-panel-strong border border-border/10 rounded-2xl shadow-float p-6 ring-1 ring-border/5">
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-text font-semibold text-lg">{{ t('chat.conversation.renameTitle') }}</h3>
        <button type="button" @click="emit('close')" class="text-text/40 hover:text-text transition-colors">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <div class="space-y-4">
        <div>
          <label class="text-xs font-medium text-faint mb-1.5 block">{{ t('chat.conversation.renameLabel') }}</label>
          <input
            v-model="draftName"
            class="w-full bg-surface/80 border border-border/10 rounded-xl px-4 py-2.5 text-text focus:border-primary/50 focus:ring-1 focus:ring-primary/50 outline-none transition"
            :placeholder="t('chat.conversation.renamePlaceholder')"
          />
        </div>

        <div class="flex gap-3">
          <button
            type="button"
            @click="emit('close')"
            class="flex-1 py-2.5 border border-border/10 text-text/70 font-semibold rounded-xl hover:bg-overlay/5 transition-colors"
          >
            {{ t('settings.cancel') }}
          </button>
          <button
            type="button"
            @click="emit('save', draftName)"
            class="flex-1 py-2.5 bg-primary text-on-primary font-semibold rounded-xl hover:bg-primary-hover transition-colors"
          >
            {{ t('settings.saveChanges') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 会话重命名弹窗：输入并提交新的会话名称。
import { ref, toRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{ name: string }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'save', name: string): void }>();

const { t } = useI18n();
const name = toRef(props, 'name');
const draftName = ref(name.value);

watch(name, (nextName) => {
  draftName.value = nextName;
});
</script>
