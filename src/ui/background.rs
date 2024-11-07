use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};

pub fn draw_background(
    frame: &mut Frame,
    process: &ratatui::prelude::Line,
    total_changes: usize,
    index: usize,
) {
    let title = Line::from(" Enhanced PMAP".bold());
    let instructions = Line::from(vec![
        "go to start ".into(),
        "<s> ".blue().bold(),
        "go to end ".into(),
        "<e> ".blue().bold(),
        "go to start ".into(),
        "<s> ".blue().bold(),
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
