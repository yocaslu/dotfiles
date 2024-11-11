use log::{ info, error };
use std::env::{ var, current_dir };
use std::process::exit;

pub fn get_home_path() -> String {
  match var("HOME") {
    Ok(path) => {
      return path;
    },

    Err(e) => {
      error!("Failed to get HOME env variable: {}", e.to_string());
      exit(-2);
    }
  } 
}

pub fn get_pwd() -> String {
  match current_dir() {
    Ok(p) => {
      p.to_str()
        .unwrap()
        .to_string()
    },

    Err(e) => {
      error!("failed to get current dir: {}", e);
      exit(-2);
    }
  }
}