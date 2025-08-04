use std::path::PathBuf;

pub fn default_exec_search_paths() -> Vec<PathBuf> {
    // Типичный дефолт, если PATH пуст.
    ["/usr/local/bin", "/usr/bin", "/bin"]
        .into_iter()
        .map(PathBuf::from)
        .collect()
}
