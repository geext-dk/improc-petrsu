use super::EberlyInteriorAlgorithm;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;

pub(super) struct FourInteriorAlgorithm { }

impl EberlyInteriorAlgorithm for FourInteriorAlgorithm {
    fn is_interior(&self, image: &BinaryImage, mut x: usize, mut y: usize) -> bool {
        if x >= image.width() - 1 || y >= image.height() - 1  || x == 0 || y == 0 { 
            false
        } else {
            x -= 1;
            y -= 1;

            let x_offset = [1, 2, 0, 1, 1];
            let y_offset = [1, 1, 1, 2, 0];

            for i in 0..x_offset.len() {
                if image.is_white(x + x_offset[i], y + y_offset[i]) {
                    return false;
                }
            }

            true
        }
    }

    fn remove_interiors(&self, image: &mut BinaryImage, is_interior: &BoolMatrix) {
        // nothing to do
    }
}