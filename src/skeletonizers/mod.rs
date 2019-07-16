pub mod rosenfeld_skeletonizer;
pub mod eberly_skeletonizer;
pub mod zhangsuen_skeletonizer;
use crate::binary_image::BinaryImage;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AdjacencyMode {
    Four,
    Eight
}

pub trait Skeletonizer {
    fn process(&self, binary_image: &mut BinaryImage);
}