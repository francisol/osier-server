use std::rc::Rc;
use crate::repository;
use std::thread;
use std::sync::{Arc, Mutex,mpsc};
use crate::runner::config::TaskInfo;
use crate::runner::script::ArgScrpit;
use std::collections::BTreeMap;
use std::collections::HashMap;
use mustache::Template;
use std::process::Command;

struct TaskRunner {
   pub info:TaskInfo,
   pub  args:Vec<ArgScrpit>,
   pub  values: HashMap<String,String>,
   pub tmpl :Template,
}

impl TaskRunner{
    fn load(&mut self){
        for item in &self.info.args {
            let s=ArgScrpit::new(item.r#type.to_string(), item.name.to_string(), item.value.to_string()).unwrap();
            s.load(&item.options);
            self.args.push(s);
        }
    }
    fn first_run(&mut self)->bool{
        for item in &self.args {
            let (data,end) =  item.next();
            if end {
                return false;
            }
            self.values.insert(item.name.to_string(), data.to_string());
        }
        return true;
    }
    fn run(&mut self){
        // let temp=Vec::new();
        loop{
            if self.values.len() ==0{
                self.first_run();
            }

            let cmd= self.tmpl.render_to_string(&self.values).unwrap();
            // Command::new("sh -c").arg(cmd).output()
        }
    }
}


struct Task {
    id: i32,
    name: String,
    base_dir: String,
    core_num: i32,
    sender:mpsc::Sender<i32>,
    runner:TaskRunner,
}

impl Task {
    fn new(id: i32, name: String, base_dir: String, core_num: i32,sender:mpsc::Sender<i32>) -> Result<Task, crate::error::Error> {
        let data=  std::fs::read(format!("{}/task.yaml",base_dir))?;
        let info :TaskInfo= serde_yaml::from_slice(data.as_slice())?;
        let args=Vec::new();
        let values= HashMap::new();
        let tmpl= mustache::compile_str(&info.command).unwrap();
        let runner = TaskRunner{
            info,args,values,tmpl
        };
       return Ok(
        Task {
            id,name,base_dir,core_num,sender,runner
        }
       );
    }

    pub fn run(&self){
        thread::spawn(move || {
            
        });
    }
}
