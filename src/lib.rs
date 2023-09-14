mod utils;

use image::{DynamicImage, ImageBuffer, Rgba};

use wasm_bindgen::prelude::*;

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
    

    pub fn new(width: u32, height: u32) -> Self {
        FilteredImage { width, height, cells: vec![0 as u8; (width * height * 4) as usize] }
    }

    pub fn edge_detection_1(&mut self, _array: &[u8]) {

        let from_raw = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
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

        let mut dyn_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
            .map(|i|DynamicImage::ImageRgba8(i))
            .expect("Failed to create image from raw data");
        
        dyn_img.invert();

        //self.cells = _array.to_vec();
        self.cells = dyn_img.to_rgba8().into_raw();
        
    }
}

#[wasm_bindgen] 
pub fn blur_image_and_draw_from_js(_array: &[u8], width: u32, height: u32) -> Vec<u8> {
    //log("blur_image_and_render_from_html called"); // you will see this log in browser console
    // convert array to image
    let input_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, _array.to_vec())
        .map(DynamicImage::ImageRgba8)
        .expect("Failed to create image from raw data");

    
    let blurred_image = input_image.blur(3.0); // blur function is from image crate
    
    if let DynamicImage::ImageRgba8(blurred_rgba_image) = blurred_image {
        blurred_rgba_image.into_raw() // convert blurred image to array and return
    } else {
        panic!("Unexpected image format.");
    }
    
}


fn print_type_of<T>(_: &T) -> String{
    format!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {

    use std::{fs::{File}, array};
    use std::io::Write;
    use super::*;


    #[test]
    pub fn test_Image_is_processed() {
        
    }
}