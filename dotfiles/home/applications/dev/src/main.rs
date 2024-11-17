use std::path::PathBuf;
use std::process::exit;

use log::warn;
use log::info;
use log::error;
use clap::Parser;

mod modules;
use modules::cli;
use modules::proc;
use modules::tmux;
use modules::logger;



fn main() {
  let args = cli::Args::parse();
  let path = match args.path {
    Some(p) => {
      if p.exists() { p }
      else {
        error!("{} does not exist.", p.to_str().unwrap());
        exit(-1);
      }
    },
    
    None => fzf()
  };
  
  if args.debug { logger::init_logger(); }
  tmux::create(&path);
}

// todo: check if an tmux session is already initiated, if so, create an diferent 
// todo: check if all applications are installed
// todo: create an config file to customize which applications will be started
// todo: use fzf to get path
