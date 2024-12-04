use std::path::PathBuf;
use std::process::exit;

use crate::proc;

use log::info;
use log::error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Session {
  pub session_name: String,
  pub workdir: PathBuf,
  pub applications: Vec<String>
}

impl Session {
  pub fn from(session_name: String, workdir: PathBuf, applications: Vec<String>) -> Session {
    Session {
      session_name,
      workdir,
      applications
    } 
  }
}

pub fn create_session(session: &Session) {
  let args = ["new-session", "-d", "-c", &session.workdir.to_str().unwrap(), "-s", &session.session_name].to_vec();
  match proc::execute("tmux", args, &session.workdir) {
    Ok(o) => info!("session {} succefully created.\nOutput: {:#?}" , &session.session_name, o),
    Err(e) => {
      error!("failed to create tmux session {}: {}. exiting", &session.session_name, e);
      exit(-1);
    }
  }
}

pub fn create_windows(session: &Session) {
  for app in session.applications.iter() {
    match proc::execute("tmux", ["new-window",  app].to_vec(), &session.workdir) {
      Ok(_) => info!("window {} succefully created.\nOutput: ", app),
      Err(e) => { 
        error!("failed to create window {}, stderr: {}. exiting.", app, e);
        kill_session(session);
        exit(-1);
      }
    }
  }
}

pub fn kill_session(session: &Session) {
  match proc::execute("tmux", ["kill-session"].to_vec(), &session.workdir) {
    Ok(_) => info!("killed session {}", &session.session_name), 
    Err(e) => error!("failed to kill {}: {}", &session.session_name, e) 
  }
}
