extern crate rayon;
extern crate walkdir;

use std::env;
use std::path::PathBuf;

use walkdir::WalkDir;
use rayon::prelude::*;

fn check_file(path: &PathBuf, binary: String) -> Result<String, String> {
    let filename_str = format!("{}", path.display());
    println!("Checking: {}", &filename_str);
    let output = std::process::Command::new(binary)
    .arg(&filename_str)
    .arg("--enable=all")
    .arg("--inconclusive")
    .arg("--max-configs=1")
    //            .arg("--debug")
    //            .arg("--verbose")
    .output()
    .expect("failed to run cppcheck!");
    if output.status.success() {
        Ok(filename_str)
    } else {
        // we crashed
        println!("Crash:        {}", &filename_str);
        Err(filename_str)
    }
}

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
    let number_of_files = files_to_check.len();
    println!("{} files gathered", number_of_files);


    let evil_files: Vec<Result<String, String>> = files_to_check
        .par_iter()
        .map(|x| check_file(x, bin.to_string()))
        .collect();

    println!("\nCrashing files:");
    for result in evil_files {
        match result {
            Ok(_) => {}
            Err(file) => {
                println!("{}", file);
            }
        }
    }
} // main
