use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};
pub fn format_byte_size(size: u64) -> String {
    const KB: u64 = 1024;
    if size < KB {
        format!("{} B", size)
    } else {
        format!("{}KB", size as f64 / KB as f64)
    }
}

pub fn get_page_size() -> Result<u64, crate::error::InfoErr> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size == -1 {
        return Err(crate::error::InfoErr::PageErr);
    }
    Ok(page_size as u64)
}

pub fn parse_info_proc(info_process: &'static str) -> Line<'_> {
    let info_proc: Vec<String> = info_process
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let out_name = Span::styled(info_proc[0].clone(), Style::default().fg(Color::White));
    let out_name_2 = Span::styled(info_proc[1].clone(), Style::default().fg(Color::White));
    let out_name_value = Span::styled(info_proc[2].clone(), Style::default().fg(Color::Red));
    let out_pid = Span::styled(info_proc[3].clone(), Style::default().fg(Color::White));
    let out_pid_value = Span::styled(info_proc[4].clone(), Style::default().fg(Color::Red));

    Line::from(vec![
        out_name,
        " ".into(),
        out_name_2,
        " : ".into(),
        out_name_value,
        " ".into(),
        out_pid,
        " : ".into(),
        out_pid_value,
    ])
}
