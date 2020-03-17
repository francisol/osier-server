mod config;
mod error;
mod handle;
mod lua;
mod repository;
mod runner;
mod server;
use crate::repository::Repository;
use crate::runner::RunnerManager;
use crate::server::Server;
use std::fs::File;
use std::iter::FromIterator;
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};
fn main() {
    env_logger::init();
    std::panic::set_hook(Box::new(|panic_info|{
        error!("panic info: {:?}",panic_info);
    }));
    let c = config::get_config();
    std::env::set_current_dir(&c.base_dir).expect("set_current_dir fail");
    let db =format!("{}/db",&c.base_dir);
    let repo= repository::sqlite::SQLliteRepository::new(&db);
    repo.clear().expect("clear status fail");
    let r=Arc::new(repo);
    let rm =runner::manager::new(c.core_num,c.core_num,r.clone());
    rm.start();
    let h= handle::JSONHandler{
        r:r.clone(),
        rm:Arc::new(rm)
    };
    let servr= server::new(Arc::new(h));
    servr.start(c.port).expect("server start fail");
}
