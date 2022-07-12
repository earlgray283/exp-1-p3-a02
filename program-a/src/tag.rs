use crate::csv::FromCsvLine;
use anyhow::Result;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Tag {
    pub id: u64,
    pub tag: String,
}

impl FromCsvLine for Tag {
    fn from_str(s: &str) -> Result<Self> {
        let tokens = s.trim().split(',').collect::<Vec<_>>();
        let (id, tag) = (tokens[0].parse()?, tokens[1].into());
        Ok(Self { id, tag })
    }
}

pub fn find_tag_by_name(tags: &[Tag], name: &str) -> Option<Vec<usize>> {
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
        let mut subtags = Vec::new();
        for (i, tag) in tags[low..].iter().enumerate() {
            if tag.tag != name {
                break;
            }
            subtags.push(i);
        }
        Some(subtags)
    } else {
        None
    }
}
