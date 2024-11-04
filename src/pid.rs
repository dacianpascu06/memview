use crate::error::FileErr;

use std::ffi::OsStr;
use std::fs::File;

use sysinfo::System;

pub fn get_pid(p_name: String) -> (String, Result<sysinfo::Pid, FileErr>) {
    let s = System::new_all();
    let mut pid = None;
    let mut ret_string = String::new();

    for process in s.processes_by_name(OsStr::new(p_name.as_str())) {
        let process_name: &str;
        match process.name().to_str() {
            Some(name) => process_name = name,
            None => return ("".to_string(), Err(FileErr::ProcessErr)),
        }
        let process_pid = process.pid().to_string();

        ret_string = format!(
            "{} {} {} {}",
            "Process name ".to_string(),
            process_name.to_string(),
            " pid ".to_string(),
            process_pid,
        );
        pid = Some(process.pid());
    }

    match pid {
        Some(pid) => (ret_string, Ok(pid)),
        None => ("".to_owned(), Err(FileErr::ProcessErr)),
    }
}

pub fn get_proc_maps_file(pid: &sysinfo::Pid) -> Result<File, FileErr> {
    let path = format!("/proc/{}/maps", pid);
    File::open(path).map_err(|_| FileErr::ProcessErr)
}

pub fn get_name(pid: &sysinfo::Pid) -> String {
    let s = System::new_all();
    if let Some(process) = s.process(*pid) {
        let process_name: &str;
        match process.name().to_str() {
            Some(name) => process_name = name,
            None => {
                eprintln!("Process doesn't exist!");
                std::process::exit(1);
            }
        }
        format!(
            "{} {} {} {}",
            "Process name ".to_string(),
            process_name.to_string(),
            " pid ".to_string(),
            process.pid().to_string(),
        )
    } else {
        eprintln!("Process doesn't exist!");
        std::process::exit(1);
    }
}
