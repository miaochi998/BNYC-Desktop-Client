#!/bin/bash

# 创建临时图标的脚本

echo "正在创建临时图标..."

cd src-tauri/icons

# 下载临时图标（使用UI Avatars生成）
curl -s -o icon-temp.png "https://ui-avatars.com/api/?name=BNYC&size=512&background=667eea&color=fff&bold=true&font-size=0.4"

# 检查是否下载成功
if [ ! -f icon-temp.png ]; then
    echo "下载失败，创建纯色占位图标..."
    # 如果下载失败，使用sips创建纯色图标（仅macOS）
    if command -v sips &> /dev/null; then
        # 创建一个简单的512x512蓝色方块
        sips -z 512 512 --setProperty format png -c 512 512 icon-temp.png 2>/dev/null || true
    fi
fi

# 生成各种尺寸
if [ -f icon-temp.png ]; then
    if command -v sips &> /dev/null; then
        echo "生成各种尺寸的图标..."
        sips -z 32 32 icon-temp.png --out 32x32.png
        sips -z 128 128 icon-temp.png --out 128x128.png
        sips -z 256 256 icon-temp.png --out 128x128@2x.png
        cp icon-temp.png icon.png
        
        echo "生成macOS icns文件..."
        # 创建iconset
        mkdir -p MyIcon.iconset
        sips -z 16 16     icon-temp.png --out MyIcon.iconset/icon_16x16.png
        sips -z 32 32     icon-temp.png --out MyIcon.iconset/icon_16x16@2x.png
        sips -z 32 32     icon-temp.png --out MyIcon.iconset/icon_32x32.png
        sips -z 64 64     icon-temp.png --out MyIcon.iconset/icon_32x32@2x.png
        sips -z 128 128   icon-temp.png --out MyIcon.iconset/icon_128x128.png
        sips -z 256 256   icon-temp.png --out MyIcon.iconset/icon_128x128@2x.png
        sips -z 256 256   icon-temp.png --out MyIcon.iconset/icon_256x256.png
        sips -z 512 512   icon-temp.png --out MyIcon.iconset/icon_256x256@2x.png
        sips -z 512 512   icon-temp.png --out MyIcon.iconset/icon_512x512.png
        sips -z 1024 1024 icon-temp.png --out MyIcon.iconset/icon_512x512@2x.png
        
        # 生成icns
        iconutil -c icns MyIcon.iconset -o icon.icns
        rm -rf MyIcon.iconset
        
        echo "✅ 临时图标创建成功！"
    else
        echo "⚠️  sips命令不可用，跳过图标生成"
        echo "   请手动准备图标文件"
    fi
else
    echo "❌ 无法创建图标文件"
    echo "   请手动准备以下文件："
    echo "   - 32x32.png"
    echo "   - 128x128.png"
    echo "   - 128x128@2x.png"
    echo "   - icon.icns"
    echo "   - icon.ico"
    echo "   - icon.png"
fi

# 清理临时文件
rm -f icon-temp.png

echo "完成！"

