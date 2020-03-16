use serde::{Deserialize, Serialize};


pub trait Handler:std::marker::Sync+std::marker::Send {
    fn handle(&self,name: &str, data: &[u8])->Result<String,String>;
}

pub trait CMDHandler {
    fn handle(&self)->Result<String,String>;
}


#[derive(Serialize, Deserialize,Debug)]
struct CreateCMD{
    pub name: String,
    pub priority:i32,
    pub base_dir:String,
    pub status:i16,
    pub core_num:i16,
}
impl CMDHandler for CreateCMD{
    fn handle(&self)->Result<String,String>{
        
        return Result::Ok(format!("{:?}",self));
    }
}
pub struct JSONHandler{

}
impl JSONHandler{
    fn _handle<T:CMDHandler>(&self,handler: &serde_json::Result<T>)->Result<String,String>{
     return   match handler{
            Ok(h)=>return h.handle(),
            Err(e)=>Err(format!("{}",e)),
        }
    }
}
impl Handler for JSONHandler{
    fn handle(&self,name: &str, data: &[u8])->Result<String,String>{
    return  match name{
            "create" =>{let cmd:serde_json::Result<CreateCMD> = serde_json::from_slice(data);return self._handle(&cmd); }
            _=> Err("no match".to_string())
        }
        
    }
}
