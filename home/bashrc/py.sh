alias py="python3"

# python3 script calls
PY_SCRIPTS_PATH=$HOME/.scripts/.pyscripts
alias pyenv='python3 $PY_SCRIPTS_PATH/pyenv.py $(pwd) && source .env/bin/activate'
