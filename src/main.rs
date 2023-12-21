use axum::{
    extract::Query,
    http::header,
    response::{AppendHeaders, Html, IntoResponse},
    routing::get,
    Router,
};
use image::{codecs::png::PngEncoder, Luma};
use qrcode::QrCode;
use serde::Deserialize;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Params {
    content: String,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/qr", get(handler))
        .route("/api/qr", get(handler_api));

    Ok(app.into())
}

async fn handler(Query(params): Query<Params>) -> Html<String> {
    let buffer = gen_qr_code(params.content.as_str());

    Html(format!(
        "<img alt=\"{}\" src=\"data:image/png;base64,{}\">",
        params.content,
        base64::encode(&buffer)
    ))
}
async fn handler_api(Query(params): Query<Params>) -> impl IntoResponse {
    let buffer = gen_qr_code(params.content.as_str());

    let headers = AppendHeaders([
        (header::CONTENT_TYPE, "image/png"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"qr.png\"",
        ),
    ]);

    (headers, buffer)
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
