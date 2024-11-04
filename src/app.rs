use super::*;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

fn draw_background(frame: &mut Frame, process: &String, total_changes: usize) {
    let title = Line::from(" Enhanced PMAP".bold());
    let instructions = Line::from(vec![
        " Total Changes : ".white(),
        format!("{}", total_changes.to_string()).red(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let empty_line = Line::from(vec!["".into()]);
    let counter_text = Text::from(vec![empty_line, Line::from(vec![process.into()])]);

    let widget = Paragraph::new(counter_text).centered().block(block);
    frame.render_widget(widget, frame.area());
}

pub fn run(info_process: String, pid: sysinfo::Pid) -> std::io::Result<()> {
    let mut info_all = InfoAll::new();
    let mut terminal = ratatui::init();
    terminal.draw(|frame| draw_background(frame, &info_process, 0))?;
    loop {
        let output = get_info_map(&mut info_all, &pid);
        match output {
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
            Ok(state) => match state {
                State::Initial => {
                    terminal.draw(|frame| {
                        draw_background(frame, &info_process, 0);
                        info_all.draw_info_map(frame, 0);
                    })?;
                }
                State::NotChanged => {}
                State::Changed => {
                    terminal.draw(|frame| {
                        draw_background(frame, &info_process, info_all.get_count() - 1);
                        info_all.draw_info_map_with_diff(frame, info_all.get_count() - 1);
                    })?;
                }
            },
        }
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        std::thread::sleep(Duration::from_secs(5));
    }

    ratatui::restore();
    Ok(())
}
