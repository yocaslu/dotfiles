use std::env::var;
use std::env::set_var;
use std::process::Command;
use std::process::Output;
use std::process::exit;
use std::path::PathBuf;
use std::io;

use log::warn;
use log::info;
use log::error;
use clap::Parser;

#[derive(Debug, Clone, Parser, PartialEq)]
#[command(author = "Dio", about = "creates an tmux session with nvim, bash, nnn and lazygit")]
struct Args {
  #[arg(help = "development working directory (where tmux will be started)")]
  pub path: PathBuf,
  
  #[arg(short, long, help = "Enable debug information")]
  pub debug: bool
}

const SESSION_NAME: &str = "dev_";
const WINDOW_NAMES: [&str ; 4] = ["nvim", "bash", "nnn", "lazygit"];

fn init_logger() {
  if var("RUST_BACKTRACE").is_err() {
    set_var("RUST_BACKTRACE", "1");
  }

  if var("RUST_LOG").is_err() {
    set_var("RUST_LOG", "trace") 
  }
  
  env_logger::init()
}

// Create proc 
fn execute(command: &str, args: Vec<&str>) -> Result<Output, io::Error> {
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

fn create(path: &PathBuf) {

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

fn main() {
  let args = Args::parse();

  if !args.path.exists() {
    error!("{} does not exist.", args.path.to_str().unwrap());
    exit(-1);
  } else if !args.path.is_dir() {
    error!("{} is not an directory.", args.path.to_str().unwrap());
    exit(-1);
  }
  
  if args.debug { init_logger(); }
  create(&args.path);
}

// todo: check if an tmux session is already initiated, if so, create an diferent 
// todo: check if all applications are installed
// todo: create an config file to customize which applications will be started
// todo: use fzf to get path
