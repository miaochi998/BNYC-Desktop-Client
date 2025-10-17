fn main() {
    // 尝试加载 ../.env 文件（desktop-client/.env）
    if let Ok(_) = dotenvy::from_filename("../.env") {
        println!("cargo:warning=已加载 desktop-client/.env 环境配置");
        
        // 将APP_URL环境变量传递给编译器
        if let Ok(app_url) = std::env::var("APP_URL") {
            println!("cargo:rustc-env=APP_URL={}", app_url);
            println!("cargo:warning=使用APP_URL: {}", app_url);
        }
    } else {
        println!("cargo:warning=未找到 desktop-client/.env 文件，将使用默认配置");
    }
    
    tauri_build::build()
}

