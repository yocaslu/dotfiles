#!/usr/bin/bash
cd /home/dio/.config/
distrobox enter dev -- \
  tmux new-session -c $(fzf --walker=dir) 
