mod modules;
use modules::{cli, env, fs, log as mylog};
use std::path::PathBuf;

fn find_modules() -> PathBuf {
    let mut pwd = env::get_pwd();
    pwd.pop();

    match fs::searchdir(&pwd, &PathBuf::from("modules")) {
        Ok(s) => s,
        Err(_) => {
            log::error!("failed to find repo's modules.");
            std::process::exit(-1);
        }
    }
}

fn main() {
    mylog::init_logger();
    match cli::get_args().cmd {
        cli::Commands::Install {
            targets,
            overwrite,
            all,
        } => setup::install(targets, overwrite, all),
        cli::Commands::Uninstall { targets, all } => setup::uninstall(targets, all),
        cli::Commands::Check => {
            todo!()
        }
    }
}

pub mod setup {
    use crate::{
        find_modules,
        modules::{env, fs},
    };
    use log::{error, info};
    use std::{
        path::{Path, PathBuf},
        process::exit,
    };

    pub fn install(_modules: Vec<String>, overwrite: bool, all: bool) {
        let modules_path = find_modules();
        if all {
            install::link_home(&modules_path, overwrite);
            install::link_config(&modules_path, overwrite);
        }
    }

    pub fn uninstall(_modules: Vec<String>, all: bool) {
        let modules_path = find_modules();
        if all {
            uninstall::unlink_home(&modules_path);
            uninstall::unlink_config(&modules_path);
        }
    }

    mod install {
        use super::*;
        use std::fs::create_dir;
        // todo: install selected modules and all modules

        pub fn link_home(dotfiles_path: &PathBuf, overwrite: bool) {
            let home_path = env::get_home_path();
            let dotfiles_home_path = match fs::searchdir(dotfiles_path, &PathBuf::from("home")) {
                Ok(s) => s,
                Err(_) => {
                    error!("failed to find home folder in dotfiles directory.");
                    exit(-1);
                }
            };

            // reading all dotfiles inside repo's home module
            let dotfiles_home_content = fs::scandir(&dotfiles_home_path);

            if overwrite {
                fs::overwrite(dotfiles_home_content, &home_path);
            } else {
                fs::link(dotfiles_home_content, &home_path);
            }
        }

        pub fn link_config(modules_path: &PathBuf, overwrite: bool) {
            let dotfiles_config_path = match fs::searchdir(modules_path, &PathBuf::from("config")) {
                Ok(s) => s,
                Err(_) => {
                    error!("failed to find config folder in dotfiles directory.");
                    exit(-1);
                }
            };

            let mut config_path = env::get_home_path();
            config_path.push(".config/");

            if !Path::new(&config_path).exists() {
                match create_dir(&config_path) {
                    Ok(_) => info!(".config was succefully created."),
                    Err(e) => {
                        error!("failed to create .config in HOME due to {}", e);
                        exit(-2);
                    }
                };
            }

            // reading all dotfiles inside repo's config module
            let dotfiles_config_content = fs::scandir(&dotfiles_config_path);

            if overwrite {
                fs::overwrite(dotfiles_config_content, &config_path);
            } else {
                fs::link(dotfiles_config_content, &config_path);
            }
        }
    }

    mod uninstall {
        use super::*;
        pub fn unlink_home(modules_path: &PathBuf) {
            let home_path = env::get_home_path();
            let dotfiles_home_path = match fs::searchdir(&modules_path, &PathBuf::from("home")) {
                Ok(p) => p,
                Err(e) => {
                    error!(
                        "failed to locate home directory in dotfiles repository due to {}",
                        e
                    );
                    exit(-2);
                }
            };

            let dotfiles_home_content = fs::scandir(&dotfiles_home_path);
            fs::unlink(&dotfiles_home_content, home_path);
        }

        pub fn unlink_config(modules_path: &PathBuf) {
            let mut config_path = env::get_home_path();
            config_path.push(".config");

            let dotfiles_config_path = match fs::searchdir(&modules_path, &PathBuf::from("config"))
            {
                Ok(p) => p,
                Err(e) => {
                    error!(
                        "failed to locate config directory in dotfiles repository due to {}",
                        e
                    );
                    exit(-2);
                }
            };

            let dotfiles_config_content = fs::scandir(&dotfiles_config_path);
            fs::unlink(&dotfiles_config_content, config_path);
        }
    }

    mod check {}
}
