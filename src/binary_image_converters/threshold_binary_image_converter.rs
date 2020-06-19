// threshold_binary_image_converter.rs - Converts to binary by threshold
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

use super::BinaryImageConverter;
use image::{Pixel, ImageBuffer};
use num_traits::{Bounded, Zero};
use std::ops::{Deref, DerefMut};

pub struct ThresholdBinaryImageConverter {
    threshold: u32,
}

impl ThresholdBinaryImageConverter {
    pub fn new(threshold: u32) -> Self {
        ThresholdBinaryImageConverter { threshold }
    }
}

impl BinaryImageConverter for ThresholdBinaryImageConverter {
    fn convert_to_binary<P, Container>(&self, image: &mut ImageBuffer<P, Container>)
        where P: Pixel + 'static,
              P::Subpixel : 'static,
              Container: Deref<Target = [P::Subpixel]> + DerefMut {
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel_mut(x, y);

                let mut is_zero = true;
                for c in pixel.channels() {
                    let pixel_value: u32 = num_traits::cast(c.clone()).unwrap();
                    if pixel_value > self.threshold {
                        is_zero = false;
                        break;
                    }
                }

                let max = <P::Subpixel as Bounded>::max_value();
                let zero =
                    <P::Subpixel as Zero>::zero();
                if is_zero {
                    pixel.apply(|_| zero);
                } else {
                    pixel.apply(|_| max);
                }
            }
        }
    }
}