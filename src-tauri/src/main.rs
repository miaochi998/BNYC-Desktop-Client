// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// 环境配置：编译时从环境变量APP_URL读取，如果未设置则使用默认值
// 修改 .env 文件后需要重新编译
const APP_URL: &str = match option_env!("APP_URL") {
    Some(url) => url,
    None => "http://localhost:6201", // 默认值
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时发生错误");
}
