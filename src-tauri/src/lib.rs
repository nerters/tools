use std::collections::HashMap;

use serde_json::{Error, Value};
mod json;
mod img;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn json_diff(old_json: String, new_json: String) -> HashMap<String, String> {
    let mut rest = HashMap::new();
    let old: Result<Value, Error> = serde_json::from_str(&old_json);
    let mut old_text = Value::default();
    match old {
        Ok(text) => old_text = text,
        Err(_) => {
            println!("解析old错误！");
        }
    }
    let mut new_text = Value::default();
    let new: Result<Value, Error> = serde_json::from_str(&new_json);
    match new {
        Ok(text) => new_text = text,
        Err(_) => {
            println!("解析new错误！");
        }
    }

    let (old_res, new_res) = json::json_utils::json_diff(&old_text, &new_text);
    rest.insert(
        "old".to_string(),
        match serde_json::to_string_pretty(&old_res) {
            Ok(data) => data
                .replace("  ", "&emsp;&emsp;&emsp;")
                .replace("\n", "<br>")
                .replace("\\\"", "\""),
            Err(_) => "".to_string(),
        },
    );
    rest.insert(
        "new".to_string(),
        match serde_json::to_string_pretty(&new_res) {
            Ok(data) => data
                .replace("  ", "&emsp;&emsp;&emsp;")
                .replace("\n", "<br>")
                .replace("\\\"", "\""),
            Err(_) => "".to_string(),
        },
    );
    rest
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![json_diff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
