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
        .setup(|app| {
            // 应用启动时的初始化逻辑
            println!("BNYC Desktop Client 启动成功");
            println!("加载URL: {}", APP_URL);
            
            // 获取主窗口
            let window = app.get_webview_window("main").expect("无法获取主窗口");
            
            // 加载远程URL
            let _ = window.navigate(url::Url::parse(APP_URL).unwrap());
            
            // 延迟2秒后注入滚动修复脚本（等待页面加载）
            let window_clone = window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let _ = window_clone.eval(
                    r#"
                    // 确保鼠标滚轮事件正确传递到滚动容器
                    (function() {
                        // 等待DOM加载完成
                        if (document.readyState === 'loading') {
                            document.addEventListener('DOMContentLoaded', initScroll);
                        } else {
                            initScroll();
                        }
                        
                        function initScroll() {
                            // 查找主内容滚动容器
                            const findScrollContainer = () => {
                                return document.querySelector('.main-content') || 
                                       document.querySelector('.ant-layout-content') ||
                                       document.body;
                            };
                            
                            // 添加滚轮事件监听
                            let scrollContainer = findScrollContainer();
                            
                            // 如果初次没找到，等待一下再试（React可能还在渲染）
                            if (!scrollContainer || scrollContainer === document.body) {
                                setTimeout(() => {
                                    scrollContainer = findScrollContainer();
                                    attachWheelHandler(scrollContainer);
                                }, 1000);
                            } else {
                                attachWheelHandler(scrollContainer);
                            }
                            
                            function attachWheelHandler(container) {
                                if (!container) return;
                                
                                // 捕获鼠标滚轮事件并传递给容器
                                document.addEventListener('wheel', function(e) {
                                    if (container && container.scrollHeight > container.clientHeight) {
                                        e.preventDefault();
                                        container.scrollTop += e.deltaY;
                                    }
                                }, { passive: false });
                                
                                console.log('[BNYC Desktop] 鼠标滚轮支持已启用');
                            }
                        }
                    })();
                    "#
                );
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("运行Tauri应用时发生错误");
}
