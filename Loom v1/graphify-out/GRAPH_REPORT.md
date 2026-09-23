# Graph Report - Loom v1  (2026-09-22)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3392 nodes · 8125 edges · 165 communities (143 shown, 22 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 327 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8b68a141`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- session/mod.rs
- app.rs
- Settings.vue
- ChatInterface.vue
- pty.rs
- terminalBridge.ts
- TerminalMessagePayload
- StorageManager
- NotificationBadgeState
- TerminalWorkspace.vue
- orchestrate_chat_dispatch
- DiagnosticsState
- projectStore.ts
- CommandCenter
- context-menu/controller.ts
- FriendsView.vue
- chatStore.ts
- ChatInput.vue
- chat/types.ts
- logger.ts
- TerminalSession
- chat_db/types.rs
- open_db
- prompt_block.rs
- MemberRow.vue
- settingsStore.ts
- terminalSnapshotAuditStore.ts
- store.rs
- avatar.ts
- MessagesList.vue
- TerminalPane.vue
- ChatDbManager
- SkillManagementModal.vue
- terminal.rs
- ScheduledTrigger
- message_service/project_members.rs
- chat_dispatch_batcher.rs
- keyboard/controller.ts
- ui_gateway/commands.rs
- SidebarNav.vue
- NotificationPreview.vue
- InviteAssistantModal.vue
- App.vue
- useFriendInvites.ts
- ChatSidebar.vue
- SessionPollSnapshot
- message.rs
- devDependencies
- dependencies
- permissions
- compilerOptions
- cloneSettings
- tauri.conf.json
- SkillStore.vue
- models.rs
- chatStorage.ts
- RoadmapModal.vue
- applySnapshotPayload
- useAppKeybinds.ts
- WeztermEmulator
- SemanticState
- globalStore.ts
- openMemberTerminalById
- dispatch_poll_actions
- snapshot_service.rs
- spawn_semantic_worker
- MemberStatusDots.vue
- TerminalSnapshotAuditReportModal.vue
- applyActiveState
- emulator.rs
- tauri/avatars.ts
- TerminalEventPort
- Arc
- emit
- onPointerUp
- index.ts
- default_members/registry.rs
- handle_trigger_event
- scripts
- InviteFriendsModal.vue
- avatarRender.ts
- .new
- RuleMask
- theme.ts
- refreshFindResults
- maybeLogPassiveSnapshotTriplet
- passiveMonitor.ts
- focusTabInPane
- WorkspaceSelection.vue
- chat_outbox_enqueue
- terminal_event.rs
- chat_list_conversations
- write_project_data
- UiTerminalSessionRepository
- after_terminal_friend_create
- project_data_write
- skillsBridge.ts
- memberSelection.ts
- AppState
- syncDefaultMemberIndex
- attachBeepTraceListeners
- terminal_session_upsert
- .prettierrc.json
- confirmDataAction
- closeEmojiPanel
- handleAvatarUpload
- attachClipboardHandlers
- terminalErrors.ts
- NoopTerminalSessionRepository
- keyboard_input.rs
- TriggerPlan
- package.json
- ChatHeader.vue
- RenameConversationModal.vue
- insertEmoji
- emoji-data.ts
- formatSkillPath
- generate_prompt
- removeMention
- env.d.ts
- removeSkillFolder
- handleRemoveProjectSkill
- openSkillFolder
- loadProjectSkillLinks
- shim.rs
- eslint-config-prettier
- globals
- postcss
- tailwindcss
- @tauri-apps/cli
- typescript
- filteredEmojis
- NotificationPreviewItem
- loom
- CommandResultPayload
- VecDeque
- CommandResultPayload
- CommandResultPayload
- AtomicUsize
- HashSet
- Sender
- TerminalType
- FnOnce
- T
- TerminalEnvironmentOption

## God Nodes (most connected - your core abstractions)
1. `ChatDbManager` - 80 edges
2. `lock_sessions()` - 53 edges
3. `TerminalManager` - 46 edges
4. `DiagnosticsState` - 41 edges
5. `logDiagnosticsEvent()` - 37 edges
6. `open_db()` - 35 edges
7. `NotificationBadgeState` - 33 edges
8. `spawn_pty_processor()` - 31 edges
9. `useChatStore` - 31 edges
10. `diagnostics_log_backend_event()` - 29 edges

## Surprising Connections (you probably didn't know these)
- `statusOptionsFor()` --calls--> `hasTerminalConfig()`  [EXTRACTED]
  src/features/chat/FriendsView.vue → src/shared/utils/terminal.ts
- `models` --calls--> `resolveBaseTerminalLabel()`  [EXTRACTED]
  src/features/chat/modals/InviteAssistantModal.vue → src/shared/constants/terminalCatalog.ts
- `accountAvatar` --calls--> `ensureAvatar()`  [EXTRACTED]
  src/shared/components/SidebarNav.vue → src/shared/utils/avatar.ts
