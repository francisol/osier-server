use glob::glob;
use std::path::Path;
fn main() {
    let mut build = cc::Build::new();
    let files = glob("lua/*.c").unwrap();
    let include_dir = Path::new("lua");
    for item in files {
        let path = format!("{}", &(item.unwrap().as_path().display()));
        println!("{}", &path);
        build.file(path);
    }
    build
        .file("lfs/lfs.c")
        .file("path/path.c")
        .file("lua_extend.c")
        .opt_level(3)
        .include(include_dir)
        .compile("lua");
}
