#!/bin/bash

# Prepare TQDM command for progress bar.
get_tqdm_command() {
  local COUNT="$1"
  which tqdm > /dev/null
  if [ $? -eq 0 ]; then
    echo "$(which tqdm) --total ${COUNT}"
  else
    # Drop all progress if there is no TQDM.
    echo "[WARN] You can see beautiful progress when you install tqdm."
    echo "$(which tee)"
  fi
}