- `canOpenTerminal` --calls--> `hasTerminalConfig()`  [EXTRACTED]
  src/features/chat/components/MemberRow.vue → src/shared/utils/terminal.ts
- `maybe_step_post_ready()` --calls--> `send_post_ready_input()`  [INFERRED]
  src-tauri/src/terminal_engine/session/post_ready.rs → src-tauri/src/terminal_engine/session/keyboard_input.rs

## Import Cycles
- None detected.

## Communities (165 total, 22 thin omitted)

### Community 0 - "session/mod.rs"
Cohesion: 0.06
Nodes (122): Any, AtomicBool, AtomicUsize, FactEvent, HashSet, MutexGuard, PostReadyMode, Receiver (+114 more)

### Community 1 - "app.rs"
Cohesion: 0.09
Nodes (96): App, FnOnce, Monitor, NotificationOpenTerminalPayload, is_main_window_label(), run(), apply_main_window_size(), apply_windows_rounding() (+88 more)

### Community 2 - "Settings.vue"
Cohesion: 0.02
Nodes (81): accountRef, activeKeybindProfile, activeSection, appearanceRef, autoScrollTimeoutId, autoTerminalOption, AVATAR_EXTENSION_BY_MIME, avatarButtonRef (+73 more)

### Community 3 - "ChatInterface.vue"
Cohesion: 0.03
Nodes (79): accountDisplayName, activeConversation, activeConversationId, activeDirectMember, activeMessages, ActiveModal, activePaging, appendMention() (+71 more)

### Community 4 - "pty.rs"
Cohesion: 0.07
Nodes (81): ChildKiller, CommandBuilder, MasterPty, build_command_spec(), candidate_names(), CommandSpec, common_binary_dirs(), default_shell_spec() (+73 more)

### Community 5 - "terminalBridge.ts"
Cohesion: 0.04
Nodes (78): handleMemberTest(), handleTerminalTest(), waitForTerminalWindowReady(), openTerminalWindow(), TerminalWindowOptions, TerminalWindowResult, ackBuffers, ackEncoder (+70 more)

### Community 6 - "TerminalMessagePayload"
Cohesion: 0.06
Nodes (54): Option, String, TerminalCursorPayload, TerminalMessageMeta, TerminalMessagePayload, plan_terminal(), Result, String (+46 more)

### Community 7 - "StorageManager"
Cohesion: 0.08
Nodes (61): ProjectInviteMembersRequest, ProjectInviteMembersResult, ProjectPurgeTerminalMembersResult, normalize_terminal_type(), project_invite_members(), project_purge_terminal_members(), ProjectInviteMembersRequest, ProjectInviteMembersResult (+53 more)

### Community 8 - "NotificationBadgeState"
Cohesion: 0.10
Nodes (42): Image, JoinHandle, Rect, build_pulse_avatar_icon(), ensure_preview_window(), hide_preview_window(), is_main_window_label(), load_transparent_icon() (+34 more)

### Community 9 - "TerminalWorkspace.vue"
Cohesion: 0.03
Nodes (59): backendMembers, bulkOpenState, closeTabSearch(), { currentWorkspace }, dragCaptureTarget, dragGhostHeight, dragGhostLeft, dragGhostStyle (+51 more)

### Community 10 - "orchestrate_chat_dispatch"
Cohesion: 0.07
Nodes (52): ChatDispatchMentions, ChatDispatchMentions, ChatDispatchPayload, Option, String, Vec, compute_backoff_ms(), dispatch_outbox_task() (+44 more)

### Community 11 - "DiagnosticsState"
Cohesion: 0.12
Nodes (46): FrontendLogEntry, diagnostics_end_run(), diagnostics_log_backend_event(), diagnostics_log_chat_consistency(), diagnostics_log_frontend_batch(), diagnostics_log_frontend_event(), diagnostics_log_snapshot_triplet(), diagnostics_register_conversation() (+38 more)

### Community 12 - "projectStore.ts"
Cohesion: 0.06
Nodes (54): useWorkspaceBootstrap(), ALLOWED_MEMBER_STATUSES, buildDefaultProjectData(), formatError(), normalizeManualStatus(), normalizeMembers(), normalizeMemberSequence(), normalizeMemberStatus() (+46 more)

### Community 13 - "CommandCenter"
Cohesion: 0.07
Nodes (47): CommandCenter, Condvar, LocalSocketStream, execute_terminal_command(), parse_arrow_command(), parse_send_command(), parse_terminal_command(), AppHandle (+39 more)

### Community 14 - "context-menu/controller.ts"
Cohesion: 0.08
Nodes (46): untrackSession(), closeTabsWithCleanup(), closeTabWithCleanup(), registerTerminalTabContextMenu(), resolveTabId(), useTerminalStore, handleCloseTab(), handleEntry() (+38 more)

