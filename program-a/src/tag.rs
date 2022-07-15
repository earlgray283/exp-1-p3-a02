use anyhow::Result;
use serde::Deserialize;
use std::{cmp::Ordering, fs::File, io::BufReader};

#[derive(Deserialize)]
pub struct Tag {
    pub tag_name: String,
    pub geotags: Vec<Geotag>,
}

#[derive(Deserialize)]
pub struct Geotag {
    pub elapsed: u64,
    pub latitude: f32,
    pub longitude: f32,
    pub farm_num: i8,
    pub directory: String,
}

pub fn load_tag_json(name: &str) -> Result<Vec<Tag>> {
    let f = File::open(name)?;
    let r = BufReader::new(f);
    let tags = serde_json::from_reader(r)?;
    Ok(tags)
}

pub fn find_tag_by_name(tags: &[Tag], name: &str) -> Option<usize> {
    let (mut low, mut high) = (0, tags.len());
    while low != high {
        let mid = (low + high) / 2;
        match tags[mid].tag_name.as_str().cmp(name) {
            Ordering::Less => {
                low = mid + 1;
            }
            Ordering::Equal | Ordering::Greater => {
                high = mid;
            }
        }
    }
    if tags[low].tag_name == name {
        Some(low)
    } else {
        None
    }
}
