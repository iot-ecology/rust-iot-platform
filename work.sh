#!/bin/bash


header='/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
'

# 遍历所有 .go 文件
find . -type f -name "*.rs" | while read -r file; do
    # 判断文件是否已有版权头（检查前几行是否包含关键词 Copyright）
    if head -n 10 "$file" | grep -q "Copyright"; then
        echo "Skipped (already has header): $file"
    else
        echo "Adding header to $file"
        # 创建临时文件，先写header，再写原文件内容
        tmpfile=$(mktemp)
        echo "$header" > "$tmpfile"
        cat "$file" >> "$tmpfile"
        mv "$tmpfile" "$file"
    fi
done