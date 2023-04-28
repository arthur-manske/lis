use std::mem;
use libc;

#[repr(C)]
struct winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

pub fn get_terminal_size() -> Option<[u16; 2]> {
    unsafe {
        let mut winsize: winsize = mem::zeroed();
        libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut winsize);
        if winsize.ws_col > 0 && winsize.ws_row > 0 {
            Some([winsize.ws_col as u16, winsize.ws_row as u16])
        } else {
            None
        }
    }
}
