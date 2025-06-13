from loguru import logger 
from logging import DEBUG, INFO
from rich import print
from rich.prompt import Prompt
from rich.console import Console
from rich.pretty import pprint
import psutil as ps
from sys import exit

logger.add(sink='logs/launch_waybar.debug', level=DEBUG)

def get_process(name: str, pid: int | None = None):
    if not name and not int:
        raise ValueError('At least one parameter must be provided.') from e
        logger.error('At least one parameter must be provided.')
        exit(1)
    
    for proc in ps.process_iter(['name', 'pid']):
       if proc.name == name or proc.pid == pid:
           logger.info(f'process found: {proc}') 
           
           
           
            
            
    
def main() -> None:
    get_process(name='waybar')
    

if __name__ == '__main__':
    main()
