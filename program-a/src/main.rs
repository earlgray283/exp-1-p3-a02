mod tag;

use crate::tag::{load_tag_json, Tag};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Result;
use chrono::{prelude::*, Duration, Utc};
use serde::Deserialize;
use std::{fmt::Write, sync::Arc};
use tag::find_tag_by_name;

const PORT: u16 = 8080;
const HTML_CAPACITY: usize = 100_000;

#[tokio::main]
async fn main() -> Result<()> {
    let tags = Arc::new(load_tag_json("csv/tag.json")?);

    for tag in tags.iter() {
        println!("{}", tag.tag_name);
    }

    println!("Listening on http://localhost:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tags.clone()))
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
    info: web::Query<GetGeotagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let target_tag = Arc::new(&info.tag);

    let tag = match find_tag_by_name(tags.as_ref(), target_tag.as_ref()) {
        Some(i) => &tags[i],
        None => return Err(ErrorNotFound("")),
    };

    let base_date = Utc.ymd(2012, 1, 1);
    let mut html = String::with_capacity(HTML_CAPACITY);
    for geotag in &tag.geotags {
        write!(
            &mut html,
            "<div>{} {} {} <img src=\"http://farm{}.static.flickr.com{}\" /></div>",
            geotag.latitude,
            geotag.longitude,
            base_date + Duration::seconds(geotag.elapsed as i64),
            geotag.farm_num,
            geotag.directory
        )
        .map_err(ErrorInternalServerError)?;
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(html))
}