### Community 15 - "FriendsView.vue"
Cohesion: 0.05
Nodes (52): emit, positionClass, props, { t }, baseStatusOptions, buildFriendMember(), chatStore, closeFriendManage() (+44 more)

### Community 16 - "chatStore.ts"
Cohesion: 0.07
Nodes (50): ChatClearResult, ChatDispatchMentions, ChatDispatchRequest, ChatHomeFeed, ChatMessageCreatedPayload, ChatMessageListener, chatMessageListeners, ChatMessageStatusListener (+42 more)

### Community 17 - "ChatInput.vue"
Cohesion: 0.04
Nodes (44): activeEmojiGroup, activeMentionIndex, cursorIndex, EMOJI_GROUP_LABEL_KEYS, EMOJI_INDEX, emojiButtonRef, emojiEmptyMessage, emojiGridRef (+36 more)

### Community 18 - "chat/types.ts"
Cohesion: 0.07
Nodes (39): friends, props, { t }, groupedItems, ALLOWED_ROLES, ALLOWED_STATUSES, loadContacts(), normalizeContact() (+31 more)

### Community 19 - "logger.ts"
Cohesion: 0.11
Nodes (42): diagnosticsEndRun(), diagnosticsLogChatConsistency(), diagnosticsLogFrontendBatch(), diagnosticsLogFrontendEvent(), diagnosticsLogSnapshotTriplet(), diagnosticsRegisterConversation(), diagnosticsRegisterMember(), diagnosticsRegisterSession() (+34 more)

### Community 20 - "TerminalSession"
Cohesion: 0.09
Nodes (27): is_output_rate_stable(), is_output_stable(), record_output_sample(), DispatchQueueItem, OutputRateSample, PostReadyAction, PostReadyMode, PostReadyQueueItem (+19 more)

### Community 21 - "chat_db/types.rs"
Cohesion: 0.12
Nodes (36): From, attachment_index_entry(), AttachmentIndexMeta, ChatClearResult, ChatDeleteMemberConversationsResult, ChatHomeFeedDto, ChatMessage, ChatMessageCreatedPayload (+28 more)

### Community 22 - "open_db"
Cohesion: 0.20
Nodes (42): decode(), encode(), open_db(), parse_ulid(), T, chat_append_terminal_message(), chat_clear_all_messages(), chat_clear_conversation() (+34 more)

### Community 23 - "prompt_block.rs"
Cohesion: 0.12
Nodes (33): apply_snapshot(), String, apply_snapshot(), String, resolve_profile(), extract_bullet_block(), extract_bullet_block_before_prompt(), extract_last_bullet_block() (+25 more)

### Community 24 - "MemberRow.vue"
Cohesion: 0.06
Nodes (35): displayMembersForTitle, friendEntries, displayMembers, baseStatusOptions, canMention, canOpenTerminal, canRemove, canRename (+27 more)

### Community 25 - "settingsStore.ts"
Cohesion: 0.08
Nodes (38): AccountSettings, ALLOWED_KEYBINDS, ALLOWED_LOCALES, ALLOWED_STATUSES, ALLOWED_THEMES, AppearanceSettings, applySettingsEffects(), buildCustomMembers() (+30 more)

### Community 26 - "terminalSnapshotAuditStore.ts"
Cohesion: 0.09
Nodes (37): memberOptions, snapshotSessionLines(), TERMINAL_OPEN_TAB_EVENT, TERMINAL_SNAPSHOT_REQUEST_EVENT, TERMINAL_SNAPSHOT_RESPONSE_EVENT, TERMINAL_TAB_OPENED_EVENT, TERMINAL_WINDOW_READY_EVENT, TERMINAL_WINDOW_READY_REQUEST_EVENT (+29 more)

### Community 27 - "store.rs"
Cohesion: 0.16
Nodes (35): build_conversation_summary(), build_message_preview(), clear_chat_storage(), compute_conversation_unread_count(), compute_total_unread_count(), compute_workspace_unread_summary(), count_unread_messages(), db_path() (+27 more)

### Community 28 - "avatar.ts"
Cohesion: 0.10
Nodes (34): accountAvatar, accountAvatar, isCustomAvatar, selectedAvatarId, selectedLocalAvatarId, avatarVars, isCss, label (+26 more)

### Community 29 - "MessagesList.vue"
Cohesion: 0.07
Nodes (29): buildMessageTokens(), emit, ensureTypewriter(), getMessageTime(), handleAvatarClick(), handleJumpToLatest(), hasMore, isLoadingMore (+21 more)

### Community 30 - "TerminalPane.vue"
Cohesion: 0.06
Nodes (33): applySnapshotCursor(), attachPhase, fatalError, findCaseSensitive, findInputRef, findInvalidRegex, findOpen, findQuery (+25 more)

