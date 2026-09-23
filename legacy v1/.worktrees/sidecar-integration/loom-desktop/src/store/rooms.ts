import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

export interface Message {
  id: string;
  speaker: string;
  text: string;
  timestamp_ms: number;
  cost_cents: number;
}

export interface Room {
  id: string;
  goal: string;
  messages: Message[];
  status: 'idle' | 'running' | 'paused' | 'stopped' | 'needs-approval';
  turns: number;
  maxTurns: number;
  totalCost: number;
  budgetCents: number;
  timeoutSeconds: number;
  participantNames: string[];
  pendingApproval?: {
    nextAgent: string;
    proposedMessage: string;
  };
}

interface RoomStore {
  rooms: Room[];
  activeRoomId: string | null;
  setActiveRoom: (id: string | null) => void;
  addMessage: (roomId: string, msg: Message) => void;
  createRoom: (goal: string, agentIds: string[], cwd: string | null, guards: { maxTurns: number; timeoutSeconds: number; budgetCents: number }) => Promise<string | null>;
  startRoom: (roomId: string) => Promise<void>;
  pauseRoom: (roomId: string) => Promise<void>;
  stopRoom: (roomId: string) => Promise<void>;
  sendManualReply: (roomId: string, text: string) => Promise<void>;
  fetchTranscript: (roomId: string) => Promise<void>;
  setupListeners: () => Promise<UnlistenFn[]>;
}

export const useRoomStore = create<RoomStore>((set, get) => ({
  rooms: [],
  activeRoomId: null,

  setActiveRoom: (id) => set({ activeRoomId: id }),

  addMessage: (roomId, msg) =>
    set((state) => ({
      rooms: state.rooms.map((r) =>
        r.id === roomId
          ? { ...r, messages: [...r.messages, msg], turns: r.turns + 1, totalCost: r.totalCost + msg.cost_cents }
          : r
      ),
    })),

  createRoom: async (goal, agentIds, cwd, guards) => {
    try {
      const roomId = await invoke<string>('create_room', {
        goal,
        agentIds,
        cwd,
        maxTurns: guards.maxTurns,
        timeoutSeconds: guards.timeoutSeconds,
        budgetCents: guards.budgetCents,
      });
      set((state) => ({
        rooms: [
          ...state.rooms,
          {
            id: roomId,
            goal,
            messages: [],
            status: 'idle',
            turns: 0,
            maxTurns: guards.maxTurns,
            totalCost: 0,
            budgetCents: guards.budgetCents,
            timeoutSeconds: guards.timeoutSeconds,
            participantNames: agentIds,
          },
        ],
        activeRoomId: roomId,
      }));
      return roomId;
    } catch (e) {
      console.error('Failed to create room:', e);
      return null;
    }
  },

  startRoom: async (roomId) => {
    try {
      await invoke('start_room', { roomId });
      set((state) => ({
        rooms: state.rooms.map((r) => (r.id === roomId ? { ...r, status: 'running' } : r)),
      }));
    } catch (e) {
      console.error('Failed to start room:', e);
    }
  },

  pauseRoom: async (roomId) => {
    try {
      await invoke('pause_room', { roomId });
      set((state) => ({
        rooms: state.rooms.map((r) => (r.id === roomId ? { ...r, status: 'paused' } : r)),
      }));
    } catch (e) {
      console.error('Failed to pause room:', e);
    }
  },

  stopRoom: async (roomId) => {
    try {
      await invoke('stop_room', { roomId });
      set((state) => ({
        rooms: state.rooms.map((r) => (r.id === roomId ? { ...r, status: 'stopped' } : r)),
      }));
    } catch (e) {
      console.error('Failed to stop room:', e);
    }
  },

  sendManualReply: async (roomId, text) => {
    try {
      await invoke('send_manual_reply', { roomId, text });
      
      // Clear pending approval state since we're responding
      set((state) => ({
        rooms: state.rooms.map((r) =>
          r.id === roomId
            ? { ...r, pendingApproval: undefined }
            : r
        ),
      }));

      // Automatically resume the room
      await get().startRoom(roomId);
    } catch (e) {
      console.error('Failed to send reply:', e);
    }
  },

  fetchTranscript: async (roomId) => {
    try {
      const messages = await invoke<Message[]>('get_room_transcript', { roomId });
      set((state) => ({
        rooms: state.rooms.map((r) => (r.id === roomId ? { ...r, messages } : r)),
      }));
    } catch {
      // Room may not exist yet
    }
  },

  setupListeners: async () => {
    const unlisteners: UnlistenFn[] = [];

    const u1 = await listen<{ room_id: string; message: Message }>('room://transcript-update', (event) => {
      get().addMessage(event.payload.room_id, event.payload.message);
    });
    unlisteners.push(u1);

    const u2 = await listen<{ room_id: string; next_agent: string; proposed_message: string }>(
      'room://needs-approval',
      (event) => {
        set((state) => ({
          rooms: state.rooms.map((r) =>
            r.id === event.payload.room_id
              ? {
                  ...r,
                  status: 'needs-approval' as const,
                  pendingApproval: {
                    nextAgent: event.payload.next_agent,
                    proposedMessage: event.payload.proposed_message,
                  },
                }
              : r
          ),
        }));
      }
    );
    unlisteners.push(u2);

    return unlisteners;
  },
}));
