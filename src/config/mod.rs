use clap::App;
use clap::Arg;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
pub struct AppConfig {
    pub core_num: i32,
    pub base_dir: String,
    pub lua_lib: String,
    pub lua_package: String,
    pub port :i16
}

unsafe impl std::marker::Sync for AppConfig {}
unsafe impl std::marker::Send for AppConfig {}
lazy_static! {
    static ref APP_CONFG: Arc<AppConfig> = init_config();
}

fn init_config() -> Arc<AppConfig> {
    let mut def_base_dir: String = format!("{}", env::current_dir().unwrap().display());
    if cfg!(unix) {
        def_base_dir = String::from_str("/var/osier").unwrap();
    }
    let matches = App::new("osier-server")
        .version("dev-1")
        .about("tiny tool developped by rust depended on lua and SQLite")
        .author("Xie Zhongtao")
        .arg(
            Arg::with_name("core_num")
                .default_value("128")
                .long("core_num")
                .help("the max thread size of task"),
        )
        .arg(
            Arg::with_name("base_dir")
                .default_value(&def_base_dir)
                .help("work space")
                .long("base_dir"),
        )
        .arg(
            Arg::with_name("lua_lib")
                .long("lua_lib")
                .help("lua scirpt dir default <base_dir>/lua_lib")
                .empty_values(false),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .help("server open port for cli")
                .default_value("1115"),
        )
        .arg(
            Arg::with_name("lua_package")
                .long("lua_package")
                .help("extra lua  search path default <base_dir>/lua_package")
                .empty_values(false),
        )
        .get_matches();
    let core_num: i32 = matches.value_of("core_num").unwrap().parse().unwrap_or(4);
    let base_dir = matches
        .value_of("base_dir")
        .unwrap_or(&def_base_dir)
        .to_string();
    let mut base_dir_path = std::env::current_dir().unwrap();
    base_dir_path.push(base_dir);
    let _ = std::fs::create_dir_all(&base_dir_path).unwrap();
    let mut def_lua_lib_path = std::path::PathBuf::from(&base_dir_path);
    def_lua_lib_path.push("lua_lib");
    let mut def_lua_package_path = std::path::PathBuf::from(&base_dir_path);
    def_lua_package_path.push("lua_package");
    let lua_lib_path = matches
        .value_of("lua_lib")
        .map( |p| {
            let mut path= env::current_dir().unwrap();
            path.push(p);
           return path;}
         )
        .unwrap_or(def_lua_lib_path);
    let lua_package_path = matches
        .value_of("lua_package")
        .map(|p| {
            let mut path= env::current_dir().unwrap();
            path.push(p);
           return path;}).unwrap_or(def_lua_package_path);

    let lua_lib=format!("{}",lua_lib_path.display());
    let base_dir=format!("{}",base_dir_path.display());
    let lua_package=format!("{}",lua_package_path.display());
    let _ = std::fs::create_dir_all(&lua_lib_path).unwrap();
    let _ = std::fs::create_dir_all(&lua_package_path).unwrap();
    let port = matches.value_of("port").unwrap().parse().unwrap();
    return Arc::new(AppConfig {
        core_num,
        base_dir,
        lua_lib,
        lua_package,
        port,
    });
}

pub fn get_config() -> Arc<AppConfig> {
    let c: Arc<AppConfig> = APP_CONFG.clone();
    return c;
}
