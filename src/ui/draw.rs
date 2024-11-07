use crate::Info;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

pub fn draw_info_map(info: &Info, frame: &mut Frame, index: usize) {
    let text = Paragraph::new(info.memory_map[index].to_string())
        .block(Block::new().padding(Padding::new(frame.area().width / 2 - 24, 0, 2, 0)))
        .alignment(Alignment::Left);

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let centered_area = vertical_chunks[1];
    frame.render_widget(text, centered_area);
}

pub fn draw_info_map_with_diff(info: &Info, frame: &mut Frame, index: usize) {
    let mut output_text: Text = Default::default();
    let prev_map = &info.memory_map[index - 1];
    let curr_map = &info.memory_map[index];

    for diff in diff::lines(&prev_map.to_string(), &curr_map.to_string()) {
        match diff {
            diff::Result::Left(l) => {
                output_text.push_line(
                    Line::from(format!("{}{}", "-", l))
                        .red()
                        .alignment(ratatui::layout::Alignment::Left),
                );
            }
            diff::Result::Both(l, _) => {
                output_text.push_line(
                    Line::from(format!("{}", l))
                        .white()
                        .alignment(ratatui::layout::Alignment::Left),
                );
            }
            diff::Result::Right(r) => {
                output_text.push_line(
                    Line::from(format!("{}{}", "+", r))
                        .green()
                        .alignment(ratatui::layout::Alignment::Left),
                );
            }
        }
    }

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let centered_area = vertical_chunks[1];

    let output_text = Paragraph::new(output_text)
        .block(Block::new().padding(Padding::new(frame.area().width / 2 - 24, 0, 2, 0)))
        .alignment(Alignment::Left);

    frame.render_widget(output_text, centered_area);
}