### Community 31 - "ChatDbManager"
Cohesion: 0.17
Nodes (34): chat_clear_all_messages(), chat_clear_conversation(), chat_create_group(), chat_delete_conversation(), chat_delete_member_conversations(), chat_ensure_direct(), chat_get_messages(), chat_list_conversations() (+26 more)

### Community 32 - "SkillManagementModal.vue"
Cohesion: 0.06
Nodes (30): activeTab, availableProjectSkills, canOpenSkillPath, channelLabel, { currentSkills }, { currentWorkspace, workspaceReadOnly }, { defaultChannelName }, emit (+22 more)

### Community 33 - "terminal.rs"
Cohesion: 0.17
Nodes (33): find_repo_root(), resolve_log_dir(), Option, Path, PathBuf, AppHandle, Option, Result (+25 more)

### Community 34 - "ScheduledTrigger"
Cohesion: 0.13
Nodes (20): BinaryHeap, Eq, Ord, Ordering, PartialEq, PartialOrd, DeferredStage, FactEvent (+12 more)

### Community 35 - "message_service/project_members.rs"
Cohesion: 0.17
Nodes (29): Map, build_member_base_name(), build_seeded_avatar(), default_avatar(), ensure_default_owner(), ensure_member_sequence(), extract_member_names(), hash_seed() (+21 more)

### Community 36 - "chat_dispatch_batcher.rs"
Cohesion: 0.14
Nodes (23): Duration, can_merge_context(), ChatDispatchBatcher, dispatch_batch(), DispatchBatch, DispatchQueue, has_message_id_conflict(), restore_failed_dispatch() (+15 more)

### Community 37 - "keyboard/controller.ts"
Cohesion: 0.13
Nodes (27): buildKeybindContext(), getInputSelectionText(), handleKeydownEvent(), isComboMatch(), isItemEnabled(), isMacOS(), matchesScope(), normalizeComboInput() (+19 more)

### Community 38 - "ui_gateway/commands.rs"
Cohesion: 0.11
Nodes (27): Fn, Invoke, export_commands(), Send, Sync, project_skills_link(), project_skills_list(), project_skills_unlink() (+19 more)

### Community 39 - "SidebarNav.vue"
Cohesion: 0.08
Nodes (24): AccountStatus, accountAvatar, accountStatus, activeTab, chatStore, { currentWorkspace }, emit, emitChange() (+16 more)

### Community 40 - "NotificationPreview.vue"
Cohesion: 0.11
Nodes (22): canOpenAnyTerminal, canOpenTerminal(), handleHover(), handleIgnoreAll(), handleOpenAllTerminals(), handleOpenConversation(), handleOpenTerminal(), handleViewAll() (+14 more)

### Community 41 - "InviteAssistantModal.vue"
Cohesion: 0.08
Nodes (25): canDecreaseInstances, canIncreaseInstances, clampInstances(), commitInstanceInput(), emit, emitInvite(), handleInstanceInput(), instances (+17 more)

### Community 42 - "App.vue"
Cohesion: 0.08
Nodes (23): armPointerReadyReset(), consumeWorkspaceBoot(), contextTitle, handleClose(), { handleContextMenuEvent }, { handleKeydownEvent }, isFocused, isMacOS (+15 more)

### Community 43 - "useFriendInvites.ts"
Cohesion: 0.13
Nodes (22): debugLog, handleMemberAction(), handleMessageAvatarOpen(), terminalMemberIds, InviteModalType, InviteModel, InviteRole, InviteRoleConfig (+14 more)

### Community 44 - "ChatSidebar.vue"
Cohesion: 0.10
Nodes (20): channelAvatarStyle(), channelItems, containerRef, directMessageItems, emit, getConversationTitle(), getLastMessagePreview(), handleAction() (+12 more)

### Community 45 - "SessionPollSnapshot"
Cohesion: 0.12
Nodes (17): ChatFlushMeta, PollAction, Option, Self, String, SessionUpdate, build_snapshot(), collect_post_ready_actions() (+9 more)

### Community 46 - "message.rs"
Cohesion: 0.23
Nodes (26): chat_clear_all_messages(), chat_clear_conversation(), chat_create_group(), chat_delete_conversation(), chat_ensure_direct(), chat_get_messages(), chat_list_conversations(), chat_mark_conversation_read_latest() (+18 more)

### Community 47 - "devDependencies"
Cohesion: 0.08
Nodes (25): autoprefixer, eslint, @eslint/js, eslint-plugin-vue, devDependencies, autoprefixer, eslint, @eslint/js (+17 more)

### Community 48 - "dependencies"
Cohesion: 0.08
Nodes (25): dependencies, pinia, @tauri-apps/api, @tauri-apps/plugin-clipboard-manager, @tauri-apps/plugin-dialog, @tauri-apps/plugin-shell, vue, vue-i18n (+17 more)

### Community 49 - "permissions"
Cohesion: 0.08
Nodes (24): clipboard-manager:allow-read-text, clipboard-manager:allow-write-text, core:default, core:window:allow-close, core:window:allow-destroy, core:window:allow-is-maximized, core:window:allow-maximize, core:window:allow-minimize (+16 more)

