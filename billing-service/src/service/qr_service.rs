use image::{DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;

pub fn generate_qr_png_bytes(data: &str) -> Result<Vec<u8>, String> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| format!("Greška pri generisanju QR koda: {}", e))?;

    let image = code.render::<Luma<u8>>().build();
    let dynamic_image = DynamicImage::ImageLuma8(image);

    let mut buffer = Cursor::new(Vec::new());
    dynamic_image
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| format!("Greška pri konverziji QR koda u PNG: {}", e))?;

    Ok(buffer.into_inner())
}