use std::env::var;
use std::env::set_var;

pub fn init_logger() {
  if var("RUST_BACKTRACE").is_err() { set_var("RUST_BACKTRACE", "1"); }
  if var("RUST_LOG").is_err() { set_var("RUST_LOG", "trace"); }
  
  env_logger::init()
}
