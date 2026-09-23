// [2026-01-23 00:59] 目的: 维护英文界面文案与键值映射，集中管理以便一致性与可翻译性; 边界: 仅包含文案资源，不引入业务逻辑或运行时行为; 设计: 键结构与功能域对齐以降低跨模块引用成本与回归风险。
export default {
  app: {
    name: 'loom',
    windowControls: {
      minimize: 'Minimize',
      maximize: 'Maximize',
      restore: 'Restore',
      close: 'Close'
    }
  },
  nav: {
    chat: 'Chat',
    friends: 'Agents',
    terminal: 'Terminal',
    workspaces: 'Workspaces',
    store: 'Skills',
    plugins: 'Plugins',
    settings: 'Settings'
  },
  terminal: {
    title: 'Terminal',
    subtitle: 'Run multiple shells in one workspace.',
    newTab: 'New tab',
    tabSearchPlaceholder: 'Search tabs...',
    tabSearchCreate: 'New tab',
    tabSearchEmpty: 'No matching tabs.',
    recentClosedTabs: 'Recently closed ({count})',
    emptyTabs: 'No terminals yet.',
    emptyTitle: 'No active terminals',
    emptySubtitle: 'Create a new terminal tab to get started.',
    splitEmpty: 'Drag a tab here or create a new terminal.',
    unavailableTitle: 'Terminal unavailable',
    unavailableSubtitle: 'Open this view inside the Tauri desktop app to use terminals.',
    errorTitle: 'Terminal failed to start',
    errorSubtitle: 'Check the desktop runtime and try again.',
    resourceLimit: 'System resources are low. Close some background terminals and try again.',
    statusLabel: 'Terminal status',
    statusOptions: {
      pending: 'Pending',
      connecting: 'Connecting',
      connected: 'Connected',
      working: 'Working',
      disconnected: 'Disconnected'
    },
    tabMenu: {
      close: 'Close',
      closeOthers: 'Close others',
      closeRight: 'Close to the Right',
      pin: 'Pin to Front',
      unpin: 'Unpin',
      layoutSingle: 'Single pane',
      layoutSplitVertical: 'Split left/right',
      layoutSplitHorizontal: 'Split top/bottom',
      layoutGrid: '2x2 grid'
    },
    contextMenu: {
      clear: 'Clear',
      find: 'Find'
    },
    findPlaceholder: 'Find...',
    findNoResults: 'No results',
    findCaseSensitive: 'Match case',
    findWholeWord: 'Whole word',
    findRegex: 'Use regex'
  },
  contextMenu: {
    copy: 'Copy',
    cut: 'Cut',
    paste: 'Paste',
    selectAll: 'Select all'
  },
    common: {
      userAvatarAlt: 'User',
      remove: 'Remove',
      openFolder: 'Open folder',
      dismiss: 'Dismiss',
      notBuilt: 'not built yet'
    },
  chat: {
    channelName: 'general',
    channelDisplay: '#general',
    channelDescription: '',
    directMessageDescription: 'Direct message with {name}',
    header: {
      todo: 'Todo',
      inventory: 'Inventory'
    },
    input: {
      placeholder: 'Message {channel}',
      directPlaceholder: 'Message {name}',
      send: 'Send',
      stop: 'Stop',
      mentions: 'Mentions',
      removeMention: 'Remove mention of {name}',
      hint: 'Enter to send • Shift+Enter for newline',
      quickPrompts: {
        summarize: 'Summarize the latest discussion',
        draftReply: 'Draft a polite reply',
        extractTasks: 'Extract action items'
      },
      emoji: {
        recent: 'Recent',
        search: 'Search emoji...',
        empty: 'No matching emoji',
        emptyRecent: 'No recent emoji yet',
        groups: {
          smileys: 'Smileys & Emotion',
          people: 'People & Body',
          component: 'Component',
          animals: 'Animals & Nature',
          food: 'Food & Drink',
          travel: 'Travel & Places',
          activities: 'Activities',
          objects: 'Objects',
          symbols: 'Symbols'
        }
      }
    },
    sidebar: {
      workspaceName: 'Workspace',
      channels: 'Channels',
      directMessages: 'Direct messages',
      channelList: {
        announcements: 'announcements',
        generalChat: 'general-chat',
        designCritique: 'design-critique',
        resources: 'resources'
      }
    },
    messages: {
      dateSeparator: 'October 24, 2023',
      roadmapHint: 'Click to view roadmap',
      userJoined: '{name} joined the server',
      joinedUser: 'James',
      sampleMessage: {
        user: 'Sarah jenkins',
        time: '11:05 AM',
        text: 'Hopefully by Friday! Just need final sign-off from product.'
      },
      autoReply: 'I have processed your request. Is there anything else you need?',
      autoReplyQuestion: 'Let me take a closer look and get back to you shortly.',
      loadHistory: 'Load earlier messages',
      loadingHistory: 'Loading history...',
      jumpToLatest: 'Jump to latest',
      typing: '{name} is working…',
      empty: 'No messages yet.',
      emptyHint: 'Mention an agent with @ to start. Agents in this room can ask each other for help.',
      status: {
        sending: 'Sending...',
        failed: 'Failed to send'
      }
    },
    conversation: {
      actions: {
        pin: 'Pin',
        unpin: 'Unpin',
        rename: 'Rename group',
        mute: 'Mute notifications',
        unmute: 'Unmute notifications',
        clear: 'Clear chat history',
        deleteChannel: 'Delete group chat',
        deleteDirect: 'Delete conversation'
      },
      renameTitle: 'Rename group chat',
      renameLabel: 'Group name',
      renamePlaceholder: 'Enter group name'
    }
  },
  members: {
    title: 'Members',
    sections: {
      owner: 'Group Owner — {count}',
      admin: 'Admins — {count}',
      assistant: 'Assistants — {count}',
      member: 'General Members — {count}'
    },
    roles: {
      owner: 'Owner',
      admin: 'Admin',
      assistant: 'Assistant',
      member: 'Member',
      aiAssistant: 'AI assistant'
    },
    actions: {
      sendMessage: 'Send message',
      mention: 'Mention',
      rename: 'Rename'
    },
    manage: {
      title: 'Manage member',
      displayName: 'Display name',
      remove: 'Remove from Group'
    },
    activity: {
      reviewingPRs: 'Reviewing prs',
      listeningSpotify: 'Listening to Spotify',
      fixingBugs: 'Fixing bugs',
      doNotDisturb: 'Do not disturb'
    }
  },
  friends: {
    title: 'Agents',
    add: 'Add',
    invite: 'Invite',
    empty: 'No friends yet',
    sections: {
      project: 'In this workspace',
      global: 'Global friends'
    },
    inviteModal: {
      titleChannel: 'Invite friends',
      titleDm: 'Create group chat',
      search: 'Search friends',
      actionInvite: 'Invite',
      actionCreate: 'Create group'
    }
  },
  invite: {
    menu: {
      title: 'Invite to Server',
      subtitle: 'Generate a unique invite link',
      admin: 'Invite as admin',
      adminDesc: 'Full server access',
      assistant: 'Invite as assistant',
      assistantDesc: 'Moderation permissions',
      member: 'General member',
      memberDesc: 'Standard access'
    },
    admin: {
      title: 'Invite as admin',
      subtitle: 'Configure access level and duration',
      uniqueLink: 'Unique invite link',
      regenerate: 'Regenerate',
      userIdentifier: 'User identifier',
      userPlaceholder: 'Username or email address',
      permissions: 'Permissions level',
      send: 'Send invitation',
      permissionsList: {
        fullAccess: {
          title: 'Full server access',
          desc: 'Can modify settings, channels & roles'
        },
        billing: {
          title: 'Billing access',
          desc: 'Manage subscription and payments'
        },
        memberManagement: {
          title: 'Member management',
          desc: 'Kick, ban, and assign lower roles'
        }
      }
    },
    assistant: {
      title: 'Invite as assistant',
      subtitle: 'Pick the CLI agent to add to this workspace.',
      instances: 'Number of instances',
      instanceLimit: 'Max {count} instances',
      unlimitedAccess: 'Unlimited mode',
      unlimitedAccessDesc: 'Bypass usage limits',
      sandboxed: 'Sandboxed environment',
      send: 'Send invitation',
      models: {
        gemini: 'Gemini CLI',
        codex: 'Codex',
        claude: 'Claude Code',
        custom: 'Custom CLI'
      }
    },
    member: {
      title: 'Invite as member'
    }
  },
  roadmap: {
    title: 'Project roadmap',
    objectiveLabel: 'Objective:',
    taskPlaceholder: 'Enter task title...',
    newTask: 'New task',
    status: {
      done: 'DONE',
      inProgress: 'IN progress',
      pending: 'PENDING'
    },
    actions: {
      edit: 'Edit task',
      changeOrder: 'Change order',
      markPriority: 'Mark as Priority',
      delete: 'Delete'
    },
    footer: '{count} Tasks • {percent}% Complete',
    addTask: 'Add task'
  },
  skills: {
    management: {
      title: 'Skill management',
      subtitle: 'Configure active skills for {channel}',
      tabs: {
        current: 'Current skills',
        library: 'My skills'
      }
    },
    current: {
      activeFolders: 'Active folders',
      syncAll: 'Sync all',
      updated: 'Updated 2h ago',
      active: 'Active'
    },
    project: {
      title: 'Project skills',
      import: 'Import my skills',
      pickerSubtitle: 'Select skills from your library to link into this workspace.',
      searchPlaceholder: 'Search skills in your library...',
      linkAction: 'Link',
      loading: 'Loading project skills...',
      empty: 'No project skills linked yet.',
      emptyLibrary: 'No skills available in your library.',
      emptySearch: 'No skills match your search.',
      readOnlyHint: 'Workspace is read-only. Linking skills is disabled.',
      removeConfirmTitle: 'Unlink project skill?',
      removeConfirmMessage: 'This will unlink "{name}" from this workspace.',
      removeConfirmOk: 'Unlink',
      removeConfirmCancel: 'Cancel'
    },
    library: {
      searchPlaceholder: 'Search your library...',
      refresh: 'Refresh',
      importTitle: 'Import skill',
      importSubtitle: 'From URL or Local File',
      removeConfirmTitle: 'Remove skill folder?',
      removeConfirmMessage: 'This will delete "{name}" from your local skills library.',
      removeConfirmOk: 'Delete',
      removeConfirmCancel: 'Cancel',
      browseShop: 'Browse skill shop'
    },
    footer: {
      documentation: 'Documentation',
      privacy: 'Privacy',
      newFolder: 'New folder',
      lastSynced: 'Last synced: 2 mins ago'
    },
    tags: {
      typography: 'Typography',
      colorPalette: 'Color palette',
      components: 'Components'
    },
    items: {
      designSystemCore: {
        name: 'Design system core'
      },
      uxResearchPatterns: {
        name: 'UX research patterns'
      },
      a11yGuidelines: {
        name: 'A11y Guidelines'
      },
      frontendToolkit: {
        name: 'Frontend toolkit',
        desc: 'Essential snippets for React, Vue, and Tailwind CSS development.'
      },
      iconAssetPack: {
        name: 'Icon asset pack',
        desc: 'Premium outline and solid icons for modern interface design.'
      },
      motionPresets: {
        name: 'Motion presets',
        desc: 'Standardized animation curves and transition timings.'
      },
      shellCommands: {
        name: 'Shell commands',
        desc: 'Quick access to common CLI scripts and deployment hooks.'
      },
      brandColors: {
        name: 'Brand colors',
        desc: 'Company color palettes and accessible contrast ratios.'
      }
    },
    assets: {
      frontendToolkit: '45 assets',
      iconAssetPack: '1.2k icons',
      motionPresets: '12 presets',
      shellCommands: '24 cmds',
      brandColors: '8 swatches'
    },
    detail: {
      sourceConfig: 'Source configuration',
      source: {
        github: 'GitHub repo',
        command: 'Command source',
        local: 'Local path'
      },
      repoLabel: 'Repository URL',
      repoPlaceholder: 'https://github.com/username/repository.git',
      repoHint: 'Supports HTTPS and SSH URLs from GitHub, GitLab, and Bitbucket.',
      syncPreferences: 'Sync preferences',
      autoSync: 'Auto-sync Updates',
      autoSyncDesc: 'Automatically pull latest changes from source',
      updateFrequency: 'Update frequency',
      updateFrequencyDesc: 'How often to check for new versions',
      frequency: {
        every15: 'Every 15 minutes',
        hour: 'Every hour',
        daily: 'Daily',
        manual: 'Manual only'
      },
      targetBranch: 'Target branch',
      targetBranchDesc: 'Branch to track for updates',
      deleteSkill: 'Delete skill',
      cancel: 'Cancel',
      saveChanges: 'Save changes'
    }
  },
  marketplace: {
    title: 'Plugins',
    subtitle: 'Tools and MCP servers that every agent in a workspace can use.',
    notBuiltBody: 'Installing plugins is not built yet, so there is nothing to browse here.',
    plannedTitle: 'Planned',
    planned: {
      github: 'Paste a GitHub repo and Loom works out whether it is an MCP server, a tool or a skill.',
      shared: 'Install it once and every CLI agent in the workspace gets it.',
      isolated: 'Installed code runs isolated, with permission prompts.'
    },
    todayTitle: 'Available today',
    todayBody: 'Agents in a room already share the Loom tools (list_agents, ask_agent, room messages) through the loom MCP server.',
    searchPlaceholder: 'Search plugins, integrations, and themes...',
    browseStore: 'Browse store',
    myPlugins: 'My plugins',
    importTitle: 'Import plugin',
    importSubtitle: 'From URL or Local File',
    categories: {
      all: 'All plugins',
      productivity: 'Productivity',
      development: 'Development',
      design: 'Design',
      communication: 'Communication',
      music: 'Music'
    },
    install: 'Install',
    installed: 'Installed',
    plugins: {
      github: {
        title: 'GitHub integration',
        desc: 'Connect your repositories, track issues, and manage pull requests directly.'
      },
      spotify: {
        title: 'Spotify player',
        desc: 'Listen together. Control playback and share your favorite tracks.'
      },
      taskManager: {
        title: 'Task manager',
        desc: 'A simple Kanban board for your team. Create, assign, and complete tasks.'
      },
      calendar: {
        title: 'Calendar sync',
        desc: 'Never miss a meeting. Sync with Google Calendar and Outlook.'
      },
      aiAssistant: {
        title: 'AI assistant',
        desc: 'Your personal AI companion. Ask questions, generate text, and summarize.'
      },
      terminal: {
        title: 'Terminal',
        desc: 'Run commands and scripts directly from the chat. For power users only.'
      },
      figma: {
        title: 'Figma preview',
        desc: 'Embed live Figma files and prototypes. Get feedback instantly.'
      },
      quickNotes: {
        title: 'Quick notes',
        desc: 'Jot down ideas and share them with your team. Supports markdown.'
      }
    }
  },
  skillStore: {
    title: 'Skills',
    subtitle: 'Skill folders you import are available to agents in your workspaces.',
    searchPlaceholder: 'Search skill folders, templates, and toolkits...',
    catalogNotBuilt: 'The skill catalog is not built yet.',
    catalogHint: 'For now, import a skill folder from your disk. A shared catalog and install-from-GitHub are on the roadmap.',
    goToMySkills: 'Go to my skills',
    emptyTitle: 'No skills imported yet.',
    emptyHint: 'Import a folder that holds a SKILL.md or other prompt files to share it with your agents.',
    tabs: {
      store: 'Store',
      catalog: 'Catalog',
      installed: 'My skills'
    },
    filters: {
      all: 'All skills',
      engineering: 'Engineering',
      design: 'Design',
      management: 'Management',
      marketing: 'Marketing',
      finance: 'Finance'
    },
    syncPlaceholder: 'Paste sync URL...',
    syncNow: 'Sync now',
    installFolder: 'Install folder',
    installed: 'My skills',
    skills: {
      automation: {
        title: 'Automation skill',
        desc: 'Streamline workflows with pre-built scripts. Auto-syncs with your repo.'
      },
      uiToolkit: {
        title: 'UI design toolkit',
        desc: 'Centralized design assets and brand guidelines. Syncs with Figma files.'
      },
      projectTracking: {
        title: 'Project tracking',
        desc: 'Task lists and kanban boards for active sprints. Syncs with Jira or Trello.'
      },
      marketingAssets: {
        title: 'Marketing assets',
        desc: 'Campaign materials and social media templates. Syncs with Drive/Dropbox.'
      },
      devOpsConfig: {
        title: 'Dev ops config',
        desc: 'Shared environment variables and docker configs. Syncs secure vaults.'
      },
      researchLibrary: {
        title: 'Research library',
        desc: 'Competitor analysis and market trends. Syncs with Notion or Evernote pages.'
      }
    }
  },
  workspace: {
    openTitle: 'Open folder',
    openSubtitle: 'Loom works inside a project folder. Pick one to start, or reopen a recent workspace.',
    recentTitle: 'Recent workspaces',
    pitch: {
      rooms: 'Put your CLI agents in one room.',
      ask: 'They ask each other for reviews and help.',
      local: 'Everything runs on this machine.'
    },
    more: 'More',
    searchPlaceholder: 'Search folders...',
    emptyTitle: 'No recent workspaces',
    emptySubtitle: 'Open a folder to start your first workspace.',
    openAction: 'Open',
    noResults: 'No matching workspaces',
    openErrorTitle: 'Unable to open workspace',
    readOnlyTitle: 'Workspace is read-only',
    readOnlySubtitle: 'This folder is not writable. Project metadata will not be saved.',
    registryMismatchTitle: 'Workspace location changed',
    registryMismatchMessage: 'The workspace location has changed.\nPrevious path: {oldPath}\nCurrent path: {newPath}\nDid you move the original project or copy a new duplicate?',
    registryMismatchMoved: 'Moved',
    registryMismatchCopied: 'Copied'
  },
  settings: {
    title: 'Settings',
    preferences: 'Preferences',
    preferencesSubtitle: 'Customize your account, notifications, and workspace preferences.',
    accountSubtitle: 'Manage your profile, presence, and contact details.',
    avatarTitle: 'Avatar',
    avatarSubtitle: 'Pick a style or upload your own.',
    avatarUpload: 'Upload image',
    avatarReset: 'Use style',
    avatarHint: 'PNG/JPG/WEBP up to 2MB.',
    avatarUploads: 'Uploads',
    avatarErrors: {
      invalidType: 'Please upload an image file.',
      tooLarge: 'Image is larger than 2MB.',
      storageFailed: 'Unable to save avatar.'
    },
    avatarOptions: {
      orbit: 'Orbit',
      ember: 'Ember',
      mint: 'Mint',
      canyon: 'Canyon',
      storm: 'Storm'
    },
    displayName: 'Display name',
    displayNamePlaceholder: 'Enter your display name',
    emailAddress: 'Email address',
    emailPlaceholder: 'name{at}example.com',
    changeEmail: 'Change email',
    jobTitle: 'Job title',
    jobTitlePlaceholder: 'e.g. Product Designer',
    timeZone: 'Time zone',
    timeZones: {
      utc: 'UTC',
      pacificMidway: 'Pacific/Midway',
      pacificHonolulu: 'Pacific/Honolulu',
      americaAnchorage: 'America/Anchorage',
      americaLosAngeles: 'America/Los_Angeles',
      americaDenver: 'America/Denver',
      americaChicago: 'America/Chicago',
      americaNewYork: 'America/New_York',
      americaHalifax: 'America/Halifax',
      americaSaoPaulo: 'America/Sao_Paulo',
      atlanticAzores: 'Atlantic/Azores',
      europeLondon: 'Europe/London',
      europeParis: 'Europe/Paris',
      europeHelsinki: 'Europe/Helsinki',
      europeMoscow: 'Europe/Moscow',
      asiaDubai: 'Asia/Dubai',
      asiaKarachi: 'Asia/Karachi',
      asiaDhaka: 'Asia/Dhaka',
      asiaBangkok: 'Asia/Bangkok',
      asiaShanghai: 'Asia/Shanghai',
      asiaTokyo: 'Asia/Tokyo',
      australiaSydney: 'Australia/Sydney',
      pacificNoumea: 'Pacific/Noumea',
      pacificAuckland: 'Pacific/Auckland'
    },
    status: 'Status',
    statusMessage: 'Status message',
    statusMessagePlaceholder: 'Share what you are working on',
    statusOptions: {
      online: 'Online',
      working: 'Working',
      dnd: 'Do not disturb',
      offline: 'Offline'
    },
    language: 'Language',
    languageDefault: 'Default',
    changesApply: 'Changes apply after restart.',
    defaultMember: 'Default member',
    selectMember: 'Select member',
    selectTerminal: 'Select terminal',
    terminalAuto: 'System default',
    terminalAutoHint: 'Uses the OS default shell.',
    terminalCustom: 'Custom terminal',
    terminalName: 'Terminal name',
    terminalNamePlaceholder: 'e.g. PowerShell',
    terminalPath: 'Terminal executable',
    terminalPathPlaceholder: 'Select a terminal executable',
    terminalBrowse: 'Browse',
    terminalEmpty: 'No terminals detected on this device.',
    terminalNotAvailable: 'Available in the desktop app only.',
    refreshList: 'Refresh list',
    cancel: 'Cancel',
    saveChanges: 'Save changes',
    userSettings: 'User settings',
    myAccount: 'My account',
    appSettings: 'App settings',
    appearance: 'Appearance',
    appearanceSubtitle: 'Choose a theme and an accent colour.',
    accentTitle: 'Accent colour',
    accentSubtitle: 'Used for focus, the active item and primary actions.',
    accent: {
      amber: 'Amber',
      teal: 'Teal',
      blue: 'Blue',
      rose: 'Rose',
      green: 'Green',
      violet: 'Violet',
      mono: 'Mono'
    },
    members: 'Members',
    notifications: 'Notifications',
    keybinds: 'Keybinds',
    createTeam: 'Create team',
    leaveTeam: 'Leave team',
    logOut: 'Log out',
    memberName: 'Member name',
    memberNamePlaceholder: 'Enter member label',
    commandInput: 'Command line input',
    commandPlaceholder: 'CLI startup command, e.g. gemini',
    confirm: 'Confirm',
    notificationsSubtitle: 'Choose when and how you want to be notified.',
    notificationOptions: {
      desktop: 'Desktop notifications',
      desktopDesc: 'Show system notifications for new messages.',
      sound: 'Sound alerts',
      soundDesc: 'Play a sound when new messages arrive.',
      mentionsOnly: 'Mentions only',
      mentionsOnlyDesc: 'Only notify when you are mentioned.',
      previews: 'Message previews',
      previewsDesc: 'Display message content in alerts.',
      quietHours: 'Quiet hours',
      quietHoursDesc: 'Silence alerts during scheduled hours.'
    },
    quietHoursFrom: 'From',
    quietHoursTo: 'To',
    keybindsSubtitle: 'Configure shortcuts and keybinding profiles.',
    keybindsProfile: 'Keybinding profile',
    keybindsEnable: 'Enable keyboard shortcuts',
    keybindsHints: 'Show shortcut hints',
    keybindsReset: 'Reset to Defaults',
    keybindsListTitle: 'Shortcut reference',
    data: 'Data',
    dataSubtitle: 'Repair or reset local chat storage for this workspace.',
    dataRepairTitle: 'Repair message database',
    dataRepairHint: 'Scan and remove unreadable or corrupted messages.',
    dataRepairAction: 'Repair messages',
    dataRepairConfirm: 'Scan this workspace chat database and remove unreadable messages?',
    dataRepairResult: 'Repair completed. Removed {count} messages.',
    dataClearTitle: 'Clear chat history',
    dataClearHint: 'Delete all messages and attachments for this workspace.',
    dataClearAction: 'Clear all messages',
    chatStreamTitle: 'Chat streaming output',
    chatStreamHint: 'Stream terminal output into chat while the command is running.',
    dataClearConfirm: 'This will permanently remove all chat messages for this workspace. Continue?',
    dataClearResult: 'Cleared {messages} messages and {attachments} attachments.',
    dataActionFailed: 'Operation failed. Please try again.',
    terminalCallChainsTitle: 'Call chains',
    terminalSnapshotAuditTitle: 'Terminal snapshot audit',
    terminalSnapshotAuditHint: 'Open and close terminal windows to compare frontend and backend snapshots.',
    terminalSnapshotAuditAction: 'Run snapshot audit',
    terminalSnapshotAuditRunning: 'Running snapshot audit...',
    terminalSnapshotAuditNotAvailable: 'Available in the desktop app only.',
    terminalSnapshotAuditStatus: {
      idle: 'Snapshot audit idle.',
      running: 'Snapshot audit running...',
      passed: 'Snapshot audit passed.',
      failed: 'Snapshot audit failed.'
    },
    terminalSnapshotAuditLegend: 'FB: front vs backend · FR: front vs reopen · BR: backend vs reopen',
    terminalSnapshotAuditRound: 'Round {round}',
    terminalSnapshotAuditCreated: 'created for audit',
    terminalSnapshotAuditNoMembers: 'No terminal members available.',
    terminalSnapshotAuditCheck: {
      ok: 'OK',
      ng: 'NG'
    },
    keybindProfiles: {
      default: 'Default',
      vscode: 'VS Code',
      slack: 'Slack'
    },
    keybindActions: {
      focusSearch: 'Focus search',
      newMessage: 'New message',
      toggleSidebar: 'Toggle sidebar',
      toggleMute: 'Toggle mute',
      jumpToLatest: 'Jump to latest',
      openSettings: 'Open settings'
    },
    memberOptions: {
      antigravity: 'Antigravity',
      gemini: 'Gemini CLI',
      codex: 'Codex',
      claude: 'Claude Code',
      opencode: 'OpenCode',
      qwen: 'Qwen Code',
      terminal: 'Terminal',
      custom: 'Custom CLI'
    },
    memberKind: {
      default: 'Default terminal',
      custom: 'Custom terminal'
    },
    memberActions: {
      menuLabel: 'Terminal actions',
      test: 'Test terminal',
      edit: 'Edit',
      remove: 'Delete'
    },
    terminalFriends: {
      title: 'Terminal friends',
      desc: 'Remove terminal members and reset name counters.',
      action: 'Delete terminal friends',
      confirmCurrent: 'Delete terminal friends in this project and reset counters?',
      resultCurrent: 'Removed {count} terminal friends from this project.',
      failed: 'Failed to delete terminal friends.',
      noWorkspace: 'Please open a workspace first.'
    },
    terminalTestErrors: {
      shellBinaryNotFound: 'Terminal executable not found: {path}',
      shellLaunchFailed: 'Failed to launch terminal: {error}'
    },
    terminalTestFailed: 'Test Terminal failed: {error}',
    themeOptions: {
      dark: {
        label: 'Dark',
        desc: 'Default theme designed for low-light focus.'
      },
      light: {
        label: 'Light',
        desc: 'Bright layout optimized for daylight work.'
      },
      system: {
        label: 'System',
        desc: 'Follows your operating system appearance.'
      }
    }
  },
  language: {
    enUS: 'English (United States)',
    zhCN: 'Chinese (Simplified)'
  }
};
