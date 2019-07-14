use image::GenericImageView;
use image::Pixel;
use image::RgbImage;
use num_traits::Zero;
use num_traits::Bounded;
use std::convert::TryInto;

pub struct BinaryImage {
    image: Vec<Vec<PixelColor>>
}

#[derive(Copy, Clone, PartialEq)]
pub enum PixelColor {
    Black,
    White
}

impl BinaryImage {
    pub fn to_rgb_image(&self) -> RgbImage {
        let mut rgb_image = RgbImage::new(self.width() as u32, self.height() as u32);

        for (x, y) in self.iter() {
            if self.is_white(x, y) {
                let pixel = rgb_image.get_pixel_mut(x as u32, y as u32);
                let max_value = <<<RgbImage as GenericImageView>::Pixel as Pixel>::Subpixel as Bounded>::max_value();

                pixel.apply_with_alpha(|_| max_value, |c| c);
            }
        }

        rgb_image
    }

    pub fn iter(&self) -> PixelIterator {
        PixelIterator::new(self)
    }

    pub fn width(&self) -> usize {
        self.image[0].len()
    }

    pub fn height(&self) -> usize {
        self.image.len()
    }

    pub fn is_black(&self, x: usize, y: usize) -> bool {
        self.image[y][x] == PixelColor::Black
    }

    pub fn is_white(&self, x: usize, y: usize) -> bool {
        self.image[y][x] == PixelColor::White
    }

    pub fn set_black(&mut self, x: usize, y: usize) {
        self.image[y][x] = PixelColor::Black;
    }

    pub fn set_white(&mut self, x: usize, y: usize) {
        self.image[y][x] = PixelColor::White;
    }

    pub fn new(width: usize, height: usize) -> Self {
        BinaryImage {
            image: vec![vec![PixelColor::Black; width]; height]
        }
    }

    pub fn from_image<T: GenericImageView>(image_view: &T) -> Self {
        let height = image_view.height().try_into().unwrap();
        let width = image_view.width().try_into().unwrap();

        let mut image = vec![vec![PixelColor::Black; width]; height];

        for y in 0..image_view.height() {
            for x in 0..image_view.width() {
                let mut pixel = image_view.get_pixel(x, y);
                let zero = <<T::Pixel as Pixel>::Subpixel as Zero>::zero();
                let mut is_zero = true;
                pixel.apply_with_alpha(|c| {
                    if c != zero {
                        is_zero = false;
                    }
                    c
                }, |c| c);
                
                if !is_zero {
                    image[y as usize][x as usize] = PixelColor::White;
                }
            }
        }

        BinaryImage {
            image
        }
    }
}

pub struct PixelIterator {
    current_x: usize,
    current_y: usize,
    width: usize,
    height: usize
}

impl PixelIterator {
    pub fn new(image: &BinaryImage) -> PixelIterator {
        PixelIterator {
            current_x: 0,
            current_y: 0,
            width: image.width(),
            height: image.height()
        }
    }
}

impl Iterator for PixelIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x == self.width {
            if self.current_y >= self.height - 1 {
                return Option::None;
            } else {
                self.current_y += 1;
                self.current_x = 0;
            }
        }
        
        let ret = (self.current_x, self.current_y);
        self.current_x += 1;

        Option::Some(ret)   
    }
}