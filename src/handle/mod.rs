use crate::error::Result;
use crate::repository;
use crate::runner::RunnerManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time;

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum HanderResult {
    None,
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
}
impl CMDHandler for CreateCMD {
    fn handle(&self, handler: &JSONHandler) -> Result<HanderResult> {
        let r = handler.r.clone();
        let rm = handler.rm.clone();
        let task = repository::Task {
            name: self.name.to_string(),
            base_dir: self.base_dir.to_string(),
            core_num: self.core_num,
            created_at: time::get_time(),
            finished_at: None,
            id: 0,
            priority: self.priority,
            status: repository::TaskStatus::Wait,
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
                let cmd: CreateCMD = serde_json::from_slice(data)?;
                return self._handle(&cmd);
            }
            _ => Err(crate::error::Error::Normal("no match".to_string())),
        };
    }
}
