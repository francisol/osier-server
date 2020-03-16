use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::raw::*;
pub type lua_State = c_void;
pub type lua_Number = f64;
pub type size_t = c_ulong;

pub type lua_KFunction = c_void;
pub type lua_KContext = isize;

extern "C" {
    pub fn luaL_openlibs(L: *mut lua_State);
    pub fn lua_close(L: *mut lua_State);
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn lua_version(L: *mut lua_State) -> *const f64;
    fn lua_getfield(L: *mut lua_State, idx: c_int, k: *const c_char) -> c_int;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;
    pub fn lua_getglobal(
        L: *mut lua_State,
        name: *const c_char,
    ) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushstring(
        L: *mut lua_State,
        s: *const c_char,
    ) -> *const c_char;
    pub fn lua_setfield(
        L: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    );
    pub fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const c_char,
        mode: *const c_char,
    ) -> c_int;

    pub fn lua_pcallk(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
        ctx: lua_KContext,
        k: *mut lua_KFunction,
    ) -> c_int;

    pub fn lua_createtable(
        L: *mut lua_State,
        narr: c_int,
        nrec: c_int,
    );
    pub fn lua_settable(L: *mut lua_State, idx: c_int);
}

pub fn getString(L: *mut lua_State, name: &String) -> String {
    let d = CString::new(name.to_string()).unwrap();
    let packge = CString::new("package").unwrap();
    unsafe {
        lua_getglobal(L, packge.as_ptr());
        lua_getfield(L, -1, d.as_ptr());
        let c = lua_tolstring(L, -1, std::ptr::null_mut());
        return CStr::from_ptr(c).to_str().unwrap_or_default().to_string();
    }
}

pub fn push_path(L: *mut lua_State, dir: &String) {
    let packge = CString::new("package").unwrap();
    let path = CString::new("path").unwrap();
    let cpath = CString::new("cpath").unwrap();
    unsafe{
        lua_getglobal(L, packge.as_ptr());
    }
    unsafe {
        lua_getfield(L, -1, path.as_ptr());
        
        let p = lua_tolstring(L, -1, std::ptr::null_mut());
        let mut _path: String = CStr::from_ptr(p).to_str().unwrap_or_default().to_string();
        _path.push(';');
        _path.push_str(dir);
        _path.push_str("/?.lua;");
        _path.push_str(dir);
        _path.push_str("/?/init.lua");
        lua_settop(L, -2);
        let c_new_path=CString::new(_path).unwrap();
        // println!("{:?}",c_new_path);
        lua_pushstring(L, c_new_path.as_ptr());
        lua_setfield(L, -2, path.as_ptr());
        lua_settop(L, 1);
    }
    unsafe{
        lua_getfield(L, -1, cpath.as_ptr());
        let c = lua_tolstring(L, -1, std::ptr::null_mut());
        let mut _path: String = CStr::from_ptr(c).to_str().unwrap_or_default().to_string();
        _path.push(';');
        _path.push_str(dir);
        _path.push_str("/?.so");
        lua_settop(L, -2);
        let c_new_path=CString::new(_path).unwrap();
        // println!("{:?}",c_new_path);
        lua_pushstring(L, c_new_path.as_ptr());
        lua_setfield(L, -2, cpath.as_ptr());
        lua_settop(L, 0);
    }

}
