use crate::error::FileErr;
use crate::Cli;

use std::ffi::OsStr;
use std::fs::File;

use colored::*;

use sysinfo::System;

pub fn get_pid(args: &Cli) -> (String, Result<sysinfo::Pid, FileErr>) {
    let s = System::new_all();
    let mut pid = None;
    let mut ret_string = String::new();

    for process in s.processes_by_name(OsStr::new(args.process_name.as_str())) {
        let process_name: &str;
        match process.name().to_str() {
            Some(name) => process_name = name,
            None => return ("".to_string(), Err(FileErr::ProcessErr)),
        }
        let process_pid = process.pid().to_string();

        ret_string = format!(
            "{:<18} {:>6} {:<4}  {:>6}",
            "Process name".bright_white(),
            process_name.purple(),
            "pid".bright_white(),
            process_pid.purple(),
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
