import re

file_path = 'src-tauri/src/terminal_engine/session/mod.rs'
with open(file_path, 'r', encoding='utf-8') as f:
    content = f.read()

pattern = r'''fn spawn_exit_watcher\(
    mut child: Box<dyn Child \+ Send \+ Sync>,
    event_port: Arc<dyn TerminalEventPort>,
    sessions: Arc<Mutex<SessionRegistry>>,
    terminal_id: String,
    spawn_epoch: u64,
\) \{
    thread::spawn\(move \|\| \{
        // .+?
        let \(code, signal, reason\) = match child\.wait\(\) \{'''

replacement = r'''fn spawn_exit_watcher(
    mut child: Box<dyn Child + Send + Sync>,
    app: tauri::AppHandle,
    event_port: Arc<dyn TerminalEventPort>,
    sessions: Arc<Mutex<SessionRegistry>>,
    terminal_id: String,
    spawn_epoch: u64,
) {
    thread::spawn(move || {
        let start = std::time::Instant::now();
        // 等待子进程退出后同步状态与语义输出，保证 UI 与聊天一致。
        let (code, signal, reason) = match child.wait() {'''

if re.search(pattern, content, flags=re.DOTALL):
    content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    print("Match 1 replaced")
else:
    print("Match 1 failed")

pattern2 = r'''            Err\(err\) => \{
                log::warn!\(
                    "terminal child wait failed terminal_id=\{\} err=\{\}",
                    terminal_id,
                    err
                \);
                \(None, Some\("error"\.to_string\(\)\), "error"\.to_string\(\)\)
            \}
        \};'''

replacement2 = r'''            Err(err) => {
                log::warn!(
                    "terminal child wait failed terminal_id={} err={}",
                    terminal_id,
                    err
                );
                (None, Some("error".to_string()), "error".to_string())
            }
        };

        let elapsed = start.elapsed();
        if elapsed.as_millis() < 2000 && code.unwrap_or(0) != 0 {
            use tauri::Emitter;
            use tauri::Manager;
            let _ = app.emit("terminal-crash", serde_json::json!({
                "terminalId": terminal_id,
                "reason": reason
            }));
            let batcher = app.state::<Arc<crate::orchestration::chat_dispatch_batcher::ChatDispatchBatcher>>();
            batcher.clear_queue(&terminal_id);
        }'''

if re.search(pattern2, content, flags=re.DOTALL):
    content = re.sub(pattern2, replacement2, content, flags=re.DOTALL)
    print("Match 2 replaced")
else:
    print("Match 2 failed")

with open(file_path, 'w', encoding='utf-8') as f:
    f.write(content)
