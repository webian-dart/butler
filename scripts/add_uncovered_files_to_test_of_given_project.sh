#!/bin/sh
file=test/register_for_coverage_tests_untouched_files_test.dart
echo "// Helper file to make coverage work for all dart files\n" > $file
echo "// ignore_for_file: unused_import" >> $file
find lib -not -name '*.g.dart' -and -name '*.dart' | cut -c4- | awk -v package=$1 '{printf "import '\''package:%s%s'\'';\n", package, $1}' >> $file
echo "void main(){}" >> $file