#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

### NNN cd on quit
nnn ()
{
    # Block nesting of nnn in subshells
    [ "${NNNLVL:-0}" -eq 0 ] || {
        echo "nnn is already running"
        return
    }

    # The behaviour is set to cd on quit (nnn checks if NNN_TMPFILE is set)
    # If NNN_TMPFILE is set to a custom path, it must be exported for nnn to
    # see. To cd on quit only on ^G, remove the "export" and make sure not to
    # use a custom path, i.e. set NNN_TMPFILE *exactly* as follows:
    #      NNN_TMPFILE="${XDG_CONFIG_HOME:-$HOME/.config}/nnn/.lastd"
    export NNN_TMPFILE="${XDG_CONFIG_HOME:-$HOME/.config}/nnn/.lastd"

    # Unmask ^Q (, ^V etc.) (if required, see `stty -a`) to Quit nnn
    # stty start undef
    # stty stop undef
    # stty lwrap undef
    # stty lnext undef

    # The command builtin allows one to alias nnn to n, if desired, without
    # making an infinitely recursive alias
    command nnn "$@"

    [ ! -f "$NNN_TMPFILE" ] || {
        . "$NNN_TMPFILE"
        rm -f -- "$NNN_TMPFILE" > /dev/null
    }
}

alias nnn='nnn -dH'
alias n='nnn -dH'

export TERM=xterm-256color
export EDITOR=nvim
export VISUAL=nvim

PS1='\u@\h \w \$ '

# shell script calls
SHELL_SCRIPTS_PATH=$HOME/.scripts/.shell-scripts
alias dev="exec $SHELL_SCRIPTS_PATH/tmux-dev.sh ~/git-repos/"

# python3 script calls
PY_SCRIPTS_PATH=$HOME/.scripts/.pyscripts
alias pyenv='python3 $PY_SCRIPTS_PATH/pyenv.py $(pwd) && source .env/bin/activate'

# using tree instead of ls
alias tl="tree -lhagpf"

# managing cpu
alias info-frequency="sudo cpupower frequency-info"
alias set-frequency="sudo cpupower frequency-set -u"
alias base-freq="sudo cpupower frequency-set -u 1400Mhz"

# programming lang
alias py="python3"

# terminal commands
alias cl='clear'
alias ll='lsd -lha'
alias ls='lsd --color=auto'
alias grep='grep --color=auto'
alias fetch='fastfetch -l arch2'

# xmodmap
alias xmmp="~/.xmmp.sh"

# package manager
alias syu="sudo pacman -Syu"
alias install-yay='sudo pacman -S --needed git base-devel && git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si'

# Set up ff key bindings and fuzzy completion
eval "$(fzf --bash)"

# starship 
eval "$(starship init bash)"

# rust
. "$HOME/.cargo/env"

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="$HOME/.sdkman"
[[ -s "$HOME/.sdkman/bin/sdkman-init.sh" ]] && source "$HOME/.sdkman/bin/sdkman-init.sh"
