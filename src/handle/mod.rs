use crate::error::Result;
use crate::repository;
use crate::runner::RunnerManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::enum_variant_names)]
#[serde(untagged)]
pub enum HanderResult {
    None,
    TaskList(Vec<repository::Task>),
    Task(repository::Task),
    ServerStatus(ServerStatus),
}
impl std::fmt::Display for HanderResult{
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
    match self{
        HanderResult::None=>write!(f,""),
        HanderResult::TaskList(data)=>write!(f,"{:?}",data),
        HanderResult::Task(data) => write!(f,"{:?}",data),
        HanderResult::ServerStatus(data)=>write!(f,"{:?}",data),
    }
 }
}

pub trait Handler: std::marker::Sync + std::marker::Send {
    fn handle(&self, name: &str, data: &[u8]) -> Result<HanderResult>;
}

pub trait CMDHandler {
    fn handle(&self, handler: &JSONHandler) -> Result<HanderResult>;
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateCMD {
    pub name: String,
    pub priority: i32,
    pub base_dir: String,
    pub core_num: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryListCMD {
    pub from: i32,
    pub to: i32,
    pub status:Option<repository::TaskStatus>,
}
#[derive(Serialize, Deserialize, Debug)]
struct RemoveCMD {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RestartCMD {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryCMD {
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ServerStatus {
    core_num: i32,
    task_num: i32,
    current_task_num: i32,
    runing_core: i32,
}



#[derive(Debug)]
struct ServerStatusCMD {}

impl CMDHandler for CreateCMD {
    fn handle(&self, handler: &JSONHandler) -> Result<HanderResult> {
        let r = handler.r.clone();
        let rm = handler.rm.clone();
        let task = repository::Task {
            name: self.name.to_string(),
            base_dir: self.base_dir.to_string(),
            core_num: self.core_num,
            created_at: chrono::Local::now(),
            finished_at: None,
            id: 0,
            priority: self.priority,
            username: self.username.to_string(),
            status: repository::TaskStatus::Wait,
            msg:None,
        };
        let resut = r.save(&task)?;
        if resut {
            rm.refresh();
            return Result::Ok(HanderResult::None);
        } else {
            return Result::Err(crate::error::Error::Normal(format!("error",)));
        }
    }
}
impl CMDHandler for QueryListCMD {
    fn handle(&self, handle: &JSONHandler) -> Result<HanderResult> {
        let data=  match &self.status {
            None =>handle.r.list(self.from, self.to,)?,
            Some(status)=> handle.r.list_by_status(self.from, self.to, status)?,
        };
        return  Ok(HanderResult::TaskList(data));
    }
}

impl CMDHandler for RestartCMD{
    
fn handle(&self, handle: &JSONHandler) -> Result<HanderResult> { 
    handle.r.reset(&self.name)?;
    handle.rm.refresh();
    Ok(HanderResult::None)
 }
}

impl CMDHandler for RemoveCMD{
    
    fn handle(&self, handle: &JSONHandler) -> Result<HanderResult> { 
        handle.r.delete(&self.name)?;
        Ok(HanderResult::None)
     }
}

impl CMDHandler for QueryCMD{
    fn handle(&self, handle: &JSONHandler) -> Result<HanderResult> { 
        let task= handle.r.query(&self.name)?;
        Ok(HanderResult::Task(task))
     }
}

impl CMDHandler for ServerStatusCMD{
    fn handle(&self, handle: &JSONHandler) -> Result<HanderResult> { 
        let st=ServerStatus{
            core_num: handle.rm.get_core_number(),
            task_num: handle.rm.get_task_count(),
            current_task_num: handle.rm.get_current_task_count(),
            runing_core: handle.rm.get_current_core_runing_count(),
        };
        Ok(HanderResult::ServerStatus(st))
     }
}

pub struct JSONHandler {
    pub r: Arc<dyn repository::Repository>,
    pub rm: Arc<dyn RunnerManager>,
}
impl JSONHandler {
    fn _handle<T: CMDHandler>(&self, handler: &T) -> Result<HanderResult> {
        return handler.handle(&self);
    }
}
impl Handler for JSONHandler {
    fn handle(&self, name: &str, data: &[u8]) -> Result<HanderResult> {
        return match name {
            "create" => {
                debug!("create {}", std::str::from_utf8(data).unwrap());
                let cmd: CreateCMD = serde_json::from_slice(data)?;
                return self._handle(&cmd);
            },
            "query_list"=>{
                let cmd:QueryListCMD= serde_json::from_slice(data)?;
                return self._handle(&cmd);
            },
            "query"=>{
                let cmd:QueryCMD= serde_json::from_slice(data)?;
                return self._handle(&cmd);
            },
            "remove" =>{
                let cmd:RemoveCMD= serde_json::from_slice(data)?;
                return self._handle(&cmd);
            },
            "restart" =>{
                let cmd:RestartCMD= serde_json::from_slice(data)?;
                return self._handle(&cmd);
            },
            "status" => {
               let cmd = ServerStatusCMD{};
               return self._handle(&cmd);
            }
            _ => Err(crate::error::Error::Normal("no match command".to_string())),
        };
    }
}
