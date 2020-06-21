// binary_image.rs - A helper struct to work with binary images
// Copyright (C) 2019 Denis Karpovskiy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use image::Pixel;
use image::RgbImage;
use image::{GenericImageView, ImageBuffer};
use num_traits::Bounded;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};

pub struct BinaryImage {
    image: Vec<Vec<PixelColor>>,
    bg_color: PixelColor,
    fg_color: PixelColor,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PixelColor {
    Black,
    White,
}

// todo: add unchecked
impl BinaryImage {
    pub fn new(width: usize, height: usize, bg: PixelColor) -> Self {
        BinaryImage {
            image: vec![vec![bg; width]; height],
            bg_color: bg,
            fg_color: if bg == PixelColor::Black {
                PixelColor::White
            } else {
                PixelColor::Black
            },
        }
    }

    pub fn from_image<P, Container>(
        image_view: &ImageBuffer<P, Container>,
        bg_color: PixelColor,
    ) -> Self
    where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]> + DerefMut,
    {
        let height = image_view.height().try_into().unwrap();
        let width = image_view.width().try_into().unwrap();

        let mut image = vec![vec![PixelColor::Black; width]; height];

        for y in 0..image_view.height() {
            for x in 0..image_view.width() {
                let pixel = image_view.get_pixel(x, y);
                let mut is_zero = true;
                for channel in pixel.channels() {
                    let channel_value: u32 = num_traits::cast(*channel).unwrap();
                    if channel_value != 0 {
                        is_zero = false;
                        break;
                    }
                }

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
            fg_color,
        }
    }

    pub fn to_rgb_image(&self) -> RgbImage {
        let mut rgb_image = RgbImage::new(self.width() as u32, self.height() as u32);

        for (x, y) in self.pixels_iter() {
            if self.is_bg(x, y) {
                let pixel = rgb_image.get_pixel_mut(x as u32, y as u32);
                let max_value = <<<RgbImage as GenericImageView>::Pixel as Pixel>::Subpixel as Bounded>::max_value();

                pixel.apply_with_alpha(|_| max_value, |c| c);
            }
        }

        rgb_image
    }

    pub fn pixels_iter(&self) -> PixelIterator {
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
        for (x, y) in self.pixels_iter() {
            self.set_color(x, y, color);
        }
    }
}

pub struct PixelIterator {
    current_x: usize,
    current_y: usize,
    width: usize,
    height: usize,
}

impl PixelIterator {
    pub fn new(image: &BinaryImage) -> PixelIterator {
        PixelIterator {
            current_x: 0,
            current_y: 0,
            width: image.width(),
            height: image.height(),
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
