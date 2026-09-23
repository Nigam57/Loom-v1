# Graph Report - loom  (2026-09-22)

## Corpus Check
- cluster-only mode â€” file stats not available

## Summary
- 3361 nodes Â· 8119 edges Â· 145 communities (136 shown, 9 thin omitted)
- Extraction: 96% EXTRACTED Â· 4% INFERRED Â· 0% AMBIGUOUS Â· INFERRED: 319 edges (avg confidence: 0.85)
- Token cost: 0 input Â· 0 output

## Graph Freshness
- Built from commit: `8b68a141`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Settings.vue
- app.rs
- ChatInterface.vue
- pty.rs
- StorageManager
- session/mod.rs
- terminalSnapshotAuditStore.ts
- TerminalMessagePayload
- DiagnosticsState
- NotificationBadgeState
- TerminalWorkspace.vue
- CommandCenter
- projectStore.ts
- context-menu/controller.ts
- FriendsView.vue
- ChatInput.vue
- TerminalManager
- chatStore.ts
- terminalBridge.ts
- logger.ts
- platform.rs
- prompt_block.rs
- TerminalPane.vue
- ChatSidebar.vue
- SkillManagementModal.vue
- chat_db/types.rs
- open_db
- orchestrate_chat_dispatch
- MemberRow.vue
- settingsStore.ts
- store.rs
- avatar.ts
- ChatDbManager
- MessagesList.vue
- models.rs
- TerminalSession
- ScheduledTrigger
- message_service/project_members.rs
- chat/types.ts
- InviteAssistantModal.vue
- chat_dispatch_batcher.rs
- ui_gateway/commands.rs
- useAppKeybinds.ts
- SidebarNav.vue
- NotificationPreview.vue
- SkillStore.vue
- SessionPollSnapshot
- message.rs
- session/commands.rs
- terminal.rs
- devDependencies
- dependencies
- permissions
- compilerOptions
- App.vue
- cloneSettings
- TerminalEventPort
- snapshot_service.rs
- tauri.conf.json
- chatStorage.ts
- keyboard/controller.ts
- RoadmapModal.vue
- applySnapshotPayload
- openMemberTerminalById
- MemberStatusDots.vue
- globalStore.ts
- emulator.rs
- memberSelection.ts
- TerminalSnapshotAuditReportModal.vue
- WeztermEmulator
- contactsStorage.ts
- tauri/avatars.ts
- applyActiveState
- avatarRender.ts
- notificationOrchestratorStore.ts
- dispatch_poll_actions
- SemanticState
- emit
- onPointerUp
- index.ts
- handle_trigger_event
- scripts
- InviteFriendsModal.vue
- RuleMask
- skillsBridge.ts
- theme.ts
- maybeLogPassiveSnapshotTriplet
- refreshFindResults
- passiveMonitor.ts
- focusTabInPane
- WorkspaceSelection.vue
- chat_outbox_enqueue
- chat_list_conversations
- UiTerminalSessionRepository
- project_data_write
- .new
- .new
- .default
- attachBeepTraceListeners
- terminal_session_upsert
- .prettierrc.json
- closeEmojiPanel
- handleAvatarUpload
- terminalErrors.ts
- NoopTerminalSessionRepository
- TriggerPlan
- package.json
- ChatHeader.vue
- InviteAdminModal.vue
- RenameConversationModal.vue
- insertEmoji
- emoji-data.ts
- emit
- generate_prompt
- removeMention
- env.d.ts
- shim.rs
- eslint-config-prettier
- globals
- tailwindcss
- vite
- vue-eslint-parser
- filteredEmojis
- loom

## God Nodes (most connected - your core abstractions)
1. `ChatDbManager` - 83 edges
2. `TerminalManager` - 67 edges
3. `lock_sessions()` - 53 edges
4. `SessionRegistry` - 43 edges
5. `DiagnosticsState` - 41 edges
6. `logDiagnosticsEvent()` - 37 edges
7. `open_db()` - 35 edges
8. `TerminalSession` - 35 edges
9. `TerminalEventPort` - 34 edges
10. `NotificationBadgeState` - 33 edges

## Surprising Connections (you probably didn't know these)
- `handleMemberTest()` --calls--> `openTerminalWindow()`  [EXTRACTED]
  src/features/Settings.vue â†’ src/features/terminal/openTerminalWindow.ts
- `buildRecentClosedSnapshot()` --calls--> `hasTerminalConfig()`  [EXTRACTED]
  src/features/terminal/TerminalWorkspace.vue â†’ src/shared/utils/terminal.ts
