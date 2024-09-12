use chrono::Local;
use tauri::{
    menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, Emitter, Manager, Runtime
};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let json_i = MenuItem::with_id(app, "json", "Json", true, None::<&str>)?;
    let cron_title_i = MenuItem::with_id(app, "cronTitle", "定时器", true, None::<&str>)?;
    let rsa_page_i = MenuItem::with_id(app, "rsaPage", "RSA", true, None::<&str>)?;
    let current_time_i = MenuItem::with_id(app, "currentTime", "获取当前时间戳", true, None::<&str>)?;



    let menu = Menu::with_items(app, &[&json_i, &cron_title_i, &rsa_page_i, &current_time_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            },
            "json" => {
                open_web(app, String::from("Json"));
            },
            "cronTitle" => {
                open_web(app, String::from("CronTitle"));
            },
            "rsaPage" => {
                open_web(app, String::from("RsaPage"));
            },
            "currentTime" => {
                let current_time = Local::now();
                match app.clipboard().write_text(current_time.timestamp_millis().to_string()) {
                    Ok(_) => {
                        let _ = app.dialog()
                        .message("复制成功")
                        .kind(MessageDialogKind::Info)
                        .title("提示")
                        .blocking_show();
                    },
                    Err(_) => {
                        println!("复制失败!");
                    },
                }
            },
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}


pub fn open_web<R: Runtime>(app: &tauri::AppHandle<R>, path: String) {
    let docs_window = tauri::WebviewWindowBuilder::new(
        app,
        ("tool-".to_string() + path.as_str()).to_string(), /* the unique window label */
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
