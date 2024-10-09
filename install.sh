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
echo "Do you want to install from Github(default) or Gitee? "

# version info and download url
URL="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"
# web service static file 
rsup_web="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"

options=("Github" "Gitee")
select opt in "${options[@]}"; 
do
    case $opt in
        Github)
            URL="https://github.com/ngd-b/rsup/releases/download/${VERSION}/rsup.tar.gz"
            rsup_web="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"
            break
            ;;
        Gitee)
            
            break
            ;;
        *)
            echo "Invalid option. Please select again."
            ;;
    esac
done

# download package
echo "download rsup froom $URL"
echo "                        "
echo "                        "
# wget $URL -O rsup.tar.gz
curl -L $URL -o rsup.tar.gz

# check file suffix type
if file rsup.tar.gz | grep -q "gzip compressed data"; then
    echo "rsup.tar.gz is valid"
else 
    echo "download file is is invalid"
    exit 1
fi

# test unzip
tar -tzvf rsup.tar.gz > /dev/null

# unzip
tar -xzvf rsup.tar.gz

# move the command to path
sudo mv rsup /usr/local/bin

# clean
rm rsup.tar.gz
echo "                        "
echo "                        "
echo "                        "
echo "                        "
echo "rsup install success,you can use rsup to manage you npm package!"