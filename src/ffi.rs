use image::{ RgbImage, ImageError };
use crate::PixelColor;

use crate::{
    binary_image::BinaryImage,
    skeletonizers::{
        zhangsuen_skeletonizer::ZhangSuenSkeletonizer,
        Skeletonizer
    }
};

#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize
}

#[no_mangle]
pub extern fn improc_petrsu_zhang_suen_skeletonization(image_bytes: *const u8, len: usize) -> Buffer {
    let skeletonizer = ZhangSuenSkeletonizer::new();

    let result_image = match skeletonize(image_bytes, len, skeletonizer) {
        Ok(img) => img,
        Err(_) => return Buffer {
            data: 0 as *mut u8,
            len: 0
        }
    };

    let mut slice = result_image.into_raw().into_boxed_slice();
    let data = slice.as_mut_ptr();
    let len = slice.len();

    std::mem::forget(slice);

    Buffer {
        data,
        len
    }
}

#[no_mangle]
pub extern fn improc_petrsu_free(buf: Buffer) {
    let slice = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    let ptr = slice.as_mut_ptr();

    unsafe {
        Box::from_raw(ptr);
    }
}

fn skeletonize<T: Skeletonizer>(image_bytes: *const u8, len: usize, skeletonizer: T) -> Result<RgbImage, ImageError>  {
    let slice = unsafe { std::slice::from_raw_parts(image_bytes, len) };

    let original_image = image::load_from_memory(slice)?.to_rgb();

    let mut binary_image = BinaryImage::from_rgb_image(&original_image, PixelColor::White);

    skeletonizer.process(&mut binary_image);

    Ok(binary_image.to_rgb_image())
}