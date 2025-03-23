use log::error;
use std::env::{ var, current_dir };
use std::path::PathBuf;
use std::process::exit;

pub fn get_home_path() -> PathBuf {
  match var("HOME") {
    Ok(path) => PathBuf::from(path + "/"),
    Err(e) => {
      error!("Failed to get HOME env variable: {}", e.to_string());
      exit(-2);
    }
  } 
}

pub fn get_pwd() -> PathBuf {
  match current_dir() {
    Ok(p) => p, 
    Err(e) => {
      error!("failed to get current dir: {}", e);
      exit(-2);
    }
  }
}