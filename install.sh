#!/bin/bash

# exit when error occurring
set -e

# VERSION="0.1.1"
# fix the tag VERSION and fix downlaod url
VERSION="latest"

echo "Thank you install rsup command tool! happy day for you!"
echo "                        "
echo "                        "
echo "                             -- by hboot"

# fail execute if any command errors
set -o pipefail

# 让用户选择从github or gitee下载
# echo "Do you want to install from Github(default) or Gitee? "

# # version info and download url
# URL="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"
# # web service static file 
# rsup_web="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"

# options=("Github" "Gitee")
# select opt in "${options[@]}"; 
# do
#     case $opt in
#         Github)
#             URL="https://github.com/ngd-b/rsup/releases/download/${VERSION}/rsup.tar.gz"
#             rsup_web="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"
#             break
#             ;;
#         Gitee)
            
#             break
#             ;;
#         *)
#             echo "Invalid option. Please select again."
#             ;;
#     esac
# done
URL=""

# 根据系统判断下载哪个执行文件
OS=$(uname -s)
case $OS in 
    Darwin)
        URL="https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-macos-latest.tar.gz"
        ;;
    Linux)
        URL = "https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-ubuntu-latest.tar.gz"
        ;;
    *)  
        echo "This script is running on a Windows system."
        echo "Please install the required software manually by following these steps:"
        echo ""
        echo "1. Download the rsup installer from https://github.com/ngd-b/rsup/releases/download/latest/rsup-installer-windows-latest.zip"
        echo "2. Unzip the file"
        echo "3. Run the installer"
        echo "4. Add the rsup directory to your PATH"
        echo "5. Run the command 'rsup'"
        echo "                        "

        exit 1
        ;;
esac

# download package
echo "download rsup froom $URL"
echo "                        "
echo "                        "
# wget $URL -O rsup-installer.tar.gz
curl -L $URL -o rsup-installer.tar.gz

# check file suffix type
if file rsup-installer.tar.gz | grep -q "gzip compressed data"; then
    echo "rsup-installer.tar.gz is valid"
else 
    echo "download file is is invalid"
    exit 1
fi

# test unzip
tar -tzvf rsup-installer.tar.gz > /dev/null

# unzip
tar -xzvf rsup-installer.tar.gz

# 执行安装文件
sudo ./installer

# clean
rm rsup-installer.tar.gz installer
echo "                        "
echo "                        "
echo "                        "
echo "                        "
echo "rsup install success,you can use rsup to manage you npm package!"