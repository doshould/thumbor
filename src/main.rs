use anyhow::Ok;
use axum::{extract::Path, handler::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;
use std::{convert::TryInto};

mod pb;

use pb::*;

//参数使用 serde 做 Deserialize, axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}


#[tokio::main]
async fn main() {
    //init tracing
    tracing_subscriber::fmt::init();

    //build router
    let app = Router::new()
        //  `GET /image` 会执行 generate 函数，并把 spec 和 url 传递过去
        .route("/image/:spec/:url", get(generate));

    //
    let addr = "127.0.0.1:13000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//解析参数
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    anyhow::Result::Ok(format!("url: {}\n spec: {:#?}", url, spec))
}