mod install;
mod uninstall;

use crate::modules::fs::searchdir;
use crate::modules::env::get_pwd;

pub fn install() {
  let dotfiles_path = look_for_dotfiles();
  install::link_home(&dotfiles_path);
  install::link_config(&dotfiles_path);
}

pub fn uninstall() {
  let dotfiles_path = look_for_dotfiles();
  
}

fn look_for_dotfiles() -> String {
  match searchdir(&get_pwd(), "dotfiles") {
    Ok(s) => s,
    Err(_) => {
      log::error!("failed to find dotfiles.");
      std::process::exit(-1);
    }
  }
}
