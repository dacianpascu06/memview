use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::InfoErr;
use crate::InfoAll;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, text::Line, Terminal};

use crate::ui::background::draw_background;

pub fn event_handler<B>(
    info: Arc<Mutex<InfoAll>>,
    mut index: usize,
    mut terminal: Terminal<CrosstermBackend<B>>,
    info_process: Line<'_>,
    receiver: Receiver<InfoErr>,
) -> Result<(), InfoErr>
where
    B: std::io::Write,
{
    let mut diff = false;
    loop {
        if let Ok(e) = receiver.try_recv() {
            break Err(e);
        }
        if event::poll(Duration::from_millis(16)).map_err(|_| InfoErr::EventErr)? {
            if let Event::Key(key) = event::read().map_err(|_| InfoErr::EventErr)? {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
                let info_all_clone = Arc::clone(&info);
                let info_all_guard = info_all_clone.lock();
                let info_all;
                match info_all_guard {
                    Ok(value) => info_all = value,
                    Err(_) => continue,
                }
                if key.code == KeyCode::Char('n') {
                    if index == info_all.get_count() - 1 || index == info_all.get_count() - 2 {
                        index = info_all.get_count() - 1;
                    } else {
                        index = index + 1;
                    }
                    if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == true {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map_with_diff(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }
                if key.code == KeyCode::Char('p') {
                    if index == 0 || index == 1 {
                        terminal
                            .draw(|frame| {
                                index = 0;
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else {
                        index -= 1;
                        if diff == false {
                            terminal
                                .draw(|frame| {
                                    draw_background(
                                        frame,
                                        &info_process,
                                        info_all.get_count() - 1,
                                        index,
                                    );
                                    info_all.draw_info_map(frame, index);
                                })
                                .map_err(|_| InfoErr::DrawErr)?;
                        } else {
                            terminal
                                .draw(|frame| {
                                    draw_background(
                                        frame,
                                        &info_process,
                                        info_all.get_count() - 1,
                                        index,
                                    );
                                    info_all.draw_info_map_with_diff(frame, index);
                                })
                                .map_err(|_| InfoErr::DrawErr)?;
                        }
                    }
                }
                if key.code == KeyCode::Char('r') {
                    if index == 0 {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map_with_diff(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }

                if key.code == KeyCode::Char('d') {
                    diff = !diff;
                    if index == 0 {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == true {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map_with_diff(frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }
                if key.code == KeyCode::Char('s') {
                    index = 0;
                    terminal
                        .draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })
                        .map_err(|_| InfoErr::DrawErr)?;
                }
                if key.code == KeyCode::Char('e') {
                    index = info_all.get_count() - 1;
                    terminal
                        .draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })
                        .map_err(|_| InfoErr::DrawErr)?;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