- `recentClosedCandidateTabs` --calls--> `hasTerminalConfig()`  [EXTRACTED]
  src/features/terminal/TerminalWorkspace.vue â†’ src/shared/utils/terminal.ts
- `accountAvatar` --calls--> `ensureAvatar()`  [EXTRACTED]
  src/shared/components/SidebarNav.vue â†’ src/shared/utils/avatar.ts
- `run()` --calls--> `spawn_chat_outbox_worker()`  [INFERRED]
  src-tauri/src/lib.rs â†’ src-tauri/src/orchestration/chat_outbox.rs

## Import Cycles
- None detected.

## Communities (145 total, 9 thin omitted)

### Community 0 - "Settings.vue"
Cohesion: 0.02
Nodes (92): clearAllChatMessages(), repairChatMessages(), accountRef, activeKeybindProfile, activeSection, appearanceRef, autoScrollTimeoutId, autoTerminalOption (+84 more)

### Community 1 - "app.rs"
Cohesion: 0.08
Nodes (102): App, Monitor, NotificationOpenTerminalPayload, is_main_window_label(), run(), AppState, Default, HashMap (+94 more)

### Community 2 - "ChatInterface.vue"
Cohesion: 0.03
Nodes (82): accountDisplayName, activeConversation, activeConversationId, activeDirectMember, activeMessages, ActiveModal, activePaging, appendMention() (+74 more)

### Community 3 - "pty.rs"
Cohesion: 0.07
Nodes (81): ChildKiller, CommandBuilder, MasterPty, build_command_spec(), candidate_names(), CommandSpec, common_binary_dirs(), default_shell_spec() (+73 more)

### Community 4 - "StorageManager"
Cohesion: 0.07
Nodes (70): ProjectInviteMembersRequest, ProjectInviteMembersResult, ProjectPurgeTerminalMembersResult, normalize_terminal_type(), project_invite_members(), project_purge_terminal_members(), ProjectInviteMembersRequest, ProjectInviteMembersResult (+62 more)

### Community 5 - "session/mod.rs"
Cohesion: 0.10
Nodes (73): Any, AtomicBool, MutexGuard, Receiver, add_unacked_bytes(), build_status_payload(), can_dispatch_now(), cleanup_ephemeral_sessions_for_window() (+65 more)

### Community 6 - "terminalSnapshotAuditStore.ts"
Cohesion: 0.05
Nodes (68): ensureChatMessageStatusListener(), ensureChatUnreadListener(), handleTerminalTest(), openTerminalWindow(), TerminalWindowOptions, TerminalWindowResult, closeSession(), createSession() (+60 more)

### Community 7 - "TerminalMessagePayload"
Cohesion: 0.06
Nodes (51): Option, String, TerminalCursorPayload, TerminalMessageMeta, TerminalMessagePayload, plan_terminal(), Result, String (+43 more)

### Community 8 - "DiagnosticsState"
Cohesion: 0.11
Nodes (51): FrontendLogEntry, diagnostics_end_run(), diagnostics_log_backend_event(), diagnostics_log_chat_consistency(), diagnostics_log_frontend_batch(), diagnostics_log_frontend_event(), diagnostics_log_snapshot_triplet(), diagnostics_register_conversation() (+43 more)

### Community 9 - "NotificationBadgeState"
Cohesion: 0.11
Nodes (41): Image, JoinHandle, NotificationPreviewItem, Rect, build_pulse_avatar_icon(), ensure_preview_window(), hide_preview_window(), is_main_window_label() (+33 more)

### Community 10 - "TerminalWorkspace.vue"
Cohesion: 0.03
Nodes (59): backendMembers, buildRecentClosedSnapshot(), bulkOpenState, closeTabSearch(), { currentWorkspace }, dragCaptureTarget, dragGhostHeight, dragGhostLeft (+51 more)

### Community 11 - "CommandCenter"
Cohesion: 0.07
Nodes (48): Condvar, LocalSocketStream, execute_terminal_command(), parse_arrow_command(), parse_send_command(), parse_terminal_command(), AppHandle, CommandResultPayload (+40 more)

### Community 12 - "projectStore.ts"
Cohesion: 0.06
Nodes (49): useWorkspaceBootstrap(), AI_ASSISTANT_ID, CURRENT_USER_ID, InviteModalType, InviteModel, InviteRole, InviteRoleConfig, ALLOWED_MEMBER_STATUSES (+41 more)

### Community 13 - "context-menu/controller.ts"
Cohesion: 0.07
Nodes (47): untrackSession(), closeTabsWithCleanup(), closeTabWithCleanup(), registerTerminalTabContextMenu(), resolveTabId(), registerTerminalContextMenu(), useTerminalStore, handleCloseTab() (+39 more)

