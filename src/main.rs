pub mod app;
pub mod domain;
pub mod infra;
pub mod server;
pub mod shared;

fn main() {
    server::start();
}
