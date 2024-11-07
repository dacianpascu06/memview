use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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

pub fn virt_to_phys(pid: u32, vaddr: u64, page_size: u64) -> std::io::Result<u64> {
    let path = format!("/proc/{}/pagemap", pid);
    let mut pagemap = File::open(path)?;

    const PAGEMAP_ENTRY_SIZE: u64 = 8;

    let vpn = vaddr / page_size;
    let offset = vpn * PAGEMAP_ENTRY_SIZE;

    pagemap.seek(SeekFrom::Start(offset))?;

    let mut entry = [0u8; 8];
    pagemap.read_exact(&mut entry)?;

    let entry = u64::from_le_bytes(entry);

    let present = entry & (1 << 63) != 0;
    if !present {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Page not present in memory",
        ));
    }
    let pfn = entry & ((1 << 55) - 1);

    let phys_addr = (pfn * page_size as u64) + (vaddr % page_size as u64);
    Ok(phys_addr)
}
