use crate::repository;
use crate::runner::config::TaskInfo;
use crate::runner::script::ArgScrpit;
use mustache::Template;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex,MutexGuard};
use std::thread;

pub struct TaskRunner {
    pub info: TaskInfo,
    pub args: Vec<ArgScrpit>,
    pub tmpl: Template,
    pub result_tmpl: Template,
    base_dir: String,
}

fn get_arg_scrpit(info:&TaskInfo,base_dir: &String)->Vec<ArgScrpit> {
    let mut args =Vec::new();
    for item in &info.args {
        let s = ArgScrpit::new(
            item.r#type.to_string(),
            item.name.to_string(),
            item.value.to_string(),
        )
        .unwrap();
        s.add_glob_var("base_dir", base_dir);
        s.load(&item.options);
        args.push(s);
    }
    return args;
}
impl TaskRunner {
    fn first_run(&self, values: &mut HashMap<String, String>) -> bool {
        let args=&self.args;
        for item in args {
            let (data, end) = item.next();
            if end {
                return false;
            }
            values.insert(item.name.to_string(), data.to_string());
        }
        return true;
    }

    fn next_run(&self,values: &mut HashMap<String, String>) -> bool {
        // debug!("run next_run");
        let mut index = 0;
        for arg in &self.args {
            let (v, end) = arg.next();
            if end {
                index+=1;
                continue;
            }
            let old_v = values.get_mut(&arg.name).unwrap();
            (*old_v) = v;
            break;
        }
        if self.args.len()==index {
            return false;
        }
        for i in 0..index {
            let arg= self.args.get(i).unwrap();
            arg.reset();
            let (v, _) = arg.next();
            let old_v = values.get_mut(&arg.name).unwrap();
            (*old_v) = v;
        }
        return true;
    }

    fn run(&self) {
        let mut values:HashMap<String,String> = HashMap::new();
        loop {
            let cmd: String;
            let resut_output: String;
            {
                if !((values.len() == 0 && self.first_run(&mut values)) || self.next_run(&mut values)) {
                    break;
                }
                cmd = self.tmpl.render_to_string(&values).unwrap();
                resut_output = self.result_tmpl.render_to_string(&values).unwrap();
            }
            let mut path = std::path::PathBuf::from(&self.base_dir);
            path.push(resut_output);
            if path.exists(){
                continue;
            }
            let tmp_path =std::path::PathBuf::from(format!("{}.tmp",path.display()));
            let parent = path.parent().unwrap();
            let _ = std::fs::create_dir_all(&parent);
            let outputs = File::create(&tmp_path).unwrap();
            let errors = outputs.try_clone().unwrap();
            debug!("cmd :{}",cmd);
            Command::new("sh")
                .arg("-c")
                .current_dir(&self.base_dir)
                .arg(cmd)
                .stdout(Stdio::from(outputs))
                .stderr(Stdio::from(errors))
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();
            let _= std::fs::rename(tmp_path, path).unwrap();
        }
    }
}

pub struct Task {
    id: i32,
    name: String,
    base_dir: String,
    core_num: i32,
    sender: mpsc::Sender<i32>,
    runner: Arc<TaskRunner>,
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        base_dir: String,
        core_num: i32,
        sender: mpsc::Sender<i32>,
    ) -> Result<Task, crate::error::Error> {
        debug!("{}/task.yaml", base_dir);
        let data = std::fs::read(format!("{}/task.yaml", base_dir))?;
        let info: TaskInfo = serde_yaml::from_slice(data.as_slice())?;
        let args = get_arg_scrpit(&info,&base_dir);
        let tmpl = mustache::compile_str(&info.command).unwrap();
        let result_tmpl = mustache::compile_str(&info.output.file).unwrap();
        let runner = TaskRunner {
            info,
            args:args,
            tmpl,
            result_tmpl,
            base_dir:base_dir.to_string(),
        };
        return Ok(Task {
            id,
            name,
            base_dir,
            core_num,
            sender,
            runner: Arc::new(runner),
        });
    }

    pub fn run(&self) {
        for _ in 0..self.core_num {
            debug!("run {} core:{}",self.name,self.core_num);
            let sennder = self.sender.clone();
            let runner = self.runner.clone();
            let id=self.id;
            thread::spawn(move || {
                    runner.run();
                    sennder.send(id);
            });
        }
    }
}
