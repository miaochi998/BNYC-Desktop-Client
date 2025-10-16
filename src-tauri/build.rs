fn main() {
    // 尝试加载 ../.env 文件（desktop-client/.env）
    // 如果文件不存在也没关系，会使用代码中的默认值
    if let Ok(_) = dotenvy::from_filename("../.env") {
        println!("cargo:warning=已加载 desktop-client/.env 环境配置");
    } else {
        println!("cargo:warning=未找到 desktop-client/.env 文件，将使用默认配置");
    }
    
    tauri_build::build()
}

