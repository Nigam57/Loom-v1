import { X, Monitor, MessageSquare, Palette, Puzzle, Info } from 'lucide-react';
import { useState } from 'react';
import { useSettingsStore } from '../store/settings';
import { motion, AnimatePresence } from 'framer-motion';

type SettingsSection = 'agents' | 'rooms' | 'appearance' | 'extensions' | 'about';

const SECTIONS: { id: SettingsSection; label: string; icon: typeof Monitor }[] = [
  { id: 'agents', label: 'Agents', icon: Monitor },
  { id: 'rooms', label: 'Conversations', icon: MessageSquare },
  { id: 'appearance', label: 'Appearance', icon: Palette },
  { id: 'extensions', label: 'Extensions', icon: Puzzle },
  { id: 'about', label: 'About', icon: Info },
];

export function SettingsDrawer() {
  const { settingsOpen, toggleSettings } = useSettingsStore();
  const [section, setSection] = useState<SettingsSection>('agents');

  return (
    <AnimatePresence>
      {settingsOpen && (
        <>
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.1 }}
            onClick={toggleSettings}
            style={{
              position: 'fixed',
              inset: 0,
              backgroundColor: 'rgba(0,0,0,0.5)',
              zIndex: 100,
            }}
          />
          {/* Drawer */}
          <motion.div
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ duration: 0.12, ease: 'easeOut' }}
            style={{
              position: 'fixed',
              top: 0,
              right: 0,
              width: 580,
              height: '100%',
              backgroundColor: 'var(--color-bg-0)',
              borderLeft: '1px solid var(--color-border)',
              zIndex: 101,
              display: 'flex',
              boxShadow: '-20px 0 40px rgba(0,0,0,0.4)',
            }}
          >
            {/* Section Nav */}
            <div
              style={{
                width: 180,
                backgroundColor: 'var(--color-bg-0)',
                borderRight: '1px solid var(--color-border)',
                padding: '24px 0',
              }}
            >
              <div
                style={{
                  padding: '8px 16px 16px',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between',
                }}
              >
                <span style={{ fontSize: 14, fontWeight: 600 }}>Settings</span>
                <button
                  onClick={toggleSettings}
                  style={{
                    border: 'none',
                    background: 'transparent',
                    color: 'var(--color-text-muted)',
                    cursor: 'pointer',
                    padding: 2,
                    display: 'flex',
                  }}
                >
                  <X size={14} />
                </button>
              </div>
              {SECTIONS.map((s) => {
                const Icon = s.icon;
                const active = section === s.id;
                return (
                  <button
                    key={s.id}
                    onClick={() => setSection(s.id)}
                    style={{
                      width: '100%',
                      display: 'flex',
                      alignItems: 'center',
                      gap: 8,
                      padding: '8px 20px',
                      border: 'none',
                      background: active ? 'var(--color-bg-2)' : 'transparent',
                      color: active ? 'var(--color-text-primary)' : 'var(--color-text-muted)',
                      cursor: 'pointer',
                      fontSize: 13,
                      fontWeight: active ? 500 : 400,
                      fontFamily: 'var(--font-sans)',
                      textAlign: 'left',
                      borderRight: active ? '2px solid var(--color-text-primary)' : '2px solid transparent',
                    }}
                  >
                    <Icon size={13} />
                    {s.label}
                  </button>
                );
              })}
            </div>

            {/* Section Content */}
            <div style={{ flex: 1, overflow: 'auto', padding: '20px 24px' }}>
              {section === 'agents' && <AgentsSection />}
              {section === 'rooms' && <RoomsSection />}
              {section === 'appearance' && <AppearanceSection />}
              {section === 'extensions' && <ExtensionsSection />}
              {section === 'about' && <AboutSection />}
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
}

function SectionTitle({ children }: { children: React.ReactNode }) {
  return (
    <h2 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16, marginTop: 0 }}>{children}</h2>
  );
}

function FieldLabel({ children }: { children: React.ReactNode }) {
  return (
    <label style={{ display: 'block', fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 4 }}>
      {children}
    </label>
  );
}

function TextInput({ value, onChange, placeholder, mono }: { value: string; onChange: (v: string) => void; placeholder?: string; mono?: boolean }) {
  return (
    <input
      type="text"
      value={value}
      onChange={(e) => onChange(e.target.value)}
      placeholder={placeholder}
      style={{
        width: '100%',
        padding: '10px 14px',
        border: '1px solid var(--color-border)',
        borderRadius: 6,
        backgroundColor: 'var(--color-bg-1)',
        color: 'var(--color-text-primary)',
        fontSize: 13,
        fontFamily: mono ? 'var(--font-mono)' : 'var(--font-sans)',
        outline: 'none',
        marginBottom: 16,
        transition: 'border-color 0.15s ease',
      }}
      onFocus={(e) => (e.target.style.borderColor = 'var(--color-text-dim)')}
      onBlur={(e) => (e.target.style.borderColor = 'var(--color-border)')}
    />
  );
}

