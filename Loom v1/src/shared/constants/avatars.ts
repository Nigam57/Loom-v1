// 头像预设与默认标识，作为 UI 主题化头像的唯一来源。
export type AvatarPreset = {
  id: string;
  labelKey: string;
  vars: Record<string, string>;
};

// CSS 头像标识前缀，用于区分远程 URL 与本地资源。
export const CSS_AVATAR_PREFIX = 'css:';

// 预设顺序影响默认值与种子选取结果，避免随意调整。
// Flat tiles: one low-chroma hue each, with initials drawn on top. Ids are persisted in member data; keep them stable.
const flatTile = (background: string): Record<string, string> => ({
  '--avatar-bg': background,
  '--avatar-ink': 'oklch(0.97 0.01 255)',
  '--avatar-spot': 'none',
  '--avatar-spot-2': 'none'
});

export const AVATAR_PRESETS: AvatarPreset[] = [
  { id: 'orbit', labelKey: 'settings.avatarOptions.orbit', vars: flatTile('oklch(0.44 0.08 245)') },
  { id: 'ember', labelKey: 'settings.avatarOptions.ember', vars: flatTile('oklch(0.47 0.10 35)') },
  { id: 'mint', labelKey: 'settings.avatarOptions.mint', vars: flatTile('oklch(0.46 0.08 165)') },
  { id: 'canyon', labelKey: 'settings.avatarOptions.canyon', vars: flatTile('oklch(0.50 0.09 75)') },
  { id: 'storm', labelKey: 'settings.avatarOptions.storm', vars: flatTile('oklch(0.40 0.02 255)') }
];
// 默认头像取第一个预设，保证在预设为空时仍有回退。
export const DEFAULT_AVATAR_ID = AVATAR_PRESETS[0]?.id ?? 'orbit';
export const DEFAULT_AVATAR = `${CSS_AVATAR_PREFIX}${DEFAULT_AVATAR_ID}`;