### Community 50 - "compilerOptions"
Cohesion: 0.08
Nodes (24): DOM, DOM.Iterable, ES2022, node, src/**/*.d.ts, src/**/*.ts, src/**/*.vue, vite/client (+16 more)

### Community 51 - "cloneSettings"
Cohesion: 0.11
Nodes (24): cloneSettings(), applyCustomTerminal(), applyDefaultTerminalSelection(), buildPersistableSettings(), handleMemberRefresh(), isTerminalOptionSelected(), normalizeTerminalPathKey(), openCustomTerminalForm() (+16 more)

### Community 52 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 53 - "SkillStore.vue"
Cohesion: 0.09
Nodes (19): createLibrarySkills(), librarySkillSeed, SkillLibraryItem, canOpenSkillPath, confirmRemoveFolder(), filters, globalStore, handleRemoveFolder() (+11 more)

### Community 54 - "models.rs"
Cohesion: 0.15
Nodes (16): Option, String, Vec, TerminalErrorPayload, TerminalExitPayload, TerminalOutputPayload, TerminalSnapshotLinesPayload, TerminalSnapshotPayload (+8 more)

### Community 55 - "chatStorage.ts"
Cohesion: 0.14
Nodes (19): flushCacheSaves(), scheduleCacheSave(), chatCachePath(), chatDataPath(), ChatSessionCache, conversationMessagesDir(), conversationMessagesPath(), encodeConversationId() (+11 more)

### Community 56 - "RoadmapModal.vue"
Cohesion: 0.10
Nodes (18): completion, editingId, editValue, emit, handleAddTask(), handleDelete(), listRef, objectiveValue (+10 more)

### Community 57 - "applySnapshotPayload"
Cohesion: 0.17
Nodes (22): attachSession(), applyRendererMode(), applySnapshotPayload(), attachWithRecovery(), attemptAttach(), collectXtermMeta(), countSnapshotLines(), debugLog (+14 more)

### Community 58 - "useAppKeybinds.ts"
Cohesion: 0.14
Nodes (19): AppKeybindOptions, findVisibleTarget(), focusByRole(), isVisibleElement(), registerAppKeybinds(), resolveScopeRoot(), ScopeRoot, waitForNextFrame() (+11 more)

### Community 59 - "WeztermEmulator"
Cohesion: 0.13
Nodes (10): ColorPalette, Screen, Vec, serialize_screen_to_ansi(), serialize_screen_to_ansi_segments(), SnapshotSegments, WeztermConfig, WeztermEmulator (+2 more)

### Community 60 - "SemanticState"
Cohesion: 0.19
Nodes (14): FilterRuntime, Self, build_semantic_payload(), extract_command_from_input(), extract_input_lines(), next_chat_seq(), Box, Option (+6 more)

### Community 61 - "globalStore.ts"
Cohesion: 0.15
Nodes (15): buildDefaultGlobalData(), formatError(), GlobalData, ImportedSkillFolder, normalizeGlobalData(), normalizeImportedSkillFolders(), useGlobalStore, activeTab (+7 more)

### Community 62 - "openMemberTerminalById"
Cohesion: 0.18
Nodes (19): trackSession(), beginBulkOpen(), buildMemberTerminalId(), clearBulkOpenState(), handleDismissRecentClosedTab(), handleNewTab(), handleOpenRecentClosedTab(), handleTabSearchMemberSelect() (+11 more)

### Community 63 - "dispatch_poll_actions"
Cohesion: 0.19
Nodes (16): default_terminal_session_repository(), Arc, Send, Sync, TerminalSessionRepository, apply_session_update(), ChatFlushLog, dispatch_poll_actions() (+8 more)

### Community 64 - "snapshot_service.rs"
Cohesion: 0.21
Nodes (18): is_code_fence_line(), is_hard_break_line(), is_indented_code_line(), is_list_line(), is_numbered_list_line(), is_paragraph_start_line(), is_prompt_line(), is_separator_line() (+10 more)

### Community 65 - "spawn_semantic_worker"
Cohesion: 0.17
Nodes (15): Instant, Send, Sync, TerminalSettingsPort, maybe_emit_stream(), resolve_chat_stream_enabled(), AppHandle, Arc (+7 more)

### Community 66 - "MemberStatusDots.vue"
Cohesion: 0.12
Nodes (17): anchorRef, baseManualOptions, borderClass, handleEnter(), handleLeave(), manualDotClass, manualOptions, props (+9 more)

### Community 67 - "TerminalSnapshotAuditReportModal.vue"
Cohesion: 0.11
Nodes (9): CallChain, CallChainStep, StepStatus, emit, snapshotAuditStatusLabel, SnapshotSignature, {
  status: terminalSnapshotAuditStatus,
  results: terminalSnapshotAuditResults,
  lastError: terminalSnapshotAuditError,
  callChains: terminalSnapshotAuditCallChains
}, { t } (+1 more)

