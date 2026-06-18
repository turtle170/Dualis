use std::sync::mpsc::Sender;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{RegisterHotKey, MOD_ALT, VK_SPACE};
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};

pub fn listen_hotkey(gui_tx: Sender<()>) {
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
                let _ = gui_tx.send(());
            }
        }
    }
}
