use crate::csv::FromCsvLine;
use anyhow::Result;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Geotag {
    pub id: u64,
    pub elapsed: u32,
    pub latitude: f32,
    pub longitude: f32,
    pub farm_num: i8,
    pub directory: String,
}

impl FromCsvLine for Geotag {
    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.split(',').collect::<Vec<_>>();
        let (id, elapsed, latitude, longitude, farm_num, directory) = (
            tokens[0].parse()?,
            tokens[1].parse()?,
            tokens[2].parse()?,
            tokens[3].parse()?,
            tokens[4].parse()?,
            tokens[5].trim(),
        );
        Ok(Self {
            id,
            elapsed,
            latitude,
            longitude,
            farm_num,
            directory: directory.to_string(),
        })
    }
}

pub fn find_geotag_by_id(geotags: &[Geotag], id: u64) -> Option<usize> {
    let (mut low, mut high) = (0, geotags.len());
    while low != high {
        let mid = (low + high) / 2;
        match geotags[mid].id.cmp(&id) {
            Ordering::Less => {
                low = mid + 1;
            }
            Ordering::Equal | Ordering::Greater => {
                high = mid;
            }
        }
    }
    if geotags[low].id == id {
        Some(low)
    } else {
        None
    }
}
