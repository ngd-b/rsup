#!/bin/bash

# build command 
cargo build --release -p core


# build file path
BINARY_PATH="target/release/rsup"

# 检查二进制文件是否存在
if [ -f "$BINARY_PATH" ]; then
    echo "Binary build successful: $BINARY_PATH"
    
    # 删除原来的压缩包
    rm  rsup.tar.gz

    # 创建一个压缩包，将编译后的二进制文件打包到根目录下
    tar -czvf rsup.tar.gz -C target/release rsup

    echo "Binary successfully compressed to rsup.tar.gz"
else
    echo "Error: Binary not found!"
    exit 1
fi