// ffi.rs - FFI bindings to the library
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

use crate::PixelColor;
use image::{DynamicImage, ImageError, ImageOutputFormat, RgbImage};

use crate::{
    binary_image::BinaryImage, AdjacencyMode, BinaryImageConverter, EberlySkeletonizer,
    RosenfeldSkeletonizer, Skeletonizer, ThresholdBinaryImageConverter, ZhangSuenSkeletonizer,
};

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
pub extern "C" fn improc_petrsu_threshold_binary_image_converter_process(
    image_bytes: *mut u8,
    len: usize,
    threshold: u32,
    report_progress: unsafe extern "C" fn(i32, i32),
) -> Buffer {
    match get_rgb_image_from_raw_data(image_bytes, len) {
        Ok(mut img) => {
            let converter = ThresholdBinaryImageConverter::new(threshold);
            converter
                .convert_to_binary_with_progress(&mut img, |a, b| unsafe { report_progress(a, b) });
            rgb_image_to_raw_buffer(img)
        }
        Err(err) => {
            eprintln!("Error: {}", err.to_string());
            Buffer {
                data: std::ptr::null_mut::<u8>(),
                len: 0,
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn improc_petrsu_zhang_suen_skeletonizer_process(
    image_bytes: *const u8,
    len: usize,
    report_progress: unsafe extern "C" fn(i32, i32),
) -> Buffer {
    let skeletonizer = ZhangSuenSkeletonizer::new();

    match skeletonize(image_bytes, len, skeletonizer, |a, b| unsafe {
        report_progress(a, b)
    }) {
        Ok(img) => img,
        Err(_) => Buffer {
            data: std::ptr::null_mut::<u8>(),
            len: 0,
        },
    }
}

#[no_mangle]
pub extern "C" fn improc_petrsu_rosenfeld_skeletonizer_process(
    image_bytes: *const u8,
    len: usize,
    adjacency_mode: i32,
    report_progress: unsafe extern "C" fn(i32, i32),
) -> Buffer {
    let mode = if adjacency_mode == 0 {
        AdjacencyMode::Eight
    } else {
        AdjacencyMode::Four
    };
    let skeletonizer = RosenfeldSkeletonizer::new(mode);

    match skeletonize(image_bytes, len, skeletonizer, |a, b| unsafe {
        report_progress(a, b)
    }) {
        Ok(img) => img,
        Err(_) => Buffer {
            data: std::ptr::null_mut::<u8>(),
            len: 0,
        },
    }
}

#[no_mangle]
pub extern "C" fn improc_petrsu_eberly_skeletonizer_process(
    image_bytes: *const u8,
    len: usize,
    report_progress: unsafe extern "C" fn(i32, i32),
) -> Buffer {
    let skeletonizer = EberlySkeletonizer::new();

    match skeletonize(image_bytes, len, skeletonizer, |a, b| unsafe {
        report_progress(a, b)
    }) {
        Ok(img) => img,
        Err(_) => Buffer {
            data: std::ptr::null_mut::<u8>(),
            len: 0,
        },
    }
}

#[no_mangle]
pub extern "C" fn improc_petrsu_free(buf: Buffer) {
    let slice = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let ptr = slice.as_mut_ptr();

    unsafe {
        Box::from_raw(ptr);
    }
}

fn skeletonize<T, F>(
    image_bytes: *const u8,
    len: usize,
    skeletonizer: T,
    increment_progress: F,
) -> Result<Buffer, ImageError>
where
    T: Skeletonizer,
    F: Fn(i32, i32),
{
    let original_image = get_rgb_image_from_raw_data(image_bytes, len)?;

    let mut binary_image = BinaryImage::from_image(&original_image, PixelColor::White);

    skeletonizer.process_with_progress(&mut binary_image, increment_progress);

    let result_image = binary_image.to_rgb_image();

    Ok(rgb_image_to_raw_buffer(result_image))
}

fn get_rgb_image_from_raw_data(image_bytes: *const u8, len: usize) -> Result<RgbImage, ImageError> {
    let slice = unsafe { std::slice::from_raw_parts(image_bytes, len) };

    Ok(image::load_from_memory(slice)?.to_rgb8())
}

fn rgb_image_to_raw_buffer(image: RgbImage) -> Buffer {
    let dyn_image = DynamicImage::ImageRgb8(image);
    let mut vector = Vec::new();
    match dyn_image.write_to(&mut vector, ImageOutputFormat::Png) {
        Ok(_) => {
            let slice = vector.into_boxed_slice();
            let len = slice.len();
            let data = Box::into_raw(slice);

            Buffer {
                data: data as *mut u8,
                len,
            }
        }
        Err(_) => Buffer {
            data: std::ptr::null_mut::<u8>(),
            len: 0,
        },
    }
}
