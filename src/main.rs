#![windows_subsystem = "windows"]

mod hotkey;
mod gui;
mod copilot;
mod ai;

use tokio::sync::mpsc;
use std::thread;

fn main() {
    // Channel to send commands from GUI to Copilot
    let (tx, rx) = mpsc::channel::<String>(32);

    // Channel to tell GUI to show/hide
    let (gui_tx, gui_rx) = std::sync::mpsc::channel::<()>();

    // Spawn Copilot thread
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            copilot::run_copilot(rx).await;
        });
    });

    // Start Hotkey listener
    thread::spawn(move || {
        hotkey::listen_hotkey(gui_tx);
    });

    // Run the main GUI (hidden initially, shown on hotkey)
    // eframe needs to run on the main thread.
    gui::run_gui(tx, gui_rx).unwrap();
}
