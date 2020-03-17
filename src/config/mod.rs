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
    if cfg!(lunix) {
        def_base_dir = String::from_str("/var/lab_tool").unwrap();
    }
    let matches = App::new("osier-server")
        .version("dev-1")
        .about("tiny tool developped by rust depended on lua and SQLite")
        .author("Xie Zhongtao")
        .arg(
            Arg::with_name("core_num")
                .default_value("4")
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
    let mut def_base_dir_path = std::path::PathBuf::from_str(&def_base_dir).unwrap();

    let _ = std::fs::create_dir_all(&def_base_dir_path);
    def_base_dir_path.push("lua_lib");
    let mut def_lua_package_path = std::path::PathBuf::from_str(&def_base_dir).unwrap();
    def_lua_package_path.push("lua_package");
    let def_lua_lib = format!("{}", def_base_dir_path.display());
    let def_lua_package = format!("{}", def_lua_package_path.display());
    let lua_lib = matches
        .value_of("lua_lib")
        .unwrap_or(&def_lua_lib)
        .to_string();
    let lua_package = matches
        .value_of("lua_package")
        .unwrap_or(&def_lua_package)
        .to_string();
    let _ = std::fs::create_dir_all(&lua_lib);
    let _ = std::fs::create_dir_all(&lua_package);
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
