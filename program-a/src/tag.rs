use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::BufReader};

#[derive(Deserialize)]
pub struct Tag {
    pub tag_name: String,
    pub geotags: Vec<Geotag>,
}

#[derive(Deserialize)]
pub struct Geotag {
    pub elapsed: u32,
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
