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
use image::{ImageBuffer, Pixel};
use std::ops::{Deref, DerefMut};

pub trait BinaryImageConverter {
    fn convert_to_binary<P, Container>(&self, image: &mut ImageBuffer<P, Container>)
    where
        P: Pixel + 'static,
        P::Subpixel: 'static,
        Container: Deref<Target = [P::Subpixel]> + DerefMut;
}
