pub mod skeletonizers;
mod bool_matrix;
mod binary_image;
pub mod ffi;

// re-exports
pub use skeletonizers::eberly_skeletonizer::EberlySkeletonizer;
pub use skeletonizers::rosenfeld_skeletonizer::RosenfeldSkeletonizer;
pub use skeletonizers::zhangsuen_skeletonizer::ZhangSuenSkeletonizer;
pub use binary_image::BinaryImage;
pub use skeletonizers::Skeletonizer;
pub use skeletonizers::AdjacencyMode;
pub use binary_image::PixelColor;