#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

export TERM=xterm-256color
export EDITOR=helix
export VISUAL=helix

PS1='\u@\h \w \$ '

REPO_PATH=$HOME/repos
for FILE in $REPO_PATH/dotfiles/modules/home/bashrc/*; do
  source $FILE
done

# Created by `pipx` on 2024-10-18 07:08:40
export PATH="$PATH:/home/dio/.local/bin"