### Community 68 - "applyActiveState"
Cohesion: 0.16
Nodes (18): setSessionActive(), subscribeError(), subscribeExit(), writeSession(), ackWrittenData(), applyActiveState(), attachOutput(), detachOutput() (+10 more)

### Community 69 - "emulator.rs"
Cohesion: 0.25
Nodes (15): Blink, CellAttributes, ColorAttribute, Intensity, Line, AttrState, emit_attr_delta(), emit_sgr() (+7 more)

### Community 70 - "tauri/avatars.ts"
Cohesion: 0.15
Nodes (15): clampAvatarMenu(), loadCustomAvatars(), positionAvatarMenu(), removeCustomAvatar(), resetAvatar(), selectAvatarPreset(), toggleAvatarMenu(), AvatarAsset (+7 more)

### Community 71 - "TerminalEventPort"
Cohesion: 0.30
Nodes (15): Send, Sync, TerminalEventPort, finish_post_ready(), maybe_start_post_ready(), maybe_step_post_ready(), queue_post_ready_steps(), Arc (+7 more)

### Community 72 - "Arc"
Cohesion: 0.18
Nodes (8): default_terminal_dispatch_gate(), NoopTerminalDispatchGate, AppHandle, Arc, Send, Sync, TerminalDispatchGate, Arc

### Community 73 - "emit"
Cohesion: 0.19
Nodes (14): applyMention(), applyPrompt(), buildSendPayload(), dropTrailingSpace(), emit, emitSend(), getCaretCoordinates(), handleInput() (+6 more)

### Community 74 - "onPointerUp"
Cohesion: 0.22
Nodes (14): cleanupDragListeners(), cleanupDragState(), isPointInTabBar(), onPointerDown(), onPointerMove(), onPointerUp(), resolveInsertTarget(), resolvePaneDropTarget() (+6 more)

### Community 75 - "index.ts"
Cohesion: 0.16
Nodes (9): AppLocale, i18n, initialLocale, LOCALE_CHANGED_EVENT, messages, setDocumentLang(), bootstrapApp(), initializeMonitoring() (+1 more)

### Community 76 - "default_members/registry.rs"
Cohesion: 0.36
Nodes (12): apply_resume_command(), apply_unlimited_access_command(), command_contains_flag(), normalize_command(), resolve_default_command_for_invite(), resolve_default_member(), Option, String (+4 more)

### Community 77 - "handle_trigger_event"
Cohesion: 0.22
Nodes (13): handle_trigger_event(), AppHandle, Arc, Mutex, Option, schedule_post_ready_ticks(), spawn_status_poller(), collect_poll_snapshots_by_ids() (+5 more)

### Community 78 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, dev:backend, dev:frontend, dev:tauri, format, format:check (+5 more)

### Community 79 - "InviteFriendsModal.vue"
Cohesion: 0.17
Nodes (8): confirmInvite(), emit, filteredFriends, props, query, selectedCount, selectedIds, { t }

### Community 80 - "avatarRender.ts"
Cohesion: 0.32
Nodes (12): canvasToPngBytes(), drawLinearBackground(), drawRadialSpot(), drawRing(), normalizeColorStop(), parseLinearGradient(), parsePercent(), parseRadialGradient() (+4 more)

### Community 81 - ".new"
Cohesion: 0.28
Nodes (10): create_emulator(), create_emulator_with_writer(), EmulatorConfig, Box, Option, Self, Send, Write (+2 more)

### Community 82 - "RuleMask"
Cohesion: 0.26
Nodes (6): BitOr, RuleMask, build_poll_actions(), resolve_rules(), Vec, RuleKind

### Community 83 - "theme.ts"
Cohesion: 0.30
Nodes (10): applyThemeToDom(), AppTheme, getSystemTheme(), startSystemListener(), stopSystemListener(), syncTheme(), THEME_CHANGED_EVENT, ThemeOption (+2 more)

### Community 84 - "refreshFindResults"
Cohesion: 0.27
Nodes (12): clearFindHighlights(), closeFind(), handleFindInput(), handleFindKeydown(), handleFindNext(), handleFindPrevious(), isEnterKey(), refreshFindResults() (+4 more)

### Community 85 - "maybeLogPassiveSnapshotTriplet"
Cohesion: 0.24
Nodes (12): collectViewportLines(), emitSnapshotResponse(), isPassiveSnapshotEnabled(), maybeLogPassiveSnapshotTriplet(), prunePassiveSnapshotCache(), readPassiveSnapshotCache(), resolvePassiveSnapshotEntry(), respondSnapshotRequest() (+4 more)

