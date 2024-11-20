use std::process::exit;
use std::path::PathBuf;

use crate::modules::proc;
use crate::modules::env;

use log::info;
use log::error;

// make it return absolute path
pub fn run() -> PathBuf {
  let result = proc::execute("fzf", ["--walker=dir"].to_vec(), &env::pwd());
  
  match result {
    Ok(o) => {
      info!("fzf executed succefully;\nOutput: {:#?}", o);
      return PathBuf::from(
        String::from_utf8(o.stdout)
          .unwrap()
      );
    },

    Err(e) => {
      error!("failed to execute fzf: {}. Aborting...", e);
      exit(-1);       
    }
  }
}
