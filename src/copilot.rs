use tokio::sync::mpsc::Receiver;
use windows::Win32::Foundation::{HWND, RECT, WPARAM, LPARAM, LRESULT};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, RegisterClassW, ShowWindow, WNDCLASSW,
    SW_SHOW, WS_POPUP, WS_VISIBLE,
};
use windows::core::{w, PCWSTR};
use image::{ImageBuffer, Rgba, ImageFormat};
use enigo::{Enigo, Mouse, Keyboard, Settings, Coordinate, Button, Direction};

use crate::ai;

pub async fn run_copilot(mut rx: Receiver<String>) {
    // Create the off-screen window using Win32 API
    let hwnd = create_offscreen_window();
    
    // Setup enigo for simulated input
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    while let Some(command) = rx.recv().await {
        println!("Copilot received command: {}", command);
        
        // Loop until AI outputs DONE
        loop {
            // 1. Take screenshot of the off-screen window
            let screenshot = take_screenshot(hwnd);
            
            // 2. Prompt the AI with screenshot + instruction
            let ai_response = ai::evaluate_step(&command, screenshot).await;
            
            if ai_response.trim() == "DONE" {
                // Notify user on main desktop
                let _ = winrt_notification::Toast::new("Dualis")
                    .title("Copilot Finished")
                    .text1(&format!("Completed task: {}", command))
                    .show();
                break;
            } else {
                // Parse AI response (JSON) and execute actions
                if let Ok(action) = serde_json::from_str::<serde_json::Value>(&ai_response) {
                    execute_action(&mut enigo, hwnd, action);
                }
            }
        }
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn create_offscreen_window() -> HWND {
    unsafe {
        let instance = windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap();
        
        let class_name = w!("DualisOffscreenClass");
        
        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };
        
        RegisterClassW(&wc);
        
        let hwnd = CreateWindowExW(
            windows::Win32::UI::WindowsAndMessaging::WS_EX_LAYERED,
            class_name,
            w!("Dualis Copilot Env"),
            WS_POPUP | WS_VISIBLE,
            -30000, -30000, // Off-screen coordinates
            1920, 1080,     // Resolution
            None,
            None,
            instance,
            None,
        ).unwrap();
        
        let _ = ShowWindow(hwnd, SW_SHOW);
        hwnd
    }
}

fn take_screenshot(hwnd: HWND) -> Vec<u8> {
    // For now, return a dummy image to allow compilation
    // In reality, we'd use Win32 BitBlt or PrintWindow to capture the HWND
    let img = ImageBuffer::<Rgba<u8>, _>::new(1920, 1080);
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut bytes);
    img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    bytes
}

fn execute_action(enigo: &mut Enigo, hwnd: HWND, action: serde_json::Value) {
    // Determine the absolute coordinates based on window position
    let base_x = -30000;
    let base_y = -30000;
    
    if let Some(act) = action.get("action").and_then(|a| a.as_str()) {
        match act {
            "click" => {
                let x = action.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
                let y = action.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
                let _ = enigo.move_mouse((base_x + x) as i32, (base_y + y) as i32, Coordinate::Abs);
                let _ = enigo.button(Button::Left, Direction::Click);
            }
            "type" => {
                if let Some(txt) = action.get("text").and_then(|v| v.as_str()) {
                    let _ = enigo.text(txt);
                }
            }
            _ => {}
        }
    }
}