### Community 86 - "passiveMonitor.ts"
Cohesion: 0.24
Nodes (10): endBeepTraceRun(), ensureBeepTraceRun(), isBeepTraceEnabled(), logBeepTrace(), resolveBeepTraceWindowLabel(), safeInvokeBeepTrace(), isFrontendPassiveEnabled(), createFrontLogger() (+2 more)

### Community 87 - "focusTabInPane"
Cohesion: 0.20
Nodes (12): clearTabSearch(), focusTabInPane(), handleTabClick(), handleTabSearchCreate(), handleTabSearchKeydown(), handleTabSearchSelect(), isEnterKey(), isTabAssigned() (+4 more)

### Community 88 - "WorkspaceSelection.vue"
Cohesion: 0.18
Nodes (8): containerRef, errorTimer, filteredMore, formatWorkspacePath(), { recentPrimary, recentMore, workspaceError }, searchQuery, { t, locale }, workspaceStore

### Community 89 - "chat_outbox_enqueue"
Cohesion: 0.44
Nodes (11): chat_outbox_claim_due(), chat_outbox_enqueue(), chat_outbox_mark_failed(), chat_outbox_mark_sent(), insert_schedule_entry(), remove_schedule_entry(), Result, String (+3 more)

### Community 90 - "terminal_event.rs"
Cohesion: 0.27
Nodes (7): default_terminal_event_port(), NoopTerminalEventPort, Arc, Option, Result, String, TerminalErrorPayload

### Community 91 - "chat_list_conversations"
Cohesion: 0.40
Nodes (10): chat_get_conversation_member_ids(), chat_get_messages(), chat_list_conversations(), compute_workspace_unread_summary(), MessageDto, Option, Result, State (+2 more)

### Community 92 - "write_project_data"
Cohesion: 0.42
Nodes (10): project_data_app_path(), ProjectDataReadResult, ProjectDataWriteResult, read_project_data(), Option, Result, String, Value (+2 more)

### Community 93 - "UiTerminalSessionRepository"
Cohesion: 0.27
Nodes (6): AppHandle, Option, Result, Self, String, UiTerminalSessionRepository

### Community 94 - "after_terminal_friend_create"
Cohesion: 0.47
Nodes (9): after_terminal_friend_create(), has_invite_meta(), AppHandle, Option, Result, String, run_post_create_flow(), start_friend_flow() (+1 more)

### Community 95 - "project_data_write"
Cohesion: 0.33
Nodes (8): ProjectDataReadResult, ProjectDataWriteResult, project_data_read(), project_data_write(), AppHandle, Result, String, Value

### Community 96 - "skillsBridge.ts"
Cohesion: 0.22
Nodes (8): closeProjectSkillPicker(), handleImportFolder(), handleLinkProjectSkill(), importSkillFolder(), linkProjectSkill(), ProjectSkillLink, SkillFolderImportResult, handleImportFolder()

### Community 97 - "memberSelection.ts"
Cohesion: 0.33
Nodes (8): resolveDefaultMemberIndex(), selectedMemberIndex, clampMemberSelectionIndex(), DEFAULT_MEMBER_INDEX, isValidGroup(), MemberSelectionIndex, normalizeMemberSelectionIndex(), parseMemberSelectionIndex()

### Community 98 - "AppState"
Cohesion: 0.28
Nodes (7): AppState, Default, HashMap, Mutex, Option, Self, String

### Community 99 - "syncDefaultMemberIndex"
Cohesion: 0.32
Nodes (8): applyCustomMember(), buildCustomMemberId(), removeCustomMember(), resetCustomMemberForm(), selectedMemberId, syncDefaultMemberIndex(), resolveMemberIdFromSelectionIndex(), resolveMemberSelectionIndexFromId()

### Community 100 - "attachBeepTraceListeners"
Cohesion: 0.36
Nodes (8): attachBeepTraceListeners(), classifyKey(), collectFocusSnapshot(), describeElement(), describeEventPath(), describeTextarea(), isBeepTraceVerbose(), shouldLogKeyEvent()

### Community 101 - "terminal_session_upsert"
Cohesion: 0.39
Nodes (6): Option, Result, String, terminal_session_delete_by_member_id(), terminal_session_get_by_member_id(), terminal_session_upsert()

### Community 102 - ".prettierrc.json"
Cohesion: 0.29
Nodes (6): arrowParens, bracketSpacing, printWidth, semi, singleQuote, trailingComma

### Community 103 - "confirmDataAction"
Cohesion: 0.29
Nodes (7): clearAllChatMessages(), repairChatMessages(), confirmDataAction(), handleClearChatDb(), handlePurgeTerminalFriends(), handleRepairChatDb(), purgeProjectTerminalMembers()

### Community 104 - "closeEmojiPanel"
Cohesion: 0.29
Nodes (7): closeEmojiPanel(), handleEmojiSearchKeydown(), handleGlobalPointerDown(), isEnterKey(), openEmojiPanel(), toggleEmojiPanel(), updateEmojiViewport()

