# 应用图标说明

## 📁 所需图标文件

本目录需要以下图标文件：

```
icons/
├── 32x32.png         # 32x32 像素
├── 128x128.png       # 128x128 像素
├── 128x128@2x.png    # 256x256 像素
├── icon.icns         # macOS 图标
├── icon.ico          # Windows 图标
└── icon.png          # 托盘图标 (通常128x128或256x256)
```

## 🎨 快速生成图标

### 方法1: 使用在线工具（推荐）

1. 访问：https://icon.kitchen/
2. 上传一张1024x1024的PNG图标
3. 下载生成的所有图标文件
4. 复制到本目录

### 方法2: 使用命令行工具

```bash
# 安装iconutil (macOS)
# 已内置，无需安装

# 从1024x1024的source.png生成所有尺寸
mkdir MyIcon.iconset
sips -z 32 32     source.png --out MyIcon.iconset/icon_32x32.png
sips -z 64 64     source.png --out MyIcon.iconset/icon_32x32@2x.png
sips -z 128 128   source.png --out MyIcon.iconset/icon_128x128.png
sips -z 256 256   source.png --out MyIcon.iconset/icon_128x128@2x.png
sips -z 256 256   source.png --out MyIcon.iconset/icon_256x256.png
sips -z 512 512   source.png --out MyIcon.iconset/icon_256x256@2x.png
sips -z 512 512   source.png --out MyIcon.iconset/icon_512x512.png
sips -z 1024 1024 source.png --out MyIcon.iconset/icon_512x512@2x.png

# 生成.icns文件
iconutil -c icns MyIcon.iconset -o icon.icns
```

### 方法3: 使用Tauri官方工具

```bash
# 安装tauri-icon
cargo install tauri-icon

# 从source.png生成所有图标
tauri-icon path/to/source.png
```

## 🔍 图标规范

### 设计建议

- **尺寸**: 源图标至少1024x1024像素
- **格式**: PNG（带透明通道）
- **内容**: 简洁、识别度高
- **背景**: 透明背景（推荐）
- **边距**: 留出10-15%的边距

### 托盘图标特殊要求

托盘图标（icon.png）建议：
- macOS: 单色、支持暗黑模式
- Windows: 彩色、16x16或32x32
- Linux: 彩色、48x48或更大

## 🖼️ 临时图标

在正式图标准备好之前，可以使用以下工具生成临时图标：

1. **Placeholder.com**: https://placeholder.com/
2. **UI Avatars**: https://ui-avatars.com/
3. **Favicon.io**: https://favicon.io/

示例：
```bash
# 下载一个临时图标
curl -o icon.png "https://ui-avatars.com/api/?name=BNYC&size=512&background=667eea&color=fff"
```

## ✅ 验证图标

图标准备完成后，检查：

- [ ] 所有必需的文件都存在
- [ ] PNG文件透明背景正常
- [ ] icns文件能在macOS上显示
- [ ] ico文件能在Windows上显示
- [ ] 图标在不同尺寸下清晰可见

## 🚀 应用图标

将图标文件放入本目录后，重新构建应用即可：

```bash
npm run build
```

---

**注意**: 如果没有准备图标，Tauri会使用默认图标。建议在正式发布前替换为项目专属图标。

