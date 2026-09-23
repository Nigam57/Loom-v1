<template>
  <div class="flex h-full w-full overflow-hidden">
    <aside class="w-14 md:w-[240px] bg-glass-sidebar glass-panel shrink-0 py-4 md:py-8 px-2 md:px-3 flex flex-col">
      <div class="px-4 mb-6 hidden md:block">
        <h2 class="text-text font-semibold text-xl tracking-tight">{{ t('settings.title') }}</h2>
      </div>

      <div class="flex-1 overflow-y-auto custom-scrollbar px-1 md:px-2 space-y-4 md:space-y-6">
        <div>
          <h3 class="px-3 mb-2 text-text/40 text-2xs font-semibold hidden md:block">{{ t('settings.userSettings') }}</h3>
          <div class="space-y-0.5">
            <button type="button" :title="t('settings.myAccount')" @click="scrollToSection('account')" :class="sectionButtonClass('account')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'account' ? 'text-primary' : ''">person</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.myAccount') }}</span>
            </button>
          </div>
        </div>

        <div>
          <h3 class="px-3 mb-2 text-text/40 text-2xs font-semibold hidden md:block">{{ t('settings.appSettings') }}</h3>
          <div class="space-y-0.5">
            <button type="button" :title="t('settings.appearance')" @click="scrollToSection('appearance')" :class="sectionButtonClass('appearance')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'appearance' ? 'text-primary' : ''">palette</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.appearance') }}</span>
            </button>
            <button type="button" :title="t('settings.language')" @click="scrollToSection('language')" :class="sectionButtonClass('language')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'language' ? 'text-primary' : ''">translate</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.language') }}</span>
            </button>
            <button type="button" :title="t('settings.members')" @click="scrollToSection('members')" :class="sectionButtonClass('members')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'members' ? 'text-primary' : ''">groups</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.members') }}</span>
            </button>
            <button type="button" :title="t('settings.notifications')" @click="scrollToSection('notifications')" :class="sectionButtonClass('notifications')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'notifications' ? 'text-primary' : ''">notifications</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.notifications') }}</span>
            </button>
            <button type="button" :title="t('settings.keybinds')" @click="scrollToSection('keybinds')" :class="sectionButtonClass('keybinds')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'keybinds' ? 'text-primary' : ''">keyboard_command_key</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.keybinds') }}</span>
            </button>
            <button type="button" :title="t('settings.data')" @click="scrollToSection('data')" :class="sectionButtonClass('data')">
              <span class="material-symbols-outlined text-xl" :class="activeSection === 'data' ? 'text-primary' : ''">database</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.data') }}</span>
            </button>
          </div>
        </div>

        <div class="pt-4 mt-4">
          <div class="space-y-2">
            <button
              type="button"
              @click="handleCreateTeam"
              :title="t('settings.createTeam')"
              class="w-full text-left flex items-center gap-3 px-3 py-2.5 rounded-lg text-text/70 hover:text-text hover:bg-overlay/5 transition group justify-center md:justify-start"
            >
              <span class="material-symbols-outlined text-xl">group_add</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.createTeam') }}</span>
            </button>
            <button
              type="button"
              @click="handleLeaveWorkspace"
              :title="t('settings.leaveTeam')"
              class="w-full text-left flex items-center gap-3 px-3 py-2.5 rounded-lg text-danger/70 hover:text-danger hover:bg-danger/10 transition group justify-center md:justify-start"
            >
              <span class="material-symbols-outlined text-xl">logout</span>
              <span class="text-base font-medium hidden md:inline">{{ t('settings.leaveTeam') }}</span>
            </button>
          </div>
        </div>
      </div>
    </aside>

    <main ref="contentRef" class="flex-1 flex flex-col bg-transparent relative min-w-0 overflow-y-auto custom-scrollbar p-12">
      <div class="max-w-3xl w-full mx-auto pb-20">
        <header class="mb-10">
          <h1 class="text-3xl font-semibold text-text mb-2">{{ t('settings.preferences') }}</h1>
          <p class="text-text/40 text-sm">{{ t('settings.preferencesSubtitle') }}</p>
        </header>

        <section ref="accountRef" id="account" class="mb-12 scroll-mt-8">
          <h2 class="mb-1 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">person</span>
            {{ t('settings.myAccount') }}
          </h2>
          <p class="text-text/40 text-sm">{{ t('settings.accountSubtitle') }}</p>

          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl p-6 mt-6">
            <div class="flex flex-col md:flex-row gap-6">
              <div class="relative">
                <button
                  ref="avatarButtonRef"
                  type="button"
                  class="group relative w-16 h-16 rounded-2xl border border-border/10 overflow-hidden"
                  @click="toggleAvatarMenu($event)"
                >
                  <AvatarBadge
                    :avatar="accountAvatar"
                    :label="t('common.userAvatarAlt')"
                    class="w-full h-full rounded-2xl"
                  />
                  <div class="absolute inset-0 bg-black/45 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                    <span class="material-symbols-outlined text-text text-lg">edit</span>
                  </div>
                </button>
                <div
                  v-if="avatarMenuOpen"
                  ref="avatarMenuRef"
                  class="fixed w-80 rounded-2xl border border-border/10 bg-panel/95 shadow-float overflow-hidden z-20"
                  :style="avatarMenuStyle"
                >
                  <div class="px-4 pt-3">
                    <div class="text-2xs font-semibold text-text/40">{{ t('settings.avatarTitle') }}</div>
                    <div class="text-xs text-text/50 mt-1">{{ t('settings.avatarSubtitle') }}</div>
                  </div>
                  <div class="grid grid-cols-5 gap-3 px-4 py-4">
                    <button
                      v-for="preset in avatarPresets"
                      :key="preset.id"
                      type="button"
                      class="relative group"
                      @click="selectAvatarPreset(preset.id)"
                    >
                      <AvatarBadge
                        :avatar="`css:${preset.id}`"
                        :label="t(preset.labelKey)"
                        class="w-11 h-11 rounded-xl border border-border/10 group-hover:border-primary/50 transition-colors"
                      />
                      <span class="sr-only">{{ t(preset.labelKey) }}</span>
                      <span
                        v-if="selectedAvatarId === preset.id"
                        class="absolute -top-1 -right-1 w-5 h-5 rounded-full bg-primary text-on-primary flex items-center justify-center "
                      >
                        <span class="material-symbols-outlined text-xs">check</span>
                      </span>
                    </button>
                  </div>
                  <div v-if="customAvatars.length" class="px-4 pb-4">
                    <div class="text-2xs font-semibold text-text/40">
                      {{ t('settings.avatarUploads') }}
                    </div>
                    <div class="grid grid-cols-5 gap-3 mt-3">
                      <div v-for="asset in customAvatars" :key="asset.id" class="relative group">
                        <button type="button" class="relative" @click="selectCustomAvatar(asset.id)">
                          <AvatarBadge
                            :avatar="toLocalAvatar(asset.id)"
                            :label="t('settings.avatarUploads')"
                            class="w-11 h-11 rounded-xl border border-border/10 group-hover:border-primary/50 transition-colors"
                          />
                          <span
                            v-if="selectedLocalAvatarId === asset.id"
                            class="absolute -top-1 -right-1 w-5 h-5 rounded-full bg-primary text-on-primary flex items-center justify-center "
                          >
                            <span class="material-symbols-outlined text-xs">check</span>
                          </span>
                        </button>
                        <button
                          type="button"
                          class="absolute bottom-1 right-1 z-10 w-5 h-5 rounded-full bg-black/70 text-text/70 hover:text-text hover:bg-danger/80 border border-border/10 opacity-0 group-hover:opacity-100 transition-opacity"
                          :aria-label="t('common.remove')"
                          @click.stop="removeCustomAvatar(asset.id)"
                        >
                          <span class="material-symbols-outlined text-xs">close</span>
                        </button>
                      </div>
                    </div>
                  </div>
                  <div class="border-t border-border/10 px-4 py-3 flex items-center gap-2">
                    <button
                      type="button"
                      class="flex-1 inline-flex items-center justify-center gap-2 px-3 py-2 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/70 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors"
                      @click="triggerAvatarUpload"
                    >
                      <span class="material-symbols-outlined text-lg">upload</span>
                      {{ t('settings.avatarUpload') }}
                    </button>
                    <button
                      v-if="isCustomAvatar"
                      type="button"
                      class="inline-flex items-center justify-center gap-2 px-3 py-2 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/60 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors"
                      @click="resetAvatar"
                    >
                      <span class="material-symbols-outlined text-lg">restart_alt</span>
                      {{ t('settings.avatarReset') }}
                    </button>
                    <input ref="avatarInputRef" type="file" class="hidden" accept="image/*" @change="handleAvatarUpload" />
                  </div>
                  <div v-if="avatarError" class="px-4 pb-2 text-2xs text-danger">{{ avatarError }}</div>
                  <div class="px-4 pb-3 text-2xs text-text/40">{{ t('settings.avatarHint') }}</div>
                </div>
              </div>
              <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                  <label class="text-2xs font-medium text-faint block">{{ t('settings.displayName') }}</label>
                  <button
                    v-if="!isEditingDisplayName"
                    type="button"
                    class="group w-full flex items-center justify-between px-4 py-2.5 rounded-lg border border-border/10 bg-overlay/[0.03] text-base font-medium text-text/60 transition hover:bg-overlay/[0.06] hover:border-primary/40 hover:text-text"
                    @click="startEditDisplayName"
                  >
                    <span :class="displayNameValue ? 'text-text' : 'text-text/40'">{{ displayNameLabel }}</span>
                    <span class="material-symbols-outlined text-lg text-text/20 group-hover:text-primary transition-colors">edit</span>
                  </button>
                  <input
                    v-else
                    ref="displayNameInputRef"
                    v-model="draftSettings.account.displayName"
                    :class="inputClass"
                    :placeholder="t('settings.displayNamePlaceholder')"
                    type="text"
                    @keydown.esc.prevent="cancelEditDisplayName"
                    @keydown.enter.prevent="commitDisplayName"
                    @blur="commitDisplayName"
                  />
                </div>
                <div class="space-y-2">
                  <label class="text-2xs font-medium text-faint block">{{ t('settings.emailAddress') }}</label>
                  <input
                    v-model="draftSettings.account.email"
                    :class="[inputClass, 'cursor-not-allowed bg-overlay/[0.03] text-text/60']"
                    :placeholder="t('settings.emailPlaceholder', { at: '@' })"
                    type="email"
                    readonly
                    aria-readonly="true"
                  />
                  <button
                    type="button"
                    class="mt-2 inline-flex items-center gap-2 px-3 py-1.5 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/60 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors"
                    @click="handleChangeEmail"
                  >
                    <span class="material-symbols-outlined text-lg">mail</span>
                    {{ t('settings.changeEmail') }}
                  </button>
                </div>
                <div class="space-y-2 md:col-span-2">
                  <label class="text-2xs font-medium text-faint block">{{ t('settings.timeZone') }}</label>
                  <select v-model="draftSettings.account.timezone" class="settings-select" :aria-label="t('settings.timeZone')">
                    <option v-for="zone in timeZones" :key="zone.id" :value="zone.id">
                      {{ t(zone.labelKey) }}
                    </option>
                  </select>
                </div>
              </div>
            </div>
          </div>

          <div class="mt-6 flex justify-end">
            <button
              type="button"
              class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-danger/80 hover:text-danger hover:bg-danger/10 transition"
            >
              <span class="material-symbols-outlined text-lg">logout</span>
              <span class="text-base font-semibold">{{ t('settings.logOut') }}</span>
            </button>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line mb-12"></div>

        <section ref="appearanceRef" id="appearance" class="mb-12 scroll-mt-8">
          <h2 class="mb-1 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">palette</span>
            {{ t('settings.appearance') }}
          </h2>
          <p class="text-text/40 text-sm">{{ t('settings.appearanceSubtitle') }}</p>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
            <button
              v-for="option in themeOptions"
              :key="option.id"
              type="button"
              @click="setTheme(option.id)"
              :class="[
                'text-left rounded-2xl border p-4 transition group relative',
                option.id === theme
                  ? 'border-primary/40 bg-primary/[0.08]'
                  : 'border-border/5 bg-overlay/[0.02] hover:bg-overlay/[0.05] hover:border-border/10'
              ]"
            >
              <div class="flex items-center justify-between">
                <span class="text-sm font-semibold text-text">{{ t(option.labelKey) }}</span>
                <span
                  v-if="option.id === theme"
                  class="w-6 h-6 rounded-full bg-primary flex items-center justify-center shadow-primary/20"
                >
                  <span class="material-symbols-outlined text-on-primary text-base font-semibold">check</span>
                </span>
              </div>
              <div class="flex items-center gap-2 mt-3">
                <div
                  :class="['h-8 w-8 rounded-lg', option.id === 'system' ? '' : 'border border-border/10']"
                  :style="{ background: themePreview[option.id].base }"
                ></div>
                <div
                  :class="['h-8 flex-1 rounded-lg', option.id === 'system' ? '' : 'border border-border/10']"
                  :style="{ background: themePreview[option.id].panel }"
                ></div>
                <div
                  :class="['h-8 w-10 rounded-lg', option.id === 'system' ? '' : 'border border-border/10']"
                  :style="{ background: themePreview[option.id].accent }"
                ></div>
              </div>
              <p class="text-xs text-text/40 mt-3 leading-relaxed">{{ t(option.descriptionKey) }}</p>
            </button>
          </div>

          <div class="mt-8">
            <h3 class="text-sm font-medium text-text">{{ t('settings.accentTitle') }}</h3>
            <p class="mt-0.5 text-xs text-faint">{{ t('settings.accentSubtitle') }}</p>
            <div
              role="radiogroup"
              :aria-label="t('settings.accentTitle')"
              class="mt-3 flex flex-wrap gap-2"
              @keydown="handleAccentKeydown"
            >
              <button
                v-for="accent in ACCENTS"
                :key="accent.id"
                :ref="(el) => setAccentButtonRef(accent.id, el)"
                type="button"
                role="radio"
                :aria-checked="accent.id === currentAccent"
                :tabindex="accent.id === currentAccent ? 0 : -1"
                :class="[
                  'flex items-center gap-2 rounded-md border px-2.5 py-1.5 text-xs transition-colors',
                  accent.id === currentAccent
                    ? 'border-line-strong bg-hover text-text'
                    : 'border-line text-muted hover:bg-hover hover:text-text'
                ]"
                @click="setAccent(accent.id)"
              >
                <span class="h-3.5 w-3.5 rounded-sm" :style="{ background: accentSwatch(accent) }"></span>
                {{ t(accent.labelKey) }}
              </button>
            </div>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line mb-12"></div>
        <section ref="languageRef" id="language" class="mb-12 scroll-mt-8">
          <h2 class="mb-4 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">translate</span>
            {{ t('settings.language') }}
          </h2>
          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl overflow-hidden">
            <template v-for="(option, index) in localeOptions" :key="option.id">
              <button
                class="w-full flex items-center justify-between p-4 transition-colors text-left group"
                :class="option.id === locale ? 'bg-primary/[0.08] border-l-[3px] border-primary cursor-default' : 'hover:bg-overlay/[0.05]'"
                @click="setLocale(option.id)"
                type="button"
              >
                <div class="flex items-center gap-4">
                  <span class="text-2xl">{{ option.flag }}</span>
                  <div class="flex flex-col">
                    <span class="text-base font-medium transition-colors" :class="option.id === locale ? 'text-text' : 'text-text/70 group-hover:text-text'">
                      {{ t(option.labelKey) }}
                    </span>
                    <span v-if="option.id === locale" class="text-text/40 text-xs">{{ t('settings.languageDefault') }}</span>
                  </div>
                </div>
                <div v-if="option.id === locale" class="w-8 h-8 rounded-full bg-primary flex items-center justify-center shadow-primary/20">
                  <span class="material-symbols-outlined text-on-primary text-sm font-semibold">check</span>
                </div>
              </button>
              <div v-if="index < localeOptions.length - 1" class="w-full h-[1px] bg-overlay/5"></div>
            </template>
          </div>
          <div class="mt-4 px-2">
            <span class="text-sm text-text/30">{{ t('settings.changesApply') }}</span>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line mb-12"></div>

        <section ref="membersRef" id="members" class="mb-12 scroll-mt-8">
          <h2 class="mb-4 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">groups</span>
            {{ t('settings.defaultMember') }}
          </h2>
          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl p-6">
            <div class="flex justify-between items-center mb-4">
              <p class="text-sm font-medium text-text/60 ">{{ t('settings.selectMember') }}</p>
              <button
                type="button"
                class="text-primary hover:text-primary-hover text-sm font-medium transition-colors flex items-center gap-1"
                @click="handleMemberRefresh"
              >
                <span :class="['material-symbols-outlined text-lg', memberRefreshSpinning ? 'spin-once' : '']">sync</span>
                {{ t('settings.refreshList') }}
              </button>
            </div>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6" @click="openMemberMenuId = null">
              <div v-for="member in memberOptions" :key="member.id" class="relative group cursor-pointer" @click="selectMemberCard(member)">
                <div
                  :class="[
                    'cursor-pointer block h-full rounded-xl border p-4 flex flex-col items-center gap-2 transition',
                    selectedMemberId === member.id
                      ? 'bg-primary/[0.08] border-primary/50'
                      : 'border-border/10 bg-overlay/[0.02] hover:bg-overlay/5 hover:border-border/20'
                  ]"
                >
                  <div
                    :class="[
                      'w-12 h-12 rounded-xl border border-border/10 flex items-center justify-center text-text ',
                      member.isCustom ? 'bg-overlay/5 group-hover:bg-overlay/10' : member.swatch
                    ]"
                  >
                    <div v-if="member.iconKind === 'opencode'" class="member-icon-opencode" aria-hidden="true">
                      <span>open</span>
                      <span>code</span>
                    </div>
                    <div v-else-if="member.iconKind === 'qwen'" class="member-icon-qwen" aria-hidden="true">
                      <svg viewBox="0 0 200 200" role="presentation">
                        <defs>
                          <linearGradient id="qwen-grad-top" x1="0%" y1="0%" x2="100%" y2="100%">
                            <stop offset="0%" stop-color="#ffffff" />
                            <stop offset="100%" stop-color="#a29bfe" />
                          </linearGradient>
                          <linearGradient id="qwen-grad-side" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" stop-color="#6c5ce7" />
                            <stop offset="100%" stop-color="#4834d4" />
                          </linearGradient>
                          <g id="qwen-arm">
                            <path d="M100 100 L100 40 L155 65 L125 115 Z" fill="url(#qwen-grad-side)" />
                            <path
                              d="M100 100 L100 30 L165 60 L135 110 Z"
                              fill="url(#qwen-grad-top)"
                              transform="translate(-5, -5)"
                            />
                          </g>
                        </defs>
                        <use href="#qwen-arm" transform="rotate(0, 100, 100)" />
                        <use href="#qwen-arm" transform="rotate(120, 100, 100)" />
                        <use href="#qwen-arm" transform="rotate(240, 100, 100)" />
                      </svg>
                    </div>
                    <span v-else class="material-symbols-outlined text-2xl">{{ member.icon }}</span>
                  </div>
                  <div class="text-center">
                    <div class="text-text font-semibold text-sm">{{ member.label }}</div>
                    <div v-if="member.command" class="text-2xs text-text/40 mt-1 truncate max-w-[120px]">{{ member.command }}</div>
                    <div class="text-2xs text-text/60 mt-1">{{ member.kindLabel }}</div>
                  </div>
                  <div v-if="selectedMemberId === member.id" class="absolute top-2 right-9 animate-in fade-in zoom-in duration-200">
                    <div class="w-5 h-5 rounded-full bg-primary flex items-center justify-center ">
                      <span class="material-symbols-outlined text-on-primary text-base font-semibold">check</span>
                    </div>
                  </div>
                </div>
                <div class="absolute top-2 right-2">
                  <button
                    type="button"
                    class="w-6 h-6 rounded-full bg-overlay/10 text-text/70 hover:text-text hover:bg-overlay/20 flex items-center justify-center transition-colors"
                    :aria-label="t('settings.memberActions.menuLabel')"
                    @click.stop="toggleMemberMenu(member.id)"
                  >
                    <span class="material-symbols-outlined text-lg">more_vert</span>
                  </button>
                  <div
                    v-if="openMemberMenuId === member.id"
                    class="absolute right-0 mt-2 w-36 rounded-xl border border-border/10 bg-panel/95 shadow-float overflow-hidden z-10"
                    @click.stop
                  >
                    <button
                      type="button"
                      class="w-full text-left px-3 py-2 text-xs text-text/80 hover:text-text hover:bg-overlay/10 transition-colors"
                      @click="handleMemberTest"
                    >
                      {{ t('settings.memberActions.test') }}
                    </button>
                    <button
                      v-if="member.isDeletable"
                      type="button"
                      class="w-full text-left px-3 py-2 text-xs text-text/80 hover:text-text hover:bg-overlay/10 transition-colors"
                      @click="startEditMember(member.id)"
                    >
                      {{ t('settings.memberActions.edit') }}
                    </button>
                    <button
                      v-if="member.isDeletable"
                      type="button"
                      class="w-full text-left px-3 py-2 text-xs text-danger/80 hover:text-danger hover:bg-danger/10 transition-colors"
                      @click="removeCustomMember(member.id)"
                    >
                      {{ t('settings.memberActions.remove') }}
                    </button>
                  </div>
                </div>
              </div>
              <div class="relative group cursor-pointer" @click="openCustomMemberForm">
                <div class="cursor-pointer block h-full rounded-xl border border-dashed border-border/20 p-4 flex flex-col items-center gap-2 transition bg-overlay/[0.02] hover:bg-overlay/5 hover:border-border/30">
                  <div class="w-12 h-12 rounded-xl border border-border/10 flex items-center justify-center text-text/60 bg-overlay/5 group-hover:bg-overlay/10">
                    <span class="material-symbols-outlined text-2xl">add</span>
                  </div>
                  <div class="text-center">
                    <div class="text-text/70 font-semibold text-sm">{{ t('settings.memberOptions.custom') }}</div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="isAddingCustom" class="animate-in fade-in slide-in-from-top-2 duration-300">
              <div class="bg-overlay/[0.02] border border-border/10 rounded-xl p-5 relative">
                <div class="grid grid-cols-1 gap-4">
                  <div class="space-y-2">
                    <label class="text-2xs font-medium text-faint block">{{ t('settings.memberName') }}</label>
                    <input
                      v-model="customName"
                      :class="inputClass"
                      :placeholder="t('settings.memberNamePlaceholder')"
                      type="text"
                    />
                  </div>
                  <div class="space-y-2">
                    <label class="text-2xs font-medium text-faint block">{{ t('settings.commandInput') }}</label>
                    <div class="relative">
                      <span class="absolute left-3 top-1/2 -translate-y-1/2 text-text/30 font-mono text-sm">$</span>
                      <input
                        v-model="customCommand"
                        class="block w-full pl-7 pr-4 py-2.5 bg-surface/80 border border-border/10 rounded-lg text-text placeholder-text/20 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition font-mono text-sm"
                        :placeholder="t('settings.commandPlaceholder')"
                        type="text"
                      />
                    </div>
                  </div>
                </div>
                <div class="flex justify-end gap-2 mt-4">
                  <button
                    type="button"
                    class="w-8 h-8 rounded-lg bg-danger/10 hover:bg-danger/20 text-danger flex items-center justify-center transition-colors"
                    :title="t('settings.cancel')"
                    @click="resetCustomMemberForm"
                  >
                    <span class="material-symbols-outlined text-sm font-semibold">close</span>
                  </button>
                  <button
                    type="button"
                    class="w-8 h-8 rounded-lg bg-success/10 hover:bg-success/20 text-success flex items-center justify-center transition-colors"
                    :title="t('settings.confirm')"
                    @click="applyCustomMember"
                  >
                    <span class="material-symbols-outlined text-sm font-semibold">check</span>
                  </button>
                </div>
              </div>
            </div>

            <div class="mt-6 pt-6 border-t border-border/5">
              <div class="flex justify-between items-center mb-4">
                <p class="text-sm font-medium text-text/60 ">{{ t('settings.selectTerminal') }}</p>
                <button
                  type="button"
                  class="text-primary hover:text-primary-hover text-sm font-medium transition-colors flex items-center gap-1"
                  @click="handleTerminalRefresh"
                >
                  <span :class="['material-symbols-outlined text-lg', terminalRefreshSpinning ? 'spin-once' : '']">sync</span>
                  {{ t('settings.refreshList') }}
                </button>
              </div>

              <p v-if="terminalOptionsHint" class="text-xs text-text/40 mb-3">{{ terminalOptionsHint }}</p>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6" @click="openTerminalMenuKey = null">
                <div
                  v-for="option in terminalDisplayOptions"
                  :key="option.id"
                  class="relative group cursor-pointer"
                  @click="selectTerminalOption(option)"
                >
                  <div
                    :class="[
                      'cursor-pointer block h-full rounded-xl border p-4 transition',
                      isTerminalOptionSelected(option)
                        ? 'bg-primary/[0.08] border-primary/50'
                        : 'border-border/10 bg-overlay/[0.02] hover:bg-overlay/5 hover:border-border/20'
                    ]"
                  >
                    <div class="text-text font-semibold text-sm">{{ option.label }}</div>
                    <div class="text-2xs text-text/40 mt-1 truncate">
                      {{ option.path || t('settings.terminalAutoHint') }}
                    </div>
                  </div>
                  <div v-if="hasTerminalMenu(option)" class="absolute top-2 right-2">
                    <button
                      type="button"
                      class="w-6 h-6 rounded-full bg-overlay/10 text-text/70 hover:text-text hover:bg-overlay/20 flex items-center justify-center transition-colors"
                      :aria-label="t('settings.memberActions.menuLabel')"
                      @click.stop="toggleTerminalMenu(option)"
                    >
                      <span class="material-symbols-outlined text-lg">more_vert</span>
                    </button>
                    <div
                      v-if="openTerminalMenuKey === resolveTerminalMenuKey(option)"
                      class="absolute right-0 mt-2 w-36 rounded-xl border border-border/10 bg-panel/95 shadow-float overflow-hidden z-10"
                      @click.stop
                    >
                      <button
                        type="button"
                        class="w-full text-left px-3 py-2 text-xs text-text/80 hover:text-text hover:bg-overlay/10 transition-colors"
                        @click="handleTerminalTest(option)"
                      >
                        {{ t('settings.memberActions.test') }}
                      </button>
                      <button
                        v-if="option.source === 'custom'"
                        type="button"
                        class="w-full text-left px-3 py-2 text-xs text-text/80 hover:text-text hover:bg-overlay/10 transition-colors"
                        @click="startEditTerminal(option)"
                      >
                        {{ t('settings.memberActions.edit') }}
                      </button>
                      <button
                        v-if="option.source === 'custom'"
                        type="button"
                        class="w-full text-left px-3 py-2 text-xs text-danger/80 hover:text-danger hover:bg-danger/10 transition-colors"
                        @click="removeCustomTerminal(option.path)"
                      >
                        {{ t('settings.memberActions.remove') }}
                      </button>
                    </div>
                  </div>
                  <div
                    v-if="isTerminalOptionSelected(option)"
                    :class="[
                      'absolute top-2 animate-in fade-in zoom-in duration-200',
                      hasTerminalMenu(option) ? 'right-10' : 'right-2'
                    ]"
                  >
                    <div class="w-5 h-5 rounded-full bg-primary flex items-center justify-center ">
                      <span class="material-symbols-outlined text-on-primary text-base font-semibold">check</span>
                    </div>
                  </div>
                </div>

                <div class="relative group cursor-pointer" @click="openCustomTerminalForm">
                  <div class="cursor-pointer block h-full rounded-xl border border-dashed border-border/20 p-4 flex flex-col items-center gap-2 transition bg-overlay/[0.02] hover:bg-overlay/5 hover:border-border/30">
                    <div class="w-10 h-10 rounded-xl border border-border/10 flex items-center justify-center text-text/60 bg-overlay/5 group-hover:bg-overlay/10">
                      <span class="material-symbols-outlined text-2xl">add</span>
                    </div>
                    <div class="text-center">
                      <div class="text-text/70 font-semibold text-sm">{{ t('settings.terminalCustom') }}</div>
                    </div>
                  </div>
                </div>
              </div>

              <div v-if="isAddingCustomTerminal" class="animate-in fade-in slide-in-from-top-2 duration-300">
                <div class="bg-overlay/[0.02] border border-border/10 rounded-xl p-5 relative">
                  <div class="grid grid-cols-1 gap-4">
                    <div class="space-y-2">
                      <label class="text-2xs font-medium text-faint block">{{ t('settings.terminalName') }}</label>
                      <input
                        v-model="customTerminalName"
                        :class="inputClass"
                        :placeholder="t('settings.terminalNamePlaceholder')"
                        type="text"
                      />
                    </div>
                    <div class="space-y-2">
                      <label class="text-2xs font-medium text-faint block">{{ t('settings.terminalPath') }}</label>
                      <div class="flex flex-col sm:flex-row gap-2">
                        <input
                          v-model="customTerminalPath"
                          :class="[inputClass, 'flex-1']"
                          :placeholder="t('settings.terminalPathPlaceholder')"
                          type="text"
                        />
                        <button
                          type="button"
                          class="inline-flex items-center justify-center gap-2 px-3 py-2 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/70 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors"
                          @click="selectCustomTerminalPath"
                        >
                          <span class="material-symbols-outlined text-lg">folder_open</span>
                          {{ t('settings.terminalBrowse') }}
                        </button>
                      </div>
                    </div>
                  </div>
                  <div class="flex justify-end gap-2 mt-4">
                    <button
                      type="button"
                      class="w-8 h-8 rounded-lg bg-danger/10 hover:bg-danger/20 text-danger flex items-center justify-center transition-colors"
                      :title="t('settings.cancel')"
                      @click="resetCustomTerminalForm"
                    >
                      <span class="material-symbols-outlined text-sm font-semibold">close</span>
                    </button>
                    <button
                      type="button"
                      class="w-8 h-8 rounded-lg bg-success/10 hover:bg-success/20 text-success flex items-center justify-center transition-colors"
                      :title="t('settings.confirm')"
                      @click="applyCustomTerminal"
                    >
                      <span class="material-symbols-outlined text-sm font-semibold">check</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line mb-12"></div>
        <section ref="notificationsRef" id="notifications" class="mb-12 scroll-mt-8">
          <h2 class="mb-1 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">notifications</span>
            {{ t('settings.notifications') }}
          </h2>
          <p class="text-text/40 text-sm">{{ t('settings.notificationsSubtitle') }}</p>

          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl mt-6 overflow-hidden">
            <div v-for="(option, index) in notificationOptions" :key="option.key">
              <div class="flex items-center justify-between gap-6 px-6 py-4">
                <div>
                  <div class="text-sm font-semibold text-text">{{ t(option.labelKey) }}</div>
                  <div class="text-xs text-text/40 mt-1">{{ t(option.descriptionKey) }}</div>
                </div>
                <div class="relative inline-block w-10 align-middle select-none">
                  <input
                    :id="`notification-${option.key}`"
                    v-model="draftSettings.notifications[option.key]"
                    :class="toggleInputClass"
                    type="checkbox"
                  />
                  <label
                    class="block overflow-hidden h-6 rounded-full bg-panel-strong/80 peer-checked:bg-primary cursor-pointer transition-colors"
                    :for="`notification-${option.key}`"
                  ></label>
                </div>
              </div>
              <div v-if="index < notificationOptions.length - 1" class="w-full h-[1px] bg-overlay/5"></div>
            </div>

            <div v-if="draftSettings.notifications.quietHoursEnabled" class="px-6 pb-5">
              <div class="flex flex-col md:flex-row gap-4 mt-1">
                <div class="flex-1 space-y-2">
                  <label class="text-2xs font-medium text-faint block">{{ t('settings.quietHoursFrom') }}</label>
                  <input v-model="draftSettings.notifications.quietHoursStart" :class="inputClass" type="time" />
                </div>
                <div class="flex-1 space-y-2">
                  <label class="text-2xs font-medium text-faint block">{{ t('settings.quietHoursTo') }}</label>
                  <input v-model="draftSettings.notifications.quietHoursEnd" :class="inputClass" type="time" />
                </div>
              </div>
            </div>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line mb-12"></div>

        <section ref="keybindsRef" id="keybinds" class="scroll-mt-8">
          <h2 class="mb-1 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">keyboard_command_key</span>
            {{ t('settings.keybinds') }}
          </h2>
          <p class="text-text/40 text-sm">{{ t('settings.keybindsSubtitle') }}</p>

          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl mt-6 p-6 space-y-6">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="space-y-2">
                <label class="text-2xs font-medium text-faint block">{{ t('settings.keybindsProfile') }}</label>
                <select v-model="draftSettings.keybinds.profile" :class="selectClass">
                  <option v-for="profile in keybindProfiles" :key="profile.id" :value="profile.id">
                    {{ t(profile.labelKey) }}
                  </option>
                </select>
              </div>
              <div class="flex items-end justify-between gap-4">
                <div class="flex items-center gap-3">
                  <span class="text-base font-medium text-text/70">{{ t('settings.keybindsEnable') }}</span>
                  <div class="relative inline-block w-10 align-middle select-none">
                    <input id="keybinds-enabled" v-model="draftSettings.keybinds.enabled" :class="toggleInputClass" type="checkbox" />
                    <label class="block overflow-hidden h-6 rounded-full bg-panel-strong/80 peer-checked:bg-primary cursor-pointer transition-colors" for="keybinds-enabled"></label>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <span class="text-base font-medium text-text/70">{{ t('settings.keybindsHints') }}</span>
                  <div class="relative inline-block w-10 align-middle select-none">
                    <input id="keybinds-hints" v-model="draftSettings.keybinds.showHints" :class="toggleInputClass" type="checkbox" />
                    <label class="block overflow-hidden h-6 rounded-full bg-panel-strong/80 peer-checked:bg-primary cursor-pointer transition-colors" for="keybinds-hints"></label>
                  </div>
                </div>
              </div>
            </div>

            <div>
              <div class="flex items-center justify-between mb-3">
                <span class="text-xs font-medium text-faint">{{ t('settings.keybindsListTitle') }}</span>
                <button type="button" class="text-primary hover:text-primary-hover text-sm font-medium transition-colors" @click="resetKeybinds">
                  {{ t('settings.keybindsReset') }}
                </button>
              </div>
              <div class="space-y-2">
                <div
                  v-for="binding in activeKeybindProfile.bindings"
                  :key="binding.actionId"
                  class="flex items-center justify-between px-4 py-2 rounded-lg bg-overlay/[0.02] border border-border/5"
                >
                  <span class="text-sm text-text/70">{{ t(binding.labelKey) }}</span>
                  <span class="text-xs font-mono text-text/70 bg-overlay/5 px-2 py-1 rounded-md border border-border/10">{{ formatKeybindKeys(binding.keys) }}</span>
                </div>
              </div>
            </div>
          </div>
        </section>

        <div class="w-full h-[1px] bg-line my-12"></div>

        <section ref="dataRef" id="data" class="scroll-mt-8">
          <h2 class="mb-1 flex items-center gap-2 text-base font-semibold text-text">
            <span class="material-symbols-outlined text-faint">database</span>
            {{ t('settings.data') }}
          </h2>
          <p class="text-text/40 text-sm">{{ t('settings.dataSubtitle') }}</p>

          <div class="bg-overlay/[0.03] border border-border/5 rounded-2xl mt-6 p-6 space-y-4">
            <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
              <div>
                <div class="text-base font-semibold text-text/80">{{ t('settings.dataRepairTitle') }}</div>
                <div class="text-xs text-text/40 mt-1">{{ t('settings.dataRepairHint') }}</div>
              </div>
              <button
                type="button"
                class="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/70 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="repairingChatDb || !currentWorkspace"
                @click="handleRepairChatDb"
              >
                <span class="material-symbols-outlined text-lg">build_circle</span>
                {{ t('settings.dataRepairAction') }}
              </button>
            </div>

            <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 pt-2 border-t border-border/5">
              <div>
                <div class="text-base font-semibold text-text/80">{{ t('settings.dataClearTitle') }}</div>
                <div class="text-xs text-text/40 mt-1">{{ t('settings.dataClearHint') }}</div>
              </div>
              <button
                type="button"
                class="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-lg border border-danger/30 bg-danger/10 text-xs font-semibold text-danger hover:text-text hover:border-danger/60 hover:bg-danger/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="clearingChatDb || !currentWorkspace"
                @click="handleClearChatDb"
              >
                <span class="material-symbols-outlined text-lg">delete_sweep</span>
                {{ t('settings.dataClearAction') }}
              </button>
            </div>

            <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 pt-4 border-t border-border/5">
              <div>
                <div class="text-base font-semibold text-text/80">{{ t('settings.chatStreamTitle') }}</div>
                <div class="text-xs text-text/40 mt-1">{{ t('settings.chatStreamHint') }}</div>
              </div>
              <div class="relative inline-block w-10 align-middle select-none">
                <input id="chat-stream-output" v-model="draftSettings.chat.streamOutput" :class="toggleInputClass" type="checkbox" />
                <label class="block overflow-hidden h-6 rounded-full bg-panel-strong/80 peer-checked:bg-primary cursor-pointer transition-colors" for="chat-stream-output"></label>
              </div>
            </div>

            <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 pt-4 border-t border-border/5">
              <div>
                <div class="text-base font-semibold text-text/80">{{ t('settings.terminalFriends.title') }}</div>
                <div class="text-xs text-text/40 mt-1">{{ t('settings.terminalFriends.desc') }}</div>
              </div>
              <button
                type="button"
                class="inline-flex items-center gap-2 px-3 py-2 rounded-lg border border-border/10 bg-overlay/5 text-xs font-semibold text-text/70 hover:text-text hover:border-primary/40 hover:bg-overlay/10 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                :disabled="purgingTerminalFriends"
                @click="handlePurgeTerminalFriends"
              >
                <span class="material-symbols-outlined text-lg">person_remove</span>
                {{ t('settings.terminalFriends.action') }}
              </button>
            </div>

            <div v-if="dataActionMessage" class="text-xs text-text/50">
              {{ dataActionMessage }}
            </div>

          </div>
        </section>

      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
