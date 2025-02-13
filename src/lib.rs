use image::{
    imageops::{resize, FilterType},
    ImageFormat,
    ImageReader
};
use std::io::{Cursor, Write};
use napi::Error;
use napi_derive::napi;


#[napi]
pub fn read_image_buffer(path: String) -> Result<Vec<u8>, Error> {
    std::fs::read(path).map_err(|e| Error::from_reason(format!("Failed to read file: {}", e)))
}

/// Creates a thumbnail from the provided image bytes with specified dimensions
/// Returns the thumbnail as a buffer of PNG-encoded bytes
#[napi]
pub fn create_thumbnail_buffer(image_bytes: &[u8], width: u32, height: u32) -> Result<Vec<u8>, Error> {
    // Create an image reader and try to determine format
    let reader = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| Error::from_reason(format!("Failed to determine image format: {}", e)))?;

    // Decode the image into memory
    let image = reader
        .decode()
        .map_err(|e| Error::from_reason(format!("Failed to decode image: {}", e)))?;

    // Resize the image using Lanczos3 filter (good quality/performance trade-off)
    let thumbnail = resize(&image, width, height, FilterType::Lanczos3);

    // Create a buffer to hold the output PNG data
    let mut thumbnail_bytes: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    // Encode the thumbnail as PNG
    thumbnail.write_to(&mut thumbnail_bytes, ImageFormat::Png)
        .map_err(|e| Error::from_reason(format!("Failed to encode thumbnail: {}", e)))?;

    Ok(thumbnail_bytes.into_inner())
}

/// Saves the provided image buffer to a file at the specified path
#[napi]
pub fn save_image_buffer(image_bytes: &[u8], path: String) -> Result<(), Error> {
    let mut file = std::fs::File::create(&path)
        .map_err(|e| Error::from_reason(format!("Failed to create file: {}", e)))?;
    
    file.write_all(image_bytes)
        .map_err(|e| Error::from_reason(format!("Failed to write to file: {}", e)))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thumbnail_buffer() {
        // Load test image, create thumbnail, save it, verify it exists, then clean up
        let image_bytes = std::fs::read("./test.png").unwrap();
        let thumbnail = create_thumbnail_buffer(&image_bytes, 100, 100).unwrap();
        assert!(!thumbnail.is_empty());

        let test_path = "./test_thumbnail.png".to_string();
        save_image_buffer(&thumbnail, test_path).unwrap();
        let data = std::fs::read("./test_thumbnail.png").unwrap();
        assert!(!data.is_empty());
        std::fs::remove_file("./test_thumbnail.png").unwrap();
    }
}