### Community 14 - "FriendsView.vue"
Cohesion: 0.05
Nodes (50): terminalMemberIds, emit, positionClass, props, { t }, canOpenTerminal, baseStatusOptions, buildFriendMember() (+42 more)

### Community 15 - "ChatInput.vue"
Cohesion: 0.04
Nodes (44): activeEmojiGroup, activeMentionIndex, cursorIndex, EMOJI_GROUP_LABEL_KEYS, EMOJI_INDEX, emojiButtonRef, emojiEmptyMessage, emojiGridRef (+36 more)

### Community 16 - "TerminalManager"
Cohesion: 0.07
Nodes (33): Instant, Send, Sync, TerminalSettingsPort, default_terminal_dispatch_gate(), NoopTerminalDispatchGate, AppHandle, Arc (+25 more)

### Community 17 - "chatStore.ts"
Cohesion: 0.08
Nodes (46): ChatClearResult, ChatDispatchMentions, ChatDispatchRequest, ChatHomeFeed, ChatMessageCreatedPayload, ChatMessageListener, chatMessageListeners, ChatMessageStatusListener (+38 more)

### Community 18 - "terminalBridge.ts"
Cohesion: 0.06
Nodes (45): ackBuffers, ackEncoder, ackSession(), ActivityListener, activityListeners, attachSession(), buffers, ChatListener (+37 more)

### Community 19 - "logger.ts"
Cohesion: 0.11
Nodes (42): diagnosticsEndRun(), diagnosticsLogChatConsistency(), diagnosticsLogFrontendBatch(), diagnosticsLogFrontendEvent(), diagnosticsLogSnapshotTriplet(), diagnosticsRegisterConversation(), diagnosticsRegisterMember(), diagnosticsRegisterSession() (+34 more)

### Community 20 - "platform.rs"
Cohesion: 0.08
Nodes (36): after_terminal_friend_create(), has_invite_meta(), AppHandle, Option, Result, String, run_post_create_flow(), start_friend_flow() (+28 more)

### Community 21 - "prompt_block.rs"
Cohesion: 0.10
Nodes (35): apply_snapshot(), String, apply_snapshot(), String, resolve_profile(), extract_bullet_block(), extract_bullet_block_before_prompt(), extract_last_bullet_block() (+27 more)

### Community 22 - "TerminalPane.vue"
Cohesion: 0.06
Nodes (42): writeSession(), applySnapshotCursor(), attachClipboardHandlers(), attachPhase, copySelection(), fatalError, findCaseSensitive, findInputRef (+34 more)

### Community 23 - "ChatSidebar.vue"
Cohesion: 0.07
Nodes (34): channelAvatarStyle(), channelItems, containerRef, directMessageItems, emit, getConversationTitle(), getLastMessagePreview(), handleAction() (+26 more)

### Community 24 - "SkillManagementModal.vue"
Cohesion: 0.05
Nodes (39): activeTab, availableProjectSkills, canOpenSkillPath, channelLabel, closeProjectSkillPicker(), confirmRemoveProjectSkill(), { currentSkills }, { currentWorkspace, workspaceReadOnly } (+31 more)

### Community 25 - "chat_db/types.rs"
Cohesion: 0.12
Nodes (36): From, attachment_index_entry(), AttachmentIndexMeta, ChatClearResult, ChatDeleteMemberConversationsResult, ChatHomeFeedDto, ChatMessage, ChatMessageCreatedPayload (+28 more)

### Community 26 - "open_db"
Cohesion: 0.20
Nodes (42): decode(), encode(), open_db(), parse_ulid(), T, chat_append_terminal_message(), chat_clear_all_messages(), chat_clear_conversation() (+34 more)

### Community 27 - "orchestrate_chat_dispatch"
Cohesion: 0.12
Nodes (37): ChatDispatchMentions, ChatDispatchMentions, ChatDispatchPayload, Option, String, Vec, compute_backoff_ms(), dispatch_outbox_task() (+29 more)

### Community 28 - "MemberRow.vue"
Cohesion: 0.06
Nodes (34): displayMembersForTitle, friendEntries, displayMembers, baseStatusOptions, canMention, canRemove, canRename, canSendMessage (+26 more)

### Community 29 - "settingsStore.ts"
Cohesion: 0.08
Nodes (38): AccountSettings, ALLOWED_KEYBINDS, ALLOWED_LOCALES, ALLOWED_STATUSES, ALLOWED_THEMES, AppearanceSettings, applySettingsEffects(), buildCustomMembers() (+30 more)

