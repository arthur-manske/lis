use crate::print::error;
use crate::arguments::Arguments;
use std::fs::{DirEntry, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};

fn human_readable_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;
    if size < KB {
        format!("│ {:2.1}B", size as f64)
    } else if size < MB {
        format!("│ {:.2}KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("│ {:.2}MB", size as f64 / MB as f64)
    } else if size < TB {
        format!("│ {:.2}GB", size as f64 / GB as f64)
    } else {
        format!("│ {:.2}TB", size as f64 / TB as f64)
    }
}

fn human_readable_permissions(mode: u32) -> String {
    let permissions: [char; 3] = ['r', 'w', 'x'];
    //let groups: [char; 3] = ['U', 'G', 'O'];
    let mut result = String::from("");
    for i in (0..9).step_by(3) {
        let bits = (mode >> i) & 0b111;
        //result.push(groups[(i / 3) as usize]);
        for (i, _) in permissions.iter().enumerate() {
            result.push(match bits & (1 << (2 - i)) != 0 {
                true => permissions[i],
                false => '-',
            });
        }
        result.push(' ');
    }
    result.push_str("│ ");
    result
}

fn days_in_month(month: &u32, year: &u32) -> u64 {
    match *month {
        1 => 31,
        2 if is_leap_year(year) => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

fn is_leap_year(year: &u32) -> bool {
    *year % 4 == 0 && (*year % 100 != 0 || *year % 400 == 0)
}

fn human_readable_date(timestamp: u64) -> String {
    let secs_per_day = 86400;
    let mut days_since_epoch = timestamp / secs_per_day;
    let mut year = 1970;
    let mut days_in_year = 365;
    while days_since_epoch >= days_in_year as u64 {
        days_since_epoch -= days_in_year as u64;
        year += 1;
        days_in_year = if is_leap_year(&year) { 366 } else { 365 };
    }
    let mut days = days_since_epoch;
    let mut month = 0;
    while days >= days_in_month(&month, &year) {
        days -= days_in_month(&month, &year);
        month += 1;
    }
    let day = days + 1;
    format!(" │ {:02}/{:02}/{:02} ", day, month, year % 100)
}

pub fn get_metadata(entry: &DirEntry) -> Metadata {
    match entry.metadata() {
        Ok(_) => entry.metadata().unwrap(),
        Err(e) => std::process::exit(error("Can't obtain metadata", &e.to_string())),
    }
}

pub fn read_permissions(metadata: &Metadata) -> String {
    human_readable_permissions(metadata.permissions().mode())
}

pub fn read_date(metadata: &Metadata) -> String {
    human_readable_date(
        metadata
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
}

pub fn read_size(metadata: &Metadata) -> String {
    human_readable_size(metadata.size())
}

pub fn read_metadata(entry: &DirEntry, arguments: &Arguments) -> [String; 3] {
    let metadata = get_metadata(&entry);    
    let date: String = match arguments.date() {
                false => String::from(""),
                true => read_date(&metadata),
    };
    let permissions = match arguments.permissions() {
                false => String::from(""),
                true => read_permissions(&metadata),
    };
    let size: String = match arguments.size() {
                false => String::from(""),
                true => read_size(&metadata),
    };
    [date, permissions, size]
}
