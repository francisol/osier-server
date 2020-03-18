use crate::repository;
use crate::runner::task::Task;
use crate::runner::RunnerManager;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
struct RunnerInfo {
    core_num: i32,
    task_num: i32,
    runing_core: i32,
}
unsafe impl std::marker::Sync for RunnerInfo {}
unsafe impl std::marker::Send for RunnerInfo {}

pub struct DefaultRunnerManager {
    info: Arc<RunnerInfo>,
    r: Arc<dyn repository::Repository>,
    sender: Mutex<mpsc::Sender<i32>>,
    receiver: Arc<Mutex<mpsc::Receiver<i32>>>,
}

pub fn new(
    core_num: i32,
    task_num: i32,
    r: Arc<dyn repository::Repository>,
) -> DefaultRunnerManager {
    let (tx, rx) = mpsc::channel();
    let info = RunnerInfo {
        core_num: core_num,
        task_num: task_num,
        runing_core: 0,
    };
    return DefaultRunnerManager {
        info: Arc::new(info),
        r: r,
        sender: Mutex::new(tx),
        receiver: Arc::new(Mutex::new(rx)),
    };
}
impl RunnerManager for DefaultRunnerManager {
    fn refresh(&self) {
        let sender = self.sender.lock().unwrap().clone();
        let _ = sender.send(0);
    }
    fn start(&self) {
        let rec = self.receiver.clone();
        let r = self.r.clone();
        let info = self.info.clone();
        let sender = self.sender.lock().unwrap().clone();
        let s=sender.clone();
        let  core_num=info.core_num;
        let mut runing_core=info.runing_core;
        thread::spawn(move || {
            let mut cache = HashMap::new();
            loop {
                let id = match rec.lock().unwrap().recv() {
                    Ok(n) => n,
                    Err(e) => return,
                };
                if id > 0 {
                    let count = cache.get_mut(&id).unwrap();
                    (*count) -= 1;
                    runing_core -=1;
                    if *count == 0 {
                        let _ = r.finished(id).unwrap();
                    }
                }
                let task = match r.get_wait_task() {
                    Ok(t) => t,
                    Err(e) => continue,
                };
                debug!("Task {:?}",&task);
                if core_num < runing_core + task.core_num {
                    return;
                }
                let t = match Task::new(
                    task.id,
                    task.name,
                    task.base_dir,
                    task.core_num,
                    task.username,
                    sender.clone(),
                ){
                    Ok(t)=>t,
                    Err(e)=>{
                        error!("{:?}",e);
                        let _= r.do_error(task.id,&format!("{:?}",e));
                        continue;
                    }
                };
                cache.insert(task.id, task.core_num);
                t.run();
                runing_core+=task.core_num;
                let _= r.doing(task.id);
                let _ = sender.send(0);
            }
        });
       let _ = s.send(0);
    }
}
impl DefaultRunnerManager {}
