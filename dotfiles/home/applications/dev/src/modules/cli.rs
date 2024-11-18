use std::process::exit;
use std::path::PathBuf;

use clap::Arg;
use clap::ArgMatches;
use clap::ArgAction;
use clap::Command;
use clap::value_parser;
use log::error;

use crate::modules::proc;
use crate::modules::env;
use crate::modules::logger;

const DEFAULT_WORKDIR: &str = "{pwd}";
const DEFAULT_SESSION_NAME: &str = "{workdir}";
//const DEFAULT_APPLICATIONS: [&str ; 4] = ["nvim", "bash", "nnn", "lazygit"];

#[derive(Debug, Clone)]
pub struct Args {
  pub session_name: String,
  pub applicatons: Vec<String>,
  pub workdir: PathBuf,
  pub debug: bool,
  pub fzf: bool
}

impl Args {
  pub fn new() -> Args {
    Args {
      session_name: String::default(),
      applicatons: Vec::default(),
      workdir: PathBuf::default(),
      debug: false,
      fzf: false
    }
  }

  fn clap_args() -> Vec<Arg> {
    let session_name = Arg::new("session_name")
      .short('n')
      .long("session_name")
      //.default_value(DEFAULT_SESSION_NAME)
      .value_parser(value_parser!(String))
      .help("define the tmux session name");
      
    let workdir = Arg::new("workdir")
      .short('d')
      .long("workdir")
      //.default_value(DEFAULT_WORKDIR)
      .value_parser(value_parser!(PathBuf))
      .help("directory where tmux and applications will be opened.");
    
    let applications = Arg::new("applications")
      .short('a')
      .long("apps")
      //.value_parser(value_parser!(String))
      .num_args(0..)
      .help("applications you want to se inside your tmux session");

    let fzf = Arg::new("fzf")
      .long("fzf")
      .action(ArgAction::SetTrue)
      .help("use fzf to fuzzy find the working directory.");

    let debug = Arg::new("debug")
      .long("debug")
      .action(ArgAction::SetTrue)
      .help("activate debug logs");

    return [
      session_name,
      workdir,
      applications,
      fzf,
      debug,
    ].to_vec();
  }

  pub fn parse(cmd: &ArgMatches) -> Args {
    let mut args: Args = Args::new();

    args.debug = cmd.get_flag("debug");
    args.fzf = cmd.get_flag("fzf");

    if args.debug {
      logger::init_logger();
    }

    if args.fzf {
      args.workdir = fzf();  
    } else {
      args.workdir = match cmd.get_one::<PathBuf>("workdir") {
        Some(p) => p.to_path_buf(),
        None => env::pwd() 
      };
    }

    args.session_name = match cmd.get_one::<String>("session_name") {
      Some(s) => s.to_string(),
      None => args.workdir.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() 
    };

    if let Some(applicatons) = cmd.get_many::<String>("applications") {
      for x in applicatons {
        args.applicatons.push(x.to_string());
      }
    }

    return args;
  }
}

pub fn parse() -> Args {
  let cmd = Command::new("tmux-dev")
    .author("dio")
    .about("creates an tmux session with nvim, bash, nnn and lazygit")
    .args(Args::clap_args());

  Args::parse(&cmd.get_matches()) 
}

pub fn fzf() -> PathBuf {
  match proc::execute("fzf", ["--walker=dir"].to_vec()) {
    Ok(o) => return PathBuf::from(String::from_utf8(o.stdout).unwrap()),
    Err(e) => {
      error!("failed to execute fzf: {}", e);
      exit(-1);
    }
  }
}


// this motherfucker is giving me stack overflow
//#[derive(Debug, Clone, Parser, PartialEq)]
//#[command(author = "Dio", about = "creates an tmux session with nvim, bash, nnn and lazygit")]
//pub struct Args {
//  #[arg(short, long, help = "development working directory (where tmux will be started)", default_value = DEFAULT_WORKDIR)]
//  pub workdir: PathBuf,
//
//  #[arg(short, long, help = "tmux session name", default_value = DEFAULT_SESSION_NAME)]
//  pub session_name: String,
//
//  #[arg(short, long, help = "applicatons to start inside tmux session")]
//  pub applicatons: Vec<String>, 
//
//  #[arg(short, long, help = "Enable debug information")]
//  pub debug: bool,
//
//  #[arg(long, help = "use fzf to fuzzy find the working directory")]
//  pub fzf: bool
//}
