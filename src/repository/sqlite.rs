use crate::repository;
use crate::repository::TaskStatus;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use time::Timespec;
pub struct SQLliteRepository {
    conn: Mutex<rusqlite::Connection>,
}

const CREATE_TABLE: &'static str=r#"
CREATE TABLE IF NOT EXISTS "tasks" (
	"id"	INTEGER PRIMARY KEY AUTOINCREMENT,
	"task_name"	TEXT UNIQUE,
	"priority"	INTEGER DEFAULT 0,
	"core_num"	INTEGER DEFAULT 1,
	"base_dir"	TEXT,
	"status"	INTEGER,
	"created_at"	TEXT,
    "finished_at"	TEXT,
    "username" TEXT,
	"msg"	TEXT
)"#;

impl SQLliteRepository {
    pub fn new(path: &String) -> SQLliteRepository {
        let conn = match Connection::open(path) {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(-1);
            }
        };
        conn.execute(CREATE_TABLE,params![]).unwrap();
        SQLliteRepository {
            conn: Mutex::new(conn),
        }
    }
    fn _update_status(
        &self,
        id: i32,
        status: TaskStatus,
        finished_at: Option<Timespec>,
    ) -> Result<usize> {
        let conn = &self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks set status = ?1,finished_at=?2 where id= ?3",
            params![status, finished_at, id],
        )
    }
}
impl repository::Repository for SQLliteRepository {
    fn save(&self, task: &repository::Task) -> Result<bool> {
        let conn = &self.conn.lock().unwrap();
        return  match conn.execute(
             "INSERT INTO tasks (task_name, priority,base_dir,status,core_num,created_at,finished_at,username) values (?1, ?2,?3,?4,?5,?6,?7,?8)",
             params![&task.name, task.priority,&task.base_dir,task.status,task.core_num,&task.created_at,&task.finished_at,&task.username],
         ){
             Ok(r)=> Ok(r>0),
             Err(e)=>Err(e),
         };
    }
    fn clear(&self)->Result<bool>{
        let conn = &self.conn.lock().unwrap();
        conn.execute(
            "UPDATE tasks set status = 0 where status= 1",
            params![])?;
        return Ok(true);
    }
    fn get_wait_task(&self) -> Result<repository::Task> {
        let conn = &self.conn.lock().unwrap();
        debug!("Query");
        let mut stmt = conn.prepare("SELECT id, task_name, priority, core_num,base_dir,status,created_at,finished_at,username FROM tasks where status==0 limit 1")?;
        let task = stmt.query_row(params![], |row| {
            return Ok(repository::Task {
                id: row.get(0)?,
                name: row.get(1)?,
                priority: row.get(2)?,
                core_num: row.get(3)?,
                base_dir: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                finished_at: row.get(7)?,
                username:row.get(8)?,
            });
        });
        return task;
    }
    fn do_error(&self,id:i32,msg:&String)->Result<usize>{
        let conn = &self.conn.lock().unwrap();
        let t =time::get_time();
        conn.execute(
            "UPDATE tasks set status = ?1,finished_at=?2,msg=?3 where id= ?4",
            params![TaskStatus::Error, t,msg, id],
        )
    }
    fn get_wait_tasks(&self) -> Result<Vec<repository::Task>> {
        let mut resut = Vec::<repository::Task>::new();
        let conn = &self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, task_name, priority, core_num,base_dir,status,created_at,finished_at,username FROM tasks where status==0")?;
        let _tasks = stmt.query_map(params![], |row| {
            resut.push(repository::Task {
                id: row.get(0)?,
                name: row.get(1)?,
                priority: row.get(2)?,
                core_num: row.get(3)?,
                base_dir: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                finished_at: row.get(7)?,
                username:row.get(8)?,
            });
            return Ok(());
        });
        return Ok(resut);
    }
    fn doing(&self, id: i32) -> Result<bool> {
        match self._update_status(id, TaskStatus::Doing, None) {
            Ok(n) => Ok(n > 0),
            Err(e) => Err(e),
        }
    }
    fn finished(&self, id: i32) -> Result<bool> {
        let t = time::get_time();
        debug!("finished {}",id);
        match self._update_status(id, TaskStatus::Done, Some(t)) {
            Ok(n) => Ok(n > 0),
            Err(e) => Err(e),
        }
    }
}
