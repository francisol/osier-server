use crate::config;
use crate::error::{Error, Result};
use crate::lua::*;
use std::collections::BTreeMap;
use std::ffi::*;
use std::os::raw::c_int;
use std::sync::Mutex;
pub struct ArgScrpit {
    state: Mutex<*mut lua_State>,
    file: String,
    pub name: String,
    value: String,
}

impl Drop for ArgScrpit {
    fn drop(&mut self) {
        unsafe {
            lua_close(*self.state.lock().unwrap());
        }
    }
}

unsafe impl std::marker::Send for ArgScrpit {}
unsafe impl std::marker::Sync for ArgScrpit {}
impl ArgScrpit {
    pub fn add_glob_var<P: AsRef<str>>(&self, key: P, value: &String) {
        let l = self.state.lock().unwrap();
        let c_key = CString::new(key.as_ref()).unwrap();
        let c_value = CString::new(value.to_string()).unwrap();
        unsafe {
            lua_pushstring(*l, c_value.as_ptr());
            lua_setglobal(*l, c_key.as_ptr());
        }
    }
    pub fn new(file: String, name: String, value: String) -> Result<ArgScrpit> {
        let config = config::get_config();
        let state = unsafe { luaL_newstate() };
        unsafe {
            luaL_openlibs(state);
            push_path(state, &config.lua_package);
            let mut base = std::path::PathBuf::from(config.lua_lib.to_string());
            base.push(format!("{}.lua", &file));
            let path: String = format!("{}", base.display());
            if !base.exists() {
                return Result::Err(Error::Lua(format!("{} not exist", path.to_string())));
            }
            debug!("Load {}", &path);
            let c_path = CString::new(path.to_string()).unwrap();
            let load_r = luaL_loadfilex(state, c_path.as_ptr(), std::ptr::null_mut());
            let res = lua_pcallk(state, 0, -1, 0, 0, std::ptr::null_mut());
            debug!("load file {}  -> {} ", load_r, res);
            Ok(ArgScrpit {
                name,
                state: Mutex::new(state),
                file: path,
                value,
            })
        }
    }

    pub fn load(&self, options: &Option<BTreeMap<String, String>>) {
        let name = CString::new("load").unwrap();
        debug!("Load");
        unsafe {
            let state = self.state.lock().unwrap();
            lua_settop(*state, 0);
            let r = lua_getglobal(*state, name.as_ptr());
            debug!("load function :{}", r);
            let value = CString::new(self.value.to_string()).unwrap();
            lua_pushstring(*state, value.as_ptr());
            match options {
                None => lua_createtable(*state, 0, 0),
                Some(t) => {
                    lua_createtable(*state, 0, t.len() as c_int);
                    for (k, v) in t {
                        let key = CString::new(k.to_string()).unwrap();
                        let value = CString::new(v.to_string()).unwrap();
                        lua_pushstring(*state, key.as_ptr());
                        lua_pushstring(*state, value.as_ptr());
                        lua_settable(*state, -3);
                    }
                }
            }
            let _ = lua_pcallk(*state, 2, 0, 0, 0, std::ptr::null_mut());
        }
    }

    pub fn next(&self) -> (String, bool) {
        let name = CString::new("next").unwrap();
        unsafe {
            let state = self.state.lock().unwrap();
            lua_settop(*state, 0);
            lua_getglobal(*state, name.as_ptr());
            let ret = lua_pcallk(*state, 0, 2, 0, 0, std::ptr::null_mut());
            let c_value = lua_tolstring(*state, 1, std::ptr::null_mut());
            let value = CStr::from_ptr(c_value).to_str().unwrap();
            let c_end = lua_toboolean(*state, 2) != 0;
            return (String::from(value), c_end);
        }
    }
    pub fn reset(&self) {
        let name = CString::new("reset").unwrap();
        unsafe {
            let state = self.state.lock().unwrap();
            lua_settop(*state, 0);
            lua_getglobal(*state, name.as_ptr());
            lua_pcallk(*state, 0, 0, 0, 0, std::ptr::null_mut());
        }
    }
}
