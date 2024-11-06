use super::*;
use aux::parse_info_proc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

fn draw_background(
    frame: &mut Frame,
    process: &ratatui::prelude::Line,
    total_changes: usize,
    index: usize,
) {
    let title = Line::from(" Enhanced PMAP".bold());
    let instructions = Line::from(vec![
        " prev ".into(),
        "<p> ".blue().bold(),
        " next ".into(),
        "<n> ".blue().bold(),
        " Current/Total : ".white(),
        format!("{}/", index.to_string()).red(),
        format!("{}", total_changes.to_string()).red(),
        " Toggle Diff ".into(),
        "<d> ".blue().bold(),
        "  Refresh ".into(),
        "<r> ".blue().bold(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let empty_line = Line::from(vec!["".into()]);
    let counter_text = Text::from(vec![empty_line, process.clone()]);

    let widget = Paragraph::new(counter_text).centered().block(block);
    frame.render_widget(widget, frame.area());
}

pub fn run(info_process: String, pid: sysinfo::Pid) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let info_process = parse_info_proc(&info_process);
    let mut info_all = InfoAll::new(pid.as_u32());

    let mut err = error::InfoErr::None;
    let mut diff: bool = false;
    let mut index: usize = 0;
    loop {
        let output = get_info_map(&mut info_all, &pid);
        match output {
            Err(e) => {
                err = e;
                break;
            }
            Ok(state) => match state {
                State::Initial => {
                    terminal.draw(|frame| {
                        draw_background(frame, &info_process, 0, 0);
                        info_all.draw_info_map(frame, 0);
                        index = 0;
                    })?;
                }
                State::NotChanged => {}
                State::Changed => {}
            },
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                if key.code == KeyCode::Char('n') {
                    if index == info_all.get_count() - 2 || index == info_all.get_count() - 1 {
                        index = info_all.get_count() - 1;
                    }
                    if diff == false {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else if diff == true {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map_with_diff(frame, index);
                        })?;
                    }
                }
                if key.code == KeyCode::Char('p') {
                    if index == 0 || index == 1 {
                        terminal.draw(|frame| {
                            index = 0;
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else {
                        index -= 1;
                        if diff == false {
                            terminal.draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map(frame, index);
                            })?;
                        } else {
                            terminal.draw(|frame| {
                                draw_background(
                                    frame,
                                    &info_process,
                                    info_all.get_count() - 1,
                                    index,
                                );
                                info_all.draw_info_map_with_diff(frame, index);
                            })?;
                        }
                    }
                }
                if key.code == KeyCode::Char('r') {
                    if index == 0 {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else if diff == false {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map_with_diff(frame, index);
                        })?;
                    }
                }

                if key.code == KeyCode::Char('d') {
                    diff = !diff;
                    if index == 0 {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else if diff == false {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map(frame, index);
                        })?;
                    } else if diff == true {
                        terminal.draw(|frame| {
                            draw_background(frame, &info_process, info_all.get_count() - 1, index);
                            info_all.draw_info_map_with_diff(frame, index);
                        })?;
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    ratatui::restore();

    match err {
        error::InfoErr::None => {}
        _ => eprintln!("{}", err),
    }

    Ok(())
}
