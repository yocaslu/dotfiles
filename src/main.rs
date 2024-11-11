mod setup;
use setup::{install, uninstall};

mod modules;
use modules::cli::{ get_args, Commands };
use modules::log::init_logger;

fn main() {
  init_logger();
  match get_args().get_cmd() {
    Commands::Install => install(),
    Commands::Uninstall => uninstall()
  } 
}

/* use std::os::unix::fs;

fn main() -> std::io::Result<()> {
    fs::symlink("a.txt", "b.txt")?;
    Ok(())
} */