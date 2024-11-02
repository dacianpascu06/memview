#[allow(unused_imports)]
use clap::{Parser, Subcommand};

use info::*;

use app::run;
use pid::*;

mod app;
mod error;
mod info;
mod pid;

#[derive(Parser)]
struct Cli {
    process_pid: Option<sysinfo::Pid>,
    #[clap(short = 'n', long = "name")]
    process_name: Option<String>,
}

fn main() -> std::io::Result<()> {
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
                eprintln!("No arguments were given");
                eprintln!("Usage: ");
                eprintln!("bpmap -n (process_name)");
                eprintln!("bpmap (process_pid)");
                std::process::exit(1);
            }
            Some(p) => {
                pid = p;
                info_proc = get_name(&pid);
            }
        },
    }

    run(info_proc, pid);
    Ok(())
}
