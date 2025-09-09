use crate::Cli;

use clap::Parser;

use crate::pid::*;

use colored::*;

pub fn parser() -> (String, sysinfo::Pid) {
    let args = Cli::parse();
    let pid;
    let (info_proc, p);

    match args.process_name {
        Some(name) => {
            (info_proc, p) = get_pid(name);
            match p {
                Ok(x) => pid = x,
                Err(e) => {
                    eprint!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        None => match args.process_pid {
            None => {
                eprintln!("{}", "No arguments were given!".to_string().red());
                eprintln!("Usage: ");
                eprintln!("memview -n (process_name)");
                eprintln!("memview (process_pid)");
                std::process::exit(1);
            }
            Some(p) => {
                pid = p;
                info_proc = get_name(&pid);
            }
        },
    }
    (info_proc, pid)
}
