use std::collections::HashMap;

use keyboard_listener_windows::{start_listen, stop_listen, Event};
use once_cell::sync::OnceCell;
use tauri::{Emitter, Manager};

use crate::{PHYSIZE, SCALE_FACTOR};

pub static KEY: OnceCell<Event> = OnceCell::new();
pub static mut KEYBOARD : bool = false; // 使用OnceCell来管理键盘状态

#[tauri::command]
pub async fn keyboard_light(handle: tauri::AppHandle) {
    let win_key = "msg-keyboard".to_string();
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("键盘监听窗口已启动!");
        unsafe {KEYBOARD = true} // 设置键盘状态为true
    } else {
        let factor= unsafe { SCALE_FACTOR };
        let win_list = handle.webview_windows();
        let mut win_num = 0;
        for (key, _value) in win_list {
            if key.starts_with("msg") {
                win_num += 1;
            }
        }

        let physize = PHYSIZE.get().expect("Error get pool from OneCell<Pool>");
        let mut width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as f64;
        let mut height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as f64;
        width = width / factor;
        height = height / factor;

        println!("height:{}", height - 1000.0);

        let docs_window = tauri::WebviewWindowBuilder::new(
            &handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/KeyDown".parse().unwrap()),
        )
        .title("键盘监听事件")
        .inner_size(0.0, 60.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .skip_taskbar(true)
        .focused(false)
        .always_on_top(true)
        .position(0.0, height - 120.0)
        .build();

        match docs_window {
            Ok(win) => {
                println!("键盘监听窗口启动成功!");
                let _ = win.hide();
                unsafe {KEYBOARD = true} 

                // 如果没有启动监听，启动监听
                println!("键盘监听!");

                start_listen(move |event| {
                    println!("Keyboard event: {:?}", event.key);
                    let key = event.key.to_string();
                    let physize = PHYSIZE.get().expect("Error get pool from OneCell<Pool>");
                    let width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as i32;
                    let height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as i32;
                    println!("按下{}", key);
                    let mut map = HashMap::new();
                    map.insert("key", key.clone());
                    map.insert("width", width.to_string());
                    map.insert("height", height.to_string());
                    map.insert("factor", factor.to_string());
                    // 异步更新窗口信息
                    if let Err(e) = win.emit("key_down_msg", map) {
                        println!("Emit error: {}", e);
                    }
                });
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }
}

#[tauri::command]
pub async fn check_keyboard() -> bool {
    unsafe { KEYBOARD } // 更简洁的状态获取方式
}

#[tauri::command]
pub fn close_win(win_key: String, handle: tauri::AppHandle) {
    if let Some(win) = handle.get_webview_window(&win_key) {
        let _ = win.close();
        println!("键盘监听窗口已关闭!");
        stop_listen();
        println!("键盘监听线程已关闭!");
    }
    unsafe { KEYBOARD = false }
}