pub mod threshold_binary_image_converter;
use image::RgbImage;

pub trait BinaryImageConverter {
    fn convert_to_binary(&self, image: &mut RgbImage);
}