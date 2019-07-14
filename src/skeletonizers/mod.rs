pub mod rosenfeld_skeletonizer;
pub mod eberly_skeletonizer;
pub mod zhangsuen_skeletonizer;
use crate::binary_image::BinaryImage;

pub trait Skeletonizer {
    fn process(binary_image: &mut BinaryImage);
}