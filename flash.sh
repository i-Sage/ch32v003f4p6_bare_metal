#!/bin/bash
set -e

~/.platformio/packages/tool-openocd-riscv-wch/bin/openocd \
  -f ~/.platformio/packages/tool-openocd-riscv-wch/bin/wch-riscv.cfg \
  -c init \
  -c halt \
  -c "program $1 verify" \
  -c "reset" \
  -c "exit"