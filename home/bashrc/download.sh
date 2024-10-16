# download SDKMAN
alias download-sdkman='curl -s "https://get.sdkman.io" | bash'

# install yay
alias install-yay='sudo pacman -S --needed git base-devel && git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si'
