<template>
  <div class="flex h-full w-full flex-col">
    <header class="flex shrink-0 flex-wrap items-end justify-between gap-4 border-b border-line px-8 pb-4 pt-8">
      <div>
        <h1 class="text-xl font-semibold text-text">{{ t('skillStore.title') }}</h1>
        <p class="mt-1 text-sm text-muted">{{ t('skillStore.subtitle') }}</p>
      </div>
      <button
        v-if="activeTab === 'installed'"
        type="button"
        class="inline-flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-on-primary transition-colors hover:bg-primary-hover disabled:cursor-not-allowed disabled:opacity-60"
        :disabled="importingFolder"
        @click="handleImportFolder"
      >
        <span class="material-symbols-outlined text-lg">create_new_folder</span>
        {{ t('skills.library.importTitle') }}
      </button>
    </header>

    <nav class="flex shrink-0 gap-4 border-b border-line px-8" role="tablist">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        type="button"
        role="tab"
        :aria-selected="activeTab === tab.id"
        :class="[
          '-mb-px flex items-center gap-2 border-b-2 py-2.5 text-sm transition-colors',
          activeTab === tab.id ? 'border-primary text-text' : 'border-transparent text-muted hover:text-text'
        ]"
        @click="setSkillStoreTab(tab.id)"
      >
        {{ t(tab.labelKey) }}
        <span v-if="tab.id === 'installed'" class="font-mono text-2xs text-faint tabular">{{ skillCount }}</span>
        <span v-else class="rounded-sm border border-line px-1 font-mono text-2xs text-faint">{{ t('common.notBuilt') }}</span>
      </button>
    </nav>

    <div class="custom-scrollbar flex-1 overflow-y-auto px-8 py-6">
      <section v-if="activeTab === 'store'" class="max-w-[60ch]">
        <p class="text-sm text-text">{{ t('skillStore.catalogNotBuilt') }}</p>
        <p class="mt-2 text-sm text-muted">{{ t('skillStore.catalogHint') }}</p>
        <button
          type="button"
          class="mt-4 inline-flex items-center gap-1.5 rounded-md border border-line px-3 py-1.5 text-sm text-text transition-colors hover:bg-hover"
          @click="setSkillStoreTab('installed')"
        >
          {{ t('skillStore.goToMySkills') }}
          <span class="material-symbols-outlined text-base">arrow_forward</span>
        </button>
      </section>

      <section v-else class="max-w-4xl">
        <div v-if="!skillCount" class="max-w-[60ch] py-6">
          <p class="text-sm text-text">{{ t('skillStore.emptyTitle') }}</p>
          <p class="mt-1 text-sm text-muted">{{ t('skillStore.emptyHint') }}</p>
        </div>

        <ul v-else class="divide-y divide-line">
          <li v-for="folder in localSkillFolders" :key="folder.id" class="group flex items-center gap-3 py-3">
            <span class="material-symbols-outlined text-lg text-faint">folder</span>
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-medium text-text">{{ folder.name }}</p>
              <p class="truncate font-mono text-2xs text-faint" :title="folder.displayPath">{{ folder.displayPath }}</p>
            </div>
            <span class="font-mono text-2xs text-faint">{{ t('skills.detail.source.local') }}</span>
            <button
              v-if="canOpenSkillPath"
              type="button"
              class="flex h-7 w-7 items-center justify-center rounded-md text-faint transition-colors hover:bg-hover hover:text-text"
              :aria-label="t('common.openFolder')"
              :title="t('common.openFolder')"
              @click="handleOpenSkillPath(folder.path)"
            >
              <span class="material-symbols-outlined text-base">folder_open</span>
            </button>
            <button
              type="button"
              class="flex h-7 w-7 items-center justify-center rounded-md text-faint transition-colors hover:bg-danger/10 hover:text-danger disabled:opacity-50"
              :aria-label="t('common.remove')"
              :title="t('common.remove')"
              :disabled="removingFolderId === folder.id"
              @click="handleRemoveFolder(folder)"
            >
              <span class="material-symbols-outlined text-base">delete</span>
            </button>
          </li>
          <li v-for="skill in installedSkills" :key="skill.id" class="flex items-center gap-3 py-3">
            <span class="material-symbols-outlined text-lg text-faint">{{ skill.icon }}</span>
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-medium text-text">{{ t(skill.nameKey) }}</p>
              <p class="truncate text-xs text-faint">{{ t(skill.descKey) }}</p>
            </div>
            <span class="font-mono text-2xs text-faint">{{ skill.ver }}</span>
            <button
              type="button"
              class="flex h-7 w-7 items-center justify-center rounded-md text-faint transition-colors hover:bg-danger/10 hover:text-danger"
              :aria-label="t('common.remove')"
              @click="handleRemove(skill.id)"
            >
              <span class="material-symbols-outlined text-base">delete</span>
            </button>
          </li>
        </ul>
      </section>
    </div>
  </div>
