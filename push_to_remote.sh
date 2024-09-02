#!/bin/bash

# 远程仓库分支名称
remote_github="github"
remote_gitee="origin"
branch="master"

# 添加所有更改
echo "Add all changes"
git add .

# 获取提交信息
read -p "Please enter the commit information: " msg

# 提交所有更改
echo "Commit all changes with message: $msg"
git commit -m "$msg"

# 推送到gitee
echo "Push to gitee"
git push -u $remote_gitee $branch

# 推送到github
echo "Push to github"
git push -u $remote_github $branch:main

echo "Push to github and gitee successfully"