function AgentsSection() {
  return (
    <div>
      <SectionTitle>Agent Configuration</SectionTitle>
      <p style={{ fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 16 }}>
        Override default CLI detection paths. Leave empty to use auto-detection.
      </p>
      {['claude', 'coder', 'opencode', 'agy', 'hermes', 'odysseus'].map((agent) => (
        <div key={agent}>
          <FieldLabel>{agent} CLI path</FieldLabel>
          <TextInput value="" onChange={() => {}} placeholder="Auto-detect" mono />
        </div>
      ))}
    </div>
  );
}

function RoomsSection() {
  const { settings } = useSettingsStore();
  return (
    <div>
      <SectionTitle>Conversation Defaults</SectionTitle>
      <FieldLabel>Default max turns</FieldLabel>
      <TextInput value={String(settings.rooms.defaultMaxTurns)} onChange={() => {}} />
      <FieldLabel>Default timeout (seconds)</FieldLabel>
      <TextInput value={String(settings.rooms.defaultTimeoutSeconds)} onChange={() => {}} />
      <FieldLabel>Default budget (cents)</FieldLabel>
      <TextInput value={String(settings.rooms.defaultBudgetCents)} onChange={() => {}} />
      <FieldLabel>Turn policy</FieldLabel>
      <select
        value={settings.rooms.defaultTurnPolicy}
        onChange={() => {}}
        style={{
          width: '100%',
          padding: '10px 14px',
          border: '1px solid var(--color-border)',
          borderRadius: 6,
          backgroundColor: 'var(--color-bg-1)',
          color: 'var(--color-text-primary)',
          fontSize: 13,
          outline: 'none',
          marginBottom: 16,
          transition: 'border-color 0.15s ease',
        }}
        onFocus={(e) => (e.target.style.borderColor = 'var(--color-text-dim)')}
        onBlur={(e) => (e.target.style.borderColor = 'var(--color-border)')}
      >
        <option value="round-robin">Round Robin</option>
        <option value="directed">Directed</option>
        <option value="mention">@Mention</option>
      </select>
    </div>
  );
}

function AppearanceSection() {
  const { settings, updateSettings } = useSettingsStore();
  return (
    <div>
      <SectionTitle>Appearance</SectionTitle>
      <FieldLabel>Theme</FieldLabel>
      <div style={{ display: 'flex', gap: 8, marginBottom: 12 }}>
        {(['dark', 'light', 'system'] as const).map((theme) => (
          <button
            key={theme}
            onClick={() => updateSettings({ appearance: { theme } })}
            style={{
              padding: '8px 20px',
              border: `1px solid ${settings.appearance.theme === theme ? 'var(--color-text-primary)' : 'var(--color-border)'}`,
              borderRadius: 6,
              background: settings.appearance.theme === theme ? 'var(--color-text-primary)' : 'transparent',
              color: settings.appearance.theme === theme ? 'var(--color-bg-0)' : 'var(--color-text-muted)',
              cursor: 'pointer',
              fontSize: 13,
              fontWeight: 500,
              fontFamily: 'var(--font-sans)',
              textTransform: 'capitalize',
              transition: 'all 0.15s ease',
            }}
          >
            {theme}
          </button>
        ))}
      </div>
    </div>
  );
}

function ExtensionsSection() {
  return (
    <div>
      <SectionTitle>Extensions</SectionTitle>
      <p style={{ fontSize: 12, color: 'var(--color-text-muted)' }}>
        Manage installed extensions from the Extensions panel in the sidebar.
      </p>
    </div>
  );
}

function AboutSection() {
  return (
    <div>
      <SectionTitle>About Loom</SectionTitle>
      <div style={{ fontSize: 13, lineHeight: 1.8 }}>
        <div><strong>Version:</strong> 0.1.0-alpha</div>
        <div><strong>License:</strong> AGPL v3 (provisional)</div>
        <div><strong>Framework:</strong> Tauri v2 + React 19</div>
        <div style={{ marginTop: 12, color: 'var(--color-text-muted)', fontSize: 12 }}>
          Loom wraps and enhances AI coding agent CLIs. It does not reimplement their agent loops.
        </div>
      </div>
    </div>
  );
}
