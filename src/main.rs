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
extern crate simple_logging;


fn main() {
    // std::panic::set_hook(Box::new(|panic_info|{
    //     let msg = match panic_info.payload().downcast_ref::<&'static str>() { 
    //         Some(s) => *s, 
    //         None => match panic_info.payload().downcast_ref::<String>() { 
    //             Some(s) => &s[..], 
    //             None => "Box<Any>", 
    //         } 
    //     }; 
    //         println!("Exit by {}",msg);

    //     // print!("panic info: {:?}",panic_info.message.);
    // }));
    let c = config::get_config();
    if cfg!(debug_assertions) {
        env_logger::init();
        println!("Debugging enabled");
    } else {
        println!("Debugging disabled");
        simple_logging::log_to_file(format!("{}/info.log",&c.base_dir), log::LevelFilter::Info).unwrap();
    }
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
