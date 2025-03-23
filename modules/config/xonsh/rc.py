from xonsh.built_ins import XSH
from os import getcwd

env = XSH.env
alias = XSH.aliases
HOME = XSH.env['HOME']
PWD = getcwd()

# adjust some paths
env["PATH"].append("/home/scopatz/sandbox/bin")
env["LD_LIBRARY_PATH"] = ["/home/scopatz/.local/lib", "/home/scopatz/miniconda3/lib"]

# pipx binaries path
PIPX_BIN_PATH = f'{HOME}/.local/bin'
env["PATH"].append(PIPX_BIN_PATH)

# cargo install put the binaries here
# so I need to add it to PATH
# it may conflict with rustup tradicional installation
# since i am using arch rustup package
RUST_BIN_PATH = f'{HOME}/.cargo/bin/'
env['PATH'].append(RUST_BIN_PATH)

# some customization options, see https://xon.sh/envvars.html for details
env["MULTILINE_PROMPT"] = "`·.,¸,.·*¯`·.,¸,.·*¯"
env["XONSH_SHOW_TRACEBACK"] = True
env["XONSH_STORE_STDOUT"] = True
env["XONSH_HISTORY_MATCH_ANYWHERE"] = True
env["COMPLETIONS_CONFIRM"] = True
env["XONSH_AUTOPAIR"] = True

TEXT_EDITOR = "helix"
env['EDITOR'] = f'{TEXT_EDITOR}'
env['VISUAL'] = f'{TEXT_EDITOR}'

LS_APP = "lsd"
alias['ls'] = f'{LS_APP} -l'
alias['ll'] = f'{LS_APP} -lha'
alias['lt'] = f'{LS_APP} --tree'

DEV_APPS = "helix nnn lazygit"
alias['fdev'] = f'dev --fzf --apps {DEV_APPS}'
alias['pdev'] = f'dev -d {PWD} --apps {DEV_APPS}'

alias['lgit'] = 'lazygit'
alias['hx'] = 'helix'
alias['rc'] = 'source ~/.xonshrc'
alias['cl'] = 'clear'
alias['fetch'] = 'fastfetch -l arch2'
alias['syu'] = 'sudo pacman -Syu'
alias['py'] = 'python3'
