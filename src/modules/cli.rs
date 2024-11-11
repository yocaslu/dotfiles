use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq, Clone, Copy)]
#[command(author, about, version)]
pub struct Args {
  #[command(subcommand)]
  cmd: Commands 
}

impl Args {
  pub fn new(cmd: Commands) -> Args {
    Args {cmd}
  }

  pub fn get_cmd(&self) -> Commands {
    return self.cmd;
  }
}

#[derive(Subcommand, Debug, Clone, Copy, PartialEq)]
pub enum Commands {
  /// symlink dotfiles files to home and .config folders
  Install,
  
  /// remove dotfiles symlinks from home and .config folders
  Uninstall
}
pub fn get_args() -> Args {
  Args::parse()
}
