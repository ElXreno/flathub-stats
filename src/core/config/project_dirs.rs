use directories::ProjectDirs;
use std::path::Path;

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs =
        ProjectDirs::from("com", "elxreno", "flathub-stats").unwrap();
}

pub fn get_cache_dir() -> &'static Path {
    PROJECT_DIRS.cache_dir()
}
