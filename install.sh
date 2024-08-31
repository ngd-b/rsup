#!/bin/bash

# VERSION="0.1.1"
# fix the tag VERSION and fix downlaod url
VERSION="latest"

echo "Thank you install rsup command! the current version is $VERSION"
# exit when error occurring
set -e

# fail execute if any command errors
set -o pipefail

# version info and download url
URL="https://gitee.com/hboot/rsup/releases/download/${VERSION}/rsup.tar.gz"

# download package
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

echo "rsup install success,you can use rsup to manage you npm package!"