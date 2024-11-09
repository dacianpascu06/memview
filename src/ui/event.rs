use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::error::InfoErr;
use crate::Info;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, text::Line, Terminal};

use super::background::draw_background;
use super::draw::*;

pub fn event_handler<B>(
    info: Arc<Mutex<Info>>,
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
                // quit
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }

                let info_state;
                match (*info).lock() {
                    Ok(state) => info_state = state,
                    Err(_) => continue,
                }

                // next
                if key.code == KeyCode::Char('n') {
                    if index == info_state.get_count() - 1 || index == info_state.get_count() - 2 {
                        index = info_state.get_count() - 1;
                    } else {
                        index = index + 1;
                    }
                    if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == true && index != 0 {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map_with_diff(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }

                // previous
                if key.code == KeyCode::Char('p') {
                    if index == 0 || index == 1 {
                        terminal
                            .draw(|frame| {
                                index = 0;
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
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
                                        info_state.get_count() - 1,
                                        index,
                                    );
                                    draw_info_map(&info_state, frame, index);
                                })
                                .map_err(|_| InfoErr::DrawErr)?;
                        } else {
                            terminal
                                .draw(|frame| {
                                    draw_background(
                                        frame,
                                        &info_process,
                                        info_state.get_count() - 1,
                                        index,
                                    );
                                    draw_info_map_with_diff(&info_state, frame, index);
                                })
                                .map_err(|_| InfoErr::DrawErr)?;
                        }
                    }
                }

                // refresh
                if key.code == KeyCode::Char('r') {
                    if index == 0 {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map_with_diff(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }

                // Toggle diff
                if key.code == KeyCode::Char('d') {
                    diff = !diff;
                    if index == 0 {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == false {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    } else if diff == true {
                        terminal
                            .draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_state.get_count() - 1,
                                    index,
                                );
                                draw_info_map_with_diff(&info_state, frame, index);
                            })
                            .map_err(|_| InfoErr::DrawErr)?;
                    }
                }

                // go to start
                if key.code == KeyCode::Char('s') {
                    index = 0;
                    terminal
                        .draw(|frame| {
                            draw_background(
                                frame,
                                &info_process,
                                info_state.get_count() - 1,
                                index,
                            );
                            draw_info_map(&info_state, frame, index);
                        })
                        .map_err(|_| InfoErr::DrawErr)?;
                }

                // go to end
                if key.code == KeyCode::Char('e') {
                    index = info_state.get_count() - 1;
                    terminal
                        .draw(|frame| {
                            draw_background(
                                frame,
                                &info_process,
                                info_state.get_count() - 1,
                                index,
                            );
                            draw_info_map(&info_state, frame, index);
                        })
                        .map_err(|_| InfoErr::DrawErr)?;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
