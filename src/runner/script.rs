use std::os::raw::c_char;
use crate::config;
use crate::error::{Error, Result};
use crate::lua::*;
use std::collections::BTreeMap;
use std::ffi::*;
use std::os::raw::c_int;
use std::sync::Mutex;

pub struct ArgScrpitResult {
    pub result: String,
    pub short_display: String,
    pub end: bool,
}
pub struct ArgScrpit {
    state: Mutex<*mut lua_State>,
    file: String,
    pub name: String,
    value: String,
}

impl Drop for ArgScrpit {
    fn drop(&mut self) {
        unsafe {
            println!("ok");
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
            debug!("luaL_openExtendlibs");
            luaL_openlibs(state);
            luaL_openExtendlibs(state);
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
            if load_r != 0 {
                let c_value = lua_tolstring(state, 1, std::ptr::null_mut());
                let result = CStr::from_ptr(c_value).to_str()?;
                return Err(Error::Lua(result.to_string()));
            }
            let res = lua_pcallk(state, 0, -1, 0, 0, std::ptr::null_mut());
            if res != 0 {
                let c_value = lua_tolstring(state, 1, std::ptr::null_mut());
                let result = CStr::from_ptr(c_value).to_str()?;
                return Err(Error::Lua(result.to_string()));
            }
            debug!("load file {}  -> {} ", load_r, res);
            Ok(ArgScrpit {
                name,
                state: Mutex::new(state),
                file: path,
                value,
            })
        }
    }

    pub fn load(&self, options: &Option<BTreeMap<String, String>>) -> Result<()> {
        let name = CString::new("load").unwrap();
        debug!("Load");
        unsafe {
            let state = self.state.lock().unwrap();
            lua_settop(*state, 0);
            let r = lua_getglobal(*state, name.as_ptr());
            if r != LUA_TFUNCTION {
                return Err(Error::Lua("load is not function".to_string()));
            }
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
            let ret = lua_pcallk(*state, 2, 0, 0, 0, std::ptr::null_mut());
            if ret != 0 {
                let c_value = lua_tolstring(*state, 1, std::ptr::null_mut());
                let result = CStr::from_ptr(c_value).to_str()?;
                return Err(Error::Lua(result.to_string()));
            }
            Ok(())
        }
    }

    pub fn next(&self) -> Result<ArgScrpitResult> {
        let name = CString::new("next").unwrap();
        unsafe {
            let state = self.state.lock().unwrap();
            lua_settop(*state, 0);
            lua_getglobal(*state, name.as_ptr());
            let ret = lua_pcallk(*state, 0, 3, 0, 0, std::ptr::null_mut());
            let c_value = lua_tolstring(*state, 1, std::ptr::null_mut());
            let result = CStr::from_ptr(c_value).to_str()?;
            if ret != 0 {
                return Err(Error::Lua(result.to_string()));
            }
            let c_short = lua_tolstring(*state, 2, std::ptr::null_mut());
            let short_display = CStr::from_ptr(c_short).to_str()?;
            let end = lua_toboolean(*state, 3) != 0;
            return Ok(ArgScrpitResult {
                result: result.to_string(),
                short_display: short_display.to_string(),
                end,
            });
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
