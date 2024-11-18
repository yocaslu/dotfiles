use std::io::Error;
use std::process::Command;
use std::process::Output;

use log::info;
use log::error;

// Create proc 
pub fn execute(command: &str, args: Vec<&str>) -> Result<Output, Error> {
    match Command::new(command).args(&args).output() {
      Ok(o) => {
        info!("succefully executed {} {:#?}", command, &args);
        return Ok(o);
      },
    
      Err(e) => {
        error!("failed to execute {} {:#?}", command, &args);
        return Err(e);
      }
    }
}
