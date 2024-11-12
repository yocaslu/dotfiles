use crate::modules::fs::{ create_directory, scandir, searchdir, symlink, DirContent };
use crate::modules::env::get_home_path;
use log::{ error, info };
use std::process::exit;
use std::path::{ Path, PathBuf };

pub fn link(content: DirContent, dest: PathBuf) {

  let dest_str = dest.to_str().unwrap();
  for file in content.files().iter() {
    
    let file_name_str = file.file_name()
      .unwrap()
      .to_str()
      .unwrap();
    
    let mut file_dest = PathBuf::from(dest_str); 
    file_dest.push(file_name_str);

    match symlink(file, &file_dest) {
      Ok(_) => info!(
        "file {} succefully linked to {}", 
        file_name_str, 
        file_dest.to_str().unwrap()
      ),

      Err(e) => error!(
        "failed to link file {} to {}, due to: {}", 
        file_name_str, 
        file_dest.to_str().unwrap(), e
      )
    };
  }

  for dir in content.dirs().iter() {

    let dir_name_str = dir.file_name().unwrap().to_str().unwrap(); 
    let mut file_dest = PathBuf::from(dest_str); 
    file_dest.push(dir_name_str);

    match symlink(dir, &file_dest) {
      Ok(_) => info!(
        "directory {} succefully linked to {}", 
        dir_name_str, 
        file_dest.to_str().unwrap()
      ),

      Err(e) => error!(
        "failed to link directory {} to {}, due to: {}", 
        dir_name_str, 
        file_dest.to_str().unwrap(), e
      )
    };
  }
}

pub fn link_home(dotfiles_path: &PathBuf) {
  let home_path = get_home_path();
  let dotfiles_home_path = match searchdir(dotfiles_path, &PathBuf::from("home")) {
    Ok(s) => s,
    Err(_) => {
      error!("failed to find home folder in dotfiles directory.");
      exit(-1);
    }
  };

  let dotfiles_home_content = scandir(&dotfiles_home_path);
  link(dotfiles_home_content, home_path); 
}

pub fn link_config(dotfile_path: &PathBuf) {
  let dotfiles_config_path = match searchdir(dotfile_path, &PathBuf::from("config")) {
    Ok(s) => s,
    Err(_) => {
      error!("failed to find config folder in dotfiles directory.");
      exit(-1);
    }
  };

  let mut config_path = get_home_path(); 
  config_path.push(".config");

  dbg!(&config_path);
  if !Path::new(&config_path).exists() {
    create_directory(&config_path);
  }

  let dotfiles_config_content = scandir(&dotfiles_config_path);
  link(dotfiles_config_content, config_path); 
}