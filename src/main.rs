extern crate walkdir;

use std::process;
use std::env;
use std::path::PathBuf;

use walkdir::WalkDir;


fn main() {
    let args: Vec<String> = env::args().collect();

    let bin = &args[1];
    println!("path to cppcheck binary: {}", bin);
    let binpath = PathBuf::from(bin);
    if !binpath.is_file() {
        panic!("ERROR: not a file: {}", binpath.display())
    }

    println!("Gathering files...");
    for entry in WalkDir::new(".") {
            let entry = entry.unwrap();
            let path = entry.path();
            let string = format!("{}", path.display());
            match path.extension() {
                Some(ext) =>  {
                    if ext == ".cpp" || ext ==  ".cxx" || ext ==  ".c" {
                            println!("Checking: {}", path.display());

                    }
                }
                None => continue,
            }

    } // walkdir

} // main
