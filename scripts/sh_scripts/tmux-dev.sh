#!/usr/bin/bash
command cd $1 ; cd $(fzf --walker=dir)
command bash -c tmux new-session
