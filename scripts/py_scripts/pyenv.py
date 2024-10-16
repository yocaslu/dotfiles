import sys
import os
import subprocess

VENV_FOLDER_NAME = '.env'

def check_env_exist(pwd: str) -> bool:
    if '.env' not in os.listdir(pwd):
        return False
    else:
        return True

def get_path() -> str:
    if not len(sys.argv) > 1:
        print('pyenv::get_path() > sys.argv <= 1')
        sys.exit(-1)
    else:
        print(f'pyenv.py::get_path() > returning path: {sys.argv[1]}')
        return sys.argv[1] 

def run_command(commands: list[str], shell: bool = False) -> str:
    proc = subprocess.run(commands, capture_output=True, text=True, shell=shell)
    if proc.returncode != 0:
        print(f'pyenv.py::run_command() > An error occurred while trying to run: {commands}')
        print(f'pyenv.py::run_command() > stderr: {proc.stderr}')
        sys.exit(-1)
    else:
        return proc.stdout

def main():
    
    pwd: str = get_path()
    if check_env_exist(pwd):
        print(f'pyenv.py::main() > .env already exists in {pwd}')
        print('pyenv.py::main() > exiting...')
        sys.exit(0)
    else:
        print(f'pyenv.py::main() > creating {VENV_FOLDER_NAME} in {pwd}')
        run_command(['python3', '-m', 'venv', f'{pwd}/{VENV_FOLDER_NAME}'])

if __name__ == '__main__':
    main()
