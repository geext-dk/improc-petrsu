use crate::skeletonizers::Skeletonizer;
use crate::binary_image::BinaryImage;
use crate::skeletonizers::eberly_interior_algorithms::EberlyInteriorAlgorithm;

pub struct EberlySkeletonizer { }

#[derive(PartialEq, Eq)]
enum ReturnStatus {
    ExitCriteriaNotMet,
    NoMoreInteriorPixels,
    CantRemoveMoreBoundaryPixels
}

impl Skeletonizer for EberlySkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        loop {
            let status = self.large_scale_thinning(image);
            if status == 
        }
    }
}

impl EberlySkeletonizer {
    pub fn new() -> Self {
        EberlySkeletonizer { }
    }

    fn thinning(&self, image: &mut BinaryImage, algorithm: impl EberlyInteriorAlgorithm) -> ReturnStatus {
        if let Some(is_interior) = self.get_interior_matrix(image, algorithm) {
            
        }

    }

    fn get_interior_matrix(&self, image: &BinaryImage, T: impl EberlyInteriorAlgorithm) -> Option<BoolMatrix> {

    }
}