### Community 30 - "store.rs"
Cohesion: 0.16
Nodes (35): build_conversation_summary(), build_message_preview(), clear_chat_storage(), compute_conversation_unread_count(), compute_total_unread_count(), compute_workspace_unread_summary(), count_unread_messages(), db_path() (+27 more)

### Community 31 - "avatar.ts"
Cohesion: 0.11
Nodes (33): accountAvatar, accountAvatar, isCustomAvatar, selectedAvatarId, selectedLocalAvatarId, isCss, label, props (+25 more)

### Community 32 - "ChatDbManager"
Cohesion: 0.17
Nodes (34): chat_clear_all_messages(), chat_clear_conversation(), chat_create_group(), chat_delete_conversation(), chat_delete_member_conversations(), chat_ensure_direct(), chat_get_messages(), chat_list_conversations() (+26 more)

### Community 33 - "MessagesList.vue"
Cohesion: 0.07
Nodes (28): buildMessageTokens(), emit, ensureTypewriter(), getMessageTime(), handleAvatarClick(), handleJumpToLatest(), hasMore, isLoadingMore (+20 more)

### Community 34 - "models.rs"
Cohesion: 0.10
Nodes (23): default_terminal_event_port(), NoopTerminalEventPort, Arc, Option, Result, String, TerminalErrorPayload, Option (+15 more)

### Community 35 - "TerminalSession"
Cohesion: 0.12
Nodes (25): can_merge_dispatch_envelope(), ensure_envelope_message_ids(), merge_dispatch_envelope(), merge_or_enqueue_dispatch(), pop_dispatch_batch(), is_output_rate_stable(), is_output_stable(), record_output_sample() (+17 more)

### Community 36 - "ScheduledTrigger"
Cohesion: 0.13
Nodes (20): BinaryHeap, Eq, Ord, Ordering, PartialEq, PartialOrd, DeferredStage, FactEvent (+12 more)

### Community 37 - "message_service/project_members.rs"
Cohesion: 0.17
Nodes (29): Map, build_member_base_name(), build_seeded_avatar(), default_avatar(), ensure_default_owner(), ensure_member_sequence(), extract_member_names(), hash_seed() (+21 more)

### Community 38 - "chat/types.ts"
Cohesion: 0.09
Nodes (24): friends, props, { t }, Conversation, ConversationAction, FriendEntry, FriendScope, MemberAction (+16 more)

### Community 39 - "InviteAssistantModal.vue"
Cohesion: 0.08
Nodes (28): canDecreaseInstances, canIncreaseInstances, clampInstances(), commitInstanceInput(), emit, emitInvite(), handleInstanceInput(), instances (+20 more)

### Community 40 - "chat_dispatch_batcher.rs"
Cohesion: 0.15
Nodes (22): Duration, can_merge_context(), ChatDispatchBatcher, dispatch_batch(), DispatchBatch, DispatchQueue, has_message_id_conflict(), restore_failed_dispatch() (+14 more)

### Community 41 - "ui_gateway/commands.rs"
Cohesion: 0.11
Nodes (27): Fn, Invoke, export_commands(), Send, Sync, project_skills_link(), project_skills_list(), project_skills_unlink() (+19 more)

### Community 42 - "useAppKeybinds.ts"
Cohesion: 0.11
Nodes (26): AppKeybindOptions, findVisibleTarget(), focusByRole(), isVisibleElement(), registerAppKeybinds(), resolveScopeRoot(), ScopeRoot, waitForNextFrame() (+18 more)

### Community 43 - "SidebarNav.vue"
Cohesion: 0.08
Nodes (24): AccountStatus, accountAvatar, accountStatus, activeTab, chatStore, { currentWorkspace }, emit, emitChange() (+16 more)

### Community 44 - "NotificationPreview.vue"
Cohesion: 0.11
Nodes (22): canOpenAnyTerminal, canOpenTerminal(), handleHover(), handleIgnoreAll(), handleOpenAllTerminals(), handleOpenConversation(), handleOpenTerminal(), handleViewAll() (+14 more)

### Community 45 - "SkillStore.vue"
Cohesion: 0.08
Nodes (23): localSkillFolders, projectSkillLinksView, createLibrarySkills(), librarySkillSeed, SkillLibraryItem, formatSkillPath(), canOpenSkillPath, confirmRemoveFolder() (+15 more)

### Community 46 - "SessionPollSnapshot"
Cohesion: 0.12
Nodes (17): ChatFlushMeta, PollAction, Option, Self, String, SessionUpdate, build_snapshot(), collect_post_ready_actions() (+9 more)

### Community 47 - "message.rs"
Cohesion: 0.23
Nodes (26): chat_clear_all_messages(), chat_clear_conversation(), chat_create_group(), chat_delete_conversation(), chat_ensure_direct(), chat_get_messages(), chat_list_conversations(), chat_mark_conversation_read_latest() (+18 more)

