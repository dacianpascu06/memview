use super::*;
use aux::parse_info_proc;

use std::sync::{Arc, Mutex};

use crate::ui::background::draw_background;
use crate::ui::event::event_handler;

pub fn run(info_proc: String, pid: sysinfo::Pid) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let info_process_clone: &'static str = Box::leak(info_proc.clone().into_boxed_str());
    let info_process = parse_info_proc(&info_process_clone);

    let err = error::InfoErr::None;
    let index: usize = 0;

    let info = Arc::new(Mutex::new(InfoAll::new(pid.as_u32())));

    let info_clone = Arc::clone(&info);

    // spawns thread that refreshes the state
    std::thread::spawn(move || {
        refresh(info_clone, pid.clone());
    });

    // makes sure that one refresh has already occured and the first pmap result is succesful
    let mut init = false;
    while init == false {
        let info_all_clone = Arc::clone(&info);
        let info_all_guard = info_all_clone.lock();
        let info_all;

        match info_all_guard {
            Ok(value) => info_all = value,
            Err(_) => continue,
        }

        if info_all.get_count() > 0 {
            init = true;
            terminal.draw(|frame| {
                draw_background(frame, &info_process, info_all.get_count() - 1, index);
                info_all.draw_info_map(frame, index);
            })?;
        }
    }

    let info_clone_2 = Arc::clone(&info);

    let handle1 = std::thread::spawn(move || -> std::io::Result<()> {
        event_handler(info_clone_2, index, terminal, info_process)
    });

    match handle1.join() {
        Ok(value) => match value {
            Ok(()) => {}
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{:?}", e),
    }

    ratatui::restore();

    match err {
        error::InfoErr::None => {}
        _ => eprintln!("{}", err),
    }

    Ok(())
}
