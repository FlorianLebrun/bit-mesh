#![allow(unused)]
use core::panic;
use image::*;
use std::{ops::Deref, thread};

fn encode_png<P, Container>(img: &ImageBuffer<P, Container>) -> Result<Vec<u8>, ImageError>
where
    P: image::Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder.write_image(img, img.width(), img.height(), P::COLOR_TYPE)?;
    Ok(buf)
}

#[get("/img")]
async fn api_get_image() -> Result<HttpResponse> {
    match bitmesh_probes::get_current_image() {
        Some(img) => Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::png())
            .body(encode_png(&img.img).unwrap())),
        None => Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("")),
    }
}

#[get("/probes")]
async fn api_probes() -> Result<HttpResponse> {
    panic!()
}

#[rocket::main]
async fn server_routine() {
    let port = 8080;
    println!("server run: http://localhost:{0}", port);
    HttpServer::new(|| App::new().service(api_get_image))
        .bind(("127.0.0.1", port))
        .unwrap()
        .run()
        .await
        .unwrap();
}

pub fn run_services() {
    thread::spawn(server_routine);
}
