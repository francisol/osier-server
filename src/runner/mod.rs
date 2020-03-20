

pub trait RunnerManager:std::marker::Send +std::marker::Send+std::marker::Sync {
    fn refresh(&self);
    fn start(&self);
    fn get_current_task_count(&self)->i32;
    fn get_current_core_runing_count(&self)->i32;
    fn get_core_number(&self)->i32;
    fn get_task_count(&self)->i32;
}

pub mod manager;
pub mod task;
pub mod script;
pub mod config;
