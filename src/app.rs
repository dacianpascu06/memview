use super::*;
use std::time::Duration;

pub fn run(info_process: String, pid: sysinfo::Pid) {
    let mut info_all = InfoAll::new(info_process);
    loop {
        let output = get_info_map(&mut info_all, &pid);
        match output {
            Err(e) => {
                println!("{}", e);
                std::process::exit(0);
            }
            Ok(state) => match state {
                State::Initial => {
                    println!("{}", info_all);
                }
                State::NotChanged => {}
                State::Changed => {
                    info_all.print_with_dif();
                }
            },
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}
