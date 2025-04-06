use inquire::Confirm;
use log::{error, info, warn};
use std::fs::{read_dir, remove_dir_all, remove_file, DirEntry};
use std::io::{Error, ErrorKind};
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

pub fn overwrite(content: Vec<PathBuf>, dest: &PathBuf) {
    let will_delete: Vec<PathBuf> = content
        .iter()
        .filter(|x| x.exists())
        .map(|x| {
            let mut path = dest.clone();
            path.push(x.file_name().unwrap());
            path
        })
        .collect();

    warn!("the following dotfiles already exists and will be DELETED: {:#?}\nRun the program without --overwrite to keep these dotfiles.", &will_delete);
    let confirmation: bool = Confirm::new("Are you sure you want to delete these files?")
        .with_default(false)
        .prompt()
        .unwrap();

    if !confirmation {
        error!("Installation aborted.");
        std::process::exit(-1);
    }

    // deleting impostor dotfiles
    will_delete.iter().for_each(|x| {
        if x.is_file() || x.is_symlink() {
            match remove_file(x) {
                Ok(_) => info!("{} succefully deleted.", x.to_str().unwrap(),),
                Err(e) => error!("failed to remove {} due to: {}.", x.to_str().unwrap(), e),
            }
        } else if x.is_dir() {
            match remove_dir_all(x) {
                Ok(_) => info!("{} directory succefully deleted.", x.to_str().unwrap()),
                Err(e) => error!(
                    "failed to remove directory {} due to: {}",
                    x.to_str().unwrap(),
                    e
                ),
            }
        }
    });

    // creating path and linking
    content.iter().for_each(|x| {
        let mut x_dest = dest.clone();
        x_dest.push(x.file_name().unwrap());

        match symlink(x, &x_dest) {
            Ok(_) => info!("{} succefully linked.", x.to_str().unwrap()),
            Err(e) => error!("failed to link {} due to: {}", x_dest.to_str().unwrap(), e),
        }
    });
}

pub fn link(content: Vec<PathBuf>, dest: &PathBuf) {
    // warning which modules wont be installed
    let wont_install: Vec<PathBuf> = content
        .iter()
        .filter(|x| x.exists())
        .map(|x| {
            let mut path = dest.clone();
            path.push(x.file_name().unwrap());
            path
        })
        .collect();

    info!("the following dotfiles already exist in {} and won't be linked: {:#?}\nRun the program with --overwrite flag to replace these dotfiles.", dest.to_str().unwrap(), &wont_install);
    content.iter().filter(|x| x.exists()).for_each(|x| {
        let mut x_dest: PathBuf = dest.clone();
        x_dest.push(x.file_name().unwrap());
        // dbg!(x_dest);
        match symlink(x, &x_dest) {
            Ok(_) => info!("{} succefully linked.", x.to_str().unwrap()),
            Err(e) => error!("failed to link {} due to: {}", x_dest.to_str().unwrap(), e),
        }
    });
}

pub fn unlink(dotfiles_src_content: &Vec<PathBuf>, src_path: PathBuf) {
    dotfiles_src_content.iter().for_each(|x| {
        let mut x_src = src_path.clone();
        x_src.push(x.file_name().unwrap());
        dbg!(&x_src);

        match remove_file(&x_src) {
            Ok(_) => info!(
                "{} succefully removed from your computer.",
                &x_src.to_str().unwrap()
            ),
            Err(e) => error!(
                "failed to remove {} due to: {}",
                &x_src.to_str().unwrap(),
                e
            ),
        }
    });
}
