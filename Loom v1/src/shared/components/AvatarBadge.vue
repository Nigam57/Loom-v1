<template>
  <div
    class="avatar-badge"
    :class="isCss ? 'avatar-badge--css' : 'avatar-badge--image'"
    :style="avatarVars"
    role="img"
    :aria-label="label"
  >
    <img v-if="!isCss" :src="resolvedAvatar" :alt="label" class="avatar-badge__image" />
    <span v-else class="avatar-badge__initials" aria-hidden="true">{{ initials }}</span>
  </div>
</template>

<script setup lang="ts">
// 头像展示组件：支持 CSS 预设（纯色方块 + 首字母）与本地资源，并处理异步加载竞态。
import { computed, ref, watch } from 'vue';
import { DEFAULT_AVATAR } from '@/shared/constants/avatars';
import { ensureAvatar, getAvatarVars, isCssAvatar, isLocalAvatar, resolveAvatarUrl } from '@/shared/utils/avatar';

const props = defineProps<{ avatar?: string; label?: string }>();

const resolvedAvatar = ref(DEFAULT_AVATAR);
const isCss = computed(() => isCssAvatar(resolvedAvatar.value));
const avatarVars = computed(() => (isCss.value ? getAvatarVars(resolvedAvatar.value) : undefined));
const label = computed(() => props.label ?? '');

// "Ada" -> "A", "Ada Lovelace" -> "AL", "room-test-assistant-claude-1" -> "RC".
const initials = computed(() => {
  const words = label.value.split(/[^\p{L}\p{N}]+/u).filter((word) => word && !/^\d+$/.test(word));
  if (!words.length) return '';
  const first = words[0][0];
  const last = words.length > 1 ? words[words.length - 1][0] : '';
  return `${first}${last}`.toUpperCase();
});

// 异步加载序号，确保仅应用最新一次解析结果。
let loadSequence = 0;

const updateAvatar = async () => {
  const current = ++loadSequence;
  const candidate = ensureAvatar(props.avatar);
  if (isCssAvatar(candidate)) {
    resolvedAvatar.value = candidate;
    return;
  }
  if (isLocalAvatar(candidate)) {
    resolvedAvatar.value = DEFAULT_AVATAR;
    const url = await resolveAvatarUrl(candidate);
    if (current === loadSequence) {
      resolvedAvatar.value = url || DEFAULT_AVATAR;
    }
    return;
  }
  resolvedAvatar.value = candidate;
};

watch(
  () => props.avatar,
  () => {
    void updateAvatar();
  },
  { immediate: true }
);
</script>

<style scoped>
.avatar-badge {
  position: relative;
  display: grid;
  place-items: center;
  overflow: hidden;
  background: oklch(var(--color-surface-2));
  container-type: inline-size;
}

.avatar-badge__image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.avatar-badge--css {
  background: var(--avatar-bg, oklch(0.40 0.02 255));
}

.avatar-badge__initials {
  font-family: theme('fontFamily.mono');
  font-weight: 500;
  font-size: max(9px, 38cqw);
  line-height: 1;
  letter-spacing: -0.02em;
  color: var(--avatar-ink, oklch(0.97 0.01 255));
  user-select: none;
}
</style>
