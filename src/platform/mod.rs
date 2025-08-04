#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use unix as imp;
#[cfg(windows)]
pub use windows as imp;

/// То, что раньше давал C-код из libc.c
pub fn default_exec_search_paths() -> Vec<std::path::PathBuf> {
    imp::default_exec_search_paths()
}
