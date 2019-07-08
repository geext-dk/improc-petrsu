pub mod rosenfeld_skeletonizer;
pub mod eberly_skeletonizer;
pub mod zhangsuen_skeletonizer;

pub trait Skeletonizer {
    fn process();
}