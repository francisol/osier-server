

pub trait RunnerManager {
    fn refresh(&self);
    fn start(&self);
}

pub mod manager;
pub mod task;
pub mod script;
pub mod config;
