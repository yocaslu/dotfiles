use std::io::Error;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use std::path::PathBuf;

use log::info;
use log::error;

pub fn execute(command: &str, args: Vec<&str>, pwd: &PathBuf) -> Result<Output, Error> {
  let proc = Command::new(command) 
    .args(&args)
    .current_dir(pwd)
    .stdout(Stdio::piped())
    .spawn();

  match proc {
    Ok(c) => {
      info!("child process {}{:#?} spawned.", command, &args);
      match c.wait_with_output() {
        Ok(o) => {
          info!(
            "child process {}{:#?} exited with code {}.", 
            command, args, o.status
          );

          return Ok(o);
        },
        
        Err(e) => {
          error!("child process {}{:#?} with error: {}",
            command, args, e.to_string()
          );

          return Err(e);
        }
      }
    },

    Err(e) => {
      error!("could not spawm process {}{:#?}",
        command, &args
      );
      return Err(e);
    }
  }
}
