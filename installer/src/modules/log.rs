use std::env::var;
use std::env::set_var;

// define the RUST_LOG variable if it is not initialized
// and initialize the env_logger
pub fn init_logger() {
  if var("RUST_LOG").is_err() {
    set_var("RUST_LOG", "trace");
  }

  env_logger::init();
}