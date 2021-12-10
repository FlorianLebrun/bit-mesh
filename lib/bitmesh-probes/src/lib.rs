#![allow(dead_code)]
use image::*;


pub struct ImageStream {
    pub img: RgbaImage,
}

static mut CUR_IMG: Option<ImageStream> = None;

pub fn write(img: &RgbaImage) {
    unsafe {
        CUR_IMG = Some(ImageStream { img: img.clone() });
    }
}

pub fn get_current_image<'a>() -> &'a Option<ImageStream> {
    unsafe { return &CUR_IMG }
}

/*let buf = encode_png(&img).unwrap();
unsafe {
    CUR_IMG = Some(ImageStream {
        typ: ContentType::png(),
        buf: buf,
    });
}

pub fn get_image() -> RgbaImage {
    let mut img = RgbaImage::new(32, 32);

    // Draw something to show in the final image
    for i in 0..32 {
        img.put_pixel(i, i, Rgba([255, 0, 0, 255]));
    }

    img
}

*/