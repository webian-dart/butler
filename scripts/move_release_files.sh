#!/bin/bash
echo "Specified Platform: $1"
target="./releases/$1/buttler"
echo "Dir: $target"
cp "./target/release/buttler" "$target"
