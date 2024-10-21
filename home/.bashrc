#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

export TERM=xterm-256color
export EDITOR='exec distrobox enter dev -- tmux neww -c nvim'
export VISUAL=nvim

PS1='\u@\h \w \$ '

REPO_PATH=$HOME/git-repos
for FILE in $REPO_PATH/dotfiles/home/bashrc/*; do
  source $FILE
done

# Created by `pipx` on 2024-10-18 07:08:40
export PATH="$PATH:/home/dio/.local/bin"

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
