mod csv;
mod geotag;
mod tag;

use crate::{csv::load_csv, geotag::Geotag, tag::Tag};
use anyhow::{anyhow, bail, Result};
use futures::future::join_all;
use std::{env, sync::Arc, time::Instant};

const THREAD_NUM: usize = 128;

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        bail!("too few argments");
    }
    let target_tag = Arc::new(args[1].clone());

    println!("[loading] start measurement");
    let begin = Instant::now();
    let (tags, geotags) = tokio::join!(
        tokio::spawn(async {
            println!("Loading tag.csv...");
            let tags = load_csv::<Tag>("../csv/tag.csv").await?;
            println!("done");
            //tags.sort_unstable_by(|x, y| x.tag.cmp(&y.tag));
            Ok::<_, anyhow::Error>(tags)
        }),
        tokio::spawn(async {
            println!("Loading geotag.csv...");
            let geotags = load_csv::<Geotag>("../csv/geotag.csv").await?;
            println!("done");
            //geotags.sort_unstable_by(|x, y| x.tag.cmp(&y.tag));
            Ok::<_, anyhow::Error>(geotags)
        })
    );
    println!("[loading] took: {}[ms]", begin.elapsed().as_millis());

    let (tags, _geotags) = (Arc::new(tags??), Arc::new(geotags??));

    println!("[search-tag] start measurement");
    let begin = Instant::now();
    let tags_len = tags.len();
    let line_count_per_thread = tags_len / THREAD_NUM;
    let mut handles = Vec::with_capacity(THREAD_NUM);
    for i in (0..tags_len).step_by(line_count_per_thread) {
        let target_tag = target_tag.clone();
        let tags = tags.clone();
        let handle = tokio::spawn(async move {
            let mut cmp_cnt = 0;
            for tag in &tags[i..(i + line_count_per_thread).min(tags.len())] {
                cmp_cnt += 1;

                if tag.tag == *target_tag {
                    println!("cmp_cnt: {}(some)", cmp_cnt);
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
    let tag = tag.ok_or_else(|| anyhow!("tag {} was not found", target_tag))?;
    dbg!(tag);
    println!("[search-tag] took: {}[ms]", begin.elapsed().as_millis());

    Ok(())
}
