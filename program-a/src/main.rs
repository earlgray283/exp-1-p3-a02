mod tag;

use crate::tag::{load_tag_json, Geotag};
use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use anyhow::Result;
use chrono::{prelude::*, Duration, Utc};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

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

    let mut itoabuf = itoa::Buffer::new();
    let mut ryubuf = ryu::Buffer::new();

    let base_date = Utc.ymd(2012, 1, 1);
    let mut html = String::with_capacity(HTML_CAPACITY);
    for geotag in geotags {
        html.push_str("<div>");
        html.push_str(
            (base_date + Duration::seconds(geotag.elapsed as i64))
                .to_string()
                .as_str(),
        );
        html.push_str(ryubuf.format_finite(geotag.latitude));
        html.push(' ');
        html.push_str(ryubuf.format_finite(geotag.longitude));
        html.push_str("<img src=\"http://farm");
        html.push_str(itoabuf.format(geotag.farm_num));
        html.push_str(".static.flickr.com");
        html.push_str(geotag.directory.as_str());
        html.push_str("\"/></div>");
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(html))
}
