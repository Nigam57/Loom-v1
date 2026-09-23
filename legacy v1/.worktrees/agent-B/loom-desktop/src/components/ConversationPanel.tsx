import { useRef, useEffect, useState } from 'react';
import { useRoomStore } from '../store/rooms';
import type { Message } from '../store/rooms';
import { Check, X, Edit3, Coins, RotateCcw, Send } from 'lucide-react';

const AGENT_COLORS: Record<string, string> = {
  claude: '#9333ea',
  'claude-code': '#9333ea',
  coder: '#2563eb',
  codex: '#2563eb',
  opencode: '#16a34a',
  agy: '#d97706',
  hermes: '#dc2626',
  odysseus: '#7c3aed',
  User: '#a1a1aa',
  router: '#52525b',
};

function getAgentColor(speaker: string): string {
  for (const [key, color] of Object.entries(AGENT_COLORS)) {
    if (speaker.toLowerCase().includes(key)) return color;
  }
  return '#71717a';
}

function formatTime(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  const rem = s % 60;
  return `${m}:${rem.toString().padStart(2, '0')}`;
}

export function ConversationPanel() {
  const { rooms, activeRoomId, sendManualReply } = useRoomStore();
  const scrollRef = useRef<HTMLDivElement>(null);
  const room = rooms.find((r) => r.id === activeRoomId);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [room?.messages.length]);

  if (!room) {
    return (
      <div
        style={{
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'var(--color-text-dim)',
          gap: 16,
          backgroundColor: 'var(--color-bg-0)',
        }}
      >
        <div style={{
          padding: 16,
          borderRadius: '50%',
          backgroundColor: 'var(--color-bg-1)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          border: '1px solid var(--color-border)',
        }}>
          <MessageSquareIcon />
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 4 }}>
          <span style={{ fontSize: 16, fontWeight: 500, color: 'var(--color-text-primary)' }}>No conversation selected</span>
          <span style={{ fontSize: 13, color: 'var(--color-text-dim)', maxWidth: 280, textAlign: 'center' }}>
            Create a new room or select one from the sidebar
          </span>
        </div>
      </div>
    );
  }

  return (
    <div style={{ height: '100%', display: 'flex', flexDirection: 'column', backgroundColor: 'var(--color-bg-0)' }}>
      {/* Guard Status Bar */}
      <div
        style={{
          padding: '12px 20px',
          borderBottom: '1px solid var(--color-border)',
          display: 'flex',
          alignItems: 'center',
          gap: 20,
          fontSize: 12,
          color: 'var(--color-text-muted)',
          backgroundColor: 'var(--color-bg-0)',
        }}
      >
        <span style={{ fontWeight: 600, color: 'var(--color-text-primary)', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
          {room.goal}
        </span>
        <span style={{ display: 'flex', alignItems: 'center', gap: 6, flexShrink: 0 }}>
          <RotateCcw size={13} />
          {room.turns} / {room.maxTurns}
        </span>
        <span style={{ display: 'flex', alignItems: 'center', gap: 6, flexShrink: 0 }}>
          <Coins size={13} />
          {room.totalCost}¢
        </span>
        <span
          style={{
            padding: '4px 10px',
            borderRadius: 6,
            backgroundColor: 'var(--color-bg-2)',
            color: 'var(--color-text-primary)',
            fontSize: 10,
            fontFamily: 'var(--font-mono)',
            textTransform: 'uppercase',
            letterSpacing: '0.05em',
            fontWeight: 600,
            flexShrink: 0,
          }}
        >
          {room.status}
        </span>
      </div>

      {/* Messages */}
      <div ref={scrollRef} style={{ flex: 1, overflow: 'auto', padding: '24px 32px' }}>
        {room.messages.length === 0 ? (
          <div style={{ color: 'var(--color-text-dim)', fontSize: 12, textAlign: 'center', padding: '40px 0' }}>
            Conversation will appear here when the room starts
          </div>
        ) : (
          room.messages.map((msg) => <MessageBubble key={msg.id} message={msg} />)
        )}
      </div>

      {/* HITL Approval Card */}
      {room.status === 'needs-approval' && room.pendingApproval && (
        <div
          style={{
            borderTop: '1px solid var(--color-border)',
            padding: '12px 16px',
            backgroundColor: 'var(--color-bg-1)',
          }}
        >
          <div style={{ fontSize: 12, color: 'var(--color-text-muted)', marginBottom: 8 }}>
            <strong>{room.pendingApproval.nextAgent}</strong> wants to respond:
          </div>
          <div
            style={{
              padding: '8px 12px',
              backgroundColor: 'var(--color-bg-2)',
              borderRadius: 4,
              fontSize: 13,
              fontFamily: 'var(--font-mono)',
              marginBottom: 8,
              maxHeight: 120,
              overflow: 'auto',
            }}
          >
            {room.pendingApproval.proposedMessage}
          </div>
          <div style={{ display: 'flex', gap: 8 }}>
            <button
              onClick={() => sendManualReply(room.id, room.pendingApproval!.proposedMessage)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 4,
                padding: '5px 12px',
                border: '1px solid var(--color-accent-green)',
                borderRadius: 4,
                background: 'transparent',
                color: 'var(--color-accent-green)',
                cursor: 'pointer',
                fontSize: 12,
                fontFamily: 'var(--font-sans)',
              }}
            >
              <Check size={12} /> Approve
            </button>
            <button
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 4,
                padding: '5px 12px',
                border: '1px solid var(--color-border)',
                borderRadius: 4,
                background: 'transparent',
                color: 'var(--color-text-muted)',
                cursor: 'pointer',
                fontSize: 12,
                fontFamily: 'var(--font-sans)',
              }}
            >
              <Edit3 size={12} /> Edit
            </button>
            <button
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: 4,
                padding: '5px 12px',
                border: '1px solid var(--color-accent-red)',
                borderRadius: 4,
                background: 'transparent',
                color: 'var(--color-accent-red)',
                cursor: 'pointer',
                fontSize: 12,
                fontFamily: 'var(--font-sans)',
              }}
            >
              <X size={12} /> Reject
            </button>
          </div>
        </div>
      )}
      {/* Message Input */}
      {room.status !== 'stopped' && (
        <MessageInput
          onSend={(text) => sendManualReply(room.id, text)}
          disabled={room.status === 'needs-approval'}
        />
      )}
    </div>
  );
}

