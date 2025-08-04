use libc::c_int;

#[cfg(unix)]
pub const STDIN_FILENO:  c_int = libc::STDIN_FILENO;
#[cfg(unix)]
pub const STDOUT_FILENO: c_int = libc::STDOUT_FILENO;
#[cfg(unix)]
pub const STDERR_FILENO: c_int = libc::STDERR_FILENO;

#[cfg(windows)]
pub const STDIN_FILENO:  c_int = 0;
#[cfg(windows)]
pub const STDOUT_FILENO: c_int = 1;
#[cfg(windows)]
pub const STDERR_FILENO: c_int = 2;
