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