### Community 48 - "session/commands.rs"
Cohesion: 0.27
Nodes (25): AppHandle, Option, Result, State, String, TerminalDispatchContext, Vec, WebviewWindow (+17 more)

### Community 49 - "terminal.rs"
Cohesion: 0.23
Nodes (25): AppHandle, Option, Result, State, String, TerminalDispatchContext, Vec, WebviewWindow (+17 more)

### Community 50 - "devDependencies"
Cohesion: 0.08
Nodes (25): autoprefixer, eslint, @eslint/js, eslint-plugin-vue, devDependencies, autoprefixer, eslint, @eslint/js (+17 more)

### Community 51 - "dependencies"
Cohesion: 0.08
Nodes (25): dependencies, pinia, @tauri-apps/api, @tauri-apps/plugin-clipboard-manager, @tauri-apps/plugin-dialog, @tauri-apps/plugin-shell, vue, vue-i18n (+17 more)

### Community 52 - "permissions"
Cohesion: 0.08
Nodes (24): clipboard-manager:allow-read-text, clipboard-manager:allow-write-text, core:default, core:window:allow-close, core:window:allow-destroy, core:window:allow-is-maximized, core:window:allow-maximize, core:window:allow-minimize (+16 more)

### Community 53 - "compilerOptions"
Cohesion: 0.08
Nodes (24): DOM, DOM.Iterable, ES2022, node, src/**/*.d.ts, src/**/*.ts, src/**/*.vue, vite/client (+16 more)

### Community 54 - "App.vue"
Cohesion: 0.09
Nodes (21): armPointerReadyReset(), consumeWorkspaceBoot(), contextTitle, handleClose(), { handleContextMenuEvent }, { handleKeydownEvent }, isFocused, isMacOS (+13 more)

### Community 55 - "cloneSettings"
Cohesion: 0.11
Nodes (24): cloneSettings(), applyCustomTerminal(), applyDefaultTerminalSelection(), buildPersistableSettings(), handleMemberRefresh(), isTerminalOptionSelected(), normalizeTerminalPathKey(), openCustomTerminalForm() (+16 more)

### Community 56 - "TerminalEventPort"
Cohesion: 0.20
Nodes (22): Send, Sync, TerminalEventPort, find_keyword_case_insensitive(), parse_session_id_after_keyword(), parse_session_id_from_line(), parse_session_id_from_lines(), Arc (+14 more)

### Community 57 - "snapshot_service.rs"
Cohesion: 0.16
Nodes (19): is_code_fence_line(), is_hard_break_line(), is_indented_code_line(), is_list_line(), is_numbered_list_line(), is_paragraph_start_line(), is_prompt_line(), is_separator_line() (+11 more)

### Community 58 - "tauri.conf.json"
Cohesion: 0.09
Nodes (22): icons/128x128@2x.png, icons/128x128.png, icons/32x32.png, icons/icon.icns, icons/icon.ico, app, security, windows (+14 more)

### Community 59 - "chatStorage.ts"
Cohesion: 0.16
Nodes (19): flushCacheSaves(), scheduleCacheSave(), chatCachePath(), chatDataPath(), ChatSessionCache, conversationMessagesDir(), conversationMessagesPath(), encodeConversationId() (+11 more)

### Community 60 - "keyboard/controller.ts"
Cohesion: 0.18
Nodes (20): buildKeybindContext(), getInputSelectionText(), handleKeydownEvent(), isComboMatch(), isItemEnabled(), isMacOS(), matchesScope(), normalizeComboInput() (+12 more)

### Community 61 - "RoadmapModal.vue"
Cohesion: 0.11
Nodes (17): completion, editingId, editValue, emit, handleAddTask(), handleDelete(), listRef, objectiveValue (+9 more)

### Community 62 - "applySnapshotPayload"
Cohesion: 0.18
Nodes (21): applyRendererMode(), applySnapshotPayload(), attachWithRecovery(), attemptAttach(), collectXtermMeta(), countSnapshotLines(), debugLog, disableCanvasAddon() (+13 more)

### Community 63 - "openMemberTerminalById"
Cohesion: 0.16
Nodes (20): trackSession(), beginBulkOpen(), buildMemberTerminalId(), clearBulkOpenState(), handleDismissRecentClosedTab(), handleNewTab(), handleOpenRecentClosedTab(), handleTabSearchMemberSelect() (+12 more)

### Community 64 - "MemberStatusDots.vue"
Cohesion: 0.12
Nodes (17): anchorRef, baseManualOptions, borderClass, handleEnter(), handleLeave(), manualDotClass, manualOptions, props (+9 more)

