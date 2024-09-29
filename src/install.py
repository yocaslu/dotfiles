import os
from os.path import isdir
import sys
import subprocess
import logging
from typing import List 

from rich.logging import RichHandler
from rich.console import Console
from rich.traceback import install
from rich import print
from rich.pretty import pprint

install() # pretty tracebacks
console = Console()
rhandler = RichHandler(
    console=console, 
    rich_tracebacks=True, 
    markup=True, 
    tracebacks_show_locals=True
)

logging.basicConfig(
    level=logging.DEBUG, 
    format="%(name)s > %(message)s", 
    handlers=[rhandler]
)

HOME_PATH: str = os.environ['HOME']
CONFIG_PATH: str = HOME_PATH + '/' + '.config'

def run_commands(commands: list[str], cwd: str = os.getcwd(), shell = False) -> tuple[int, str]:
    logger = logging.getLogger('run_commands()')
    if not len(commands) > 1:
        logging.info('command list is empty')
    else:
        proc = subprocess.run(commands, cwd=cwd, shell=shell, capture_output=True, text=True)
        if proc.returncode != 0:
            return (proc.returncode, proc.stderr.strip())
        else: 
            return (proc.returncode, proc.stderr.strip())
    return (0, '')

# refatorar usando access()
def install_home():
    logger = logging.getLogger('install_home()')
    if not os.path.exists(HOME_PATH):
        logger.critical('linux home folder does not exist')
        sys.exit(-1)

    cwd = os.getcwd() + '/' + 'home'
    home_files = os.listdir('home'); 
    if not len(home_files) > 1:
        logger.info('repo home folder is empty')
        return
    else:
        failed = {}        
        for file in home_files:
            code, msg = run_commands(['ln', '-s', f'{cwd}/{file}', HOME_PATH], cwd)
            if code != 0:
                failed[file] = msg 

        if len(failed) > 0:
            for (_, msg) in failed.items():
                logger.warning(msg)

# refatorar usando access()
def install_config():
    logger = logging.getLogger('install_config()')
    if not os.path.exists(CONFIG_PATH):
        logger.info(f'.config folder does not exist in {HOME_PATH}, creating it...')
        if run_commands(['mkdir', CONFIG_PATH]) != 0:
            logger.critical(f'Failed to create {CONFIG_PATH}')
            sys.exit(-1)
        else:
            logger.info(f'{CONFIG_PATH} folder was created.')

    cwd = os.getcwd() + '/' + 'config'
    config_files = os.listdir('config');
    if not len(config_files) > 1:
        logger.info('config files folder is empty')
    else:
        failed = {}
        for file in config_files:
            # logger.info(f'linking {file} to {CONFIG_PATH}...')
            code, msg = run_commands(['ln', '-s', f'{cwd}/{file}', CONFIG_PATH], cwd)
            if code != 0:
                failed[file] = msg
        if len(failed) > 0:
            for (_, msg) in failed.items():
                logger.warning(msg)

def main():
    install_home()
    install_config()

if __name__ == '__main__':
    main()
