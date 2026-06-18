use tauri::{Emitter, Manager, PhysicalPosition};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, VK_SPACE};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};

pub fn listen_hotkey(app_handle: tauri::AppHandle) {
    unsafe {
        // Register Alt+Space (id = 1)
        if RegisterHotKey(HWND(std::ptr::null_mut()), 1, MOD_ALT, VK_SPACE.0 as u32).is_ok() {
            println!("Hotkey Alt+Space registered.");
        } else {
            eprintln!("Failed to register hotkey.");
            return;
        }

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).into() {
            if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY {
                // Hotkey pressed
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        if let Ok(Some(monitor)) = window.current_monitor() {
                            let size = monitor.size();
                            let win_size = window.outer_size().unwrap_or_default();
                            // Position bottom center (60px from bottom)
                            let x = size.width.saturating_sub(win_size.width) / 2;
                            let y = size.height.saturating_sub(win_size.height).saturating_sub(60);
                            let _ = window.set_position(PhysicalPosition::new(x, y));
                        }
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.emit("window-shown", ());
                    }
                }
            }
        }
    }
}
