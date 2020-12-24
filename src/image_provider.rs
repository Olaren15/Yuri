use rand::{thread_rng, Rng};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ImageProvider;

impl ImageProvider {
    pub fn get_random(folder_name: &String) -> Option<PathBuf> {
        let path = format!("images/{}", folder_name);
        let path = Path::new(&path);

        if path.exists() && path.is_dir() {
            let mut paths = vec![];

            for item in fs::read_dir(path).unwrap() {
                if let Ok(item) = item {
                    paths.push(item.path());
                }
            }

            let mut rng = thread_rng();
            let index = rng.gen_range(0..paths.len());

            return Some(paths[index].clone());
        }

        None
    }
}
