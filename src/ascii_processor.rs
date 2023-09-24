
use image::{DynamicImage, GenericImageView};

use super::log;

use std::cmp::max;


enum Format {
    OneOne,
    FourThirds,
    SixteenNinths,
    SixteenTenths,
}

pub fn generate_ascii(
    image: DynamicImage,
    rate: f32,
    characters: String,
    mut buffer: &mut Vec<u8>,
    mut buffer_size: &mut Vec<u32>,
) {
    let (width, height) = image.dimensions();
    let (format, new_W, new_H) = get_format(rate, width, height);
    let width_step = (width / (width - new_W)) as usize;
    let height_step = (height / (height - new_H)) as usize;

    let mut x_max: u32 = 0;
    let mut y_max: u32 = 0;

    for y in (0..height).step_by(height_step) {
        for x in (0..width).step_by(width_step) {
            let element = get_String(
                image.get_pixel(x, y),
                characters.chars().collect::<Vec<char>>().as_ref(),
            );

            buffer.append(element.into_bytes().as_mut());

            if y == 0 {
                x_max += 1;
            }

            if x == (width - width_step as u32) {
                buffer.append(String::from("\n").into_bytes().as_mut());
            }
        }

        y_max += 1;
    }

    buffer_size[0] = x_max;
    buffer_size[1] = y_max;
}

fn get_format(rate: f32, width: u32, height: u32) -> (Format, u32, u32) {
    let ratio = width as f32 / height as f32;
    let new_W = (width as f32 / 100.0 * rate).round() as u32;
    let new_H = (height as f32 / 100.0 * rate).round() as u32;
    match ratio {
        _ if ratio == 1.0 => (Format::OneOne, new_W, new_H),
        _ if ratio == (4.0 / 3.0) => (Format::FourThirds, new_W, new_H),
        _ if ratio == (16.0 / 9.0) => (Format::SixteenNinths, new_W, new_H),
        _ if ratio == (16.0 / 10.0) => (Format::SixteenTenths, new_W, new_H),
        _ => panic!("image format not recognized"),
    }
}

fn get_String(pixel: image::Rgba<u8>, characters: &Vec<char>) -> String {
    let intent = if pixel[3] == 0 {
        0
    } else {
        pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
    };

    let ch = characters[(intent / (32 + 7 - (7 + (characters.len() - 7)) as u8)) as usize];

    String::from(ch)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_generate_ascii() {
        let testimg =
            image::open("./effect_broken_rust.png").expect("Failed to create image from raw data");

        let characters = " .,-~!;:=*&%$@#".to_string();
        //characters: " .:-=+░▒▓▓".to_string(),

        let mut buffer: Vec<u8> = Vec::new();
        let mut buffer_size: Vec<u32> = vec![0 as u32; 2];

        generate_ascii_new(testimg, characters, &mut buffer, &mut buffer_size);

        //buffer.iter().for_each(|&c| print!("{}", c as char));
        print!(
            "{}",
            String::from_utf8(buffer).expect("cannot convert buffer to string")
        );
    }
}
