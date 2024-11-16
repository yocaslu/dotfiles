# application folder aliases
alias pdev="dev $(pwd) && tmux attach"

# Set up ff key bindings and fuzzy completion
eval "$(fzf --bash)"

# zoxide
eval "$(zoxide init bash)"

# starship 
eval "$(starship init bash)"
