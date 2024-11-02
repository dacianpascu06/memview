use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::error::InfoErr;
use crate::pid::get_proc_maps_file;
use crate::Cli;

const LENGTH_NO_PATH: usize = 5;
const LENGTH_WITH_PATH: usize = 6;
const INDEX_PERMISSION: usize = 1;
const INDEX_ADDRESS: usize = 0;
const INDEX_PATH: usize = 5;

fn format_byte_size(size: u64) -> String {
    const KB: u64 = 1024;
    if size < KB {
        format!("{} B", size)
    } else {
        format!("{}KB", size as f64 / KB as f64)
    }
}

fn get_page_size() -> Result<u64, InfoErr> {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if page_size == -1 {
        return Err(InfoErr::PageErr);
    }
    Ok(page_size as u64)
}

pub fn get_info_map(args: &Cli) -> Result<String, InfoErr> {
    let file: File;
    match get_proc_maps_file(args) {
        Ok(file_value) => file = file_value,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1); // Exit the program if there's an error
        }
    }

    let reader = BufReader::new(file);
    let page_size = get_page_size()?;
    let mut formatted_output = String::new();

    for line in reader.lines() {
        let line = line.map_err(|_| InfoErr::LineErr)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        let (start_addr, mut end_addr) =
            parts[INDEX_ADDRESS].split_at(parts[INDEX_ADDRESS].find("-").expect("- was not found"));
        end_addr = end_addr.trim_start_matches("-");
        let start_addr_u64 = u64::from_str_radix(start_addr, 16).expect("invalid address");
        let end_addr_u64 = u64::from_str_radix(end_addr, 16).expect("invalid address");
        let rounded_dif = ((end_addr_u64 - start_addr_u64 + page_size - 1) / page_size) * page_size;

        let address_string =
            u64::from_str_radix(start_addr, 16).map_err(|_| InfoErr::AddrFmtErr)?;

        let mut path: String = String::new();

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

        let formatted_line = format!(
            "{:<18} {:>6} {:>4}  {:>6}\n",
            address_string,
            format_byte_size(rounded_dif),
            parts[INDEX_PERMISSION],
            path,
        );
        formatted_output.push_str(&formatted_line);
    }
    if formatted_output.is_empty() {
        return Err(InfoErr::OutputErr);
    }
    Ok(formatted_output)
}