### Community 65 - "globalStore.ts"
Cohesion: 0.15
Nodes (14): buildDefaultGlobalData(), formatError(), GlobalData, ImportedSkillFolder, normalizeGlobalData(), normalizeImportedSkillFolders(), useGlobalStore, activeTab (+6 more)

### Community 66 - "emulator.rs"
Cohesion: 0.25
Nodes (15): Blink, CellAttributes, ColorAttribute, Intensity, Line, AttrState, emit_attr_delta(), emit_sgr() (+7 more)

### Community 67 - "memberSelection.ts"
Cohesion: 0.17
Nodes (16): resolveDefaultMemberIndex(), applyCustomMember(), buildCustomMemberId(), removeCustomMember(), resetCustomMemberForm(), selectedMemberId, selectedMemberIndex, syncDefaultMemberIndex() (+8 more)

### Community 68 - "TerminalSnapshotAuditReportModal.vue"
Cohesion: 0.12
Nodes (8): CallChain, CallChainStep, StepStatus, snapshotAuditStatusLabel, SnapshotSignature, {
  status: terminalSnapshotAuditStatus,
  results: terminalSnapshotAuditResults,
  lastError: terminalSnapshotAuditError,
  callChains: terminalSnapshotAuditCallChains
}, { t }, terminalSnapshotAuditStore

### Community 69 - "WeztermEmulator"
Cohesion: 0.19
Nodes (9): Screen, Option, Vec, serialize_screen_to_ansi(), serialize_screen_to_ansi_segments(), SnapshotMetrics, SnapshotSegments, WeztermEmulator (+1 more)

### Community 70 - "contactsStorage.ts"
Cohesion: 0.23
Nodes (13): ALLOWED_ROLES, ALLOWED_STATUSES, loadContacts(), normalizeContact(), normalizeContacts(), normalizeRole(), normalizeStatus(), saveContacts() (+5 more)

### Community 71 - "tauri/avatars.ts"
Cohesion: 0.15
Nodes (14): clampAvatarMenu(), loadCustomAvatars(), positionAvatarMenu(), removeCustomAvatar(), resetAvatar(), selectAvatarPreset(), toggleAvatarMenu(), AvatarAsset (+6 more)

### Community 72 - "applyActiveState"
Cohesion: 0.19
Nodes (15): setSessionActive(), subscribeError(), subscribeExit(), ackWrittenData(), applyActiveState(), attachOutput(), detachOutput(), finalizeAttachFailure() (+7 more)

### Community 73 - "avatarRender.ts"
Cohesion: 0.27
Nodes (14): avatarVars, getAvatarVars(), canvasToPngBytes(), drawLinearBackground(), drawRadialSpot(), drawRing(), normalizeColorStop(), parseLinearGradient() (+6 more)

### Community 74 - "notificationOrchestratorStore.ts"
Cohesion: 0.22
Nodes (13): setNotificationActiveWindow(), updateNotificationState(), AppTabId, SkillStoreTab, useNavigationStore, buildPreviewFallback(), isMainWindowLabel(), normalizeUnreadCount() (+5 more)

### Community 75 - "dispatch_poll_actions"
Cohesion: 0.26
Nodes (13): backend_passive_enabled(), env_flag(), apply_session_update(), ChatFlushLog, dispatch_poll_actions(), handle_semantic_flush(), AppHandle, Arc (+5 more)

### Community 76 - "SemanticState"
Cohesion: 0.30
Nodes (12): build_semantic_payload(), extract_command_from_input(), extract_input_lines(), next_chat_seq(), Box, Option, Self, String (+4 more)

### Community 77 - "emit"
Cohesion: 0.19
Nodes (14): applyMention(), applyPrompt(), buildSendPayload(), dropTrailingSpace(), emit, emitSend(), getCaretCoordinates(), handleInput() (+6 more)

### Community 78 - "onPointerUp"
Cohesion: 0.22
Nodes (14): cleanupDragListeners(), cleanupDragState(), isPointInTabBar(), onPointerDown(), onPointerMove(), onPointerUp(), resolveInsertTarget(), resolvePaneDropTarget() (+6 more)

### Community 79 - "index.ts"
Cohesion: 0.16
Nodes (9): AppLocale, i18n, initialLocale, LOCALE_CHANGED_EVENT, messages, setDocumentLang(), bootstrapApp(), initializeMonitoring() (+1 more)

### Community 80 - "handle_trigger_event"
Cohesion: 0.22
Nodes (13): handle_trigger_event(), AppHandle, Arc, Mutex, Option, schedule_post_ready_ticks(), spawn_status_poller(), collect_poll_snapshots_by_ids() (+5 more)

