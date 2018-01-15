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

    let mut evil_files = Vec::new();

    for entry in WalkDir::new(dir_to_check) {
        let entry = entry.unwrap();
        let filename_path = entry.path();
        let filename_str = format!("{}", filename_path.display());
        match filename_path.extension() {
            Some(ext) => {
                if ext == "c" || ext == "cl" || ext == "cpp" || ext == "cxx" || ext == "cc"
                    || ext == "c++" || ext == "tpp" || ext == "txx" || ext == "C"
                {
                    println!("Checking: {}", &filename_str);
                    let output = std::process::Command::new(bin)
                        .arg(&filename_str)
                        .arg("--enable=all")
                        .arg("--inconclusive")
                        .arg("--max-configs=1")
                        .arg("--debug")
                        .arg("--verbose")
                        .output()
                        .expect("failed to run cppcheck!");
                    if !output.status.success() {
                        println!("Crash: {}", &filename_str);
                        evil_files.push(filename_str.clone());
                    }
                }
            }
            None => continue,
        }
    } // walkdir

    println!("\nCrashing files:");
    for file in evil_files {
        println!("{}", file);
    }
} // main
