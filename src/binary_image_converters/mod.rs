// binary_image_converters/mod.rs - Binary image converters
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

pub mod threshold_binary_image_converter;
use image::{GenericImage, Pixel};

pub trait BinaryImageConverter {
    fn convert_to_binary<Img, Pix>(&self, image: &mut Img)
    where
        Pix: Pixel,
        Img: GenericImage<Pixel = Pix>;
    fn convert_to_binary_with_progress<Img, Pix, F>(&self, image: &mut Img, report_progress: F)
    where
        Img: GenericImage<Pixel = Pix>,
        Pix: Pixel,
        F: Fn(i32, i32);
}
