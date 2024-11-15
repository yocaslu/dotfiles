mod install;
mod uninstall;
mod check;

use std::path::PathBuf;
use crate::modules::fs::searchdir;
use crate::modules::env::get_pwd;

pub fn install() {
  let dotfiles_path = look_for_dotfiles();
  install::link_home(&dotfiles_path);
  install::link_config(&dotfiles_path);
}

pub fn uninstall() {
  let dotfiles_path = look_for_dotfiles();
  uninstall::unlink_home(&dotfiles_path);
  uninstall::unlink_config(&dotfiles_path); 
}

pub fn check() {
  let dotfiles_path = look_for_dotfiles();
  check::check_home(&dotfiles_path);
  check::check_config(&dotfiles_path);
}

fn look_for_dotfiles() -> PathBuf {
  match searchdir(&get_pwd(), &PathBuf::from("dotfiles")) {
    Ok(s) => s,
    Err(_) => {
      log::error!("failed to find dotfiles.");
      std::process::exit(-1);
    }
  }
}
