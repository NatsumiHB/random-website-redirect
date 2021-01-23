use actix_web::http::header::LOCATION;
use actix_web::http::HeaderValue;
use actix_web::middleware::Logger;
use actix_web::web::{Data, Json};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use rand::seq::IteratorRandom;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;

fn get_urls_from_json(json_path: &str) -> anyhow::Result<HashMap<String, String>> {
    let file = File::open(json_path)?;

    Ok(serde_json::from_reader(file)?)
}

#[derive(Clone, Serialize)]
struct Urls(HashMap<String, String>);

#[derive(serde::Serialize)]
struct Url {
    pub name: String,
    pub url: String,
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let urls = Urls(get_urls_from_json("./urls.json")?);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(choose_json)
            .service(get_json)
            .service(all_urls)
            .service(choose)
            .service(random)
            .data(urls.clone())
    })
    .bind("0.0.0.0:5001")?
    .run()
    .await?;

    Ok(())
}

#[get("/all_urls")]
async fn all_urls(urls: Data<Urls>) -> Json<Urls> {
    Json(urls.as_ref().clone())
}

#[get("/get_json")]
async fn get_json(urls: Data<Urls>) -> Json<Url> {
    let name = urls
        .as_ref()
        .0
        .keys()
        .choose(&mut rand::thread_rng())
        .unwrap();

    Json(Url {
        name: name.to_owned(),
        url: urls.as_ref().0.get(name.as_str()).unwrap().to_owned(),
    })
}

#[get("/get_json/{id}")]
async fn choose_json(id: web::Path<String>, urls: Data<Urls>) -> actix_web::Result<Json<Url>> {
    match urls.as_ref().0.get(id.as_str()) {
        Some(url) => Ok(Json(Url {
            name: id.to_string(),
            url: url.to_owned(),
        })),
        None => Err(HttpResponse::NotFound().into()),
    }
}

#[get("/")]
async fn random(urls: Data<Urls>) -> HttpResponse {
    let redirect_url = urls
        .as_ref()
        .0
        .values()
        .choose(&mut rand::thread_rng())
        .unwrap();

    HttpResponse::TemporaryRedirect()
        .set_header(LOCATION, HeaderValue::from_str(redirect_url).unwrap())
        .finish()
}

#[get("/{id}")]
async fn choose(id: web::Path<String>, urls: Data<Urls>) -> actix_web::Result<HttpResponse> {
    match urls.as_ref().0.get(id.as_str()) {
        Some(url) => Ok(HttpResponse::TemporaryRedirect()
            .set_header(LOCATION, HeaderValue::from_str(url).unwrap())
            .finish()),
        None => Err(HttpResponse::NotFound().into()),
    }
}
