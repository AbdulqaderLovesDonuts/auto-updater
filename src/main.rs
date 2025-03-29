// Add these lines to src/main.rs
mod cli;
mod config; // This declares the config module
mod repositories;
mod update;
mod utils;

// You can re-export specific items if needed
pub use config::*;

fn main() {
    println!("Hello, world!");
}
