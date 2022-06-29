use crate::csv::FromCsvLine;
use anyhow::Result;

#[derive(Debug)]
pub struct Geotag {
    pub id: usize,
    pub date: String,
    pub latitude: f64,
    pub longitude: f64,
    pub url: String,
}

impl FromCsvLine for Geotag {
    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.split(',').collect::<Vec<_>>();
        let (id, date, latitude, longitude, url) = (
            tokens[0].parse()?,
            tokens[1].to_string(),
            tokens[2].parse()?,
            tokens[3].parse()?,
            tokens[4].to_string(),
        );
        Ok(Self {
            id,
            date,
            latitude,
            longitude,
            url,
        })
    }
}
