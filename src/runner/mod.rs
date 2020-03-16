

pub trait RunnerManager:std::marker::Send +std::marker::Send+std::marker::Sync {
    fn refresh(&self);
    fn start(&self);
}

pub mod manager;
pub mod task;
pub mod script;
pub mod config;
