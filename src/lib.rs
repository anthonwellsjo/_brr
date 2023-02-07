use std::{process::Command, fs};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

pub fn get_argument_at(index: usize) -> Option<String> {
    match get_args().get(index) {
        Some(arg) => Some(arg.to_owned()),
        None => None,
    }
}

pub fn get_user() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("failed to execute process");

    let mut user = std::str::from_utf8(&output.stdout).unwrap().to_owned();
    //Removes /n from string
    user.pop().unwrap().to_string();
    user
}

#[derive(Debug)]
pub struct ProcessCount {
    pub pid: Vec<u32>,
    pub name: String,
}

impl ProcessCount {
    pub fn new(name: &str) -> Self {
        ProcessCount {
            name: name.to_string(),
            pid: Vec::<u32>::new(),
        }
    }
}

pub fn get_processes(name: &str, excl_current: bool) -> ProcessCount {
    let s = System::new_all();
    let mut p_count = ProcessCount::new(name);
    for process in s.processes_by_exact_name(&name) {
        if excl_current && std::process::id() == process.pid().as_u32() {
            continue;
        }
        p_count.pid.push(process.pid().as_u32());
    }

    p_count
}

/// Gets db-path depending on environment and os. Creates path if not yet there.
pub fn get_app_path(app_name: &str) -> String {
    if cfg!(test) {
        String::from("./test-db.sql")
    } else {
        match dirs::home_dir() {
            Some(dir) => {
                let path = dir.to_str().unwrap().to_owned() + "/Library/Application Support/" + app_name + "/";
                fs::create_dir_all(&path).unwrap();
                path + "db.sql"
            }
            None => panic!("Could not find a home directory"),
        }
    }
}
