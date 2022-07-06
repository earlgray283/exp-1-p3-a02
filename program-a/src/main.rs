mod csv;
mod geotag;
mod tag;

use crate::{
    csv::load_csv,
    geotag::{find_geotag_by_id, Geotag},
    tag::Tag,
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Result;
use futures::future::join_all;
use serde::Deserialize;
use std::{fmt::Write, sync::Arc, time::Instant};
use tag::find_tag_by_name;
use tokio::sync::Mutex;

const THREAD_NUM: usize = 128;
const SUBTAGS_LIMIT: usize = 100;

#[tokio::main]
async fn main() -> Result<()> {
    println!("[loading] start measurement");
    let begin = Instant::now();
    let (tags, geotags) = tokio::join!(
        tokio::spawn(async {
            println!("Loading and sorting tag.csv...");
            let mut tags = load_csv::<Tag>("../csv/tag.csv").await?;
            tags.sort_unstable_by(|x, y| x.tag.cmp(&y.tag));
            println!("done");
            Ok::<_, anyhow::Error>(tags)
        }),
        tokio::spawn(async {
            println!("Loading and sorting geotag.csv...");
            let mut geotags = load_csv::<Geotag>("../csv/geotag.csv").await?;
            geotags.sort_unstable_by(|x, y| x.id.cmp(&y.id));
            println!("done");
            Ok::<_, anyhow::Error>(geotags)
        })
    );
    let (tags, geotags) = (Arc::new(tags??), Arc::new(geotags??));
    println!("[loading] took: {}[ms]", begin.elapsed().as_millis());

    println!("Li&stening on http://localhost:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tags.clone()))
            .app_data(Data::new(geotags.clone()))
            .service(handle_get_geotags)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct GetGeotagRequest {
    tag: String,
}

#[get("/program")]
async fn handle_get_geotags(
    tags: Data<Arc<Vec<Tag>>>,
    geotags: Data<Arc<Vec<Geotag>>>,
    info: web::Query<GetGeotagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let target_tag = Arc::new(info.tag.clone());
    println!("tag: {}", target_tag);

    println!("[search-tag] start measurement");
    let begin = Instant::now();
    let subtags = match find_tag_by_name(tags.as_ref(), target_tag.as_ref()) {
        Some(tags) => tags,
        None => return Err(ErrorNotFound("")),
    };
    println!(
        "[search-tag] took: {}[ns](found {} entries)",
        begin.elapsed().as_nanos(),
        subtags.len()
    );

    println!("[search-geotag] start measurement");
    let begin = Instant::now();
    let mut handles = Vec::with_capacity(subtags.len());
    let geotag_indexs = Arc::new(Mutex::new(Vec::with_capacity(subtags.len())));
    for &subtag in subtags.iter() {
        let geotags = geotags.clone();
        let tags = tags.clone();
        let geotag_ids = geotag_indexs.clone();
        let handle = tokio::spawn(async move {
            let geotag_i = find_geotag_by_id(geotags.as_ref(), tags[subtag].id).unwrap();
            geotag_ids.lock().await.push(geotag_i);
        });
        handles.push(handle);
    }
    join_all(handles).await;
    let mut subgeotags = Vec::with_capacity(subtags.len());
    let geotag_indexs = geotag_indexs.lock().await;
    for &geotag_i in geotag_indexs.iter() {
        subgeotags.push(&geotags[geotag_i]);
    }
    println!(
        "[search-geotag] took: {}[ns](found {} entries)",
        begin.elapsed().as_nanos(),
        geotag_indexs.len()
    );

    subgeotags.sort_unstable_by(|a, b| b.date.cmp(&a.date));

    let mut html = String::with_capacity(1_000_000);
    writeln!(&mut html, "<!DOCTYPE html>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<html>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<head>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<meta charset=\"UTF-8\" />").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<title>実装Ａの結果</title>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "</head>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<body>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<h1>{}</h1>", target_tag).map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<table>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<tr>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<th>id</th>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<th>latitude</th>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<th>longitude</th>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "<th>date</th>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "</tr>").map_err(ErrorInternalServerError)?;
    for subgeotag in &subgeotags[..SUBTAGS_LIMIT] {
        writeln!(&mut html, "<tr>").map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.id).map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.latitude).map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.longitude)
            .map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.date).map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<img src=\"{}\" />", &subgeotag.url)
            .map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "</tr>").map_err(ErrorInternalServerError)?;
    }
    writeln!(&mut html, "</table>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "</body>").map_err(ErrorInternalServerError)?;
    writeln!(&mut html, "</html>").map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(html))
}
