use rusqlite::Result;
use num;
use serde::{Serialize,Deserialize};
 use chrono::{DateTime,Local};
// use num_traits::{FromPrimitive,ToPrimitive};
#[derive(FromPrimitive,Debug,ToPrimitive,Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum TaskStatus {
    Wait,
    Doing,
    Done,
    Error,
}

impl rusqlite::types::ToSql for TaskStatus {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(num::ToPrimitive::to_i32(self).unwrap_or(0).into()),
        ))
    }
}

impl rusqlite::types::FromSql for TaskStatus {
    fn column_result(v: rusqlite::types::ValueRef<'_>) -> std::result::Result<Self, rusqlite::types::FromSqlError> { 
        match v.as_i64(){
            Ok(v)=> Ok(num::FromPrimitive::from_i64(v).unwrap_or(TaskStatus::Wait)),
            Err(e)=>Ok(TaskStatus::Wait),
        }
     }
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub priority: i32,
    pub base_dir: String,
    pub status: TaskStatus,
    pub core_num: i32,
    pub created_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
    pub username:String,
    pub msg:Option<String>,
}


pub trait Repository: std::marker::Sync + std::marker::Send {
    fn save(&self, task: &Task) -> Result<bool>;
    fn clear(&self)->Result<bool>;
    fn do_error(&self,id:i32,msg:&String)->Result<usize>;
    fn get_wait_tasks(&self) -> Result<Vec<Task>>;
    fn get_wait_task(&self) -> Result<Task>;
    fn doing(&self, id: i32) -> Result<bool>;
    fn finished(&self, id: i32) -> Result<bool>;
    fn delete(&self, name: &String) -> Result<bool>;
    fn reset(&self, name: &String) -> Result<bool>;
    fn query(&self, name: &String) -> Result<Task>;
    fn list(&self, form: i32,to:i32) -> Result<Vec<Task>>;
    fn list_by_status(&self, form: i32,to:i32,status:&TaskStatus) -> Result<Vec<Task>>;
}

pub mod sqlite;