### Community 81 - "scripts"
Cohesion: 0.15
Nodes (13): scripts, build, dev, dev:backend, dev:frontend, dev:tauri, format, format:check (+5 more)

### Community 82 - "InviteFriendsModal.vue"
Cohesion: 0.17
Nodes (8): confirmInvite(), emit, filteredFriends, props, query, selectedCount, selectedIds, { t }

### Community 83 - "RuleMask"
Cohesion: 0.26
Nodes (6): BitOr, RuleMask, build_poll_actions(), resolve_rules(), Vec, RuleKind

### Community 84 - "skillsBridge.ts"
Cohesion: 0.17
Nodes (11): confirmRemoveFolder(), handleImportFolder(), handleOpenSkillPath(), handleRemoveFolder(), importSkillFolder(), openSkillFolder(), ProjectSkillLink, removeSkillFolder() (+3 more)

### Community 85 - "theme.ts"
Cohesion: 0.30
Nodes (10): applyThemeToDom(), AppTheme, getSystemTheme(), startSystemListener(), stopSystemListener(), syncTheme(), THEME_CHANGED_EVENT, ThemeOption (+2 more)

### Community 86 - "maybeLogPassiveSnapshotTriplet"
Cohesion: 0.24
Nodes (12): snapshotSessionLines(), collectViewportLines(), isPassiveSnapshotEnabled(), maybeLogPassiveSnapshotTriplet(), prunePassiveSnapshotCache(), readPassiveSnapshotCache(), resolvePassiveSnapshotEntry(), respondSnapshotRequest() (+4 more)

### Community 87 - "refreshFindResults"
Cohesion: 0.27
Nodes (12): clearFindHighlights(), closeFind(), handleFindInput(), handleFindKeydown(), handleFindNext(), handleFindPrevious(), isEnterKey(), refreshFindResults() (+4 more)

### Community 88 - "passiveMonitor.ts"
Cohesion: 0.24
Nodes (10): endBeepTraceRun(), ensureBeepTraceRun(), isBeepTraceEnabled(), logBeepTrace(), resolveBeepTraceWindowLabel(), safeInvokeBeepTrace(), isFrontendPassiveEnabled(), createFrontLogger() (+2 more)

### Community 89 - "focusTabInPane"
Cohesion: 0.20
Nodes (12): clearTabSearch(), focusTabInPane(), handleTabClick(), handleTabSearchCreate(), handleTabSearchKeydown(), handleTabSearchSelect(), isEnterKey(), isTabAssigned() (+4 more)

### Community 90 - "WorkspaceSelection.vue"
Cohesion: 0.18
Nodes (8): containerRef, errorTimer, filteredMore, formatWorkspacePath(), { recentPrimary, recentMore, workspaceError }, searchQuery, { t, locale }, workspaceStore

### Community 91 - "chat_outbox_enqueue"
Cohesion: 0.44
Nodes (11): chat_outbox_claim_due(), chat_outbox_enqueue(), chat_outbox_mark_failed(), chat_outbox_mark_sent(), insert_schedule_entry(), remove_schedule_entry(), Result, String (+3 more)

### Community 92 - "chat_list_conversations"
Cohesion: 0.40
Nodes (10): chat_get_conversation_member_ids(), chat_get_messages(), chat_list_conversations(), compute_workspace_unread_summary(), MessageDto, Option, Result, State (+2 more)

### Community 93 - "UiTerminalSessionRepository"
Cohesion: 0.27
Nodes (6): AppHandle, Option, Result, Self, String, UiTerminalSessionRepository

### Community 94 - "project_data_write"
Cohesion: 0.33
Nodes (8): ProjectDataReadResult, ProjectDataWriteResult, project_data_read(), project_data_write(), AppHandle, Result, String, Value

### Community 95 - ".new"
Cohesion: 0.44
Nodes (7): create_emulator(), create_emulator_with_writer(), EmulatorConfig, Box, Send, Write, TerminalEmulator

### Community 96 - ".new"
Cohesion: 0.39
Nodes (6): PtyResponseWriter, Box, Mutex, Result, Send, Write

### Community 97 - ".default"
Cohesion: 0.25
Nodes (4): ColorPalette, Self, WeztermConfig, TerminalConfiguration

### Community 98 - "attachBeepTraceListeners"
Cohesion: 0.36
Nodes (8): attachBeepTraceListeners(), classifyKey(), collectFocusSnapshot(), describeElement(), describeEventPath(), describeTextarea(), isBeepTraceVerbose(), shouldLogKeyEvent()

