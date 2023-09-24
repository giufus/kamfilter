mod ascii_processor;
mod utils;

use image::{DynamicImage, ImageBuffer, Rgba};

use wasm_bindgen::prelude::*;

use std::vec;

use crate::ascii_processor::generate_ascii;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn process() -> String {
    "HELLO KAMFILTER!".to_string()
}

#[wasm_bindgen]
pub struct FilteredImage {
    width: u32,
    height: u32,
    cells: Vec<u8>,
    chars: Vec<u8>,
    chars_size: Vec<u32>,
}

#[wasm_bindgen]
impl FilteredImage {
    pub fn render(&self) -> String {
        todo!()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn chars(&self) -> *const u8 {
        self.chars.as_ptr()
    }

    pub fn chars_size(&self) -> *const u32 {
        self.chars_size.as_ptr()
    }

    pub fn chars_length(&self) -> u32 {
        self.chars.len() as u32
    }

    pub fn new(width: u32, height: u32) -> Self {
        FilteredImage {
            width,
            height,
            cells: vec![0 as u8; (width * height * 4) as usize],
            chars: vec![],
            chars_size: vec![0 as u32; 2],
        }
    }

    pub fn art(&mut self, _array: &[u8]) {
        let mut dyn_img =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
                .map(|i| DynamicImage::ImageRgba8(i))
                .expect("Failed to create image from raw data");

        let characters = " .:-=+*#%@".to_string();

        generate_ascii(dyn_img, 5.0 as f32, characters, &mut self.chars, &mut self.chars_size);
    }

    pub fn edge_detection_1(&mut self, _array: &[u8]) {
        let from_raw =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
                .expect("cannot read image");

        let detection = edge_detection::canny(
            DynamicImage::ImageRgba8(from_raw).into_luma8(),
            2.0,  // sigma
            0.1,  // strong threshold
            0.01, // weak threshold
        );

        //self.cells = _array.to_vec();
        self.cells = detection.as_image().to_rgba8().into_raw();
    }

    pub fn invert(&mut self, _array: &[u8]) {
        let mut dyn_img =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
                .map(|i| DynamicImage::ImageRgba8(i))
                .expect("Failed to create image from raw data");

        dyn_img.invert();

        //self.cells = _array.to_vec();
        self.cells = dyn_img.to_rgba8().into_raw();
    }
}



fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::io::Write;
    use std::{array, fs::File};

    #[test]
    pub fn test_Image_is_processed() {

    }
}
