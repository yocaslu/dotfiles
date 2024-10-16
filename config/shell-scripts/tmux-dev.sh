#!/usr/bin/bash
cd $1
distrobox enter dev -- \
  tmux new-session -t "dev" -c $(fzf --walker=dir)  
