use rusqlite::Result;
use time::Timespec;
use num;

// use num_traits::{FromPrimitive,ToPrimitive};
#[derive(FromPrimitive,ToPrimitive)]
pub enum TaskStatus {
    Wait,
    Doing,
    Done,
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
pub struct Task {
    pub id: i32,
    pub name: String,
    pub priority: i32,
    pub base_dir: String,
    pub status: TaskStatus,
    pub core_num: i32,
    pub created_at: Timespec,
    pub finished_at: Option<Timespec>,
}

pub trait Repository: std::marker::Sync + std::marker::Send {
    fn save(&self, task: &Task) -> Result<bool>;
    fn get_wait_tasks(&self) -> Result<Vec<Task>>;
    fn get_wait_task(&self) -> Result<Task>;
    fn doing(&self, id: i32) -> Result<bool>;
    fn finished(&self, id: i32) -> Result<bool>;
}

pub mod sqlite;
