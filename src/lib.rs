use image::{
    imageops::{resize, FilterType},
    ImageFormat,
    ImageReader
};
use std::io::{Cursor, Write};

pub fn create_thumbnail_buffer(image_bytes: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| format!("Failed to determine image format: {}", e))?;

    let image = reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let thumbnail = resize(&image, width, height, FilterType::Lanczos3);

    // Use Cursor<Vec<u8>> to allow seeking
    let mut thumbnail_bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    thumbnail.write_to(&mut thumbnail_bytes, ImageFormat::Png) // Or ImageOutputFormat::Jpeg
        .map_err(|e| format!("Failed to encode thumbnail: {}", e))?;

    Ok(thumbnail_bytes.into_inner()) // Get the Vec<u8> back from the Cursor
}

pub fn save_image_buffer(image_bytes: &[u8], path: &str) -> Result<(), String> {
    let mut file = std::fs::File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(image_bytes).map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thumbnail_buffer() {
        let image_bytes = std::fs::read("./test.png").unwrap();
        let thumbnail = create_thumbnail_buffer(&image_bytes, 100, 100).unwrap();
        assert!(!thumbnail.is_empty());

        let test_path = "./test_thumbnail.png";

        save_image_buffer(&thumbnail, test_path).unwrap();

        let data = std::fs::read(test_path).unwrap();
        
        assert!(!data.is_empty());

        std::fs::remove_file(test_path).unwrap();
    }
}
