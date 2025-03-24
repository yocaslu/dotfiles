use log::{error, info};
use std::fs::{create_dir, read_dir, remove_dir, remove_file, DirEntry};
use std::io::{self, Error, ErrorKind};
use std::ops::IndexMut;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::exit;

fn craw(path: PathBuf, target: PathBuf) -> Result<PathBuf, Error> {
    let mut stack: Vec<PathBuf> = Vec::new();
    stack.push(PathBuf::from(path));

    while !stack.is_empty() {
        let curr: PathBuf = stack.pop().unwrap(); // popping an new directory to craw
        match read_dir(&curr) {
            Ok(content) => {
                for entry in content.flatten() {
                    // iterating through all the elements inside the directory
                    if entry.file_name() == target {
                        return Ok(entry.path());
                    } else if entry.path().is_dir() {
                        // stacking an new directory to craw
                        stack.push(entry.path());
                    }
                }
            }

            Err(e) => {
                error!("failed to read {} due to: {}", &curr.to_str().unwrap(), e);
            }
        }
    }

    return Err(Error::from(ErrorKind::NotFound));
}

pub fn searchdir(origin: &PathBuf, target: &PathBuf) -> Result<PathBuf, Error> {
    if !origin.exists() {
        error!("origin path does not exist: {}", origin.to_str().unwrap());
        return Err(Error::from(ErrorKind::NotFound));
    }

    match craw(PathBuf::from(origin), PathBuf::from(target)) {
        Ok(p) => {
            if p.is_dir() {
                Ok(p)
            } else {
                Err(Error::from(ErrorKind::NotFound))
            }
        }

        Err(e) => {
            error!(
                "{} not found crawling through {}",
                target.to_str().unwrap(),
                origin.to_str().unwrap()
            );
            Err(e)
        }
    }
}

pub fn scandir(path: &PathBuf) -> Vec<PathBuf> {
    let path = Path::new(path);
    let mut dir_content = Vec::new();

    let content = match path.read_dir() {
        Ok(c) => c,
        Err(e) => {
            error!("failed to read {}: {}", path.to_str().unwrap(), e);
            exit(-2);
        }
    };

    let content: Vec<DirEntry> = content.flatten().collect();
    for x in &content {
        dir_content.push(x.path());
    }

    return dir_content;
}

pub fn link(content: &mut Vec<PathBuf>, dest: &PathBuf, overwrite: bool) {
    for entry in content.iter() {
        let dest_with_file_name = if entry.is_file() {
            dest.with_file_name(entry.file_name().unwrap()) // append file name to destination path
        } else {
            entry.to_path_buf()
        };

        // trying to link entry to dest
        match symlink(entry, &dest_with_file_name) {
            Ok(_) => info!(
                "{} succefully linked to {}",
                entry.to_str().unwrap(),
                dest.to_str().unwrap()
            ),

            // looking if it already exist
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    if overwrite {
                        // creting path to the non dotfiles entry
                        let impostor_entry =
                            PathBuf::from(dest).with_file_name(entry.file_name().unwrap());

                        // removing the non dotfile entry from dest
                        if entry.is_file() {
                            match remove_file(&impostor_entry) {
                                Ok(_) => {
                                    info!(
                                        "succefully removed {} from {}.",
                                        entry.file_name().unwrap().to_str().unwrap(),
                                        dest.to_str().unwrap()
                                    );

                                    // pushing entry again to content
                                    // so it tries to link again
                                    content.push(entry.to_owned());
                                }

                                Err(e) => {
                                    error!("")
                                }
                            }
                        } else if entry.is_dir() {
                            remove_dir(&impostor_entry);
                        }
                    }
                }

                _ => {
                    error!(
                        "failed to link {} to {} due to: {}",
                        entry.file_name().unwrap().to_str().unwrap(),
                        dest.to_str().unwrap(),
                        e
                    );
                }
            },
        }
    }
}

pub fn unlink(dotfiles_src_content: &Vec<PathBuf>, src_path: PathBuf) {}
