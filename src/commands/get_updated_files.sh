#!/bin/sh

git status -s -uall | while IFS= read -r line; do
    code="${line:0:2}"
    file="${line:3}"

    if [ "$code" = "M " ] || [ "$code" = "A " ]; then
            stats=$(git diff --staged --numstat -- "$file")
            added=$(echo "$stats" | cut -f1)
            removed=$(echo "$stats" | cut -f2)
            echo "STAGED|$code|$file|+$added|-$removed"

        elif [ "$code" = " M" ] || [ "$code" = " D" ]; then
            stats=$(git diff --numstat -- "$file")
            added=$(echo "$stats" | cut -f1)
            removed=$(echo "$stats" | cut -f2)
            echo "UNSTAGED|$code|$file|+$added|-$removed"

        elif [ "$code" = "MM" ]; then
            staged_stats=$(git diff --staged --numstat -- "$file")
            unstaged_stats=$(git diff --numstat -- "$file")

            staged_added=$(echo "$staged_stats" | cut -f1)
            staged_removed=$(echo "$staged_stats" | cut -f2)
            unstaged_added=$(echo "$unstaged_stats" | cut -f1)
            unstaged_removed=$(echo "$unstaged_stats" | cut -f2)

            echo "STAGED|$code|$file|+$staged_added|-$staged_removed"
            echo "UNSTAGED|$code|$file|+$unstaged_added|-$unstaged_removed"

        elif [ "$code" = "??" ]; then
            if [ -f "$file" ]; then
                added=$(wc -l < "$file" | tr -d ' ')
                echo "UNTRACKED|$code|$file|+$added|-0"
            else
                echo "UNTRACKED|$code|$file|+0|-0 (not a regular file)"
            fi
        fi
done