// 设置页逻辑：处理用户偏好、成员配置与数据维护，并与本地持久化同步。
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { isTauri } from '@tauri-apps/api/core';
import { emitTo, listen } from '@tauri-apps/api/event';
import { ask, open } from '@tauri-apps/plugin-dialog';
import { themeOptions, type AppTheme } from '@/features/global/theme';
import { ACCENTS, type Accent } from '@/features/global/accents';
import { openTerminalWindow } from '@/features/terminal/openTerminalWindow';
import { closeSession, createSession } from '@/features/terminal/terminalBridge';
import {
  TERMINAL_OPEN_TAB_EVENT,
  TERMINAL_WINDOW_READY_EVENT,
  TERMINAL_WINDOW_READY_REQUEST_EVENT,
  type TerminalOpenTabPayload,
  type TerminalWindowReadyPayload
} from '@/features/terminal/terminalEvents';
import { openWorkspaceSelectionWindow } from '@/shared/tauri/windows';
import { listTerminalEnvironments } from '@/shared/tauri/terminal';
import { purgeProjectTerminalMembers } from '@/shared/tauri/projectData';
import { useWorkspaceStore } from '@/features/workspace/workspaceStore';
import { useChatStore } from '@/features/chat/chatStore';
import { clearAllChatMessages, repairChatMessages } from '@/features/chat/chatBridge';
import AvatarBadge from '@/shared/components/AvatarBadge.vue';
import { AVATAR_PRESETS, DEFAULT_AVATAR_ID } from '@/shared/constants/avatars';
import { useSpinOnce } from '@/shared/animations/useSpinOnce';
import {
  clearAvatarUrlCache,
  ensureAvatar,
  getCssAvatarId,
  getLocalAvatarId,
  isCssAvatar,
  toCssAvatar,
  toLocalAvatar
} from '@/shared/utils/avatar';
import { BASE_TERMINALS, CUSTOM_TERMINAL_ICON, resolveBaseTerminalLabel } from '@/shared/constants/terminalCatalog';
import { TIME_ZONE_OPTIONS } from '@/shared/constants/timeZones';
import {
  deleteAvatarAsset,
  listAvatarAssets,
  storeAvatarAsset,
  type AvatarAsset
} from '@/shared/tauri/avatars';
import {
  DEFAULT_MEMBER_INDEX,
  clampMemberSelectionIndex,
  resolveMemberIdFromSelectionIndex,
  resolveMemberSelectionIndexFromId,
  type MemberSelectionIndex
} from '@/shared/utils/memberSelection';
import {
  cloneSettings,
  localeOptions,
  useSettingsStore,
  type SettingsState
} from '@/features/global/settingsStore';
import { KEYBIND_PROFILES, type KeybindProfile } from '@/shared/keyboard/profiles';
import { useProjectStore } from '@/features/workspace/projectStore';
import { useToastStore } from '@/stores/toastStore';
import { CURRENT_USER_ID } from '@/features/chat/data';
import { generateUlid } from '@/features/chat/chatBridge';
import type { TerminalEnvironmentOption } from '@/shared/types/terminal';
import { parseTerminalError, resolveTerminalErrorI18nKey } from '@/shared/utils/terminalErrors';

