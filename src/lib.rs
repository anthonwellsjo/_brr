use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt, PidExt};

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

pub struct ProcessCount{
    pid: Vec<u32>,
    name: String
}

impl ProcessCount {
    pub fn new(name: &str) -> Self {ProcessCount{name: name.to_string(), pid: Vec::<u32>::new()}}
}

pub fn get_processes(name: &str) -> ProcessCount{
    let s = System::new_all();
    let mut p_count = ProcessCount::new(name);
    for process in s.processes_by_exact_name(&name) {
        p_count.pid.push(process.pid().as_u32());
        println!("{} {}", process.pid(), process.name());
    }

    p_count
}
