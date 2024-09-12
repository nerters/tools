use std::sync::Arc;

use idgen::IDGen;
use lazy_static::lazy_static;
use tauri::Runtime;
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_plugin_shell::{open::Program, ShellExt};
use tokio::sync::Mutex;

use crate::dao::hot_key::{self, HotKey};
mod dao;

lazy_static! {
    pub static ref CRON_MAP: Arc<Mutex<Vec<String>>> =
        Arc::new(Mutex::new(Vec::new()));
}



pub fn create_host_key<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let list = hot_key::get_list();
    let strs: Vec<&str> = list.iter().map(|ele| ele.key.as_str()).collect();

    if app.clone().remove_plugin("global-shortcut")  {
        println!("成功");
    } else {
        println!("失败");
    }  

    match app.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_shortcuts(strs).unwrap()
            .with_handler(move |app, shortcut, event| {
                println!("{}", shortcut.into_string());
                for ele in &list {
                    // 添加新的事件处理逻辑
                    if ele.key.eq(&shortcut.into_string()) && event.state == ShortcutState::Pressed {
                        if ele.path.eq("webPage") {
                            println!("{}", ele.url.clone());
                            app.shell().open(ele.url.clone(), None);
                        } else {
                            open_web(app, ele.path.clone(), ele.overopen == 1);
                        }
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
    let docs_window = tauri::WebviewWindowBuilder::new(
        app,
        lebel, /* the unique window label */
        tauri::WebviewUrl::App(("/main/".to_string() + path.as_str()).parse().unwrap()),
    )
    .title(("tool-".to_string() + path.as_str()))
    .inner_size(800.0, 600.0)
    .decorations(false)
    .focused(true)
    .position(500.0, 400.0)
    .build();
    match docs_window {
        Ok(win) => {
            println!("启动窗口成功!");
        }
        Err(_) => {
            println!("启动窗口失败!");
        }
    }
}