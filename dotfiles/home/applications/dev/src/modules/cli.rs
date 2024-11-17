use std::path::PathBuf;
use clap::Parser;
use log::error;
use crate::proc

#[derive(Debug, Clone, Parser, PartialEq)]
#[command(author = "Dio", about = "creates an tmux session with nvim, bash, nnn and lazygit")]
pub struct Args {
  #[arg(help = "development working directory (where tmux will be started)")]
  pub path: Option<PathBuf>,
  
  #[arg(short, long, help = "Enable debug information")]
  pub debug: bool
}

fn fzf() -> PathBuf {
  if proc::which("fzf") {
    match proc::execute("fzf", ["--walker=dir"].to_vec()) {
      Ok(o) => return PathBuf::from(String::from_utf8(o.stdout).unwrap()),
      Err(e) => {
        error!("failed to execute fzf: {}", e);
        exit(-1);
      }
    }
  } else {
    error!("path was passed as argument and fzf is not installed on your system. impossible to proceed.");
    exit(-1);
  }
}
