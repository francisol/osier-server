use crate::repository;
use crate::repository::TaskStatus;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use time::Timespec;
struct SQLliteRepository {
    conn: Mutex<rusqlite::Connection>,
}

impl SQLliteRepository {
    pub fn new(path: &String) -> SQLliteRepository {
        let conn = match Connection::open(path) {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(-1);
            }
        };
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
             "INSERT INTO tasks (task_name, priority,base_dir,status,core_num,created_at,finished_at) values (?1, ?2,?3,?4,?5,?6,?7)",
             params![&task.name, task.priority,&task.base_dir,task.status,task.core_num,&task.created_at,&task.finished_at],
         ){
             Ok(r)=> Ok(r>0),
             Err(e)=>Err(e),
         };
    }
    fn get_wait_task(&self) -> Result<repository::Task> {
        let conn = &self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, task_name, priority, core_num,base_dir,status,created_at,finished_at FROM tasks where status==1 limit 1")?;
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
            });
        });
        return task;
    }

    fn get_wait_tasks(&self) -> Result<Vec<repository::Task>> {
        let mut resut = Vec::<repository::Task>::new();
        let conn = &self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, task_name, priority, core_num,base_dir,status,created_at,finished_at FROM tasks where status==1")?;
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
        match self._update_status(id, TaskStatus::Doing, Some(t)) {
            Ok(n) => Ok(n > 0),
            Err(e) => Err(e),
        }
    }
}
