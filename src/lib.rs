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
        FilteredImage { width, height, cells: vec![0; (width * height) as usize] }
    }

    pub fn fill_cells(&mut self, _array: &[u8]) {

        let mut dyn_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, _array.to_vec())
            .map(|i|DynamicImage::ImageRgba8(i))
            .expect("Failed to create image from raw data");
        
        let modified = dyn_img.adjust_contrast(50.0);

        //self.cells = _array.to_vec();
        self.cells = dyn_img.to_rgba8().to_vec();
        
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
