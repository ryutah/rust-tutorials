#[macro_use]
extern crate log;
extern crate env_logger;

// Exec like `RUST_LOG=debug cargo run`
fn main() {
    env_logger::init();

    debug!("[{}:L{}] This is debug", file!(), line!());
    info!("[{}:L{}] This is info", file!(), line!());
    warn!("[{}:L{}] This is warn!", file!(), line!());
    error!("[{}:L{}] This is error", file!(), line!());
}
