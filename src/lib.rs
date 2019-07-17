pub mod skeletonizers;
pub mod bool_matrix;
// pub mod binary_image_converters;
pub mod binary_image;
pub mod ffi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _x = crate::binary_image::BinaryImage::new(3, 3);
        assert_eq!(2 + 2, 4);
    }
}
