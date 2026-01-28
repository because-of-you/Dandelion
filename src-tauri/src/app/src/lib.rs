use log::info;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn my_custom_command() -> String {
    info!("被调用了");
    println!("I was invoked from JavaScript!");
    "I was invoked from JavaScript!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("开始运行内容");
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .targets([Target::new(TargetKind::Stdout)])
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools()
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
