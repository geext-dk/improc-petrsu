// lib.rs - The main entry point to the library
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

mod binary_image;
pub mod binary_image_converters;
mod bool_matrix;
pub mod ffi;
pub mod skeletonizers;

// re-exports
pub use skeletonizers::eberly_skeletonizer::EberlySkeletonizer;
pub use skeletonizers::rosenfeld_skeletonizer::RosenfeldSkeletonizer;
pub use skeletonizers::zhangsuen_skeletonizer::ZhangSuenSkeletonizer;
pub use skeletonizers::Skeletonizer;

pub use binary_image::BinaryImage;
pub use binary_image::PixelColor;
pub use binary_image_converters::threshold_binary_image_converter::ThresholdBinaryImageConverter;
pub use binary_image_converters::BinaryImageConverter;
pub use skeletonizers::AdjacencyMode;
