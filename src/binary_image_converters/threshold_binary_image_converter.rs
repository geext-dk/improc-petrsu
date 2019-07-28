use image::RgbImage;
use crate::image_processer::ImageProcesser;

pub struct ThresholdBinaryImageConverter {

}

impl ImageProcesser for ThresholdBinaryImageConverter {
    fn process(image: &RgbIMage) -> RgbImage {
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                let zero = <<<RgbImage as GenericImageView>::Pixel as Pixel>::Subpixel as Zero>::zero();
            }
        }
    }
}