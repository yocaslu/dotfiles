use log::{ info, error };
use std::fs::{ DirEntry, read_dir, create_dir };
use std::io::{ Error, ErrorKind };
use std::path::{ Path, PathBuf };
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub struct DirContent {
  files: Vec<PathBuf>, // change to Vec<PathBuf>
  dirs: Vec<PathBuf> // change to Vec<PathBuf>
}

impl DirContent {

  pub fn new() -> DirContent {
    DirContent {
      files: Vec::new(), 
      dirs: Vec::new()
    }
  }

  pub fn from(files: Vec<PathBuf>, dirs: Vec<PathBuf>) -> DirContent {
    DirContent {files, dirs}
  }

  pub fn files(&self) -> &Vec<PathBuf> {
    return &self.files;
  } 

  pub fn dirs(&self) -> &Vec<PathBuf> {
    return &self.dirs;
  } 
}

fn craw(path: PathBuf, target: PathBuf) -> Result<PathBuf, Error> {
  let mut stack: Vec<PathBuf> = Vec::new();
  stack.push(PathBuf::from(path));

  while !stack.is_empty() {
    let curr: PathBuf = stack.pop().unwrap(); // popping an new directory to craw
    match read_dir(&curr) {
      Ok(content) => {
        for entry in content.flatten() { // iterating through all the elements inside the directory
          if entry.file_name() == target {
            return Ok(entry.path());
          } else if entry.path().is_dir() { // stacking an new directory to craw
            stack.push(entry.path());
          } 
        }
      },

      Err(e) => {
        error!("failed to read {} due to: {}", &curr.to_str().unwrap(), e);
      }
    }
  } 

  return Err(Error::from(ErrorKind::NotFound)); 
}

pub fn searchdir(origin: &PathBuf, target: &PathBuf) -> Result<PathBuf, Error> {
  if !Path::new(origin).exists() {
    error!("origin path does not exist: {}", origin.to_str().unwrap());
    return Err(Error::from(ErrorKind::NotFound));
  }

  match craw(PathBuf::from(origin), PathBuf::from(target)) {
    Ok(p) => {
      if p.is_dir() {
        Ok(p)
      } else {
        error!("the element found is not a directory: {}", p.to_str().unwrap());
        Err(Error::from(ErrorKind::NotFound))
      }
    },

    Err(e) => {
      error!("{} not found crawling {}", target.to_str().unwrap(), origin.to_str().unwrap());
      Err(e) 
    }
  }
}

pub fn scandir(path: &PathBuf) -> DirContent {
  let path = Path::new(path);
  let mut files: Vec<PathBuf> = Vec::new();
  let mut dirs: Vec<PathBuf> = Vec::new();

  let content = match path.read_dir() {
    Ok(c) => {
      info!("{} succefully readed.", path.to_str().unwrap());
      c
    },
    Err(e) => {
      error!("failed to read {}: {}", path.to_str().unwrap(), e);
      exit(-2);
    }
  };

  let content: Vec<DirEntry> = content.flatten().collect();

  for x in &content {
    if x.path().is_dir() {
      dirs.push(x.path());

    } else if x.path().is_file() {
      files.push(x.path());
    }
  }

  DirContent::from(files, dirs)
}

pub fn symlink(src: &PathBuf, dest: &PathBuf) -> Result<(), Error>{
  let src = Path::new(src);
  let dest = Path::new(dest);
  
  if dest.exists() {
    return Err(Error::from(ErrorKind::AlreadyExists));
  }

  match std::os::unix::fs::symlink(src, dest) {
    Ok(_) => return Ok(()),
    Err(e) => return Err(e) 
  };
}

pub fn create_directory(path: &PathBuf) {
  let path = Path::new(path);
  if path.exists() {
    error!("{} already exists. Aborting.", path.to_str().unwrap());
    return;     
  }

  match create_dir(path) {
    Ok(_) => info!("{} succefully created.", path.to_str().unwrap()),
    Err(e) => {
      error!("failed to create {}: {}", path.to_str().unwrap(), e);
      exit(-2);
    }
  }
} 