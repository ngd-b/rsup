#!/bin/bash

# version info and download url
VERSION="0.1.0"
URL="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"

# download package
# wget $URL -O rsup.tar.gz
curl -L $URL -o rsup.tar.gz

# unzip
tar -xzvf rsup.tar.gz

# move the command to path
sudo mv rsup /usr/local/bin

# clean
rm rsup.tar.gz

echo "rsup install success"