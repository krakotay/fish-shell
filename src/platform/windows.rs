use std::path::PathBuf;

pub fn default_exec_search_paths() -> Vec<PathBuf> {
    // На Windows разумно брать PATH из окружения; если пусто — фоллбэк.
    if let Ok(path) = std::env::var("PATH") {
        return std::env::split_paths(&path).collect();
    }
    vec![
        PathBuf::from(r"C:\Windows\System32"),
        PathBuf::from(r"C:\Windows"),
    ]
}
