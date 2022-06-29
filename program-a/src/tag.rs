use crate::csv::FromCsvLine;
use anyhow::Result;

#[derive(Debug)]
pub struct Tag {
    pub id: usize,
    pub tag: String,
}
impl FromCsvLine for Tag {
    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.trim().split(',').collect::<Vec<_>>();
        let (id, tag) = (tokens[0].parse()?, tokens[1].into());
        Ok(Self { id, tag })
    }
}
