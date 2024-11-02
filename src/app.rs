use super::*;
use colored::*;
use std::time::Duration;

pub fn run(info_process: String, pid: sysinfo::Pid) {
    let mut past_info = String::new();
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
            println!("{}", info_process);
            println!("{}", info);
        } else {
            if past_info.ne(&info) {
                println!("{}", info_process);
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
}