type MemberDisplayOption = {
  id: string;
  label: string;
  command: string;
  kindLabel: string;
  icon: string;
  iconKind: 'material' | 'opencode' | 'qwen';
  swatch?: string;
  isCustom: boolean;
  isDeletable: boolean;
  group: 0 | 1;
  index: number;
};

type TerminalDisplayOption = TerminalEnvironmentOption & {
  source: 'auto' | 'detected' | 'custom';
};

type CustomTerminalEntry = {
  name: string;
  path: string;
};

type NotificationToggleKey = 'desktop' | 'sound' | 'mentionsOnly' | 'previews' | 'quietHoursEnabled';

const emit = defineEmits<{ (e: 'logout'): void }>();
const workspaceStore = useWorkspaceStore();
const { closeWorkspace } = workspaceStore;
const { currentWorkspace } = storeToRefs(workspaceStore);
const projectStore = useProjectStore();

type SectionId = 'account' | 'appearance' | 'language' | 'members' | 'notifications' | 'keybinds' | 'data';

const { t } = useI18n();
const settingsStore = useSettingsStore();
const { settings, locale, theme } = storeToRefs(settingsStore);
const { saveSettings, setAccountDisplayName, setLocale, setTheme, setAccent } = settingsStore;

const currentAccent = computed(() => settings.value.appearance.accent);
const accentButtons = new Map<string, HTMLElement>();
const setAccentButtonRef = (id: string, el: unknown) => {
  if (el instanceof HTMLElement) accentButtons.set(id, el);
  else accentButtons.delete(id);
};
// Swatch shows the accent as it looks in the current mode.
const accentSwatch = (accent: Accent) =>
  `oklch(${document.documentElement.dataset.resolvedTheme === 'light' ? accent.light.primary : accent.dark.primary})`;
