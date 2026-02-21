use libc::{
    ECHO, FD_SET, FD_ZERO, ICANON, STDIN_FILENO, TCSANOW, VMIN, VTIME, fd_set, select, tcgetattr,
    tcsetattr, termios, timeval,
};

#[allow(dead_code)]
static mut ORIGINAL_TERMINAL_SETTINGS: Option<termios> = None;

pub fn disable_input_buffering() {
    unsafe {
        let mut tty: termios = std::mem::zeroed();
        if tcgetattr(STDIN_FILENO, &mut tty) == 0 {
            ORIGINAL_TERMINAL_SETTINGS = Some(tty);
            let mut raw = tty;
            // ICANON: Disables canonical mode (waits for Enter).
            // ECHO: Disables automatic printing of typed characters.
            raw.c_lflag &= !(ICANON | ECHO);
            // Ensure read returns immediately after 1 character.
            raw.c_cc[VMIN] = 1;
            raw.c_cc[VTIME] = 0;

            let _ = tcsetattr(STDIN_FILENO, TCSANOW, &raw);
        }
    }
}

pub fn restore_input_buffering() {
    unsafe {
        if let Some(tty) = ORIGINAL_TERMINAL_SETTINGS {
            let _ = tcsetattr(STDIN_FILENO, TCSANOW, &tty);
        }
    }
}

pub fn check_key() -> bool {
    unsafe {
        let mut readfds: fd_set = std::mem::zeroed();
        FD_ZERO(&mut readfds);
        FD_SET(STDIN_FILENO, &mut readfds);
        let mut timeout = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };

        select(
            STDIN_FILENO + 1,
            &mut readfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        ) > 0
    }
}
