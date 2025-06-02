use image::*;
use std::io::Cursor;
use egui::IconData;

pub fn get_ico() -> IconData {
    // Image decoded
    let imgbytes = include_bytes!("assets/globe.ico");
    let reader = ImageReader::with_format(Cursor::new(imgbytes), ImageFormat::Ico);
    let ico = reader.decode().unwrap().to_rgba8();
    let (ico_width, ico_height) = ico.dimensions();

    let icon = IconData {
        rgba: ico.into_raw(),
        width: ico_width,
        height: ico_height,
    };

    icon
}
