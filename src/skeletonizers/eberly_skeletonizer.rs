use crate::skeletonizers::Skeletonizer;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
use crate::skeletonizers::eberly_interior_algorithms::EberlyInteriorAlgorithm;
use crate::skeletonizers::is_local_articulation_point;
use crate::skeletonizers::AdjacencyMode;

pub struct EberlySkeletonizer { }

#[derive(PartialEq, Eq)]
enum ReturnStatus {
    ExitCriteriaNotMet,
    NoMoreInteriorPixels,
    CantRemoveMoreBoundaryPixels
}

impl Skeletonizer for EberlySkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        // let large
        // while (self.thinning(image, algorithm: impl EberlyInteriorAlgorithm))
        // loop {
        //     let status = self.thinning(image);

        //     if status != ReturnStatus::ExitCriteriaNotMet {
        //         break;
        //     }
        // }
    }
}

impl EberlySkeletonizer {
    pub fn new() -> Self {
        EberlySkeletonizer { }
    }

    fn thinning(&self, image: &mut BinaryImage, algorithm: impl EberlyInteriorAlgorithm) -> ReturnStatus {
        if let Some(is_interior) = self.get_interior_matrix(image, &algorithm) {
            let amount_removed = self.remove_boundaries(image, &is_interior);
            if amount_removed == 0 {
                algorithm.remove_interiors(image, &is_interior);
                ReturnStatus::CantRemoveMoreBoundaryPixels
            } else {
                ReturnStatus::ExitCriteriaNotMet
            }
        } else {
            ReturnStatus::NoMoreInteriorPixels
        }
    }

    fn get_interior_matrix(&self, image: &BinaryImage, algorithm: &impl EberlyInteriorAlgorithm) -> Option<BoolMatrix> {
        let mut is_interior = BoolMatrix::new(image.height(), image.width(), false);
        let mut is_interior_exists = false;

        for (x, y) in image.iter() {
            if algorithm.is_interior(image, x, y) {
                is_interior.set(x, y);
                is_interior_exists = true;
            }
        }

        if is_interior_exists {
            Some(is_interior)
        } else {
            None
        }
    }

    fn remove_boundaries(&self, image: &mut BinaryImage, is_interior: &BoolMatrix) -> usize {
        let mut amount = 0;
        for (x, y) in image.iter() {
            if self.is_boundary(image, x, y, is_interior) {
                image.set_white(x, y);
                amount += 1;
            }
        }

        amount
    }

    fn is_boundary(&self, image: &BinaryImage, x: usize, y: usize, is_interior: &BoolMatrix) -> bool {
        image.is_black(x, y)
            && !is_interior.check(x, y)
            && self.is_adjacent_to_zero(image, x, y)
            && self.is_adjacent_to_interior(image, x, y, is_interior)
            && is_local_articulation_point(image, x, y, AdjacencyMode::Eight)
    }

    fn is_adjacent_to_zero(&self, image: &BinaryImage, mut x: usize, mut y: usize) -> bool {
        if x >= image.width() - 1 || y >= image.height() - 1 {
            return true;
        }

        x -= 1;
        y -= 1;

        for i in 0..9 {
            if image.is_white(x + i % 3, y + i / 3) { 
                return true;
            }
        }

        false
    }

    fn is_adjacent_to_interior(&self, image: &BinaryImage, mut x: usize, mut y: usize, is_interior: &BoolMatrix) -> bool {
        x -= 1;
        y -= 1;
        for i in 0..3 {
            for j in 0..3 {
                if self.is_in_range(x + i , 0, image.width() - 1)
                        && self.is_in_range(y + j, 0, image.height() - 1)
                        && is_interior.check(x + j, x + i) {
                    return true;
                }
            }
        }

        false
    }

    fn is_in_range(&self, value: usize, left: usize, right: usize) -> bool {
        value >= left && value <= right
    }
}