# qr-web

A web service to generate qr code created by rust.

## Dependencies

- axum[tokio, serde] (Web Framework)
- qrcode[image] (Generate QR Code)
- base64 (Show PNG QR Code as html)
- tracing[tracing-subscriber] (Logging)

## Run

```
cargo install
RUST_LOG=INFO cargo run
```

## Use

visit http://localhost:3000/qr?content=hello,world

## Author

Du Yixian <duyixian1234@qq.com>
