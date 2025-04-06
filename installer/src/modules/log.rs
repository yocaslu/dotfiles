use std::env::set_var;
use std::env::var;

// define the RUST_LOG variable if it is not initialized
// and initialize the env_logger
pub fn init_logger() {
    if var("RUST_LOG").is_err() {
        set_var("RUST_LOG", "info");
    }

    env_logger::init();
}
