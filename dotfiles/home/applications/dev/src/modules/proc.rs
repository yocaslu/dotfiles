use std::process::Command;
use std::process::Output;

pub fn which(app_name: &str) -> bool {
  match proc::execute("which", [app_name].to_vec()) {
    Ok(_) => return true,
    Err(_) => return false
  };
}

// Create proc 
pub fn execute(command: &str, args: Vec<&str>) -> Result<Output, io::Error> {
  match Command::new(command).args(&args).output() {
    Ok(o) => {
      info!("succefully executed {} {:#?}", command, &args);
      return Ok(o);
    },
  
    Err(e) => {
      error!("failed to execute {} {:#?}", command, &args);
      return Err(e);
    }
  };
}
