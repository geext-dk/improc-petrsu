use image::GenericImageView;
use image::Pixel;
use image::RgbImage;
use num_traits::Zero;
use num_traits::Bounded;
use std::convert::TryInto;

pub struct BinaryImage {
    image: Vec<Vec<PixelColor>>,
    bg_color: PixelColor,
    fg_color: PixelColor
}

#[derive(Copy, Clone, PartialEq)]
pub enum PixelColor {
    Black,
    White
}

// todo: add unchecked 
impl BinaryImage {
    pub fn to_rgb_image(&self) -> RgbImage {
        let mut rgb_image = RgbImage::new(self.width() as u32, self.height() as u32);

        for (x, y) in self.iter() {
            if self.is_bg(x, y) {
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

    pub fn get_bg_color(&self) -> PixelColor {
        self.bg_color
    }

    pub fn get_fg_color(&self) -> PixelColor {
        self.fg_color
    }

    pub fn is_fg(&self, x: usize, y: usize) -> bool {
        self.get_color(x, y) == self.fg_color
    }

    pub fn is_bg(&self, x: usize, y: usize) -> bool {
        self.get_color(x, y) == self.bg_color
    }

    pub fn set_fg(&mut self, x: usize, y: usize) {
        self.set_color(x, y, self.fg_color);
    }

    pub fn set_bg(&mut self, x: usize, y: usize) {
        self.set_color(x, y, self.bg_color);
    }

    fn set_color(&mut self, x: usize, y: usize, color: PixelColor) {
        self.image[y][x] = color;
    }

    fn get_color(&self, x: usize, y: usize) -> PixelColor {
        self.image[y][x]
    }

    pub fn fill(&mut self, color: PixelColor) {
        for (x, y) in self.iter() {
            self.set_color(x, y, color);
        }
    }

    pub fn new(width: usize, height: usize, bg: PixelColor) -> Self {
        BinaryImage {
            image: vec![vec![bg; width]; height],
            bg_color: bg,
            fg_color: if bg == PixelColor::Black {
                    PixelColor::White
                } else {
                    PixelColor::Black
                }
        }
    }

    pub fn from_image<T: GenericImageView>(image_view: &T, bg_color: PixelColor) -> Self {
        let height = image_view.height().try_into().unwrap();
        let width = image_view.width().try_into().unwrap();

        let mut image = vec![vec![PixelColor::Black; width]; height];

        for y in 0..image_view.height() {
            for x in 0..image_view.width() {
                let mut pixel = image_view.get_pixel(x, y);
                let zero = <<T::Pixel as Pixel>::Subpixel as Zero>::zero();
                let mut is_zero = true;
                pixel.apply_without_alpha(|c| {
                    if c != zero {
                        is_zero = false;
                    }
                    c
                });
                
                if !is_zero {
                    image[y as usize][x as usize] = PixelColor::White;
                }
            }
        }

        let fg_color = if bg_color == PixelColor::Black {
            PixelColor::White
        } else {
            PixelColor::Black
        };

        BinaryImage {
            image,
            bg_color,
            fg_color
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