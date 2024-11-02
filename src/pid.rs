use crate::error::FileErr;
use crate::Cli;

use std::ffi::OsStr;
use std::fs::File;

use colored::*;

use sysinfo::System;

pub fn get_proc_maps_file(args: &Cli) -> (String, Result<File, FileErr>) {
    let s = System::new_all();
    let mut pid = None;
    let mut ret_string = String::new();

    for process in s.processes_by_name(OsStr::new(args.process_name.as_str())) {
        //ret_string = format!("Process name: {:?} {:?}\n", process.name(), process.pid());
        ret_string = format!(
            "{:<18} {:>6?} {:>4?}\n",
            "Process name".to_string().bright_white(),
            process.name(),
            process.pid(),
        );

        pid = Some(process.pid());
    }

    let path = match pid {
        Some(pid) => format!("/proc/{}/maps", pid),
        None => return ("".to_owned(), Err(FileErr::ProcessErr)),
    };
    (
        ret_string,
        File::open(path).map_err(|_| FileErr::ProcessErr),
    )
}
