#!/bin/bash

# 远程仓库分支名称
remote_github="github"
remote_gitee="origin"
branch="master"

# 是否需要添加所有更改
read -p "Do you want to add all changes and commit? [y/n]: " is_add

# 根据输入确认是否默认提交
if [ "$is_add" == "y" ] || [ "$is_add" == "Y" ]; then
  echo "Add all changes and commit"
  git add .
  # 获取提交信息
  read -p "Please enter the commit information: " msg

  # 提交所有更改
  echo "Commit all changes with message: $msg"
  git commit -m "$msg"
else    
  echo "Skip add all changes and commit"
fi


# 推送到gitee
echo "Push to gitee"
git push -u $remote_gitee $branch

# 推送到github
echo "Push to github"
git push -u $remote_github $branch:main

echo "Push to github and gitee successfully"
