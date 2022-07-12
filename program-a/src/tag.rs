use crate::csv::FromCsvLine;
use anyhow::Result;
use std::cmp::Ordering;

pub struct Tag {
    pub tag: String,
    pub ids: Vec<u64>,
}

impl FromCsvLine for Tag {
    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.trim().split(',').collect::<Vec<_>>();
        let tag = tokens[0];
        let mut ids = Vec::new();
        for token in &tokens[1..] {
            ids.push(token.parse()?);
        }
        Ok(Self {
            tag: tag.to_string(),
            ids,
        })
    }
}

pub fn find_tag_by_name<'a>(tags: &'a [Tag], name: &'a str) -> Option<&'a Vec<u64>> {
    let (mut low, mut high) = (0, tags.len());
    while low != high {
        let mid = (low + high) / 2;
        match tags[mid].tag.as_str().cmp(name) {
            Ordering::Less => {
                low = mid + 1;
            }
            Ordering::Equal | Ordering::Greater => {
                high = mid;
            }
        }
    }
    if tags[low].tag == name {
        Some(&tags[low].ids)
    } else {
        None
    }
}
