use super::EberlyInteriorAlgorithm;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
use crate::skeletonizers::is_in_range;
use crate::skeletonizers::is_local_articulation_point;
use crate::skeletonizers::AdjacencyMode;

pub(crate) struct ThreeInteriorAlgorithm { }

impl EberlyInteriorAlgorithm for ThreeInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool {
        image.is_black(x, y) && Self::count_black_neighbours(image, x, y) == 3
    }

    fn remove_interiors(image: &mut BinaryImage, is_interior: &BoolMatrix) {
        for (x, y) in image.iter() {
            if is_interior.check(x, y) && !is_local_articulation_point(image, x, y, AdjacencyMode::Eight) {
                image.set_white(x, y);
            }
        }
    }
}

impl ThreeInteriorAlgorithm {
    fn count_black_neighbours(image: &BinaryImage, x: usize, y: usize) -> usize {
        let mut count = 0;
        let x_offsets = [1, 2, 1, 0];
        let y_offsets = [0, 1, 2, 1];

        for i in 0..x_offsets.len() {
            let new_x = x + x_offsets[i] - 1;
            let new_y = y + y_offsets[i] - 1;

            if is_in_range(new_x, 0, image.width() - 1)
                    && is_in_range(new_y, 0, image.height() - 1)
                    && image.is_black(new_x, new_y) {
                count += 1;
            }
        }

        count
    }
}