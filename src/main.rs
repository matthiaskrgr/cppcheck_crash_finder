extern crate walkdir;

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


    let mut files_to_check = Vec::new();

    for entry in WalkDir::new(dir_to_check) {
        let entry = entry.unwrap();
        let filename_path = entry.path().to_path_buf();
        // don't check directory if directory is named "bla.c/"
        if !filename_path.is_file() {
            continue;
        }
        match filename_path.extension() {
            Some(ext) => {
                if ext == "c" || ext == "cl" || ext == "cpp" || ext == "cxx" || ext == "cc"
                    || ext == "c++" || ext == "tpp" || ext == "txx" || ext == "C"
                {
                    files_to_check.push(filename_path.clone());
                }
            }
            None => continue,
        }
    } // walkdir


    let mut evil_files = Vec::new();

    println!("Files gathered");
    for file in files_to_check {
        let filename_str = format!("{}", file.display());
        println!("Checking: {}", &filename_str);
        let output = std::process::Command::new(bin)
        .arg(&filename_str)
        .arg("--enable=all")
        .arg("--inconclusive")
        .arg("--max-configs=1")
//            .arg("--debug")
//            .arg("--verbose")
        .output()
        .expect("failed to run cppcheck!");
        if !output.status.success() {
            println!("Crash:        {}", &filename_str);
            evil_files.push(filename_str.clone());
        }
    }

    println!("\nCrashing files:");
    for file in evil_files {
        println!("{}", file);
    }
} // main
