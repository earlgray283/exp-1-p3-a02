mod tag;

use crate::tag::{load_tag_json, Geotag};
use actix_web::{
    error::{ErrorInternalServerError},
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Result;
use chrono::{prelude::*, Duration, Utc};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Write, sync::Arc};

const PORT: u16 = 8080;
const HTML_CAPACITY: usize = 100_000;

#[tokio::main]
async fn main() -> Result<()> {
    let tags = load_tag_json("csv/tag.json")?;
    let mut tags_map = HashMap::with_capacity(tags.len());
    for tag in tags {
        tags_map.insert(tag.tag_name, tag.geotags);
    }
    let tags_map_arc = Arc::new(tags_map);

    println!("Listening on http://localhost:8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tags_map_arc.clone()))
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
    tags: Data<Arc<HashMap<String, Vec<Geotag>>>>,
    info: web::Query<GetGeotagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let geotags = tags.get(&info.tag).unwrap();

    let base_date = Utc.ymd(2012, 1, 1);
    let mut html = String::with_capacity(HTML_CAPACITY);
    for geotag in geotags {
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
