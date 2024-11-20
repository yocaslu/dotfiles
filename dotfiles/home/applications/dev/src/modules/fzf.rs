use std::process::exit;
use std::path::PathBuf;

use crate::modules::proc;
use crate::modules::env;

use log::info;
use log::error;

pub fn run() -> PathBuf {
  let result = proc::execute("fzf", ["--walker=dir"].to_vec(), &env::pwd());
  match result {
    Ok(o) => {
      info!("fzf executed succefully;\nOutput: {:#?}", o);
      let mut path = env::pwd(); // creating an absolute path
      path.push(
        PathBuf::from(
          String::from_utf8(o.stdout)
            .unwrap()
            .trim() // removing '\n' in the end of the string
        )
      );
      
      return path;
    },

    Err(e) => {
      error!("failed to execute fzf: {}. Aborting...", e);
      exit(-1);       
    }
  }
}
