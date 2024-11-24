use std::collections::HashMap;

use once_cell::sync::OnceCell;
use rdev::{listen, Event};
use tauri::{Emitter, Manager};

use crate::{PHYSIZE, SCALE_FACTOR};

pub static KEY: OnceCell<Event> = OnceCell::new();
pub static mut KEY_LISTEN : bool = false;
pub static mut KEYBOARD : bool = false;

#[tauri::command]
pub async fn keyboard_light(handle: tauri::AppHandle) {
    let win_key = "msg-keyboard".to_string();
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("键盘监听窗口已启动!");
       unsafe {
        KEYBOARD = true;
       }
    } else {
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
        width = width / unsafe { SCALE_FACTOR };
        height = height / unsafe { SCALE_FACTOR };
        println!("height:{}", height - 1000.0);
        // 1+1等于多少
        let docs_window = tauri::WebviewWindowBuilder::new(
            &handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/KeyDown".parse().unwrap()),
        )
        .title("键盘监听事件")
        .inner_size(width, 60.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .position(0.0, height - 120.0)
        .build();

        match docs_window {
            Ok(win) => {
                println!("键盘监听窗口启动成功!");
                let _ = win.set_always_on_top(true);
                let _ = win.hide();
                unsafe {
                    KEYBOARD = true;
                    if !KEY_LISTEN {
                        println!("键盘监听!");
                        KEY_LISTEN = true;
                        if let Err(error) = listen(move |evet| {
                            let key_check = KEY.get();
                            match key_check {
                                Some(key_check) => {
                                    if key_check.event_type.eq(&evet.event_type) {
                                        println!("--{:?}--", evet.time.duration_since(key_check.time));
                                    }
                                },
                                None => {},
                            }
                            match evet.event_type {
                                rdev::EventType::KeyPress(key) => {
                                    let key = format!("{:?}", key);
                                    let mut width = 1;
                                    let mut height =  1;
                                    let mut factor = 1.0;
                                    // 获取主窗口所在的屏幕
                                    if let Some(monitor) = win.primary_monitor().unwrap() {
                                        
                                        let size = monitor.size();
                                        width = size.width;
                                        height = size.height;
                                        factor = monitor.scale_factor();
                                        println!("Screen resolution: {}x{}", width, height);
                                    } else {
                                        println!("Could not get monitor information");
                                    }
                                    
                                    // KEY
                                    // .set(evet.clone())
                                    // .unwrap_or_else(|_| println!("try insert pool cell failure!"));
                                    println!("按下{}", key.clone());
                                    let mut map = HashMap::new();
                                    map.insert("key", key.clone());
                                    map.insert("width", width.to_string());
                                    map.insert("height", height.to_string());
                                    map.insert("factor", factor.to_string());
                                    win.emit("key_down_msg", map).unwrap();
                                    return ;
                                },
                                rdev::EventType::KeyRelease(key) => {
                                    println!("释放{:?}", key);
                                },
                                rdev::EventType::ButtonPress(button) => {},
                                rdev::EventType::ButtonRelease(button) => {},
                                rdev::EventType::MouseMove { x, y } => {},
                                rdev::EventType::Wheel { delta_x, delta_y } => {},
                            };
                        }) {
                            println!("Error: {:?}", error)
                        }
                      
                    }
                }
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }


}


#[tauri::command]
pub async fn check_keyboard() -> bool {
    unsafe {
        return KEYBOARD;
    }
    //match map.get(&win_key) {
    //    Some(booll) => {
    //        println!("查询到：{}", booll);
    //        return *booll;
    //    },
    //    None => {
    //        println!("没有查询到");
    //        return false;
    //    },
    //};
}

#[tauri::command]
pub async fn colse_win(win_key: String, handle: tauri::AppHandle) {
    if let Some(win) = handle.get_webview_window(&win_key) {
        let _ = win.close();
        println!("键盘监听窗口已启动!");
        unsafe {
            KEYBOARD = false;
        }
    }
}