const handleAccentKeydown = (event: KeyboardEvent) => {
  const step = event.key === 'ArrowRight' || event.key === 'ArrowDown' ? 1 : event.key === 'ArrowLeft' || event.key === 'ArrowUp' ? -1 : 0;
  if (!step) return;
  event.preventDefault();
  const index = ACCENTS.findIndex((accent) => accent.id === currentAccent.value);
  const next = ACCENTS[(index + step + ACCENTS.length) % ACCENTS.length];
  setAccent(next.id);
  accentButtons.get(next.id)?.focus();
};
const toastStore = useToastStore();
const { pushToast } = toastStore;
const chatStore = useChatStore();
const { loadSession, reset: resetChat } = chatStore;
const { updateMember, hydrate } = projectStore;

const draftSettings = ref<SettingsState>(cloneSettings(settings.value));
const customName = ref('');
const customCommand = ref('');
const isAddingCustom = ref(false);
const terminalOptions = ref<TerminalEnvironmentOption[]>([]);
const terminalOptionsLoading = ref(false);
const terminalOptionsError = ref(false);
const customTerminalName = ref('');
const customTerminalPath = ref('');
const isAddingCustomTerminal = ref(false);
const openMemberMenuId = ref<string | null>(null);
const editingMemberId = ref<string | null>(null);
const openTerminalMenuKey = ref<string | null>(null);
const purgingTerminalFriends = ref(false);
const { spinning: memberRefreshSpinning, triggerSpin: triggerMemberRefreshSpin } = useSpinOnce();
const { spinning: terminalRefreshSpinning, triggerSpin: triggerTerminalRefreshSpin } = useSpinOnce();

