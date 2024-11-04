use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::aux::*;
use crate::error::InfoErr;
use crate::pid::get_proc_maps_file;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

const LENGTH_NO_PATH: usize = 5;
const LENGTH_WITH_PATH: usize = 6;
const INDEX_PERMISSION: usize = 1;
const INDEX_ADDRESS: usize = 0;
const INDEX_PATH: usize = 5;

pub enum State {
    Initial,
    Changed,
    NotChanged,
}

#[derive(Debug)]
pub struct InfoAll {
    memory_map: Vec<InfoMemoryMap>,
    count: usize,
}
impl InfoAll {
    pub fn new() -> Self {
        InfoAll {
            memory_map: Vec::new(),
            count: 0,
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn draw_info_map(&self, frame: &mut Frame, index: usize) {
        //let text = Paragraph::new(self.memory_map[index].to_string()).alignment(Alignment::Center);
        let text = Paragraph::new(self.memory_map[index].to_string())
            .block(Block::new().padding(Padding::new(
                frame.area().width / 2 - 18, // left
                0,                           // right
                2,                           // top
                0,                           // bottom
            )))
            .alignment(Alignment::Left);

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Top padding (2 lines)
                Constraint::Min(0),    // Centered area
                Constraint::Length(4), // Bottom padding (2 lines)
            ])
            .split(frame.area());

        let centered_area = vertical_chunks[1];
        frame.render_widget(text, centered_area);
    }

    pub fn draw_info_map_with_diff(&self, frame: &mut Frame, index: usize) {
        let mut output_text: Text = Default::default();
        let prev_map = &self.memory_map[index - 1];
        let curr_map = &self.memory_map[index];

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
                Constraint::Length(4), // Top padding (2 lines)
                Constraint::Min(0),    // Centered area
                Constraint::Length(4), // Bottom padding (2 lines)
            ])
            .split(frame.area());

        let centered_area = vertical_chunks[1];

        let output_text = Paragraph::new(output_text)
            .block(Block::new().padding(Padding::new(
                frame.area().width / 2 - 18, // left
                0,                           // right
                2,                           // top
                0,                           // bottom
            )))
            .alignment(Alignment::Left);

        //frame.render_widget(Clear, vertical_chunks[1]);
        frame.render_widget(output_text, centered_area);
    }
}

impl std::fmt::Display for InfoAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for map in self.memory_map.iter() {
            write!(f, "{}", map)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InfoMemoryMap {
    memory_segments: Vec<InfoMemorySegment>,
    count: usize,
    size_total: String,
}

impl InfoMemoryMap {
    pub fn new() -> Self {
        InfoMemoryMap {
            memory_segments: Vec::new(),
            count: 0,
            size_total: String::new(),
        }
    }
}
impl std::fmt::Display for InfoMemoryMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in self.memory_segments.iter() {
            write!(f, "{}", segment)?;
        }
        write!(f, "{}", self.size_total)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InfoMemorySegment {
    address: u64,
    size: String,
    permissions: String,
    path: String,
}

impl InfoMemorySegment {
    pub fn new(address: u64, size: String, permissions: String, path: String) -> Self {
        InfoMemorySegment {
            address,
            size,
            permissions,
            path,
        }
    }
}
impl std::fmt::Display for InfoMemorySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_output = format!(
            "{:0>16x} {:>6} {:>4}  {:>6}\n",
            self.address, self.size, self.permissions, self.path,
        );
        write!(f, "{}", formatted_output)
    }
}

pub fn get_info_map(info_all: &mut InfoAll, pid: &sysinfo::Pid) -> Result<State, InfoErr> {
    let file: File;
    let mut total_size: u64 = 0;
    let f = get_proc_maps_file(&pid);
    match f {
        Ok(x) => file = x,
        Err(_) => {
            return Err(InfoErr::StoppedErr);
        }
    }
    let reader = BufReader::new(file);
    let page_size = get_page_size()?;

    info_all.memory_map.push(InfoMemoryMap::new());

    let curr_map = &mut info_all.memory_map[info_all.count];
    curr_map.count = 0;
    info_all.count += 1;

    for line in reader.lines() {
        let line = line.map_err(|_| InfoErr::LineErr)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let (start_addr, mut end_addr) =
            parts[INDEX_ADDRESS].split_at(parts[INDEX_ADDRESS].find("-").expect("- was not found"));
        end_addr = end_addr.trim_start_matches("-");
        let start_addr_u64 = u64::from_str_radix(start_addr, 16).expect("invalid address");
        let end_addr_u64 = u64::from_str_radix(end_addr, 16).expect("invalid address");
        let rounded_dif = ((end_addr_u64 - start_addr_u64 + page_size - 1) / page_size) * page_size;

        total_size = total_size + rounded_dif;

        let address = u64::from_str_radix(start_addr, 16).map_err(|_| InfoErr::AddrFmtErr)?;

        let mut path: String = String::new();

        // if the memory segment has a path that it is not mapped anonymous
        if parts.len() == LENGTH_WITH_PATH {
            path = {
                if parts[INDEX_PATH].starts_with("/") {
                    let paths: Vec<&str> = parts[5].split("/").collect();
                    paths[paths.len() - 1].to_owned()
                } else {
                    parts[INDEX_PATH].to_owned()
                }
            };
        } else if parts.len() == LENGTH_NO_PATH {
            path = "[anon]".to_owned();
        }

        // add the current memory_segment to the memory_map
        let memory_segment = InfoMemorySegment::new(
            address,
            format_byte_size(rounded_dif),
            parts[INDEX_PERMISSION].to_string(),
            path,
        );
        curr_map.memory_segments.push(memory_segment);
        curr_map.count += 1;
    }
    if curr_map.count == 0 {
        return Err(InfoErr::OutputErr);
    }

    // add total size
    total_size = ((total_size + page_size - 1) / page_size) * page_size;
    let formatted_line_size = format!("{:<16} {:>6} \n", "total", format_byte_size(total_size),);
    curr_map.size_total = formatted_line_size;

    if info_all.count > 1 {
        let curr_map = &info_all.memory_map[info_all.count - 1];
        let prev_map = &info_all.memory_map[info_all.count - 2];

        if prev_map == curr_map {
            info_all.memory_map.remove(info_all.count - 1);
            info_all.count -= 1;
            Ok(State::NotChanged)
        } else {
            Ok(State::Changed)
        }
    } else if info_all.count == 1 {
        // info_count = 1 => init
        Ok(State::Initial)
    } else {
        Err(InfoErr::StoppedErr)
    }
}