</template>
<script setup lang="ts">
// Skills page: "My skills" lists imported skill folders (working); the catalog is honestly marked as not built yet.
import { computed, ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { createLibrarySkills } from '@/features/skills/skillLibrary';
import { formatSkillPath } from '@/features/skills/skillPath';
import { importSkillFolder, openSkillFolder, removeSkillFolder } from '@/features/skills/skillsBridge';
import { useGlobalStore } from '@/features/global/globalStore';
import { useNavigationStore, type SkillStoreTab } from '@/stores/navigationStore';
import { ask } from '@tauri-apps/plugin-dialog';
import { isTauri } from '@tauri-apps/api/core';

const tabs: Array<{ id: SkillStoreTab; labelKey: string }> = [
  { id: 'installed', labelKey: 'skillStore.tabs.installed' },
  { id: 'store', labelKey: 'skillStore.tabs.catalog' }
];

const navigationStore = useNavigationStore();
const { skillStoreTab } = storeToRefs(navigationStore);
const { setSkillStoreTab } = navigationStore;
const activeTab = skillStoreTab;
const globalStore = useGlobalStore();
const { installedSkillIds, importedSkillFolders } = storeToRefs(globalStore);
const { removeSkill, addImportedSkillFolder, removeImportedSkillFolder } = globalStore;
const librarySkills = createLibrarySkills();
const installedSkills = computed(() => librarySkills.filter((skill) => installedSkillIds.value.includes(skill.id)));
const localSkillFolders = computed(() =>
  importedSkillFolders.value.map((folder) => ({
    ...folder,
    displayPath: formatSkillPath(folder.path)
  }))
);
const skillCount = computed(() => localSkillFolders.value.length + installedSkills.value.length);

const { t } = useI18n();
const canOpenSkillPath = isTauri();

const handleRemove = (id: number) => {
  void removeSkill(id);
};

const handleOpenSkillPath = async (path: string) => {
  if (!canOpenSkillPath || !path) {
    return;
  }
  try {
    await openSkillFolder(path);
  } catch (error) {
    console.error('Failed to open skill path.', error);
  }
};

const importingFolder = ref(false);
const removingFolderId = ref<string | null>(null);

const confirmRemoveFolder = async (name: string) => {
  const message = t('skills.library.removeConfirmMessage', { name });
  const title = t('skills.library.removeConfirmTitle');
  if (isTauri()) {
    return ask(message, {
      title,
      kind: 'warning',
      okLabel: t('skills.library.removeConfirmOk'),
      cancelLabel: t('skills.library.removeConfirmCancel')
    });
  }
  return window.confirm(`${title}\n${message}`);
};

const handleImportFolder = async () => {
  if (importingFolder.value) return;
  importingFolder.value = true;
  try {
    const result = await importSkillFolder();
    if (!result) {
      return;
    }
    await addImportedSkillFolder({
      name: result.folderName,
      path: result.destPath
    });
  } catch (error) {
    console.error('Failed to import skill folder.', error);
  } finally {
    importingFolder.value = false;
  }
};

const handleRemoveFolder = async (folder: { id: string; name: string; path: string }) => {
  if (removingFolderId.value) return;
  const confirmed = await confirmRemoveFolder(folder.name);
  if (!confirmed) return;
  removingFolderId.value = folder.id;
  try {
    await removeSkillFolder(folder.path);
    await removeImportedSkillFolder(folder.id);
  } catch (error) {
    console.error('Failed to remove skill folder.', error);
  } finally {
    removingFolderId.value = null;
  }
};
</script>