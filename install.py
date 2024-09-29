import os
import sys
import subprocess
import logging 
from pprint import pprint

logging.basicConfig(level=logging.DEBUG)

HOME_PATH: str = os.environ['HOME']
CONFIG_PATH: str = HOME_PATH + '/' + '.config'

def run_commands(commands: list[str], cwd: str = os.getcwd(), shell = False) -> int:
    logger = logging.getLogger('run_commands()')
    if not len(commands) > 1:
        logging.info('command list is empty')
    else:
        proc = subprocess.run(commands, cwd=cwd, shell=shell, capture_output=True, text=True)
        if proc.returncode != 0:
            logger.error(f'An error occurred while trying to execute: {commands}\n{proc.stderr}')
            return proc.returncode
        else: 
            # logger.info(f'{commands} ran without errors')
            return proc.returncode

# refatorar usando access()
def install_home():
    logger = logging.getLogger('install_home()')
    if not os.path.exists(HOME_PATH):
        logger.critical('linux home folder does not exist')
        sys.exit(-1)
    
    home_files = os.listdir('home')
    if not len(home_files) > 1:
        logger.info('repo home folder is empty')
        return
    else:
        cwd = os.getcwd() + '/' + 'home'
        for file in home_files:
            logger.info(f'linking: {file} to {HOME_PATH}...')
            run_commands(['ln', '-s', f'{cwd}/{file}', HOME_PATH], cwd)

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

    
    config_files = os.listdir('config')
    if not len(config_files) > 1:
        logger.info('config files folder is empty')
    else:
        cwd = os.getcwd() + '/' + 'config'
        for file in config_files:
            logger.info(f'linking {file} to {CONFIG_PATH}...')
            run_commands(['ln', '-s', f'{cwd}/{file}', CONFIG_PATH], cwd)

    

def main():
    logger = logging.getLogger('main()')
    install_home()
    install_config()

if __name__ == '__main__':
    main()
