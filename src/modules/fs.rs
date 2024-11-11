use log::{ info, error };
use std::fs::{ DirEntry, read_dir, create_dir };
use std::io::{ Error, ErrorKind };
use std::path::{ Path, PathBuf };
use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub struct DirContent {
  files: Vec<String>,
  dirs: Vec<String>
}

impl DirContent {
  pub fn new(files: Vec<String>, dirs: Vec<String>) -> DirContent {
    DirContent {files, dirs}
  }

  pub fn files(&self) -> &Vec<String> {
    return &self.files;
  } 

  pub fn dirs(&self) -> &Vec<String> {
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

pub fn searchdir(origin: &str, target: &str) -> Result<String, Error> {
  if !Path::new(origin).exists() {
    error!("origin path does not exist: {}", origin);
    return Err(Error::from(ErrorKind::NotFound));
  }

  match craw(PathBuf::from(origin), PathBuf::from(target)) {
    Ok(p) => {
      if p.is_dir() {
        Ok(String::from(p.to_str().unwrap()))
      } else {
        error!("the element found is not a directory: {}", p.to_str().unwrap());
        Err(Error::from(ErrorKind::NotFound))
      }
    },

    Err(e) => {
      error!("{} not found crawling {}", target, origin);
      Err(e) 
    }
  }
}

pub fn scandir(path: &str) -> DirContent {
  let path = Path::new(path);
  let mut dirs: Vec<String> = Vec::new();
  let mut files: Vec<String> = Vec::new();

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
      dirs.push(
        x.path()
          .to_str()
          .unwrap()
          .to_string()
      );

    } else if x.path().is_file() {
      files.push(
        x.path()
          .to_str()
          .unwrap()
          .to_string()
      );
    }
  }

  DirContent::new(files, dirs)
}

pub fn symlink(src: &str, dest: &str) -> Result<(), Error>{
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

pub fn create_directory(path: &str) {
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