#[allow(unused_imports)]
use clap::{Parser, Subcommand};

use info::*;

use app::run;

mod app;
mod args;
mod aux;
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
    let (info_proc, pid) = args::parser();
    run(info_proc, pid);
    Ok(())
}
