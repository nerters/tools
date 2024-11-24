use std::{thread::sleep, time};

use once_cell::sync::OnceCell;
use rdev::{listen, Event};
use tauri::{Emitter, Manager, PhysicalPosition, Runtime};

use crate::{PHYSIZE, SCALE_FACTOR};

pub static KEY: OnceCell<Event> = OnceCell::new();

pub fn keyboard_light(handle: tauri::AppHandle) {

    let win_key = "msg-keydownaa".to_string();
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("键盘监听窗口已启动!");
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
                win.hide();
          
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
                            
                            // KEY
                            // .set(evet.clone())
                            // .unwrap_or_else(|_| println!("try insert pool cell failure!"));
                            println!("按下{}", key.clone());
                            win.emit("key_down_msg", key.clone()).unwrap();
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
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }


}




pub fn open_msg<R: Runtime>(handle: &tauri::AppHandle<R>, msg: String) {
    println!("执行msg");
    let win_key = "msg-keydown".to_string();
    if let Some(_win) = handle.get_webview_window(&win_key) {
        println!("窗口已启动!");
    } else {
        let win_list = handle.webview_windows();
        let mut win_num = 0;
        for (key, _value) in win_list {
            if key.starts_with("msg") {
                win_num += 1;
            }
        }

        let physize = PHYSIZE.get().expect("Error get pool from OneCell<Pool>");
        let width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as f64;
        let height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as f64;

        // 1+1等于多少
        let docs_window = tauri::WebviewWindowBuilder::new(
            handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/KeyDown".parse().unwrap()),
        )
        .title(msg)
        .inner_size(width, 60.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .position(0.0, height - 400.0)
        .build();

        match docs_window {
            Ok(win) => {
                let _ = win.set_always_on_top(true);
                // 获取主窗口所在的屏幕
                if let Some(_monitor) = win.primary_monitor().unwrap() {
                    let _ = win.set_position(PhysicalPosition::new(0.0 + win_num as f64 * 60.0, 400.0));
        

                } else {
                    println!("Could not get monitor information");
                }
                let ten_millis = time::Duration::from_millis(3000);
                sleep(ten_millis);
                win.close();
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }
}