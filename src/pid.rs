use crate::error::FileErr;
use crate::Cli;

use std::ffi::OsStr;
use std::fs::File;

use sysinfo::System;

pub fn get_proc_maps_file(args: &Cli) -> Result<File, FileErr> {
    let s = System::new_all();
    let mut pid = None;

    for process in s.processes_by_name(OsStr::new(args.process_name.as_str())) {
        println!("{:?} {:?} ", process.pid(), process.name());
        pid = Some(process.pid());
    }
    let path = match pid {
        Some(pid) => format!("/proc/{}/maps", pid),
        None => return Err(FileErr::ProcessErr),
    };
    File::open(path).map_err(|_| FileErr::ProcessErr)
}
