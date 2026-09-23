import { useEffect, useState, useCallback } from 'react';
import { useAgentStore } from './store/agents';
import { useRoomStore } from './store/rooms';
import { useSettingsStore } from './store/settings';
import { Sidebar } from './components/Sidebar';
import { ConversationPanel } from './components/ConversationPanel';
import { TerminalGrid } from './components/TerminalGrid';
import { GraphView } from './components/GraphView';
import { ArenaDiffView } from './components/ArenaDiffView';
import { FlowBuilder } from './components/FlowBuilder';
import { ExtensionPanel } from './components/ExtensionPanel';
import { SettingsDrawer } from './components/SettingsDrawer';
import { NewRoomModal } from './components/NewRoomModal';

function App() {
  const { activeView } = useSettingsStore();
  const { setupListeners } = useRoomStore();
  const [newRoomOpen, setNewRoomOpen] = useState(false);

  // Setup Tauri event listeners
  useEffect(() => {
    let unlisteners: (() => void)[] = [];
    setupListeners().then((u) => {
      unlisteners = u;
    });
    return () => {
      unlisteners.forEach((u) => u());
    };
  }, [setupListeners]);

  const handleSpawnAgent = useCallback((agentId: string) => {
    // Find agent name
    const store = useAgentStore.getState();
    const agent = store.agents.find((a) => a.id === agentId);
    const label = agent?.name || agentId;

    // For now, switch to terminals view and trigger a pane add
    useSettingsStore.getState().setView('terminals');
    // We'll use a simple event for terminal pane creation
    window.dispatchEvent(new CustomEvent('loom:spawn-agent', { detail: { agentId, label } }));
  }, []);

  const handleAddToConversation = useCallback((_agentId: string) => {
    // Open the new room modal with this agent pre-selected
    setNewRoomOpen(true);
  }, []);

  const renderMainContent = () => {
    switch (activeView) {
      case 'terminals':
        return <TerminalGrid onAddToConversation={handleAddToConversation} />;
      case 'conversation':
        return <ConversationPanel />;
      case 'graph':
        return <GraphView />;
      case 'arena':
        return <ArenaDiffView />;
      case 'automation':
        return <FlowBuilder />;
      case 'extensions':
        return <ExtensionPanel />;
      default:
        return <TerminalGrid onAddToConversation={handleAddToConversation} />;
    }
  };

  return (
    <div style={{ display: 'flex', height: '100vh', width: '100vw', overflow: 'hidden' }}>
      <Sidebar onSpawnAgent={handleSpawnAgent} onNewRoom={() => setNewRoomOpen(true)} />
      <main style={{ flex: 1, overflow: 'hidden', backgroundColor: 'var(--color-bg-0)' }}>
        {renderMainContent()}
      </main>
      <SettingsDrawer />
      <NewRoomModal open={newRoomOpen} onClose={() => setNewRoomOpen(false)} />
    </div>
  );
}

export default App;
