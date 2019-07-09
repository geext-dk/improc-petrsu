use image::DynamicImage;
use std::path::Path;

pub trait ImageProcesser {
    fn process(image: &DynamicImage);
}