const inputClass =
  'block w-full px-4 py-2.5 bg-surface/80 border border-border/10 rounded-lg text-text placeholder-text/20 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition text-sm';
const selectClass = `${inputClass} appearance-none`;
const toggleInputClass =
  'peer absolute block w-5 h-5 rounded-full bg-text border-4 border-border/20 appearance-none cursor-pointer transition duration-300 top-[2px] checked:right-0 checked:border-primary';

const contentRef = ref<HTMLElement | null>(null);
const accountRef = ref<HTMLElement | null>(null);
const appearanceRef = ref<HTMLElement | null>(null);
const languageRef = ref<HTMLElement | null>(null);
const membersRef = ref<HTMLElement | null>(null);
const notificationsRef = ref<HTMLElement | null>(null);
const keybindsRef = ref<HTMLElement | null>(null);
const dataRef = ref<HTMLElement | null>(null);
const sectionRefs: Array<{ id: SectionId; ref: typeof accountRef }> = [
  { id: 'account', ref: accountRef },
  { id: 'appearance', ref: appearanceRef },
  { id: 'language', ref: languageRef },
  { id: 'members', ref: membersRef },
  { id: 'notifications', ref: notificationsRef },
  { id: 'keybinds', ref: keybindsRef },
  { id: 'data', ref: dataRef }
];
const activeSection = ref<SectionId>('account');
// 自动滚动期间避免滚动监听反复切换高亮。
const isAutoScrolling = ref(false);
const autoScrollTimeoutId = ref<number | null>(null);
const targetScrollTop = ref<number | null>(null);
const displayNameInputRef = ref<HTMLInputElement | null>(null);
const isEditingDisplayName = ref(false);
const displayNameValue = computed(() => draftSettings.value.account.displayName.trim());
const displayNameSnapshot = ref('');
const skipDisplayNameCommit = ref(false);
const DEFAULT_OWNER_NAME = 'Owner';
const displayNameLabel = computed(() => displayNameValue.value || DEFAULT_OWNER_NAME);
const avatarPresets = AVATAR_PRESETS;
const avatarMenuOpen = ref(false);
const avatarMenuRef = ref<HTMLElement | null>(null);
const avatarButtonRef = ref<HTMLElement | null>(null);
const avatarInputRef = ref<HTMLInputElement | null>(null);
const avatarError = ref<string | null>(null);
const avatarMenuPosition = ref({ left: 0, top: 0 });
const avatarMenuStyle = computed(() => ({
  left: `${avatarMenuPosition.value.left}px`,
  top: `${avatarMenuPosition.value.top}px`
}));
const lastCssAvatarId = ref(DEFAULT_AVATAR_ID);
const accountAvatar = computed(() => ensureAvatar(draftSettings.value.account.avatar));
const selectedAvatarId = computed(() => getCssAvatarId(accountAvatar.value));
const selectedLocalAvatarId = computed(() => getLocalAvatarId(accountAvatar.value));
const isCustomAvatar = computed(() => !isCssAvatar(accountAvatar.value));
// 头像上传大小限制为 2MB，避免存储膨胀与性能退化。
const MAX_AVATAR_BYTES = 2 * 1024 * 1024;
// 菜单距离视口边界的安全边距，防止被裁切。
const AVATAR_MENU_PADDING = 12;
const supportsAvatarStorage = isTauri();
const customAvatars = ref<AvatarAsset[]>([]);
const customAvatarsLoading = ref(false);
const repairingChatDb = ref(false);
const clearingChatDb = ref(false);
const dataActionMessage = ref<string | null>(null);


// 对比设置时忽略会频繁变化的字段，避免触发循环保存。
const serializeSettings = (value: SettingsState) => {
  const { account, ...rest } = value;
  const accountRest = { ...account };
  delete accountRest.status;
  return JSON.stringify({ ...rest, account: accountRest });
};
const syncDraftSettings = (next: SettingsState) => {
  draftSettings.value = cloneSettings(next);
};

// 构造可持久化的设置，避免覆盖正在编辑或由其他通道管理的字段。
const buildPersistableSettings = (value: SettingsState) => {
  const nextDraft = cloneSettings(value);
  nextDraft.account.status = settings.value.account.status;
  if (isEditingDisplayName.value) {
    nextDraft.account.displayName = settings.value.account.displayName;
  }
  return nextDraft;
};

const startEditDisplayName = async () => {
  displayNameSnapshot.value = draftSettings.value.account.displayName;
  skipDisplayNameCommit.value = false;
  isEditingDisplayName.value = true;
  await nextTick();
  displayNameInputRef.value?.focus();
  displayNameInputRef.value?.select();
};

const commitDisplayName = () => {
  if (skipDisplayNameCommit.value) {
    skipDisplayNameCommit.value = false;
    return;
  }
  const nextName = draftSettings.value.account.displayName.trim();
  if (!nextName) {
    draftSettings.value.account.displayName = displayNameSnapshot.value;
    isEditingDisplayName.value = false;
    return;
  }
  const next = setAccountDisplayName(nextName);
  draftSettings.value.account.displayName = next.account.displayName;
  displayNameSnapshot.value = next.account.displayName;
  isEditingDisplayName.value = false;
};

const cancelEditDisplayName = () => {
  skipDisplayNameCommit.value = true;
  draftSettings.value.account.displayName = displayNameSnapshot.value;
  isEditingDisplayName.value = false;
};

// 保证头像菜单在视口内可见，避免溢出导致无法关闭。
const clampAvatarMenu = () => {
  if (!avatarMenuRef.value) return;
  const rect = avatarMenuRef.value.getBoundingClientRect();
  const maxLeft = window.innerWidth - rect.width - AVATAR_MENU_PADDING;
  const maxTop = window.innerHeight - rect.height - AVATAR_MENU_PADDING;
  avatarMenuPosition.value = {
    left: Math.max(AVATAR_MENU_PADDING, Math.min(avatarMenuPosition.value.left, maxLeft)),
    top: Math.max(AVATAR_MENU_PADDING, Math.min(avatarMenuPosition.value.top, maxTop))
  };
};

// 优先使用点击位置定位菜单，回退到按钮下方。
const positionAvatarMenu = (event?: MouseEvent) => {
  const button = avatarButtonRef.value;
  const clickLeft = event?.clientX;
  const clickTop = event?.clientY;
  if (typeof clickLeft === 'number' && typeof clickTop === 'number') {
    avatarMenuPosition.value = { left: clickLeft, top: clickTop };
  } else if (button) {
    const rect = button.getBoundingClientRect();
    avatarMenuPosition.value = { left: rect.left, top: rect.bottom + 8 };
  }
  nextTick(() => {
    clampAvatarMenu();
  });
};

const toggleAvatarMenu = (event?: MouseEvent) => {
  if (avatarMenuOpen.value) {
    avatarMenuOpen.value = false;
    return;
  }
  avatarError.value = null;
  avatarMenuOpen.value = true;
  positionAvatarMenu(event);
  void loadCustomAvatars();
};

const selectAvatarPreset = (id: string) => {
  draftSettings.value.account.avatar = toCssAvatar(id);
  avatarError.value = null;
  avatarMenuOpen.value = false;
};

const triggerAvatarUpload = () => {
  avatarError.value = null;
  avatarInputRef.value?.click();
};

// 仅接受常见图片类型，其他类型统一回退为 png。
const AVATAR_EXTENSION_BY_MIME: Record<string, string> = {
  'image/png': 'png',
  'image/jpeg': 'jpg',
  'image/jpg': 'jpg',
  'image/webp': 'webp',
  'image/gif': 'gif'
};

const resolveImageExtension = (file: File) => {
  if (file.type && AVATAR_EXTENSION_BY_MIME[file.type]) {
    return AVATAR_EXTENSION_BY_MIME[file.type];
  }
  const match = file.name.toLowerCase().match(/\.([a-z0-9]+)$/);
  if (match?.[1]) {
    return match[1];
  }
  return 'png';
};


const fileToBytes = async (file: File) => {
  const buffer = await file.arrayBuffer();
  return Array.from(new Uint8Array(buffer));
};


const upsertCustomAvatar = (asset: AvatarAsset) => {
  const existing = customAvatars.value.find((item) => item.id === asset.id);
  if (existing) {
    customAvatars.value = customAvatars.value.map((item) => (item.id === asset.id ? asset : item));
    return;
  }
  customAvatars.value = [asset, ...customAvatars.value];
};

const loadCustomAvatars = async () => {
  if (!supportsAvatarStorage || customAvatarsLoading.value) {
    return;
  }
  customAvatarsLoading.value = true;
  try {
    const assets = await listAvatarAssets();
    customAvatars.value = Array.isArray(assets) ? assets : [];
    await nextTick();
    clampAvatarMenu();
  } catch (error) {
    console.error('Failed to load avatar assets.', error);
  } finally {
    customAvatarsLoading.value = false;
  }
};

const selectCustomAvatar = (id: string) => {
  draftSettings.value.account.avatar = toLocalAvatar(id);
  avatarError.value = null;
  avatarMenuOpen.value = false;
};

const removeCustomAvatar = async (id: string) => {
  avatarError.value = null;
  try {
    await deleteAvatarAsset(id);
    customAvatars.value = customAvatars.value.filter((item) => item.id !== id);
    clearAvatarUrlCache(id);
    if (selectedLocalAvatarId.value === id) {
      resetAvatar();
    }
    await nextTick();
    clampAvatarMenu();
  } catch (error) {
    console.error('Failed to delete avatar asset.', error);
    avatarError.value = t('settings.avatarErrors.storageFailed');
  }
};

