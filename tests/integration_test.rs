use std::io::Cursor;

use http_cat::HttpCat;
use image::ImageFormat;

#[test]
fn status_to_code() {
    assert_eq!(HttpCat::ImATeapot as u16, 418);
}

#[tokio::test]
async fn image() {
    let mut test_image = image::io::Reader::new(Cursor::new(include_bytes!("418.jpeg")));

    test_image.set_format(ImageFormat::Jpeg);

    assert_eq!(
        HttpCat::ImATeapot.get().await.unwrap(),
        test_image.decode().unwrap(),
    );
}
