#![windows_subsystem = "windows"]

mod hotkey;
mod copilot;
mod ai;

use tokio::sync::mpsc;
use std::thread;
use std::sync::Mutex;

struct AppState {
    tx: Mutex<mpsc::Sender<String>>,
}

#[tauri::command]
async fn process_command(command: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let tx = state.tx.lock().unwrap().clone();
    tx.send(command).await.map_err(|e| e.to_string())
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>(32);

    // Spawn Copilot thread
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            copilot::run_copilot(rx).await;
        });
    });

    tauri::Builder::default()
        .manage(AppState { tx: Mutex::new(tx) })
        .invoke_handler(tauri::generate_handler![process_command])
        .setup(|app| {
            let app_handle = app.handle().clone();
            // Start Hotkey listener
            thread::spawn(move || {
                hotkey::listen_hotkey(app_handle);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
