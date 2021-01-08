use std::{io, mem, os::unix::io::AsRawFd};

pub fn echo(stdin: &io::Stdin, echo: bool) {
    unsafe {
        let mut termios: libc::termios = mem::zeroed();
        libc::tcgetattr(stdin.as_raw_fd(), &mut termios);
        if echo {
            termios.c_lflag |= libc::ICANON | // no input processing
                libc::ECHO; // no input echoing
        } else {
            termios.c_lflag &= !(
                libc::ICANON | // no input processing
                libc::ECHO
                // no input echoing
            );
        }
        libc::tcsetattr(stdin.as_raw_fd(), libc::TCSANOW, &termios);
    }
}
