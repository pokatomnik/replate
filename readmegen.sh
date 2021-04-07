#!/usr/bin/env sh

./target/debug/tpl \
  --template README.tpl \
  -name Replate \
  -default-template-filename template.tpl \
  -build-date "$(date)" \
> README.md