const handleAvatarUpload = async (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) return;
  avatarError.value = null;
  if (!file.type.startsWith('image/')) {
    avatarError.value = t('settings.avatarErrors.invalidType');
    input.value = '';
    return;
  }
  if (file.size > MAX_AVATAR_BYTES) {
    avatarError.value = t('settings.avatarErrors.tooLarge');
    input.value = '';
    return;
  }
  if (!supportsAvatarStorage) {
    avatarError.value = t('settings.avatarErrors.storageFailed');
    input.value = '';
    return;
  }
  try {
    const bytes = await fileToBytes(file);
    const extension = resolveImageExtension(file);
    const asset = await storeAvatarAsset(bytes, extension);
    upsertCustomAvatar(asset);
    draftSettings.value.account.avatar = toLocalAvatar(asset.id);
    avatarMenuOpen.value = false;
  } catch {
    avatarError.value = t('settings.avatarErrors.storageFailed');
  } finally {
    input.value = '';
  }
};

const resetAvatar = () => {
  draftSettings.value.account.avatar = toCssAvatar(lastCssAvatarId.value);
  avatarError.value = null;
  avatarMenuOpen.value = false;
};

const handleChangeEmail = () => {
  // [TODO/Settings, 2026-01-23] 补齐邮箱变更流程与后端接口对接。
  void 0;
};

watch(
  settings,
  (next) => {
    if (serializeSettings(next) === serializeSettings(draftSettings.value)) {
      return;
    }
    syncDraftSettings(next);
  },
  { deep: true }
);

watch(
  draftSettings,
  (next) => {
    const candidate = buildPersistableSettings(next);
    if (serializeSettings(candidate) === serializeSettings(settings.value)) {
      return;
    }
    const saved = saveSettings(candidate);
    syncDraftSettings(saved);
  },
  { deep: true }
);

watch(accountAvatar, (next) => {
  const id = getCssAvatarId(next);
  if (id) {
    lastCssAvatarId.value = id;
  }
});

watch(
  () => draftSettings.value.account.avatar,
  (next, prev) => {
    if (!next || next === prev) return;
    void updateMember(CURRENT_USER_ID, { avatar: next });
  }
);

const selectedMemberIndex = computed({
  get: () => draftSettings.value.members.defaultMemberIndex,
  set: (value: MemberSelectionIndex) => {
    draftSettings.value.members.defaultMemberIndex = clampMemberSelectionIndex(
      value,
      draftSettings.value.members.customMembers
    );
  }
});

const selectedMemberId = computed(() => {
  const resolved = resolveMemberIdFromSelectionIndex(
    selectedMemberIndex.value,
    draftSettings.value.members.customMembers
  );
  return resolved ?? BASE_TERMINALS.value[0]?.id ?? '';
});

const syncDefaultMemberIndex = (preferredId?: string) => {
  const candidateId =
    preferredId ??
    resolveMemberIdFromSelectionIndex(
      selectedMemberIndex.value,
      draftSettings.value.members.customMembers
    );
  if (candidateId) {
    const nextIndex = resolveMemberSelectionIndexFromId(candidateId, draftSettings.value.members.customMembers);
    if (nextIndex) {
      selectedMemberIndex.value = nextIndex;
      return;
    }
  }
  selectedMemberIndex.value = DEFAULT_MEMBER_INDEX;
};

const normalizeTerminalPathKey = (value: string) => value.trim().toLowerCase();

const resolveTerminalLabelFromPath = (value: string) => {
  const trimmed = value.trim();
  if (!trimmed) {
    return '';
  }
  const lastSegment = trimmed.split(/[\\/]/).pop() ?? '';
  if (!lastSegment) {
    return trimmed;
  }
  return lastSegment.replace(/\.[^.]+$/, '') || lastSegment;
};

const resolveCustomTerminalEntry = (path: string) => {
  const key = normalizeTerminalPathKey(path);
  if (!key) {
    return null;
  }
  return (
    customTerminalEntries.value.find(
      (entry) => normalizeTerminalPathKey(entry.path) === key
    ) ?? null
  );
};

const upsertCustomTerminals = (
  entries: CustomTerminalEntry[],
  name: string,
  path: string
) => {
  const key = normalizeTerminalPathKey(path);
  if (!key) {
    return entries;
  }
  let updated = false;
  const next = entries.map((entry) => {
    if (normalizeTerminalPathKey(entry.path) !== key) {
      return entry;
    }
    updated = true;
    return { name, path };
  });
  if (!updated) {
    next.push({ name, path });
  }
  return next;
};

const selectedTerminalPath = computed(() => draftSettings.value.members.defaultTerminalPath.trim());
const selectedTerminalName = computed(() => draftSettings.value.members.defaultTerminalName.trim());
const customTerminalEntries = computed<CustomTerminalEntry[]>(
  () => draftSettings.value.members.customTerminals ?? []
);
const autoTerminalOption = computed<TerminalDisplayOption>(() => ({
  id: 'auto',
  label: t('settings.terminalAuto'),
  path: '',
  source: 'auto'
}));

const terminalDisplayOptions = computed<TerminalDisplayOption[]>(() => {
  const customMap = new Map<string, CustomTerminalEntry>();
  customTerminalEntries.value.forEach((entry) => {
    const key = normalizeTerminalPathKey(entry.path);
    if (!key || customMap.has(key)) {
      return;
    }
    customMap.set(key, entry);
  });
  const detected = terminalOptions.value.map((option) => {
    const key = normalizeTerminalPathKey(option.path);
    const custom = customMap.get(key);
    if (custom) {
      return {
        ...option,
        label: custom.name || option.label,
        source: 'custom' as const
      };
    }
    return {
      ...option,
      source: 'detected' as const
    };
  });
  const detectedKeys = new Set(detected.map((option) => normalizeTerminalPathKey(option.path)));
  const options: TerminalDisplayOption[] = [autoTerminalOption.value, ...detected];
  customMap.forEach((entry, key) => {
    if (detectedKeys.has(key)) {
      return;
    }
    const label = entry.name || resolveTerminalLabelFromPath(entry.path) || t('settings.terminalCustom');
    options.push({
      id: `custom-${key}`,
      label,
      path: entry.path,
      source: 'custom'
    });
  });
  if (selectedTerminalPath.value) {
    const storedKey = normalizeTerminalPathKey(selectedTerminalPath.value);
    if (storedKey && !detectedKeys.has(storedKey) && !customMap.has(storedKey)) {
      options.push({
        id: `custom-${storedKey}`,
        label: selectedTerminalName.value || resolveTerminalLabelFromPath(selectedTerminalPath.value) || t('settings.terminalCustom'),
        path: selectedTerminalPath.value,
        source: 'custom'
      });
    }
  }
  return options;
});

const terminalOptionsHint = computed(() => {
  if (!isTauri()) {
    return t('settings.terminalNotAvailable');
  }
  if (terminalOptionsLoading.value) {
    return '';
  }
  if (terminalOptionsError.value || terminalOptions.value.length === 0) {
    return t('settings.terminalEmpty');
  }
  return '';
});

const isTerminalOptionSelected = (option: TerminalDisplayOption) => {
  if (option.source === 'auto') {
    return !selectedTerminalPath.value;
  }
  if (!selectedTerminalPath.value) {
    return false;
  }
  return normalizeTerminalPathKey(option.path) === normalizeTerminalPathKey(selectedTerminalPath.value);
};

const loadTerminalEnvironments = async () => {
  if (!isTauri()) {
    terminalOptions.value = [];
    terminalOptionsError.value = false;
    return;
  }
  if (terminalOptionsLoading.value) {
    return;
  }
  terminalOptionsLoading.value = true;
  terminalOptionsError.value = false;
  try {
    const options = await listTerminalEnvironments();
    terminalOptions.value = Array.isArray(options) ? options : [];
  } catch (error) {
    console.error('Failed to load terminal environments.', error);
    terminalOptionsError.value = true;
    terminalOptions.value = [];
  } finally {
    terminalOptionsLoading.value = false;
  }
};

const handleTerminalRefresh = () => {
  triggerTerminalRefreshSpin();
  void loadTerminalEnvironments();
};

const applyDefaultTerminalSelection = (name: string, path: string) => {
  draftSettings.value.members.defaultTerminalName = name;
  draftSettings.value.members.defaultTerminalPath = path;
  persistMemberSettings(draftSettings.value.members);
};

const selectTerminalOption = (option: TerminalDisplayOption) => {
  openTerminalMenuKey.value = null;
  if (option.source === 'auto') {
    applyDefaultTerminalSelection('', '');
  } else {
    applyDefaultTerminalSelection(option.label, option.path);
  }
  resetCustomTerminalForm();
};

const openCustomTerminalForm = () => {
  openTerminalMenuKey.value = null;
  const storedPath = selectedTerminalPath.value;
  const storedKey = storedPath ? normalizeTerminalPathKey(storedPath) : '';
  const detectedKeys = new Set(
    terminalOptions.value.map((option) => normalizeTerminalPathKey(option.path))
  );
  const customEntry = storedPath ? resolveCustomTerminalEntry(storedPath) : null;
  if (storedKey && (!detectedKeys.has(storedKey) || customEntry)) {
    customTerminalName.value = customEntry?.name || selectedTerminalName.value;
    customTerminalPath.value = customEntry?.path ?? storedPath;
  } else {
    customTerminalName.value = '';
    customTerminalPath.value = '';
  }
  isAddingCustomTerminal.value = true;
};

const resetCustomTerminalForm = () => {
  customTerminalName.value = '';
  customTerminalPath.value = '';
  isAddingCustomTerminal.value = false;
};

const applyCustomTerminal = () => {
  const path = customTerminalPath.value.trim();
  const name = customTerminalName.value.trim();
  if (!path) {
    return;
  }
  const resolvedName = name || resolveTerminalLabelFromPath(path) || t('settings.terminalCustom');
  const nextMembers = cloneSettings(draftSettings.value).members;
  nextMembers.defaultTerminalName = resolvedName;
  nextMembers.defaultTerminalPath = path;
  nextMembers.customTerminals = upsertCustomTerminals(
    nextMembers.customTerminals ?? [],
    resolvedName,
    path
  );
  persistMemberSettings(nextMembers);
  resetCustomTerminalForm();
};

const resolveTerminalMenuKeyFromPath = (path: string) => normalizeTerminalPathKey(path);

const resolveTerminalMenuKey = (option: TerminalDisplayOption) => {
  if (option.source === 'auto') {
    return 'auto';
  }
  return resolveTerminalMenuKeyFromPath(option.path);
};

const removeCustomTerminal = (path: string) => {
  const trimmed = path.trim();
  if (!trimmed) {
    return;
  }
  const key = resolveTerminalMenuKeyFromPath(trimmed);
  if (openTerminalMenuKey.value === key) {
    openTerminalMenuKey.value = null;
  }
  const nextMembers = cloneSettings(draftSettings.value).members;
  const existing = nextMembers.customTerminals ?? [];
  const filtered = existing.filter(
    (entry) => resolveTerminalMenuKeyFromPath(entry.path) !== key
  );
  const isSelected =
    resolveTerminalMenuKeyFromPath(nextMembers.defaultTerminalPath) === key;
  if (!isSelected && filtered.length === existing.length) {
    return;
  }
  nextMembers.customTerminals = filtered;
  if (isSelected) {
    nextMembers.defaultTerminalName = '';
    nextMembers.defaultTerminalPath = '';
  }
  persistMemberSettings(nextMembers);
};

