use log::{error, info};
use std::fs::{create_dir, read_dir, remove_file, DirEntry};
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::exit;

// is it really necessary?
// what if i just use an vector of absolute paths to the files?
#[derive(Debug, PartialEq, Clone)]
pub struct DirContent {
    files: Vec<PathBuf>,
    symlinks: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl DirContent {
    pub fn new() -> DirContent {
        DirContent {
            files: Vec::new(),
            symlinks: Vec::new(),
            dirs: Vec::new(),
        }
    }

    pub fn from(files: Vec<PathBuf>, symlinks: Vec<PathBuf>, dirs: Vec<PathBuf>) -> DirContent {
        DirContent {
            files,
            symlinks,
            dirs,
        }
    }

    pub fn insert(&mut self, element: &PathBuf) {
        if element.is_file() {
            self.files.push(element.to_owned());
        }
    }

    pub fn files(&self) -> &Vec<PathBuf> {
        return &self.files;
    }

    pub fn symlinks(&self) -> &Vec<PathBuf> {
        return &self.symlinks;
    }

    pub fn dirs(&self) -> &Vec<PathBuf> {
        return &self.dirs;
    }

    // really wanted to do this, idk why
    pub fn get_vectors_as_mut(
        &mut self,
    ) -> (&mut Vec<PathBuf>, &mut Vec<PathBuf>, &mut Vec<PathBuf>) {
        return (&mut self.files, &mut self.symlinks, &mut self.dirs);
    }

    pub fn contains(&self, path: &PathBuf) -> bool {
        if self.files().contains(path) {
            return true;
        }
        if self.dirs().contains(path) {
            return true;
        }
        if self.symlinks().contains(path) {
            return true;
        }

        return false;
    }
}

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
                error!(
                    "the element found is not a directory: {}",
                    p.to_str().unwrap()
                );
                Err(Error::from(ErrorKind::NotFound))
            }
        }

        Err(e) => {
            error!(
                "{} not found crawling {}",
                target.to_str().unwrap(),
                origin.to_str().unwrap()
            );
            Err(e)
        }
    }
}

// make it return Result<DirContnet, io::Error>
pub fn scandir(path: &PathBuf) -> DirContent {
    let path = Path::new(path);
    let mut dir_content = DirContent::new();
    let (files, symlinks, dirs) = dir_content.get_vectors_as_mut();

    let content = match path.read_dir() {
        Ok(c) => c,
        Err(e) => {
            error!("failed to read {}: {}", path.to_str().unwrap(), e);
            exit(-2);
        }
    };

    let content: Vec<DirEntry> = content.flatten().collect();

    for x in &content {
        if x.path().is_dir() {
            files.push(x.path());
        }

        if x.path().is_file() {
            dirs.push(x.path());
        }

        if x.path().is_symlink() {
            symlinks.push(x.path());
        }
    }

    return dir_content;
}

pub fn symlink(src: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
    if dest.exists() {
        return Err(Error::from(ErrorKind::AlreadyExists));
    }

    match std::os::unix::fs::symlink(src, dest) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}

pub fn create_directory(path: &PathBuf) -> Result<(), io::Error> {
    if path.exists() {
        error!("{} already exists. Aborting.", path.to_str().unwrap());
        return Err(Error::from(ErrorKind::NotFound));
    }

    match create_dir(path) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn delete_file(path: &PathBuf) -> Result<(), io::Error> {
    if !path.exists() {
        error!("{} does not exist. aborting...", path.to_str().unwrap());
        return Err(Error::from(ErrorKind::NotFound));
    }

    match remove_file(path) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn link(content: &DirContent, dest: &PathBuf, overwrite: bool) {
    let dest_str = dest.to_str().unwrap();
    for file in content.files().iter() {
        let file_name_str = file.file_name().unwrap().to_str().unwrap();

        let mut file_dest = PathBuf::from(dest_str);
        file_dest.push(file_name_str);

        match symlink(file, &file_dest) {
            Ok(_) => info!(
                "file {} succefully linked to {}",
                file_name_str,
                file_dest.to_str().unwrap()
            ),

            Err(e) => error!(
                "failed to link {} to {}, due to: {}",
                file_name_str,
                file_dest.to_str().unwrap(),
                e
            ),
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
                file_dest.to_str().unwrap(),
                e
            ),
        };
    }
}

pub fn unlink(dotfiles_src_content: DirContent, src_path: PathBuf) {
    let src_content = scandir(&src_path);
    let src_symlinks = src_content.symlinks();

    for symlink in src_symlinks {
        let follow_symlink = match symlink.read_link() {
            Ok(p) => p,
            Err(e) => {
                error!(
                    "failed to follow {}, due to: {}",
                    symlink.to_str().unwrap(),
                    e
                );
                continue;
            }
        };

        if dotfiles_src_content.contains(&follow_symlink) {
            match delete_file(symlink) {
                Ok(_) => info!(
                    "{} succefully removed from your system.",
                    symlink.to_str().unwrap()
                ),
                Err(e) => error!(
                    "failed to remove {} from your system due to: {}",
                    symlink.to_str().unwrap(),
                    e
                ),
            };
        }
    }
}
