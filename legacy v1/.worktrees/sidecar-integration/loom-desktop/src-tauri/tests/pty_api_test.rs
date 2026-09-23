use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_pty_bridge_api() {
    // Assert HTTP server boots and can interact with PTY manager
    let pty_manager = std::sync::Arc::new(std::sync::Mutex::new(app_lib::pty::manager::PtyManager::new()));
    let broadcaster = std::sync::Arc::new(std::sync::Mutex::new(HashMap::new()));
    
    let state = app_lib::pty::server::ServerState {
        pty_manager: pty_manager.clone(),
        broadcaster: broadcaster.clone(),
    };
    
    // Spawn server on a random port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    
    let server_handle = tokio::spawn(async move {
        app_lib::pty::server::start_server_with_listener(listener, state).await;
    });
    
    sleep(Duration::from_millis(500)).await;
    
    let client = Client::new();
    
    // Test that the server is up and returns a 404 for unknown routes or we can just try /pty/123/events
    let res = client.get(&format!("http://127.0.0.1:{}/pty/123/events", port))
        .send()
        .await
        .expect("Failed to execute request");
    
    assert!(res.status().is_success());
    assert_eq!(res.headers().get("content-type").unwrap().to_str().unwrap(), "text/event-stream");
    
    // Also test POST /pty/:id/write
    let write_res = client.post(&format!("http://127.0.0.1:{}/pty/123/write", port))
        .json(&serde_json::json!({ "data": "hello" }))
        .send()
        .await
        .expect("Failed to execute write request");
    
    // It should return 500 because pid 123 doesn't exist
    assert_eq!(write_res.status(), reqwest::StatusCode::INTERNAL_SERVER_ERROR);
    
    server_handle.abort();
}
