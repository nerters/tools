use std::sync::Arc;

use idgen::IDGen;
use lazy_static::lazy_static;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tauri::{async_runtime::spawn, utils::config::Position, Manager, PhysicalPosition, Runtime};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_plugin_shell::ShellExt;
use tokio::sync::Mutex;

use crate::{dao::hot_key::{self, HotKey}, ALLAMA, PHYSIZE};

lazy_static! {
    pub static ref CRON_MAP: Arc<Mutex<Vec<String>>> =
        Arc::new(Mutex::new(Vec::new()));
}



pub fn create_host_key<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    if app.clone().remove_plugin("global-shortcut")  {
        println!("成功");
    } else {
        println!("失败");
    }  

    match app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts([
                "shift+control+Digit1",
                "shift+control+Digit2",
                "shift+control+Digit3",
                "shift+control+Digit4",
                "shift+control+Digit5",
                "shift+control+Digit6",
                "shift+control+Digit7",
                "shift+control+Digit8",
                "shift+control+Digit9",
                "shift+control+Digit0",
                "shift+control+KeyA",
                "shift+control+KeyB",
                "shift+control+KeyC",
                "shift+control+KeyD",
                "shift+control+KeyE",
                "shift+control+KeyF",
                "shift+control+KeyG",
                "shift+control+KeyH",
                "shift+control+KeyI",
                "shift+control+KeyJ",
                "shift+control+KeyK",
                "shift+control+KeyL",
                "shift+control+KeyM",
                "shift+control+KeyN",
                "shift+control+KeyO",
                "shift+control+KeyP",
                "shift+control+KeyQ",
                "shift+control+KeyR",
                "shift+control+KeyS",
                "shift+control+KeyT",
                "shift+control+KeyU",
                "shift+control+KeyV",
                "shift+control+KeyW",
                "shift+control+KeyY",

                "shift+control+alt+KeyA",
                "shift+control+alt+KeyB",
                "shift+control+alt+KeyC",
                "shift+control+alt+KeyD",
                "shift+control+alt+KeyE",
                "shift+control+alt+KeyF",
                "shift+control+alt+KeyG",
                "shift+control+alt+KeyH",
                "shift+control+alt+KeyI",
                "shift+control+alt+KeyJ",
                "shift+control+alt+KeyK",
                "shift+control+alt+KeyL",
                "shift+control+alt+KeyM",
                "shift+control+alt+KeyN",
                "shift+control+alt+KeyO",
                "shift+control+alt+KeyP",
                "shift+control+alt+KeyQ",
                "shift+control+alt+KeyR",
                "shift+control+alt+KeyS",
                "shift+control+alt+KeyT",
                "shift+control+alt+KeyU",
                "shift+control+alt+KeyV",
                "shift+control+alt+KeyW",
                "shift+control+alt+KeyX",
                "shift+control+alt+KeyY",
                "shift+control+alt+KeyZ",

            ]).unwrap()
            .with_handler(move |app, shortcut: &tauri_plugin_global_shortcut::Shortcut, event| {
                println!("{}", shortcut.into_string());

                if event.state() == ShortcutState::Pressed {
                    let ele = hot_key::get_info_by_key(shortcut.into_string());
                    println!("{}", ele.path);
                    let shell = ele.shell;

                    if ele.path.eq("webPage") {
                        println!("{}", ele.url.clone());
                        let _ = app.shell().open(ele.url.clone(), None);
                    
                    } if ele.path.eq("openProgram") {
                        let app_clone = app.clone();
                        let shell = shell.clone();
                        if shell.is_empty() {
                            let _ = app_clone.dialog()
                            .message("shell为空！")
                            .kind(MessageDialogKind::Info)
                            .title("提示")
                            .blocking_show();
                            return;
                        }
                        spawn(async move {
                            let output = app_clone.shell()
                            .command(shell) 
                            .output()
                            .await;
                            match output {
                                Ok(output) => {
                                    if output.status.success() {
                                        println!("Result: {:?}", String::from_utf8(output.stdout));
                                    } else {
                                        let path = format!("执行失败: {}", output.status.code().unwrap());
                                        let _ = app_clone.dialog()
                                        .message(path)
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                                        println!("Exit with code: {}", output.status.code().unwrap());
                                    }
                                },
                                Err(e) => {
                                    let path = format!("执行失败：{}", e);
                                    let _ = app_clone.dialog()
                                        .message(path)
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                                    println!("{}", e);
                                    println!("执行shell失败！");
                                },
                            }
                        }); 
                    } if ele.path.eq("doShell") {
                        let app_clone = app.clone();
                        let shell = shell.clone();
                        if shell.is_empty() {
                            let _ = app_clone.dialog()
                            .message("shell为空！")
                            .kind(MessageDialogKind::Info)
                            .title("提示")
                            .blocking_show();
                            return;
                        }
                        spawn(async move {
                            let output = app_clone.shell()
                            .command("powershell") 
                            .args(&[
                                shell
                            ])
                            .output()
                            .await;
                            match output {
                                Ok(output) => {
                                    if output.status.success() {
                                        let msg = String::from_utf8(output.stdout).unwrap();
                                        let _ = app_clone.dialog()
                                        .message(msg.clone())
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                                        //println!("Result: {:?}", msg);
                                    } else {
                                        let path = format!("执行失败: {}", output.status.code().unwrap());
                                        let _ = app_clone.dialog()
                                        .message(path)
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                                        //println!("Exit with code: {}", output.status.code().unwrap());
                                    }
                                },
                                Err(e) => {
                                    let path = format!("执行失败：{}", e);
                                    let _ = app_clone.dialog()
                                        .message(path)
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                                    println!("{}", e);
                                    println!("执行shell失败！");
                                },
                            }
                        });
                    } else if ele.path.eq("openFun") {
                        println!("open");
                        open_web(app, ele.url.clone(), ele.overopen == 1);
                    } else if ele.path.eq("ollama") {
                        //app.clipboard().write_text("Tauri is awesome!".to_string()).unwrap();

                        // Read content from clipboard
                        let content = app.clipboard().read_text();
                        let ask =  content.unwrap();
                        println!("{:?}", ask);
                        let app_clone = app.clone();
                        let model = shell.clone();
                        if model.is_empty() {
                            let _ = app_clone.dialog()
                            .message("模型为空！")
                            .kind(MessageDialogKind::Info)
                            .title("提示")
                            .blocking_show();
                            return;
                        }
                        spawn(async move {
                            let allama = ALLAMA.get().expect("Error get pool from OneCell<Pool>");
                            println!("{}", model);
                            let res = allama.generate(GenerationRequest::new(model.clone(), ask)).await;
                        
                            if let Ok(res) = res {
                                let msg = res.response.to_string();
                                open_msg(&app_clone, msg).await;
                                println!("{}", res.response);
                            } else {
                                let path = format!("{} 执行失败！", model);
                                let _ = app_clone.dialog()
                                        .message(path)
                                        .kind(MessageDialogKind::Info)
                                        .title("提示")
                                        .blocking_show();
                            }
                        });
                    }
                }
            })
            .build(),
    ){
        Ok(_) => {
            println!("成功")
        },
        Err(e) => {
            println!("失败： {}", e)
        },
    }
    Ok(())
}


