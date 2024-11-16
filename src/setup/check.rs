use std::path::PathBuf;
use std::ffi::OsStr;
use std::process::exit;
use log::error;

use crate::modules::fs::searchdir;
use crate::modules::fs::scandir;
use crate::modules::env::get_home_path;

fn check_dir(src_symlinks: &Vec<PathBuf>, compare_to: &Vec<PathBuf>) -> Vec<PathBuf> {
  dbg!(src_symlinks);

  let mut broken_links: Vec<PathBuf> = Vec::new();
  let src_symlinks: Vec<&PathBuf> = src_symlinks.iter().filter(|x| { // removing intrusive links
    let compare_file_names: Vec<&OsStr> = compare_to.iter().flat_map(|x| x.file_name()).collect();
    if !compare_file_names.contains(&x.file_name().unwrap()) { true } else { false }
  }).collect();
  
  dbg!(&src_symlinks);
  dbg!(compare_to);
  for symlink in src_symlinks {
    match symlink.read_link() {
      Ok(p) => {
        if compare_to.contains(&p) { // pointing wrong 
          continue;
        } else {
          error!("{} is pointing to an wrong place: {}", symlink.to_str().unwrap(), p.to_str().unwrap());
          broken_links.push(symlink.to_path_buf());
        }
      },

      Err(e) => { // broken link
        error!("failed to read {} due to {}", symlink.to_str().unwrap(), e);
        broken_links.push(symlink.to_path_buf());
        continue;
      }
    };  
  }

  return broken_links;
}

pub fn check_home(dotfiles_path: &PathBuf) {
  let home_path = get_home_path(); 
  let dotfiles_home_path = match searchdir(dotfiles_path, &PathBuf::from("home")) {
    Ok(p) => p,
    Err(e) => {
      error!("failed to read {} due to {}", dotfiles_path.to_str().unwrap(), e);
      exit(-2);
    }
  };
  
  let dotfiles_home_content = scandir(&dotfiles_home_path);
  let home_content = scandir(&home_path);

  let broken_links = check_dir(&home_content.symlinks(), &dotfiles_home_content.symlinks());
  dbg!(&broken_links);
}
pub fn check_config(dotfiles_path: &PathBuf) {
  let mut config_path = get_home_path(); config_path.push(".config"); 
  let dotfiles_config_path = match searchdir(dotfiles_path, &PathBuf::from("config")) {
    Ok(p) => p,
    Err(e) => {
      error!("failed to read {} due to {}", dotfiles_path.to_str().unwrap(), e);
      exit(-2);
    }
  };
  
  let dotfiles_config_content = scandir(&dotfiles_config_path);
  let config_content = scandir(&config_path);

  let broken_links = check_dir(&config_content.symlinks(), &dotfiles_config_content.symlinks());
  dbg!(&broken_links);
}
