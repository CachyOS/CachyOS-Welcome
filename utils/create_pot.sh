#!/bin/sh
# Script to generate pot file
xgettext --from-code=UTF-8 --files-from=./files_to_translate --keyword=translatable --keyword=_ --output=../po/cachyos-hello.pot
