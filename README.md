# BNYC 桌面客户端

> RustDesk客户端定制SaaS系统 - 桌面客户端

## 📋 项目简介

BNYC桌面客户端是基于Tauri 2.0开发的WebView容器应用，提供与Web端完全一致的功能，同时增加了桌面特有的体验：

- ✅ 双击图标即可启动，无需打开浏览器
- ✅ 系统托盘支持（最小化到托盘）
- ✅ 系统通知
- ✅ 自动登录（保存Token）
- ✅ 开机自启动（可选）

## 🏗️ 技术架构

**核心特性：WebView容器式架构**

```
桌面客户端 (Tauri Shell)
    ↓
加载远程Web应用
    ↓
开发环境: http://localhost:6201
生产环境: https://app.bnyc.com
```

**优势：**
- Web端更新，桌面端自动更新
- 无需每次重新发布客户端
- 用户体验与微信/钉钉等主流应用一致

## 🚀 快速开始

### 环境要求

- **Rust**: 1.70+
- **Node.js**: 18+
- **操作系统**: Windows 10+, macOS 10.13+, 或 Linux

### 安装依赖

```bash
# 确保已安装Rust
# 如未安装，运行：
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 确保已安装Tauri CLI
cargo install tauri-cli

# 安装Node依赖（如有）
npm install
```

### 开发模式

```bash
# 启动开发模式（会加载 http://localhost:6201）
npm run dev

# 或使用cargo命令
cargo tauri dev
```

**注意：** 开发模式下，需要先启动Web端开发服务器：

```bash
# 在另一个终端
cd ../saas/customizer-web
npm run dev
```

### 构建生产版本

```bash
# 构建当前平台的安装包
npm run build

# 构建特定平台
npm run build:windows          # Windows x64
npm run build:macos-intel       # macOS Intel
npm run build:macos-silicon     # macOS Apple Silicon
npm run build:macos-universal   # macOS Universal（推荐）
```

## 📦 构建产物

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── msi/              # Windows安装包
├── dmg/              # macOS安装包
├── deb/              # Linux Debian包
└── appimage/         # Linux AppImage
```

## 🎨 图标配置

应用图标位于 `src-tauri/icons/` 目录：

```
src-tauri/icons/
├── 32x32.png         # 32x32 图标
├── 128x128.png       # 128x128 图标
├── 128x128@2x.png    # 256x256 图标
├── icon.icns         # macOS图标
├── icon.ico          # Windows图标
└── icon.png          # 托盘图标
```

**如需自定义图标：**

1. 准备一张1024x1024的PNG图标
2. 使用在线工具生成各种尺寸：https://icon.kitchen/
3. 替换 `src-tauri/icons/` 目录下的所有文件

## 🔧 配置说明

### 环境配置

在 `src-tauri/src/main.rs` 中配置服务器地址：

```rust
// 开发环境
#[cfg(debug_assertions)]
const APP_URL: &str = "http://localhost:6201";

// 生产环境
#[cfg(not(debug_assertions))]
const APP_URL: &str = "https://app.bnyc.com"; // 👈 修改为实际域名
```

### Tauri配置

核心配置文件：`src-tauri/tauri.conf.json`

主要配置项：
- `identifier`: 应用唯一标识符
- `productName`: 应用显示名称
- `version`: 应用版本号
- `bundle`: 打包相关配置

## 📱 功能特性

### 系统托盘

- 双击托盘图标：显示/隐藏主窗口
- 右键菜单：
  - 显示主窗口
  - 隐藏窗口
  - 退出程序

### 窗口行为

- 点击关闭按钮：隐藏到托盘（不退出）
- 完全退出：通过托盘菜单选择"退出程序"

### 系统通知

使用Tauri通知插件，提供原生系统通知体验。

## 🚀 GitHub Actions 自动编译

项目配置了GitHub Actions自动编译，支持：

- ✅ Windows x64
- ✅ macOS Intel
- ✅ macOS Apple Silicon

**触发方式：**

```bash
# 创建版本标签
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions自动编译3个平台
# 编译完成后在Actions页面下载安装包
```

## 📖 开发指南

### 项目结构

```
desktop-client/
├── public/              # 静态文件（加载页面）
│   └── index.html
├── src-tauri/          # Rust后端
│   ├── src/
│   │   └── main.rs    # 主程序入口
│   ├── icons/         # 应用图标
│   ├── Cargo.toml     # Rust依赖
│   ├── tauri.conf.json # Tauri配置
│   └── build.rs       # 构建脚本
├── package.json       # Node配置
└── README.md         # 本文档
```

### 添加新功能

1. **系统托盘菜单项**：修改 `src-tauri/src/main.rs` 中的 `SystemTrayMenu`
2. **窗口配置**：修改 `src-tauri/tauri.conf.json` 中的 `windows` 配置
3. **应用权限**：修改 `Cargo.toml` 中的 `features`

### 调试技巧

```bash
# 开发模式下查看Rust日志
RUST_LOG=debug npm run dev

# 查看完整的编译输出
npm run build -- --verbose
```

## ⚠️ 常见问题

### Q1: 开发模式无法启动

**A:** 确保Web端开发服务器已启动（端口6201）

```bash
cd ../saas/customizer-web
npm run dev
```

### Q2: 构建失败

**A:** 检查Rust和Tauri CLI版本：

```bash
rustc --version  # 应该 >= 1.70
cargo tauri --version
```

### Q3: 图标不显示

**A:** 确保 `src-tauri/icons/` 目录下有所有必需的图标文件

### Q4: 生产环境无法连接

**A:** 修改 `src-tauri/src/main.rs` 中的 `APP_URL` 为实际的生产域名

### Q5: 鼠标滚轮无法滚动页面（已知问题）

**A:** 这是Tauri 2.x在macOS上的已知限制：

- ✅ 浏览器中滚轮正常工作
- ✅ 客户端中可以使用鼠标拖动滚动条
- ✅ 触控板（两指滑动）可能可以正常使用
- ❌ 鼠标滚轮在Tauri客户端中不工作

**临时解决方案：**
1. 使用鼠标拖动滚动条
2. 使用触控板的两指滑动（如果可用）
3. 使用Web浏览器版本（完全正常）

此问题正在等待Tauri官方修复。

## 📝 更新日志

### v1.0.2 (2025-10-17)

- 🧹 **代码优化**
  - 清理了所有无用的测试代码
  - 移除自动打开开发者工具
  - 简化main.rs代码结构
  - 删除测试用HTML文件

### v1.0.1 (2025-10-17)

- 🔧 **滚轮问题调查**
  - 确认为Tauri 2.x + macOS WKWebView的已知问题
  - Web应用CSS已优化，浏览器中完全正常
  - 鼠标拖动滚动条功能正常

### v1.0.0 (2025-01-15)

- ✅ 初始版本发布
- ✅ 基于Tauri 2.0
- ✅ WebView容器式架构
- ✅ 系统托盘支持
- ✅ 系统通知支持
- ✅ 跨平台编译（Windows/macOS/Linux）

## 📞 技术支持

- **项目仓库**: https://github.com/your-org/bnyc-desktop-client
- **问题反馈**: https://github.com/your-org/bnyc-desktop-client/issues
- **文档**: https://bnyc.com/docs

## 📄 许可证

MIT License

Copyright © 2025 BNYC Team. All rights reserved.

---

**开发团队**: BNYC Team  
**最后更新**: 2025-01-15