const toggleTerminalMenu = (option: TerminalDisplayOption) => {
  const key = resolveTerminalMenuKey(option);
  if (!key) {
    openTerminalMenuKey.value = null;
    return;
  }
  openTerminalMenuKey.value = openTerminalMenuKey.value === key ? null : key;
};

const hasTerminalMenu = (option: TerminalDisplayOption) =>
  option.source === 'auto' || Boolean(option.path?.trim());

const TERMINAL_READY_TIMEOUT_MS = 8000;

const waitForTerminalWindowReady = async (windowLabel: string) => {
  if (!isTauri() || !windowLabel) {
    return false;
  }
  return new Promise<boolean>((resolve) => {
    let settled = false;
    let stopListener: (() => void) | null = null;
    let timeoutId = 0;
    const finish = (ready: boolean) => {
      if (settled) {
        return;
      }
      settled = true;
      if (stopListener) {
        stopListener();
        stopListener = null;
      }
      window.clearTimeout(timeoutId);
      resolve(ready);
    };
    timeoutId = window.setTimeout(() => {
      finish(false);
    }, TERMINAL_READY_TIMEOUT_MS);
    void listen<TerminalWindowReadyPayload>(TERMINAL_WINDOW_READY_EVENT, (event) => {
      if (event.payload.windowLabel !== windowLabel) {
        return;
      }
      finish(true);
    }).then((stop) => {
      stopListener = stop;
      if (settled) {
        stopListener();
        stopListener = null;
      }
    });
    void emitTo(windowLabel, TERMINAL_WINDOW_READY_REQUEST_EVENT, {}).catch(() => {});
  });
};

const handleTerminalTest = async (option: TerminalDisplayOption) => {
  openTerminalMenuKey.value = null;
  const path = option.path?.trim();
  const isAuto = option.source === 'auto';
  if (!isAuto && !path) {
    return;
  }
  try {
    const workspace = currentWorkspace.value;
    const terminalId = await createSession({
      cwd: workspace?.path,
      workspaceId: workspace?.id,
      terminalType: 'shell',
      terminalPath: isAuto ? undefined : path,
      strictShell: true
    });
    const result = await openTerminalWindow({
      workspaceId: workspace?.id,
      workspaceName: workspace?.name,
      workspacePath: workspace?.path,
      autoTab: false
    });
    if (!result?.label) {
      await closeSession(terminalId, { preserve: false }).catch(() => {});
      throw new Error('terminal window unavailable');
    }
    const ready = await waitForTerminalWindowReady(result.label);
    if (!ready) {
      await closeSession(terminalId, { preserve: false }).catch(() => {});
      throw new Error('terminal window not ready');
    }
    const title =
      option.label?.trim() ||
      (isAuto ? t('settings.terminalAuto') : resolveTerminalLabelFromPath(path ?? '')) ||
      t('settings.terminalCustom');
    const payload: TerminalOpenTabPayload = {
      terminalId,
      title,
      terminalType: 'shell',
      keepAlive: false
    };
    await emitTo(result.label, TERMINAL_OPEN_TAB_EVENT, payload).catch(async (error) => {
      await closeSession(terminalId, { preserve: false }).catch(() => {});
      throw error;
    });
  } catch (error) {
    const parsed = parseTerminalError(error);
    const message = parsed.message;
    const key = resolveTerminalErrorI18nKey(parsed.code);
    if (key === 'settings.terminalTestErrors.shellBinaryNotFound') {
      pushToast(t(key, { path: parsed.detail?.path ?? message }), { tone: 'error' });
    } else if (key) {
      pushToast(t(key, { error: message }), { tone: 'error' });
    } else {
      pushToast(t('settings.terminalTestFailed', { error: message }), { tone: 'error' });
    }
    console.error('Failed to test terminal.', error, { terminalPath: option.path, source: option.source });
  }
};

const startEditTerminal = (option: TerminalDisplayOption) => {
  const entry = resolveCustomTerminalEntry(option.path);
  if (!option.path) {
    openTerminalMenuKey.value = null;
    return;
  }
  customTerminalName.value = entry?.name || option.label;
  customTerminalPath.value = entry?.path ?? option.path;
  isAddingCustomTerminal.value = true;
  openTerminalMenuKey.value = null;
};

const selectCustomTerminalPath = async () => {
  if (!isTauri()) {
    return;
  }
  try {
    const selection = await open({ directory: false, multiple: false });
    const path = Array.isArray(selection) ? selection[0] : selection;
    if (!path) {
      return;
    }
    customTerminalPath.value = path;
    if (!customTerminalName.value.trim()) {
      customTerminalName.value = resolveTerminalLabelFromPath(path);
    }
  } catch (error) {
    console.error('Failed to select terminal executable.', error);
  }
};

const resetMemberDraft = () => {
  const next = cloneSettings(settings.value);
  draftSettings.value.members = next.members;
  customName.value = '';
  customCommand.value = '';
  isAddingCustom.value = false;
  resetCustomTerminalForm();
  openMemberMenuId.value = null;
  openTerminalMenuKey.value = null;
  editingMemberId.value = null;
  syncDefaultMemberIndex();
};

const handleMemberRefresh = () => {
  triggerMemberRefreshSpin();
  resetMemberDraft();
};

const persistMemberSettings = (nextMembers: SettingsState['members']) => {
  const next = cloneSettings(settings.value);
  next.members = nextMembers;
  const saved = saveSettings(next);
  draftSettings.value.members = cloneSettings(saved).members;
  return saved;
};

const buildCustomMemberId = async () => {
  try {
    return await generateUlid();
  } catch (error) {
    console.error('Failed to generate custom member id.', error);
    return null;
  }
};

const openCustomMemberForm = () => {
  editingMemberId.value = null;
  isAddingCustom.value = true;
  customName.value = '';
  customCommand.value = '';
};

const handleLeaveWorkspace = () => {
  closeWorkspace();
  emit('logout');
};

const handleCreateTeam = async () => {
  if (!isTauri()) {
    return;
  }
  try {
    await openWorkspaceSelectionWindow();
  } catch (error) {
    console.error('Failed to open create team window.', error);
  }
};

const applyCustomMember = async () => {
  const name = customName.value.trim();
  const command = customCommand.value.trim();
  if (!name && !command) {
    return;
  }
  if (editingMemberId.value) {
    draftSettings.value.members.customMembers = draftSettings.value.members.customMembers.map((member) =>
      member.id === editingMemberId.value ? { ...member, name, command } : member
    );
  } else {
    const existingIds = new Set(draftSettings.value.members.customMembers.map((member) => member.id));
    let id = await buildCustomMemberId();
    if (!id) {
      return;
    }
    while (existingIds.has(id)) {
      id = await buildCustomMemberId();
      if (!id) {
        return;
      }
    }
    draftSettings.value.members.customMembers = [...draftSettings.value.members.customMembers, { id, name, command }];
    syncDefaultMemberIndex(id);
  }
  persistMemberSettings(draftSettings.value.members);
  resetCustomMemberForm();
};

const resetCustomMemberForm = () => {
  customName.value = '';
  customCommand.value = '';
  isAddingCustom.value = false;
  editingMemberId.value = null;
};

const removeCustomMember = (id: string) => {
  const currentDefaultId = resolveMemberIdFromSelectionIndex(
    selectedMemberIndex.value,
    draftSettings.value.members.customMembers
  );
  draftSettings.value.members.customMembers = draftSettings.value.members.customMembers.filter((member) => member.id !== id);
  if (!currentDefaultId || currentDefaultId === id) {
    selectedMemberIndex.value = DEFAULT_MEMBER_INDEX;
  } else {
    syncDefaultMemberIndex(currentDefaultId);
  }
  persistMemberSettings(draftSettings.value.members);
  if (editingMemberId.value === id) {
    resetCustomMemberForm();
  }
  if (openMemberMenuId.value === id) {
    openMemberMenuId.value = null;
  }
};

const selectMemberCard = (member: MemberDisplayOption) => {
  if (selectedMemberId.value === member.id) {
    openMemberMenuId.value = null;
    return;
  }
  selectedMemberIndex.value = [member.group, member.index];
  persistMemberSettings(draftSettings.value.members);
  openMemberMenuId.value = null;
};

const toggleMemberMenu = (id: string) => {
  openMemberMenuId.value = openMemberMenuId.value === id ? null : id;
};

const handleMemberTest = async () => {
  openMemberMenuId.value = null;
  try {
    const workspace = currentWorkspace.value;
    await openTerminalWindow({
      workspaceId: workspace?.id,
      workspaceName: workspace?.name,
      workspacePath: workspace?.path
    });
  } catch (error) {
    console.error('Failed to open terminal window.', error);
  }
};

const startEditMember = (id: string) => {
  const target = draftSettings.value.members.customMembers.find((member) => member.id === id);
  if (!target) {
    openMemberMenuId.value = null;
    return;
  }
  editingMemberId.value = id;
  isAddingCustom.value = true;
  customName.value = target.name;
  customCommand.value = target.command;
  openMemberMenuId.value = null;
};

const timeZones = TIME_ZONE_OPTIONS;


const notificationOptions: Array<{ key: NotificationToggleKey; labelKey: string; descriptionKey: string }> = [
  { key: 'desktop', labelKey: 'settings.notificationOptions.desktop', descriptionKey: 'settings.notificationOptions.desktopDesc' },
  { key: 'sound', labelKey: 'settings.notificationOptions.sound', descriptionKey: 'settings.notificationOptions.soundDesc' },
  { key: 'mentionsOnly', labelKey: 'settings.notificationOptions.mentionsOnly', descriptionKey: 'settings.notificationOptions.mentionsOnlyDesc' },
  { key: 'previews', labelKey: 'settings.notificationOptions.previews', descriptionKey: 'settings.notificationOptions.previewsDesc' },
  { key: 'quietHoursEnabled', labelKey: 'settings.notificationOptions.quietHours', descriptionKey: 'settings.notificationOptions.quietHoursDesc' }
];

const keybindProfiles: Array<{
  id: KeybindProfile;
  labelKey: string;
  bindings: Array<{ actionId: string; labelKey: string; keys: string | string[] }>;
}> = KEYBIND_PROFILES;

const activeKeybindProfile = computed(
  () => keybindProfiles.find((profile) => profile.id === draftSettings.value.keybinds.profile) ?? keybindProfiles[0]
);

const formatKeybindKeys = (keys: string | string[]) => (Array.isArray(keys) ? keys.join(' / ') : keys);

const resetKeybinds = () => {
  draftSettings.value.keybinds = {
    enabled: true,
    showHints: true,
    profile: 'default'
  };
};

