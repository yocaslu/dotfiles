use std::path::PathBuf;
use std::process::exit;

use crate::proc::execute;

use log::warn;
use log::info;
use log::error;

const SESSION_NAME: &str = "dev_";
const WINDOW_NAMES: [&str ; 4] = ["nvim", "bash", "nnn", "lazygit"];

// todo: make an function that follow an symlink and extract the dir name
fn make_session_name(path: &PathBuf) -> String {
  let file_name = match path.file_name() {
    Some(s) => s.to_str().unwrap(),
    None => {
      warn!("failed to extract dir to session name. None returned.");
      return format!("{SESSION_NAME}?"); 
    },
  };

  String::from(SESSION_NAME.to_owned() + file_name + " ")
}

// create fn create_session
// create fn create_window

pub fn create(path: &PathBuf) {

  let session_name = make_session_name(path);  
  match execute("tmux", ["new-session", "-d", "-s", &session_name, WINDOW_NAMES[0]].to_vec()) {
    Ok(_) => info!("tmux session created."),
    Err(_) => {
      error!("failed to create tmux session, aborting...");
      exit(-1);
    }
  };

  for window in WINDOW_NAMES[1..].to_vec() {
    //                                    command       flag  name    exec
    match execute("tmux", ["new-window", "-n", window, window].to_vec()) {
      Ok(_) => info!("window {} succefully created.", window),
      Err(e) => error!("failed to create window {}. stderr: {}", window, e) 
    };
  }
}
