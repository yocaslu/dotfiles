mod modules;
use modules::cli;
use modules::proc;
use modules::tmux;

fn main() {
  let args = cli::parse();
  let session = tmux::Session::from(
    args.session_name, 
    args.workdir, 
    args.applicatons
  );
  
  tmux::create_session(&session);
  tmux::create_windows(&session);
  tmux::attach(&session);
}

// todo: generate more logs with more informations about the variables
// todo: refactor proc, spawning more sophisticated process
// todo: check if an tmux session is already initiated, if so, create an diferent 
// todo: check if all applications are installed
// todo: create an config file to customize which applications will be started
