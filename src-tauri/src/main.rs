// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// 环境配置
// 开发环境加载本地Web端
#[cfg(debug_assertions)]
const APP_URL: &str = "http://localhost:6201";

// 生产环境加载远程Web服务器
#[cfg(not(debug_assertions))]
const APP_URL: &str = "https://app.bnyc.com"; // TODO: 替换为实际的生产域名

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // 应用启动时的初始化逻辑
            println!("BNYC Desktop Client 启动成功");
            println!("加载URL: {}", APP_URL);
            
            // 获取主窗口
            let window = app.get_webview_window("main").expect("无法获取主窗口");
            
            // 加载远程URL
            let _ = window.navigate(url::Url::parse(APP_URL).unwrap());
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时发生错误");
}