// 使用原生对话框提升可信度，非 Tauri 环境回退到浏览器确认框。
const confirmDataAction = async (message: string, title: string) => {
  if (isTauri()) {
    return ask(message, {
      title,
      kind: 'warning',
      okLabel: t('settings.confirm'),
      cancelLabel: t('settings.cancel')
    });
  }
  return window.confirm(message);
};

const handlePurgeTerminalFriends = async () => {
  const workspaceId = currentWorkspace.value?.id;
  if (!workspaceId) {
    pushToast(t('settings.terminalFriends.noWorkspace'), { tone: 'warning' });
    return;
  }
  if (purgingTerminalFriends.value) {
    return;
  }
  const confirmMessage = t('settings.terminalFriends.confirmCurrent');
  const confirmed = await confirmDataAction(confirmMessage, t('settings.terminalFriends.title'));
  if (!confirmed) {
    return;
  }
  purgingTerminalFriends.value = true;
  try {
    const result = await purgeProjectTerminalMembers(workspaceId, 'current');
    if (result.warnings?.length) {
      console.warn('Terminal friend purge warnings.', result.warnings);
    }
    if (currentWorkspace.value) {
      await hydrate();
    }
    await loadSession();
    const message = t('settings.terminalFriends.resultCurrent', { count: result.totalRemoved });
    pushToast(message, { tone: 'success' });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    pushToast(message || t('settings.terminalFriends.failed'), { tone: 'error' });
  } finally {
    purgingTerminalFriends.value = false;
  }
};

// 修复聊天数据库后需刷新会话，避免 UI 与数据不一致。
const handleRepairChatDb = async () => {
  const workspaceId = currentWorkspace.value?.id;
  if (!workspaceId || repairingChatDb.value) return;
  const confirmed = await confirmDataAction(
    t('settings.dataRepairConfirm'),
    t('settings.dataRepairTitle')
  );
  if (!confirmed) return;
  repairingChatDb.value = true;
  dataActionMessage.value = null;
  try {
    const result = await repairChatMessages(workspaceId);
    await loadSession();
    dataActionMessage.value = t('settings.dataRepairResult', { count: result.removedMessages });
  } catch (error) {
    console.error('Failed to repair chat database.', error);
    dataActionMessage.value = t('settings.dataActionFailed');
  } finally {
    repairingChatDb.value = false;
  }
};

// 清空聊天数据库后需重建会话与提示统计结果。
const handleClearChatDb = async () => {
  const workspaceId = currentWorkspace.value?.id;
  if (!workspaceId || clearingChatDb.value) return;
  const confirmed = await confirmDataAction(
    t('settings.dataClearConfirm'),
    t('settings.dataClearTitle')
  );
  if (!confirmed) return;
  clearingChatDb.value = true;
  dataActionMessage.value = null;
  try {
    const result = await clearAllChatMessages(workspaceId);
    resetChat();
    await loadSession();
    dataActionMessage.value = t('settings.dataClearResult', {
      messages: result.removedMessages,
      attachments: result.removedAttachments
    });
  } catch (error) {
    console.error('Failed to clear chat database.', error);
    dataActionMessage.value = t('settings.dataActionFailed');
  } finally {
    clearingChatDb.value = false;
  }
};

const memberOptions = computed<MemberDisplayOption[]>(() => {
  const baseOptions = BASE_TERMINALS.value.map((member, index) => ({
    id: member.id,
    label: resolveBaseTerminalLabel(member, t),
    command: member.command,
    kindLabel: t('settings.memberKind.default'),
    icon: member.icon,
    iconKind: member.id === 'opencode' ? 'opencode' : member.id === 'qwen-code' ? 'qwen' : 'material',
    swatch: member.swatch,
    isCustom: false,
    isDeletable: false,
    group: 0 as const,
    index
  }));

  const customOptions = draftSettings.value.members.customMembers.map((member, index) => ({
    id: member.id,
    label: member.name || t('settings.memberOptions.custom'),
    command: member.command,
    kindLabel: t('settings.memberKind.custom'),
    icon: CUSTOM_TERMINAL_ICON,
    iconKind: 'material',
    isCustom: true,
    isDeletable: true,
    group: 1 as const,
    index
  }));

  return [...baseOptions, ...customOptions];
});

const sectionButtonClass = (section: SectionId) => [
  'relative w-full text-left flex items-center gap-3 px-3 py-2.5 rounded-lg border border-transparent transition group justify-center md:justify-start md:gap-3 gap-0',
  activeSection.value === section
    ? "bg-primary/15 text-text border-primary/30 shadow-[0_1px_3px_rgba(0,0,0,0.18)] ring-1 ring-primary/30 before:content-[''] before:absolute before:left-1 before:top-2 before:bottom-2 before:w-1 before:rounded-full before:bg-primary"
    : 'text-text/60 hover:text-text hover:bg-overlay/5 hover:border-border/10'
];

// Previews use the real tokens and follow the chosen accent.
const themePreview = computed<Record<AppTheme, { base: string; panel: string; accent: string }>>(() => {
  const accent = ACCENTS.find((item) => item.id === currentAccent.value) ?? ACCENTS[0];
  const dark = { base: 'oklch(0.155 0.006 255)', panel: 'oklch(0.215 0.008 255)', accent: `oklch(${accent.dark.primary})` };
  const light = { base: 'oklch(0.985 0.003 255)', panel: 'oklch(0.945 0.005 255)', accent: `oklch(${accent.light.primary})` };
  const split = (a: string, b: string) => `linear-gradient(135deg, ${a} 0%, ${a} 50%, ${b} 50%, ${b} 100%)`;
  return {
    dark,
    light,
    system: { base: split(dark.base, light.base), panel: split(dark.panel, light.panel), accent: split(dark.accent, light.accent) }
  };
});

// 平滑滚动到指定区域，期间暂时锁定自动高亮。
const scrollToSection = (section: SectionId) => {
  const sectionMap: Record<SectionId, HTMLElement | null> = {
    account: accountRef.value,
    appearance: appearanceRef.value,
    language: languageRef.value,
    members: membersRef.value,
    notifications: notificationsRef.value,
    keybinds: keybindsRef.value,
    data: dataRef.value
  };
  const target = sectionMap[section];
  if (!contentRef.value || !target) return;
  activeSection.value = section;
  const containerTop = contentRef.value.getBoundingClientRect().top;
  const targetTop = target.getBoundingClientRect().top;
  const top = targetTop - containerTop + contentRef.value.scrollTop;
  isAutoScrolling.value = true;
  targetScrollTop.value = top;
  if (autoScrollTimeoutId.value !== null) {
    window.clearTimeout(autoScrollTimeoutId.value);
  }
  contentRef.value.scrollTo({ top, behavior: 'smooth' });
  // 800ms 为滚动动画的保守结束时间，避免过早解除锁定。
  autoScrollTimeoutId.value = window.setTimeout(() => {
    isAutoScrolling.value = false;
    targetScrollTop.value = null;
    autoScrollTimeoutId.value = null;
  }, 800);
};

// 根据滚动位置估算当前聚焦分区，避免轻微滚动抖动。
const updateActiveSection = () => {
  if (!contentRef.value) return;
  const container = contentRef.value;
  const containerTop = container.getBoundingClientRect().top;
  // 0.35 与 200 用于限定视线焦点区，避免标题刚进入视口就被选中。
  const focusOffset = Math.min(container.clientHeight * 0.35, 200);
  const focusLine = container.scrollTop + focusOffset;
  const bottomDistance = container.scrollHeight - (container.scrollTop + container.clientHeight);
  let nextSection = sectionRefs[0]?.id ?? activeSection.value;
  let maxTop = -Infinity;

  // 距离底部 80px 内强制落在最后分区，解决尾部无法选中问题。
  if (bottomDistance <= 80) {
    const lastSection = sectionRefs[sectionRefs.length - 1]?.id;
    if (lastSection) {
      activeSection.value = lastSection;
      return;
    }
  }

  sectionRefs.forEach(({ id, ref }) => {
    if (!ref.value) return;
    const top = ref.value.getBoundingClientRect().top - containerTop + container.scrollTop;
    if (top <= focusLine && top >= maxTop) {
      maxTop = top;
      nextSection = id;
    }
  });

  if (nextSection !== activeSection.value) {
    activeSection.value = nextSection;
  }
};

// 自动滚动期间通过目标位置判断结束，避免误触发手动滚动逻辑。
const handleContentScroll = () => {
  if (!contentRef.value) return;
  if (!isAutoScrolling.value) {
    updateActiveSection();
    return;
  }
  if (targetScrollTop.value !== null) {
    const distance = Math.abs(contentRef.value.scrollTop - targetScrollTop.value);
    if (distance <= 2) {
      isAutoScrolling.value = false;
      targetScrollTop.value = null;
      if (autoScrollTimeoutId.value !== null) {
        window.clearTimeout(autoScrollTimeoutId.value);
        autoScrollTimeoutId.value = null;
      }
      return;
    }
  }
  if (autoScrollTimeoutId.value !== null) {
    window.clearTimeout(autoScrollTimeoutId.value);
  }
  // 120ms 用于兜底结束自动滚动状态，避免卡死在锁定态。
  autoScrollTimeoutId.value = window.setTimeout(() => {
    isAutoScrolling.value = false;
    targetScrollTop.value = null;
    autoScrollTimeoutId.value = null;
  }, 120);
};

const handleOverlayMenuOutside = (event: MouseEvent) => {
  const target = event.target as Node;
  if (avatarMenuOpen.value) {
    if (avatarMenuRef.value?.contains(target) || avatarButtonRef.value?.contains(target)) {
      return;
    }
    avatarMenuOpen.value = false;
  }
};

onMounted(() => {
  if (contentRef.value) {
    contentRef.value.addEventListener('scroll', handleContentScroll, { passive: true });
    updateActiveSection();
  }
  document.addEventListener('mousedown', handleOverlayMenuOutside);
  void loadTerminalEnvironments();
});

onBeforeUnmount(() => {
  if (autoScrollTimeoutId.value !== null) {
    window.clearTimeout(autoScrollTimeoutId.value);
  }
  contentRef.value?.removeEventListener('scroll', handleContentScroll);
  document.removeEventListener('mousedown', handleOverlayMenuOutside);
});
</script>

<style scoped>
.member-icon-opencode {
  font-family: 'Rajdhani', 'Orbitron', 'Share Tech Mono', 'Fira Mono', 'Consolas', monospace;
  font-size: 8.5px;
  font-weight: 700;
  letter-spacing: 0.4px;
  line-height: 1;
  text-transform: lowercase;
  white-space: nowrap;
  color: rgba(255, 255, 255, 0.92);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.35);
  user-select: none;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1px;
  transform: translateY(1px);
}

.member-icon-qwen {
  position: relative;
  width: 30px;
  height: 30px;
  filter: drop-shadow(0 3px 6px rgba(0, 0, 0, 0.35));
}

.member-icon-qwen svg {
  width: 100%;
  height: 100%;
  display: block;
}
</style>