pub fn add_host_key<R: Runtime>(app: &tauri::AppHandle<R>, hot_key: HotKey) -> tauri::Result<()> {
    println!("添加快捷键：{}, path:{}", hot_key.key, hot_key.path);
    match app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts([hot_key.key.as_str()]).unwrap()
            .with_handler(move |app, shortcut, event| {
                // 添加新的事件处理逻辑
                if hot_key.key.eq(&shortcut.into_string()) && event.state == ShortcutState::Pressed {
                    
                    open_web(app, hot_key.path.clone(), hot_key.overopen == 1);
                }
            })
            .build(),
    ){
        Ok(_) => {
            println!("成功")
        },
        Err(e) => {
            println!("失败： {}", e)
        },
    }
    Ok(())
}




pub fn open_web<R: Runtime>(app: &tauri::AppHandle<R>, path: String, overopen: bool) {
    let mut lebel = ("tool-".to_string() + path.as_str()).to_string();
    if overopen {
        let idgen = IDGen::new(1);
        let id = idgen.new_id();
        lebel += id.to_string().as_str();
    }
    let w = 800.0;
    let h = 600.0;
    let physize = PHYSIZE.get().expect("Error get pool from OneCell<Pool>");
    let width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as f64;
    let height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as f64;

    let mut position_x = width / 2.0 - w / 2.0;
    let mut position_y = height / 2.0 - h / 2.0;
    if position_y < 0.0 {
        position_y =  height / 2.0;
    }
    if position_x < 0.0 {
        position_x =  width / 2.0;
    }
    let docs_window = tauri::WebviewWindowBuilder::new(
        app,
        lebel, /* the unique window label */
        tauri::WebviewUrl::App(("/main/".to_string() + path.as_str()).parse().unwrap()),
    )
    .title(("tool-".to_string() + path.as_str()))
    .inner_size(w, h)
    .decorations(false)
    .focused(true)
    .position(position_x, position_y)
    .build();
    match docs_window {
        Ok(win) => {
            // 获取主窗口所在的屏幕
            if let Some(monitor) = win.primary_monitor().unwrap() {
                let size = monitor.size();
                let width = size.width as f64;
                let height = size.height as f64;

                let mut position_x = width / 2.0 - w / 2.0;
                let mut position_y = height / 2.0 - h / 2.0;
                if position_y < 0.0 {
                    position_y =  height / 2.0;
                }
                if position_x < 0.0 {
                    position_x =  width / 2.0;
                }
                let _ = win.set_position(PhysicalPosition::new(position_x, position_y));
            
                println!("Screen resolution: {}x{}", width, height);
            } else {
                println!("Could not get monitor information");
            }
            println!("启动窗口成功!");
        }
        Err(_) => {
            println!("启动窗口失败!");
        }
    }
}



