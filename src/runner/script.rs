use crate::config;
use crate::error::{Error, Result};
use crate::lua::*;
use std::collections::BTreeMap;
use std::ffi::*;
use std::os::raw::c_int;
pub struct ArgScrpit {
    state: *mut lua_State,
    file: String,
    pub name: String,
    value: String,
}

impl Drop for ArgScrpit {
    fn drop(&mut self) {
        unsafe {
            lua_close(self.state);
        }
    }
}

impl ArgScrpit {
    pub fn new(
        file: String,
        name: String,
        value: String
    ) -> Result<ArgScrpit> {
        let config = config::get_config();
        let state = unsafe { luaL_newstate() };
        unsafe {
            luaL_openlibs(state);
            push_path(state, &config.lua_package);
            let mut base = std::path::PathBuf::from(config.lua_lib.to_string());
            base.push(format!("{}.lua", file));
            let path: String = format!("{}", base.display());
            if !base.exists() {
                return Result::Err(Error::Lua(format!("{} not exist", path.to_string())));
            }
            let c_path = CString::new(path.to_string()).unwrap();
            luaL_loadfilex(state, c_path.as_ptr(), std::ptr::null_mut());
            lua_pcallk(state, 0, -1, 0, 0, std::ptr::null_mut());
            Ok(ArgScrpit {
                name,
                state,
                file: path,
                value,
            })
        }
    }

    pub fn load(&self,options: &Option<BTreeMap<String, String>>) {
        let name = CString::new("load").unwrap();
        unsafe {
            lua_settop(self.state, 0);
            let r = lua_getglobal(self.state, name.as_ptr());
            let value = CString::new(self.value.to_string()).unwrap();
            lua_pushstring(self.state, value.as_ptr());
            match options {
                None => lua_createtable(self.state, 0, 0),
                Some(t) => {
                    lua_createtable(self.state, 0, t.len() as c_int);
                    for (k, v) in t {
                        let key = CString::new(k.to_string()).unwrap();
                        let value = CString::new(v.to_string()).unwrap();
                        lua_pushstring(self.state, key.as_ptr());
                        lua_pushstring(self.state, value.as_ptr());
                        lua_settable(self.state, -3);
                    }
                }
            }
            let ddd = lua_pcallk(self.state, 2, 0, 0, 0, std::ptr::null_mut());
        }
    }

    pub fn next(&self)->(String,bool){
        let name = CString::new("next").unwrap();
        unsafe {
            lua_settop(self.state, 0);
            lua_getglobal(self.state, name.as_ptr());
            lua_pcallk(self.state, 0, 2, 0, 0, std::ptr::null_mut());
            let c_value = lua_tolstring(self.state, -1, std::ptr::null_mut());
            let value= CStr::from_ptr(c_value).to_str().unwrap();
            let c_end= lua_toboolean(self.state, -2) !=0;
            return(String::from(value),c_end);
        }
    }
    pub fn reset(&self){
        let name = CString::new("reset").unwrap();
        unsafe {
            lua_settop(self.state, 0);
            lua_getglobal(self.state, name.as_ptr());
            lua_pcallk(self.state, 0, 0, 0, 0, std::ptr::null_mut());
        }
    }
}