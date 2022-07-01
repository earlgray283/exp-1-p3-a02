use std::sync::Arc;

use crate::{csv::FromCsvLine, THREAD_NUM};
use anyhow::{anyhow, Result};
use futures::future::join_all;

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

// linear-search
pub async fn find_tag_by_tag_name(tags: Arc<Vec<Tag>>, tag_name: Arc<String>) -> Result<Tag> {
    let tags_len = tags.len();
    let line_count_per_thread = tags_len / THREAD_NUM;
    let mut handles = Vec::with_capacity(THREAD_NUM);
    for i in (0..tags_len).step_by(line_count_per_thread) {
        let target_tag = tag_name.clone();
        let tags = tags.clone();
        let handle = tokio::spawn(async move {
            for tag in &tags[i..(i + line_count_per_thread).min(tags.len())] {
                if tag.tag == *target_tag {
                    return Some(Tag {
                        id: tag.id,
                        tag: tag.tag.clone(),
                    });
                }
            }
            None
        });
        handles.push(handle);
    }
    let res_list = join_all(handles).await;
    let mut tag = None;
    for res in res_list {
        if let Some(sub_tag) = res.map_err(|e| anyhow!("{:?}", e))? {
            tag = Some(sub_tag);
            break;
        }
    }
    tag.ok_or_else(|| anyhow!("tag {} was not found", tag_name))
}