async fn open_msg<R: Runtime>(handle: &tauri::AppHandle<R>, msg: String) {
    println!("执行msg");
    let idgen = IDGen::new(1);
    let id = idgen.new_id();
    let win_key = "msg-".to_string() + id.to_string().as_str();
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
        //let width = physize.get("width").unwrap_or(&(1920 as u32)).clone() as f64;
        let height = physize.get("height").unwrap_or(&(1080 as u32)).clone() as f64;
        //println!("Screen resolution: {}x{}", width, height);
        let position_x = 0.0 + (win_num as f64) * 50.0;
        let mut position_y = (height - 400.0) as f64;
        let t: i32 = (win_num / 2) % 2;
        if t == 1 {
            position_y = (height - 400.0) + (win_num % 2) as f64 * 50.0;
        } else {
            position_y = (height - 400.0) + (2 - win_num % 2) as f64 * 50.0;
        }
        println!("Screen resolution: {}", position_y);
// 1+1等于多少
        let docs_window = tauri::WebviewWindowBuilder::new(
            handle,
            win_key, /* the unique window label */
            tauri::WebviewUrl::App("/hint/message".parse().unwrap()),
        )
        .title(msg)
        .inner_size(300.0, 150.0)
        .decorations(false)
        .transparent(true)
        .resizable(false)
        .position(position_x, position_y)
        .build();

        match docs_window {
            Ok(win) => {
                let _ = win.set_always_on_top(true);
                // 获取主窗口所在的屏幕
                if let Some(monitor) = win.primary_monitor().unwrap() {
                    let size = monitor.size();
                    let width = size.width;
                    let height = size.height;

                    let position_x = 0.0 + (win_num as f64) * 50.0;
                    let mut position_y = (height - 400) as f64;
                    let t: i32 = (win_num / 2) % 2;
                    if t == 1 {
                        position_y = (height - 400) as f64 + (win_num % 2) as f64 * 50.0;
                    } else {
                        position_y = (height - 400) as f64 + (2 - win_num % 2) as f64 * 50.0;
                    }
                    let _ = win.set_position(PhysicalPosition::new(position_x, position_y));
                
                    println!("Screen resolution: {}x{}", width, height);
                } else {
                    println!("Could not get monitor information");
                }
            }
            Err(_) => {
                println!("启动窗口失败!");
            }
        }
    }
}

#[test]
fn test1() {
    test()
}

#[tokio::main]
async fn test() {
    // For custom values:
    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "llama3.2".to_string();
    let prompt = "Why is the sky blue?".to_string();


    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        let msg = res.response.to_string();
       
        println!("{}", res.response);
    }
 
}