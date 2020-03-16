use glob::glob;
use std::path::Path;
fn main() {
    let mut build=cc::Build::new();
    let files= glob("lua/*.c").unwrap();
    let include_dir=Path::new("lua-5.3.5/src");
    for item  in files {
        let path=format!("{}",&(item.unwrap().as_path().display()));
        println!("{}",&path);
        build.file(path);
    }
    
        build
        .opt_level(3)
        .include(include_dir)
        .compile("lua");
}