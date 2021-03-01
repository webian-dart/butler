#!/bin/bash
echo "Specified Platform: $1"
target="./releases/$1/butler"
echo "Dir: $target"
cp "./target/release/butler" "$target"
