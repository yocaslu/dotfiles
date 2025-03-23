use std::env::current_dir;
use std::path::PathBuf;
use log::error;
use std::process::exit;

pub fn pwd() -> PathBuf {
  match current_dir() {
    Ok(p) => p,
    Err(e) => {
      error!("failed to get pwd: {}", e);
      exit(-1); 
    }
  }
}
