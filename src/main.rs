use axum::{extract::Query, response::Html, routing::get, Router};
use image::{codecs::png::PngEncoder, Luma};
use qrcode::QrCode;
use serde::Deserialize;
use std::{io::Write, net::SocketAddr};

#[derive(Debug, Deserialize)]
struct Params {
    content: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/qr", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Query(params): Query<Params>) -> Html<String> {
    let buffer = gen_qr_code(params.content.as_str());

    Html(format!(
        "<img alt=\"{}\" src=\"data:image/png;base64,{}\">",
        params.content,
        base64::encode(&buffer)
    ))
}

fn gen_qr_code(content: &str) -> Vec<u8> {
    tracing::info!("Generating QR code for {}", content);

    let image = QrCode::new(content)
        .unwrap()
        .render::<Luma<u8>>()
        .min_dimensions(512, 512)
        .build();

    let mut buffer = Vec::new();
    PngEncoder::new(buffer.by_ref())
        .encode(&image, image.width(), image.height(), image::ColorType::L8)
        .unwrap();

    buffer
}
