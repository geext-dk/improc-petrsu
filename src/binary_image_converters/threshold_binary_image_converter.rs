use image::{ RgbImage, GenericImageView, Pixel };
use num_traits::{ Zero, Bounded };
use super::BinaryImageConverter;

pub struct ThresholdBinaryImageConverter {
    threshold: u8
}

impl ThresholdBinaryImageConverter {
    pub fn new(threshold: u8) -> Self {
        ThresholdBinaryImageConverter {
            threshold
        }
    }
}

impl BinaryImageConverter for ThresholdBinaryImageConverter {
    fn convert_to_binary(&self, image: &mut RgbImage) {
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel_mut(x, y);

                let mut is_zero = true;
                for c in pixel.channels() {
                    if *c > self.threshold {
                        is_zero = false;
                        break;
                    }
                }

                let max = <<<RgbImage as GenericImageView>::Pixel as Pixel>::Subpixel as Bounded>::max_value();
                let zero = <<<RgbImage as GenericImageView>::Pixel as Pixel>::Subpixel as Zero>::zero();
                if is_zero {
                    pixel.apply(|_| { zero });
                } else {
                    pixel.apply(|_| { max });
                }
            }
        }
    }
}