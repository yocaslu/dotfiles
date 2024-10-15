#!/usr/bin/bash
cd $1
distrobox enter dev -- \
  tmux new-session -c $(fzf --walker=dir)
