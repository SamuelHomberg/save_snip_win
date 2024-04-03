use clipboard_win::{formats, get_clipboard};
// use image::GenericImageView;
use std::io::Cursor;

fn main() {
    let bitmap = get_clipboard(formats::Bitmap).expect("get bitmap from clip");
    let cursor = Cursor::new(bitmap);
    let img = image::load(cursor, image::ImageFormat::Bmp).expect("bitmap to image");
    // let (width, height) = img.dimensions();
    // println!("Width: {}, Height: {}", width, height);
    let _ = img.save("clip.png");
}