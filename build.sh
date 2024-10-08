#!/bin/bash

# 编译环境,当前的系统环境
os=$1

# build command 
cargo build --release -p core

# build file path
BINARY_PATH="target/release/rsup"

# 检查二进制文件是否存在
if [ -f "$BINARY_PATH" ]; then
    echo "Binary build successful: $BINARY_PATH"
    
    # 删除原来的压缩包
    if ["$os" = "windows-latest"];then
        rm rsup.zip
    else 
        rm rsup.tar.gz
    fi

    # 创建一个压缩包，将编译后的二进制文件打包到根目录下
    if ["$os" = "windows-latest"];then
        echo "Compressing binary for windows..."
        zip -r rsup.zip target/release/rsup.exe
        echo "Binary successfully compressed to rsup.zip"
    else 
        echo "Compressing binary for $os..."
        tar -czvf rsup.tar.gz -C target/release rsup
        echo "Binary successfully compressed to rsup.tar.gz"
    fi
else
    echo "Error: Binary not found!"
    exit 1
fi