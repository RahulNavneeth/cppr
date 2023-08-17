use chrono;
use std::io::Result;
use std::process::Command;
use std::time::Instant;

fn main() -> Result<()> {
    let r: i32 = chrono::offset::Local::now().timestamp_millis() as i32;
    if std::path::Path::new("src/main.cpp").exists() {
        if !std::path::Path::new("exe").exists() {
            std::fs::create_dir("exe")?;
        }
        let compile_status = Command::new("g++")
            .arg("-std=c++11")
            .arg("-o")
            .arg(format!("exe/main-{}", r))
            .arg("src/main.cpp")
            .status();

        if let Ok(status) = compile_status {
            if status.success() {
                execute_main_executable(r);
            } else {
                println!("COMPILATION FAILED.");
            }
        } else {
            println!("ERROR WHILE COMPILING MAIN.CPP");
        }
    } else {
        println!("ERROR: *SRC/MAIN.CPP* FILE NOT FOUND.");
    }
    Ok(())
}

fn execute_main_executable(id: i32) {
    if std::path::Path::new(&format!("exe/main-{}", id)).exists() {
        let instance = Instant::now();
        let execute_status = Command::new(&format!("exe/main-{}", id)).status();

        if let Ok(status) = execute_status {
            if !status.success() {
                println!("ERROR: FAILED TO EXECUTE THE *MAIN* EXECUTABLE.");
            } else {
                println!("{:.2?}", instance.elapsed());
            }
        } else {
            println!("ERROR WHILE EXECUTING *MAIN* EXECUTABLE");
        }
    } else {
        println!("ERROR: *MAIN* EXECUTABLE NOT FOUND.");
    }
}
