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
use chrono::{prelude::*, Duration, Utc};
use futures::future::join_all;
use serde::Deserialize;
use std::{fmt::Write, sync::Arc};
use tag::find_tag_by_name;

const SUBTAGS_LIMIT: usize = 100;
const PORT: u16 = 8080;
const HTML_CAPACITY: usize = 100_000;

#[tokio::main]
async fn main() -> Result<()> {
    let (res_tags, res_geotags) = tokio::join!(
        tokio::spawn(async {
            let mut tags = load_csv::<Tag>("csv/new_tag.csv").await?;
            tags.sort_unstable_by(|x, y| x.tag.cmp(&y.tag));
            Ok::<_, anyhow::Error>(tags)
        }),
        tokio::spawn(async {
            let mut geotags = load_csv::<Geotag>("csv/new_geotag.csv").await?;
            geotags.sort_unstable_by(|x, y| x.id.cmp(&y.id));
            Ok::<_, anyhow::Error>(geotags)
        })
    );
    let (tags, geotags) = (Arc::new(res_tags??), Arc::new(res_geotags??));

    println!("Listening on http://localhost:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tags.clone()))
            .app_data(Data::new(geotags.clone()))
            .service(handle_get_geotags)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await?;

    Ok(())
}

#[derive(Deserialize)]
struct GetGeotagRequest {
    tag: String,
}

#[get("/program")]
async fn handle_get_geotags(
    tags: Data<Arc<Vec<Tag>>>,
    geotags: Data<Arc<Vec<Geotag>>>,
    info: web::Query<GetGeotagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let target_tag = Arc::new(&info.tag);

    let tag = match find_tag_by_name(tags.as_ref(), target_tag.as_ref()) {
        Some(i) => &tags[i],
        None => return Err(ErrorNotFound("")),
    };

    let mut handles = Vec::with_capacity(tag.ids.len());
    for &id in &tag.ids {
        let geotags = geotags.clone();
        let handle = tokio::spawn(async move { find_geotag_by_id(geotags.as_ref(), id).unwrap() });
        handles.push(handle);
    }
    let mut subgeotags = Vec::with_capacity(tag.ids.len());
    for res in join_all(handles).await.into_iter() {
        let i = res.map_err(ErrorInternalServerError)?;
        subgeotags.push(&geotags[i]);
    }

    subgeotags.sort_unstable_by(|a, b| b.elapsed.cmp(&a.elapsed));

    let base_date = Utc.ymd(2012, 1, 1);

    let mut html = String::with_capacity(HTML_CAPACITY);
    // writeln!(&mut html, "<!DOCTYPE html>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<html>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<head>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<meta charset=\"UTF-8\" />").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<title>実装Ａの結果</title>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "</head>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<body>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<h1>{}</h1>", target_tag).map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<table>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<tr>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<th>id</th>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<th>latitude</th>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<th>longitude</th>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "<th>date</th>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "</tr>").map_err(ErrorInternalServerError)?;
    for subgeotag in &subgeotags[..SUBTAGS_LIMIT.min(subgeotags.len())] {
        writeln!(&mut html, "<tr>").map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.id).map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.latitude).map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "<td>{}</td>", subgeotag.longitude)
            .map_err(ErrorInternalServerError)?;
        writeln!(
            &mut html,
            "<td>{}</td>",
            base_date + Duration::seconds(subgeotag.elapsed as i64)
        )
        .map_err(ErrorInternalServerError)?;
        writeln!(
            &mut html,
            "<img src=\"http://farm{}.static.flickr.com{}\" />",
            subgeotag.farm_num, subgeotag.directory
        )
        .map_err(ErrorInternalServerError)?;
        writeln!(&mut html, "</tr>").map_err(ErrorInternalServerError)?;
    }
    // writeln!(&mut html, "</table>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "</body>").map_err(ErrorInternalServerError)?;
    // writeln!(&mut html, "</html>").map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(html))
}
