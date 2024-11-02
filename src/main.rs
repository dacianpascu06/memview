use std::time::Duration;

use clap::Parser;
use info::*;

use colored::*;

use pid::get_pid;

mod error;
mod info;
mod pid;

#[derive(Parser)]
struct Cli {
    process_name: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let mut past_info = String::new();
    let (info_proc, p) = get_pid(&args);
    let pid;
    match p {
        Ok(x) => pid = x,
        Err(e) => {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }

    loop {
        let output = get_info_map(&pid);
        let info;
        match output {
            Ok(out) => {
                info = out;
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        if info.is_empty() {
            break;
        }
        if past_info.is_empty() == true {
            println!("{}", info_proc);
            println!("{}", info);
        } else {
            if past_info.ne(&info) {
                println!("{}", info_proc);
                for diff in diff::lines(&past_info, &info) {
                    match diff {
                        diff::Result::Left(l) => println!("{}{}", "-".red(), l.red()),
                        diff::Result::Both(l, _) => println!(" {}", l.bright_white()),
                        diff::Result::Right(r) => println!("{}{}", "+".green(), r.green()),
                    }
                }
            }
        }
        past_info = info;
        std::thread::sleep(Duration::from_secs(5));
    }
    Ok(())
}
