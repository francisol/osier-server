use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Arg{
    pub name: String,
    pub r#type: String,
    pub value: String,
    pub options:Option<BTreeMap<String,String>>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output{
    pub dir:String,
    pub file:String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskInfo{
   pub name: String,
   pub command:String,
   pub priority:i32,
   pub core_num:i32,
   pub script_lib:String,
   pub args:Vec<Arg>,
   pub output:Output,
}