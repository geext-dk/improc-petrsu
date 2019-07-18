use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
pub(super) mod four_interior_algorithm;

pub(super) trait EberlyInteriorAlgorithm {
    fn is_interior(&self, image: &BinaryImage, x: usize, y: usize) -> bool;
    fn remove_interiors(&self, image: &mut BinaryImage, is_interior: &BoolMatrix);
}