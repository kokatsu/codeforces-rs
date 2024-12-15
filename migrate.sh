#!/bin/bash
set -x # デバッグモードを有効化

find . -regextype posix-basic -regex "./problemset/problem[0-9]\{4\}_[0-9]\{4\}/problem[0-9]\{4\}/Cargo\.toml" | while read -r file; do
    echo "Processing file: $file"
    problem_number=$(basename $(dirname "$file") | grep -o '[0-9]\{4\}')
    echo "Problem number: $problem_number"

    # ファイルの内容を表示
    echo "Current content:"
    cat "$file"

    temp_file=$(mktemp)

    awk -v num="$problem_number" '
    {
        if ($0 ~ /^name = "([a-z][0-9]?)"/) {
            match($0, /"([a-z][0-9]?)"/, suffix)
            new_name = sprintf("name = \"p%s%s\"", num, suffix[1])
            print "Replacing line: " $0 " with: " new_name > "/dev/stderr"
            print new_name
        }
        else {
            print $0
        }
    }
    ' "$file" >"$temp_file"

    echo "New content:"
    cat "$temp_file"

    if cmp -s "$file" "$temp_file"; then
        echo "No changes needed for $file"
    else
        cp "$file" "${file}.bak"
        mv "$temp_file" "$file"
        echo "Updated $file"
    fi
done
