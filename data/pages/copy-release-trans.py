#!/usr/bin/python

import os

targets = ['da', 'es', 'fr', 'hu', 'it', 'ko-KR', 'nl', 'pt-BR', 'pt-PT', 'ro-RO', 'sv', 'tr', 'zh-CN']

for target in targets:
  print(f"copying en/release -> {target}/release")
  os.system(f"cp ./en/release {target}/release")
