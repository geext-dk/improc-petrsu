use super::EberlyInteriorAlgorithm;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
use crate::skeletonizers::is_local_articulation_point;
use crate::skeletonizers::AdjacencyMode;

pub(crate) struct TwoInteriorAlgorithm { }

impl EberlyInteriorAlgorithm for TwoInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool {
        if image.is_black(x, y) {
            let mut horizontal_black_count = 0;
            let mut vertical_black_count = 0;

            if y != 0 && image.is_black(x, y - 1) {
                vertical_black_count += 1;
            }

            if x != image.width() - 1 && image.is_black(x + 1, y) {
                horizontal_black_count += 1;
            }

            if y != image.height() - 1 && image.is_black(x, y + 1) {
                vertical_black_count += 1
            }

            if x != 0 && image.is_black(x - 1, y) {
                horizontal_black_count += 1;
            }

            if horizontal_black_count == 1 && vertical_black_count == 1 {
                true
            } else { 
                false
            }
        } else {
            false
        }
    }

    fn remove_interiors(image: &mut BinaryImage, is_interior: &BoolMatrix) {
        for (x, y) in image.iter() {
            if is_interior.check(x, y) && !is_local_articulation_point(image, x, y, AdjacencyMode::Eight) {
                image.set_white(x, y);
            }
        }
    }
}