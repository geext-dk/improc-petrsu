use image::GenericImageView;
use image::Pixel;
use num_traits::Zero;
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
                let pixel = image_view.get_pixel(x, y);
                let channels = pixel.channels();

                let zero = &<<T::Pixel as Pixel>::Subpixel as Zero>::zero();
                for channel in channels {
                    if channel != zero {
                        image[y as usize][x as usize] = PixelColor::White;
                        break;
                    }
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