### Community 99 - "terminal_session_upsert"
Cohesion: 0.39
Nodes (6): Option, Result, String, terminal_session_delete_by_member_id(), terminal_session_get_by_member_id(), terminal_session_upsert()

### Community 100 - ".prettierrc.json"
Cohesion: 0.29
Nodes (6): arrowParens, bracketSpacing, printWidth, semi, singleQuote, trailingComma

### Community 101 - "closeEmojiPanel"
Cohesion: 0.29
Nodes (7): closeEmojiPanel(), handleEmojiSearchKeydown(), handleGlobalPointerDown(), isEnterKey(), openEmojiPanel(), toggleEmojiPanel(), updateEmojiViewport()

### Community 102 - "handleAvatarUpload"
Cohesion: 0.29
Nodes (7): fileToBytes(), handleAvatarUpload(), resolveImageExtension(), selectCustomAvatar(), upsertCustomAvatar(), storeAvatarAsset(), toLocalAvatar()

### Community 103 - "terminalErrors.ts"
Cohesion: 0.29
Nodes (6): parseTerminalError(), resolveTerminalErrorI18nKey(), TERMINAL_ERROR_CODES, TerminalErrorCode, TerminalErrorDetail, TerminalErrorPayload

### Community 104 - "NoopTerminalSessionRepository"
Cohesion: 0.48
Nodes (4): NoopTerminalSessionRepository, Option, Result, String

### Community 105 - "TriggerPlan"
Cohesion: 0.52
Nodes (6): plan_fact(), plan_trigger(), String, Vec, TriggerPlan, TriggerTargets

### Community 106 - "package.json"
Cohesion: 0.33
Nodes (5): name, packageManager, private, type, version

### Community 107 - "ChatHeader.vue"
Cohesion: 0.33
Nodes (5): emit, headerDescription, headerTitle, props, { t }

### Community 108 - "InviteAdminModal.vue"
Cohesion: 0.40
Nodes (5): emit, handleInvite(), identifier, permissions, { t }

### Community 109 - "RenameConversationModal.vue"
Cohesion: 0.33
Nodes (5): draftName, emit, name, props, { t }

### Community 110 - "insertEmoji"
Cohesion: 0.40
Nodes (5): insertEmoji(), loadRecentEmojiIds(), persistRecentEmojiIds(), recordRecentEmoji(), syncRecentEmojiEntries()

### Community 111 - "emoji-data.ts"
Cohesion: 0.40
Nodes (4): EMOJI_DATA, EMOJI_GROUPS, EmojiEntry, EmojiGroup

### Community 112 - "emit"
Cohesion: 0.40
Nodes (5): emit, emitSnapshotResponse(), emitTabOpened(), emitWindowReady(), resolveCurrentWindow()

### Community 113 - "generate_prompt"
Cohesion: 0.50
Nodes (4): generate_prompt(), PromptType, Option, String

### Community 114 - "removeMention"
Cohesion: 0.50
Nodes (4): escapeRegExp(), focusInput(), removeMention(), removeMentionText()

## Knowledge Gaps
- **760 isolated node(s):** `CustomTerminalEntry`, `MemberDisplayOption`, `NotificationToggleKey`, `SectionId`, `TerminalDisplayOption` (+755 more)
  These have â‰¤1 connection - possible missing edges or undocumented components.
- **9 thin communities (<3 nodes) omitted from report** â€” run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TerminalManager` connect `TerminalManager` to `.new`, `TerminalSession`, `session/mod.rs`, `chat_dispatch_batcher.rs`, `session/commands.rs`, `handle_trigger_event`, `terminal.rs`, `platform.rs`, `TerminalEventPort`, `orchestrate_chat_dispatch`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **Why does `ChatDbManager` connect `ChatDbManager` to `app.rs`, `terminal_session_upsert`, `orchestrate_chat_dispatch`, `NotificationBadgeState`, `CommandCenter`, `message.rs`, `open_db`, `chat_outbox_enqueue`, `chat_list_conversations`, `store.rs`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Why does `orchestrate_chat_dispatch()` connect `orchestrate_chat_dispatch` to `ChatDbManager`, `TerminalManager`, `chat_list_conversations`, `StorageManager`?**
  _High betweenness centrality (0.025) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `lock_sessions()` (e.g. with `terminal_ack()` and `terminal_attach()`) actually correct?**
  _`lock_sessions()` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CustomTerminalEntry`, `MemberDisplayOption`, `NotificationToggleKey` to the rest of the system?**
  _760 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Settings.vue` be split into smaller, more focused modules?**
  _Cohesion score 0.02072714916751614 - nodes in this community are weakly interconnected._
- **Should `app.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07601078167115903 - nodes in this community are weakly interconnected._