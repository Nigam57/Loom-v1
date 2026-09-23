import { useState } from 'react';
import { X, FolderOpen } from 'lucide-react';
import { useAgentStore } from '../store/agents';
import { useRoomStore } from '../store/rooms';
import { useSettingsStore } from '../store/settings';
import { motion, AnimatePresence } from 'framer-motion';

interface NewRoomModalProps {
  open: boolean;
  onClose: () => void;
}

export function NewRoomModal({ open, onClose }: NewRoomModalProps) {
  const { agents } = useAgentStore();
  const { createRoom, startRoom } = useRoomStore();
  const { settings } = useSettingsStore();
  const [goal, setGoal] = useState('');
  const [selectedAgents, setSelectedAgents] = useState<string[]>([]);
  const [maxTurns, setMaxTurns] = useState(settings.rooms.defaultMaxTurns);
  const [timeout, setTimeout] = useState(settings.rooms.defaultTimeoutSeconds);
  const [budget, setBudget] = useState(settings.rooms.defaultBudgetCents);

  const installedAgents = agents.filter((a) => a.installed);

  const toggleAgent = (id: string) => {
    setSelectedAgents((prev) => (prev.includes(id) ? prev.filter((a) => a !== id) : [...prev, id]));
  };

  const handleCreate = async () => {
    if (!goal.trim() || selectedAgents.length < 2) return;
    const roomId = await createRoom(goal, selectedAgents, settings.workspace.currentFolder, { maxTurns, timeoutSeconds: timeout, budgetCents: budget });
    if (roomId) {
      await startRoom(roomId);
      onClose();
    }
  };

  return (
    <AnimatePresence>
      {open && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.1 }}
            onClick={onClose}
            style={{ position: 'fixed', inset: 0, backgroundColor: 'rgba(0,0,0,0.6)', zIndex: 200 }}
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.98 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.98 }}
            transition={{ duration: 0.1 }}
            style={{
              position: 'fixed',
              top: '50%',
              left: '50%',
              transform: 'translate(-50%, -50%)',
              width: 480,
              backgroundColor: 'var(--color-bg-1)',
              border: '1px solid var(--color-border)',
              borderRadius: 8,
              boxShadow: '0 24px 48px rgba(0,0,0,0.4)',
              zIndex: 201,
              overflow: 'hidden',
            }}
          >
            {/* Header */}
            <div
              style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: '12px 16px',
                borderBottom: '1px solid var(--color-border)',
              }}
            >
              <span style={{ fontSize: 14, fontWeight: 600 }}>New Conversation Room</span>
              <button
                onClick={onClose}
                style={{ border: 'none', background: 'transparent', color: 'var(--color-text-muted)', cursor: 'pointer', padding: 2, display: 'flex' }}
              >
                <X size={14} />
              </button>
            </div>

            {/* Body */}
            <div style={{ padding: '16px' }}>
              {settings.workspace.currentFolder && (
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 12, color: 'var(--color-text-dim)', marginBottom: 12, fontFamily: 'var(--font-mono)' }}>
                  <FolderOpen size={14} />
                  <span>{settings.workspace.currentFolder}</span>
                </div>
              )}
              {/* Goal */}
              <label style={{ display: 'block', fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 4 }}>
                Goal prompt
              </label>
              <textarea
                value={goal}
                onChange={(e) => setGoal(e.target.value)}
                placeholder="Describe what the agents should work on together..."
                rows={3}
                style={{
                  width: '100%',
                  padding: '12px 14px',
                  border: '1px solid var(--color-border)',
                  borderRadius: 6,
                  backgroundColor: 'var(--color-bg-0)',
                  color: 'var(--color-text-primary)',
                  fontSize: 13,
                  fontFamily: 'var(--font-sans)',
                  outline: 'none',
                  resize: 'vertical',
                  marginBottom: 20,
                  transition: 'border-color 0.2s',
                }}
                onFocus={(e) => (e.target.style.borderColor = 'var(--color-text-dim)')}
                onBlur={(e) => (e.target.style.borderColor = 'var(--color-border)')}
              />

              {/* Agent Selection */}
              <label style={{ display: 'block', fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 8 }}>
                Participants (select 2+)
              </label>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, marginBottom: 16 }}>
                {installedAgents.map((agent) => {
                  const selected = selectedAgents.includes(agent.id);
                  return (
                    <button
                      key={agent.id}
                      onClick={() => toggleAgent(agent.id)}
                      style={{
                        padding: '6px 14px',
                        border: `1px solid ${selected ? agent.color || 'var(--color-text-dim)' : 'var(--color-border)'}`,
                        borderRadius: 6,
                        background: selected ? 'var(--color-bg-2)' : 'transparent',
                        color: selected ? 'var(--color-text-primary)' : 'var(--color-text-muted)',
                        cursor: 'pointer',
                        fontSize: 12,
                        fontWeight: selected ? 500 : 400,
                        fontFamily: 'var(--font-sans)',
                        transition: 'all 0.15s ease',
                      }}
                    >
                      {agent.name}
                    </button>
                  );
                })}
                {installedAgents.length === 0 && (
                  <span style={{ fontSize: 12, color: 'var(--color-text-dim)' }}>No agents installed</span>
                )}
              </div>

              {/* Guards */}
              <label style={{ display: 'block', fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 8 }}>
                Guards
              </label>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: 8, marginBottom: 16 }}>
                <div>
                  <div style={{ fontSize: 11, color: 'var(--color-text-dim)', marginBottom: 2 }}>Max turns</div>
                  <input
                    type="number"
                    value={maxTurns}
                    onChange={(e) => setMaxTurns(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '5px 8px',
                      border: '1px solid var(--color-border)',
                      borderRadius: 4,
                      backgroundColor: 'var(--color-bg-2)',
                      color: 'var(--color-text-primary)',
                      fontSize: 12,
                      fontFamily: 'var(--font-mono)',
                      outline: 'none',
                    }}
                  />
                </div>
                <div>
                  <div style={{ fontSize: 11, color: 'var(--color-text-dim)', marginBottom: 2 }}>Timeout (s)</div>
                  <input
                    type="number"
                    value={timeout}
                    onChange={(e) => setTimeout(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '5px 8px',
                      border: '1px solid var(--color-border)',
                      borderRadius: 4,
                      backgroundColor: 'var(--color-bg-2)',
                      color: 'var(--color-text-primary)',
                      fontSize: 12,
                      fontFamily: 'var(--font-mono)',
                      outline: 'none',
                    }}
                  />
                </div>
                <div>
                  <div style={{ fontSize: 11, color: 'var(--color-text-dim)', marginBottom: 2 }}>Budget (¢)</div>
                  <input
                    type="number"
                    value={budget}
                    onChange={(e) => setBudget(Number(e.target.value))}
                    style={{
                      width: '100%',
                      padding: '5px 8px',
                      border: '1px solid var(--color-border)',
                      borderRadius: 4,
                      backgroundColor: 'var(--color-bg-2)',
                      color: 'var(--color-text-primary)',
                      fontSize: 12,
                      fontFamily: 'var(--font-mono)',
                      outline: 'none',
                    }}
                  />
                </div>
              </div>
            </div>

            {/* Footer */}
            <div
              style={{
                display: 'flex',
                justifyContent: 'flex-end',
                gap: 8,
                padding: '12px 16px',
                borderTop: '1px solid var(--color-border)',
              }}
            >
              <button
                onClick={onClose}
                style={{
                  padding: '8px 20px',
                  border: 'none',
                  borderRadius: 6,
                  background: 'transparent',
                  color: 'var(--color-text-muted)',
                  cursor: 'pointer',
                  fontSize: 13,
                  fontWeight: 500,
                  fontFamily: 'var(--font-sans)',
                }}
              >
                Cancel
              </button>
              <button
                onClick={handleCreate}
                disabled={!goal.trim() || selectedAgents.length < 2}
                style={{
                  padding: '8px 24px',
                  border: '1px solid var(--color-border)',
                  borderRadius: 6,
                  background: 'var(--color-text-primary)',
                  color: 'var(--color-bg-0)',
                  cursor: goal.trim() && selectedAgents.length >= 2 ? 'pointer' : 'default',
                  fontSize: 13,
                  fontWeight: 600,
                  fontFamily: 'var(--font-sans)',
                  opacity: goal.trim() && selectedAgents.length >= 2 ? 1 : 0.4,
                }}
              >
                Create & Start
              </button>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
}
