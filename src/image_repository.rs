use rand::seq::IteratorRandom;
use std::fs;

pub struct ImageRepository;

impl ImageRepository {
    pub fn get_random_link_from_file(filename: &str) -> Option<String> {
        if let Ok(links) = fs::read_to_string(format!("images/{}", filename)) {
            if let Some(link) = links.split('\n').choose(&mut rand::thread_rng()) {
                return Some(String::from(link));
            }
        }

        None
    }
}
