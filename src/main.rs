extern crate walkdir;

use std::process;
use std::env;
use std::path::PathBuf;

use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    let bin = &args[1];
    let dir_to_check = &args[2];
    println!("path to cppcheck binary: {}", bin);
    let binpath = PathBuf::from(bin);
    if !binpath.is_file() {
        panic!("ERROR: not a file: {}", binpath.display())
    }

    println!("Gathering files...");
    for entry in WalkDir::new(dir_to_check) {
        let entry = entry.unwrap();
        let path = entry.path();
        let string = format!("{}", path.display());
        match path.extension() {
            Some(ext) => {
                if ext == "cpp" || ext == "cxx" || ext == "c" || ext == "C" {
                    println!("Checking: {}", path.display());
                }
            }
            None => continue,
        }
    } // walkdir
} // main
