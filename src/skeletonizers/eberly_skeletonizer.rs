use crate::skeletonizers::Skeletonizer;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
use crate::skeletonizers::eberly_interior_algorithms::EberlyInteriorAlgorithm;
use crate::skeletonizers::is_local_articulation_point;
use crate::skeletonizers::AdjacencyMode;
use crate::skeletonizers::is_in_range;
use super::eberly_interior_algorithms::{
    four_interior_algorithm::FourInteriorAlgorithm,
    three_interior_algorithm::ThreeInteriorAlgorithm,
    two_interior_algorithm::TwoInteriorAlgorithm
};


pub struct EberlySkeletonizer { }

#[derive(PartialEq, Eq)]
enum ReturnStatus {
    ExitCriteriaNotMet,
    NoMoreInteriorPixels,
    CantRemoveMoreBoundaryPixels
}

impl Skeletonizer for EberlySkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        while Self::thinning::<FourInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {
        }

        while Self::thinning::<ThreeInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {
        }
        
        while Self::thinning::<TwoInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {
        }
    }
}

impl EberlySkeletonizer {
    pub fn new() -> Self {
        EberlySkeletonizer { }
    }

    fn thinning<T: EberlyInteriorAlgorithm>(image: &mut BinaryImage) -> ReturnStatus {
        if let Some(is_interior) = Self::get_interior_matrix::<T>(image) {
            let amount_removed = Self::remove_boundaries(image, &is_interior);
            if amount_removed == 0 {
                T::remove_interiors(image, &is_interior);
                ReturnStatus::CantRemoveMoreBoundaryPixels
            } else {
                ReturnStatus::ExitCriteriaNotMet
            }
        } else {
            ReturnStatus::NoMoreInteriorPixels
        }
    }

    fn get_interior_matrix<T: EberlyInteriorAlgorithm>(image: &BinaryImage) -> Option<BoolMatrix> {
        let mut is_interior = BoolMatrix::new(image.height(), image.width(), false);
        let mut is_interior_exists = false;

        for (x, y) in image.iter() {
            if T::is_interior(image, x, y) {
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

    fn remove_boundaries(image: &mut BinaryImage, is_interior: &BoolMatrix) -> usize {
        let mut amount = 0;
        for (x, y) in image.iter() {
            if Self::is_boundary(image, x, y, is_interior) {
                image.set_white(x, y);
                amount += 1;
            }
        }

        amount
    }

    fn is_boundary(image: &BinaryImage, x: usize, y: usize, is_interior: &BoolMatrix) -> bool {
        image.is_black(x, y)
            && !is_interior.check(x, y)
            && Self::is_adjacent_to_zero(image, x, y)
            && Self::is_adjacent_to_interior(image, x, y, is_interior)
            && is_local_articulation_point(image, x, y, AdjacencyMode::Eight)
    }

    fn is_adjacent_to_zero(image: &BinaryImage, mut x: usize, mut y: usize) -> bool {
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

    fn is_adjacent_to_interior(image: &BinaryImage, mut x: usize, mut y: usize, is_interior: &BoolMatrix) -> bool {
        x -= 1;
        y -= 1;
        for i in 0..3 {
            for j in 0..3 {
                if is_in_range(x + i , 0, image.width() - 1)
                        && is_in_range(y + j, 0, image.height() - 1)
                        && is_interior.check(x + j, x + i) {
                    return true;
                }
            }
        }

        false
    }
}