use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    if std::path::Path::new("main.cpp").exists() {
        let compile_status = Command::new("g++")
            .arg("-std=c++11")
            .arg("-o")
            .arg("main")
            .arg("main.cpp")
            .status();

        if let Ok(status) = compile_status {
            if status.success() {
                println!();
                execute_main_executable();
                println!();
            } else {
                println!("COMPILATION FAILED.");
            }
        } else {
            println!("ERROR WHILE COMPILING MAIN.CPP");
        }
    } else {
        println!("ERROR: *MAIN.CPP* FILE NOT FOUND.");
    }
    Ok(())
}

fn execute_main_executable() {
    if std::path::Path::new("main").exists() {
        let execute_status = Command::new("./main").status();

        if let Ok(status) = execute_status {
            if !status.success() {
                println!("ERROR: FAILED TO EXECUTE THE *MAIN* EXECUTABLE.");
            }
        } else {
            println!("ERROR WHILE EXECUTING *MAIN* EXECUTABLE");
        }
    } else {
        println!("ERROR: *MAIN* EXECUTABLE NOT FOUND.");
    }
}
