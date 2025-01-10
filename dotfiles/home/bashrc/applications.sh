# Set up ff key bindings and fuzzy completion
eval "$(fzf --bash)"

# zoxide
eval "$(zoxide init bash)"

# starship 
export STARSHIP_CONFIG="$HOME/.config/starship/starship.toml"
eval "$(starship init bash)"
