use super::*;
use aux::parse_info_proc;
use clap::error::Result;
use error::InfoErr;

use std::sync::{Arc, Mutex};

use crate::ui::background::draw_background;
use crate::ui::draw::*;
use crate::ui::event::event_handler;

pub fn run(info_proc: String, pid: sysinfo::Pid) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let info_process_static: &'static str = Box::leak(info_proc.clone().into_boxed_str());
    let info_process = parse_info_proc(&info_process_static);

    let mut err = error::InfoErr::None;
    let index: usize = 0;

    let info = Arc::new(Mutex::new(Info::new(pid.as_u32())));

    let info_clone = Arc::clone(&info);

    let (sender_refr, receiver_event) = std::sync::mpsc::channel();

    // spawns thread that refreshes the state
    std::thread::spawn(move || {
        refresh(info_clone, pid.clone(), sender_refr);
    });

    // makes sure that one refresh has already occured and the first pmap result is succesful
    let mut init = false;
    while init == false {
        match (*info).lock() {
            Ok(info_state) => {
                if info_state.get_count() > 0 {
                    init = true;
                    terminal.draw(|frame| {
                        draw_background(frame, &info_process, info_state.get_count() - 1, index);
                        draw_info_map(&info_state, frame, index);
                    })?;
                }
            }
            Err(_) => continue,
        }
    }

    let info_clone = Arc::clone(&info);
    let handle1 = std::thread::spawn(move || -> Result<(), InfoErr> {
        event_handler(info_clone, index, terminal, info_process, receiver_event)
    });

    match handle1.join() {
        Ok(value) => match value {
            Ok(()) => {}
            Err(e) => err = e,
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
