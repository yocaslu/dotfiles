use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq, Clone)]
#[command(author, about, version)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// symlink dotfiles
    Install {
        #[arg(help = "path to modules you want to install")]
        targets: Vec<String>,

        #[arg(long, default_value = "false", help = "instal all modules")]
        all: bool,

        #[arg(
            long,
            default_value = "false",
            help = "overwrite the actual configuration (be careful)"
        )]
        overwrite: bool,
    },

    /// remove dotfiles symlinks
    Uninstall {
        #[arg(help = "path to modules you want to remove")]
        targets: Vec<String>,

        #[arg(short, long, default_value = "false", help = "uninstall all modules")]
        all: bool,
    },

    /// Check if all links still valid
    Check,
}

pub fn get_args() -> Args {
    Args::parse()
}
