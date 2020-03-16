mod config;
mod error;
mod handle;
mod lua;
mod repository;
mod runner;
mod server;
use std::iter::FromIterator;
use std::sync::{mpsc, Arc, Mutex};
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;
fn main() {
    let c = config::get_config();
}
