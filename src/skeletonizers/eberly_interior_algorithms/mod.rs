use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
pub(crate) mod four_interior_algorithm;
pub(crate) mod three_interior_algorithm;
pub(crate) mod two_interior_algorithm;

pub(crate) trait EberlyInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool;
    fn remove_interiors(image: &mut BinaryImage, is_interior: &BoolMatrix);
}