function MessageBubble({ message }: { message: Message }) {
  const accentColor = getAgentColor(message.speaker);
  const isUser = message.speaker === 'User';

  return (
    <div style={{ marginBottom: 24, display: 'flex', flexDirection: 'column', alignItems: isUser ? 'flex-end' : 'flex-start' }}>
      <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 6 }}>
        <span style={{ fontSize: 12, color: isUser ? 'var(--color-text-primary)' : accentColor, fontWeight: 600 }}>{message.speaker}</span>
        <span style={{ fontSize: 10, color: 'var(--color-text-dim)', fontFamily: 'var(--font-mono)' }}>
          {formatTime(message.timestamp_ms)}
        </span>
      </div>
      <div
        style={{
          padding: '12px 16px',
          backgroundColor: isUser ? 'var(--color-bg-2)' : 'var(--color-bg-1)',
          border: `1px solid var(--color-border)`,
          borderRadius: 8,
          fontSize: 14,
          lineHeight: 1.6,
          color: 'var(--color-text-primary)',
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-word',
          maxWidth: '85%',
        }}
      >
        {message.text}
      </div>
    </div>
  );
}

function MessageSquareIcon() {
  return (
    <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
    </svg>
  );
}

function MessageInput({ onSend, disabled }: { onSend: (text: string) => void; disabled?: boolean }) {
  const [text, setText] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = () => {
    const trimmed = text.trim();
    if (!trimmed || disabled) return;
    onSend(trimmed);
    setText('');
    inputRef.current?.focus();
  };

  return (
    <div
      style={{
        borderTop: '1px solid var(--color-border)',
        padding: '12px 20px',
        display: 'flex',
        gap: 8,
        alignItems: 'center',
        backgroundColor: 'var(--color-bg-1)',
      }}
    >
      <input
        ref={inputRef}
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={(e) => e.key === 'Enter' && handleSubmit()}
        placeholder="Send a message to the room..."
        disabled={disabled}
        style={{
          flex: 1,
          padding: '10px 14px',
          border: '1px solid var(--color-border)',
          borderRadius: 6,
          backgroundColor: 'var(--color-bg-0)',
          color: 'var(--color-text-primary)',
          fontSize: 13,
          fontFamily: 'var(--font-sans)',
          outline: 'none',
          transition: 'border-color 0.15s',
          opacity: disabled ? 0.5 : 1,
        }}
        onFocus={(e) => (e.target.style.borderColor = 'var(--color-text-dim)')}
        onBlur={(e) => (e.target.style.borderColor = 'var(--color-border)')}
      />
      <button
        onClick={handleSubmit}
        disabled={!text.trim() || disabled}
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          width: 36,
          height: 36,
          border: 'none',
          borderRadius: 6,
          backgroundColor: text.trim() && !disabled ? 'var(--color-text-primary)' : 'var(--color-bg-2)',
          color: text.trim() && !disabled ? 'var(--color-bg-0)' : 'var(--color-text-dim)',
          cursor: text.trim() && !disabled ? 'pointer' : 'default',
          flexShrink: 0,
        }}
      >
        <Send size={16} />
      </button>
    </div>
  );
}
