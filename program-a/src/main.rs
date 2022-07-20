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
use fxhash::FxHashMap as HashMap;
use once_cell::sync::Lazy;
use sailfish::runtime::Buffer;
use serde::Deserialize;
use std::{ops::Add, sync::Arc};

const PORT: u16 = 8080;
const HTML_CAPACITY: usize = 100_000;
static BASE_DATE: Lazy<Date<Utc>> = Lazy::new(|| Utc.ymd(2012, 1, 1));

#[tokio::main]
async fn main() -> Result<()> {
    let tags = load_tag_json("csv/tag.json")?;
    let mut tags_map = HashMap::default();
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
    tag_map: Data<Arc<HashMap<String, Vec<Geotag>>>>,
    info: web::Query<GetGeotagRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let geotags = tag_map.get(&info.tag).unwrap();

    let mut ryubuf = ryu::Buffer::new();

    let mut json = Buffer::with_capacity(HTML_CAPACITY);

    json.push_str(r#"{"tag": ""#);
    json.push_str(&info.tag);
    json.push_str(r#"","geotags":["#);
    for geotag in &geotags[..geotags.len() - 1] {
        json.push_str(r#"{"lat":"#);
        json.push_str(ryubuf.format(geotag.latitude));
        json.push_str(r#","lon":"#);
        json.push_str(ryubuf.format(geotag.longitude));
        json.push_str(r#","date":""#);
        json.push_str(&(BASE_DATE.add(Duration::seconds(geotag.elapsed as i64))).to_string());
        json.push_str(r#"","url":"https://farm"#);
        unsafe {
            let ptr = json.as_mut_ptr().add(json.len());
            itoap::write_to_ptr(ptr, geotag.farm_num);
        }
        json.push_str(".static.flickr.com");
        json.push_str(&geotag.directory);
        json.push_str(r#""},"#);
    }
    let geotag = &geotags[geotags.len() - 1];
    json.push_str(r#"{"lat":"#);
    json.push_str(ryubuf.format(geotag.latitude));
    json.push_str(r#","lon":"#);
    json.push_str(ryubuf.format(geotag.longitude));
    json.push_str(r#","date":""#);
    json.push_str(&(BASE_DATE.add(Duration::seconds(geotag.elapsed as i64))).to_string());
    json.push_str(r#"","url":"https://farm"#);
    unsafe {
        let ptr = json.as_mut_ptr().add(json.len());
        itoap::write_to_ptr(ptr, geotag.farm_num);
    }
    json.push_str(".static.flickr.com");
    json.push_str(&geotag.directory);
    json.push_str(r#""}]}"#);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(json.into_string()))
}
