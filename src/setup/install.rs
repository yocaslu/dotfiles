use crate::modules::fs::{ symlink, searchdir, scandir, create_directory };
use crate::modules::env::get_home_path;
use log::{ error, info };
use std::process::exit;
use std::path::Path;

pub fn link_home(dotfiles_path: &str) {
  let home_path = get_home_path();
  let dotfiles_home_path = match searchdir(dotfiles_path, "home") {
    Ok(s) => s,
    Err(_) => {
      error!("failed to find home folder in dotfiles directory.");
      exit(-1);
    }
  };

  let dotfiles_home_content = scandir(&dotfiles_home_path);
  for file in dotfiles_home_content.files().iter() {
    match symlink(file, &home_path) {
      Ok(_) => info!("file {} succefully linked to {}", file, home_path),
      Err(e) => error!("failed to link file {} to {}, due to: {}", file, home_path, e) 
    };
  }

  for dir in dotfiles_home_content.dirs().iter() {
    match symlink(dir, &home_path) {
      Ok(_) => info!("dir {} succefully linked to {}", dir, home_path),
      Err(e) => error!("failed to link dir {} to {}, due to: {}", dir, home_path, e.to_string())
    }
  }
}

pub fn link_config(dotfile_path: &str) {
  let dotfiles_config_path = match searchdir(dotfile_path, "config") {
    Ok(s) => s,
    Err(_) => {
      error!("failed to find config folder in dotfiles directory.");
      exit(-1);
    }
  };

  let mut config_path = get_home_path(); config_path.push_str("/.config");
  if !Path::new(&config_path).exists() {
    create_directory(&config_path);
  }

  let dotfiles_config_content = scandir(&dotfiles_config_path);
  for file in dotfiles_config_content.files().iter() {
    match symlink(file, &config_path) {
      Ok(_) => info!("file {} succefully linked to {}", file, config_path),
      Err(e) => error!("failed to link file {} to {}, due to: {}", file, config_path, e) 
    };
  }

  for dir in dotfiles_config_content.dirs().iter() {
    match symlink(dir, &config_path) {
      Ok(_) => info!("dir {} succefully linked to {}", dir, config_path),
      Err(e) => error!("failed to link dir {} to {}, due to: {}", dir, config_path, e.to_string())
    }
  }
}