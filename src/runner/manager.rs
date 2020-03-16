
use std::rc::Rc;
use crate::repository;
use crate::runner::RunnerManager;
use std::thread;
use std::sync::{Arc, Mutex,mpsc};

struct RunnerInfo{
    core_num:i32,
    task_num:i32,
    runing_core:i32,
}
unsafe impl std::marker::Sync for RunnerInfo{}
unsafe impl std::marker::Send for RunnerInfo{}


struct DefaultRunnerManager{
    info:Arc<RunnerInfo>,
    r:Arc<dyn repository::Repository>,
    sender:mpsc::Sender<i32>,
    receiver:Arc<Mutex<mpsc::Receiver<i32>>>,
}

fn new(core_num:i32,
    task_num:i32,
    runing_core:i32,
    r:&Arc<dyn repository::Repository>)->DefaultRunnerManager{
    let (tx, rx) =  mpsc::channel();
    let info=RunnerInfo{
        core_num:core_num,
        task_num:task_num,
        runing_core:runing_core,
    };
    return DefaultRunnerManager{
        info:Arc::new(info),
        r:r.clone(),
        sender:tx,
        receiver:Arc::new(Mutex::new(rx)),
    }
}
impl RunnerManager for DefaultRunnerManager {
    fn refresh(&self){
        let sender=self.sender.clone();
        let _ = sender.send(0);
    }
    fn start(&self){
        let rec =self.receiver.clone();
        let r=self.r.clone();
        let info = self.info.clone();
        let sender=self.sender.clone();
        thread::spawn(move || {
            loop{
                let id = match rec.lock().unwrap().recv(){
                    Ok(n)=>n,
                    Err(e)=>return
                };
                if id >0 {
                   let _ = r.finished(id);
                }
                let task= match r.get_wait_task(){
                    Ok(t)=>t,
                    Err(e)=>return,
                };
                if info.core_num < info.runing_core+task.core_num {
                    return;
                }
                let _=sender.send(0);
            }
        });
        
    }
}
impl DefaultRunnerManager {}