use std::path::PathBuf;
use std::process::exit;
use crate::modules::fs::{DirContent, searchdir, scandir, delete_file};
use crate::modules::env::get_home_path;
use log::{info, error};

fn unlink(dotfiles_src_content: DirContent, src_path: PathBuf) {
  let src_content = scandir(&src_path);
  let src_symlinks = src_content.symlinks();

  for symlink in src_symlinks {
    let follow_symlink = match symlink.read_link() {
      Ok(p) => p,
      Err(e) => {
        error!("failed to follow {}, due to: {}", symlink.to_str().unwrap(), e);
        continue;
      }
    };

    if dotfiles_src_content.contains(&follow_symlink) {
      match delete_file(symlink) {
        Ok(_) => info!("{} succefully removed from your system.", symlink.to_str().unwrap()),
        Err(e) => error!("failed to remove {} from your system due to: {}", symlink.to_str().unwrap(), e)
      };
    }
  } 
}

pub fn unlink_home(dotfiles_path: &PathBuf) {
  let home_path = get_home_path();
  let dotfiles_home_path = match searchdir(&dotfiles_path, &PathBuf::from("home")) {
    Ok(p) => p,
    Err(e) => {
      error!("failed to locate home directory in dotfiles repository due to {}", e);
      exit(-2);
    }
  };

  let dotfiles_home_content = scandir(&dotfiles_home_path);
  unlink(dotfiles_home_content, home_path); 
}

pub fn unlink_config(dotfiles_path: &PathBuf) {
  let mut config_path = get_home_path();
  config_path.push(".config");

  let dotfiles_config_path = match searchdir(&dotfiles_path,&PathBuf::from("config")) {
    Ok(p) => p,
    Err(e) => {
      error!("failed to locate config directory in dotfiles repository due to {}", e);
      exit(-2);
    }
  };

  let dotfiles_config_content = scandir(&dotfiles_config_path);
  unlink(dotfiles_config_content, config_path);
}