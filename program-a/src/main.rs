mod csv;
mod geotag;
mod tag;

use crate::{csv::load_csv, geotag::Geotag, tag::Tag};
use anyhow::{bail, Result};
use std::{env, sync::Arc, time::Instant};
use tag::find_tag_by_tag_name;

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
    let tag = find_tag_by_tag_name(tags.clone(), target_tag.clone()).await?;
    dbg!(tag);
    println!("[search-tag] took: {}[ms]", begin.elapsed().as_millis());

    Ok(())
}