### Community 105 - "handleAvatarUpload"
Cohesion: 0.29
Nodes (7): fileToBytes(), handleAvatarUpload(), resolveImageExtension(), selectCustomAvatar(), upsertCustomAvatar(), storeAvatarAsset(), toLocalAvatar()

### Community 106 - "attachClipboardHandlers"
Cohesion: 0.43
Nodes (7): attachClipboardHandlers(), copySelection(), openFind(), pasteClipboardText(), registerTerminalContextMenu(), startPasteHold(), stopPasteHold()

### Community 107 - "terminalErrors.ts"
Cohesion: 0.29
Nodes (6): parseTerminalError(), resolveTerminalErrorI18nKey(), TERMINAL_ERROR_CODES, TerminalErrorCode, TerminalErrorDetail, TerminalErrorPayload

### Community 108 - "NoopTerminalSessionRepository"
Cohesion: 0.48
Nodes (4): NoopTerminalSessionRepository, Option, Result, String

### Community 109 - "keyboard_input.rs"
Cohesion: 0.67
Nodes (6): find_keyword_case_insensitive(), parse_session_id_after_keyword(), parse_session_id_from_line(), parse_session_id_from_lines(), Option, String

### Community 110 - "TriggerPlan"
Cohesion: 0.52
Nodes (6): plan_fact(), plan_trigger(), String, Vec, TriggerPlan, TriggerTargets

### Community 111 - "package.json"
Cohesion: 0.33
Nodes (5): name, packageManager, private, type, version

### Community 112 - "ChatHeader.vue"
Cohesion: 0.33
Nodes (5): emit, headerDescription, headerTitle, props, { t }

### Community 113 - "RenameConversationModal.vue"
Cohesion: 0.33
Nodes (5): draftName, emit, name, props, { t }

### Community 114 - "insertEmoji"
Cohesion: 0.40
Nodes (5): insertEmoji(), loadRecentEmojiIds(), persistRecentEmojiIds(), recordRecentEmoji(), syncRecentEmojiEntries()

### Community 115 - "emoji-data.ts"
Cohesion: 0.40
Nodes (4): EMOJI_DATA, EMOJI_GROUPS, EmojiEntry, EmojiGroup

### Community 116 - "formatSkillPath"
Cohesion: 0.40
Nodes (4): localSkillFolders, projectSkillLinksView, formatSkillPath(), localSkillFolders

### Community 117 - "generate_prompt"
Cohesion: 0.50
Nodes (4): generate_prompt(), PromptType, Option, String

### Community 118 - "removeMention"
Cohesion: 0.50
Nodes (4): escapeRegExp(), focusInput(), removeMention(), removeMentionText()

### Community 120 - "removeSkillFolder"
Cohesion: 0.67
Nodes (3): confirmRemoveFolder(), handleRemoveFolder(), removeSkillFolder()

### Community 121 - "handleRemoveProjectSkill"
Cohesion: 0.67
Nodes (3): confirmRemoveProjectSkill(), handleRemoveProjectSkill(), unlinkProjectSkill()

### Community 122 - "openSkillFolder"
Cohesion: 0.67
Nodes (3): handleOpenSkillPath(), openSkillFolder(), handleOpenSkillPath()

### Community 123 - "loadProjectSkillLinks"
Cohesion: 0.67
Nodes (3): handleSyncProjectSkills(), loadProjectSkillLinks(), listProjectSkillLinks()

## Knowledge Gaps
- **762 isolated node(s):** `TerminalErrorCode`, `TerminalErrorDetail`, `TerminalErrorPayload`, `EmojiGroup`, `ImportMetaEnv` (+757 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `StorageManager` connect `StorageManager` to `orchestrate_chat_dispatch`, `message_service/project_members.rs`, `pty.rs`?**
  _High betweenness centrality (0.036) - this node is a cross-community bridge._
- **Why does `TerminalManager` connect `session/mod.rs` to `spawn_semantic_worker`, `TerminalMessagePayload`, `TerminalEventPort`, `Arc`, `orchestrate_chat_dispatch`, `handle_trigger_event`, `TerminalSession`, `after_terminal_friend_create`, `dispatch_poll_actions`?**
  _High betweenness centrality (0.025) - this node is a cross-community bridge._
- **Why does `orchestrate_chat_dispatch()` connect `orchestrate_chat_dispatch` to `session/mod.rs`, `chat_list_conversations`, `StorageManager`, `ChatDbManager`?**
  _High betweenness centrality (0.022) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `lock_sessions()` (e.g. with `terminal_ack()` and `terminal_attach()`) actually correct?**
  _`lock_sessions()` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `TerminalErrorCode`, `TerminalErrorDetail`, `TerminalErrorPayload` to the rest of the system?**
  _762 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `session/mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06311515748031496 - nodes in this community are weakly interconnected._
- **Should `app.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0858405217757206 - nodes in this community are weakly interconnected._