pub mod rosenfeld_skeletonizer;
pub mod eberly_skeletonizer;
pub mod zhangsuen_skeletonizer;
use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AdjacencyMode {
    Four,
    Eight
}

pub trait Skeletonizer {
    fn process(&self, binary_image: &mut BinaryImage);
}

fn is_local_articulation_point(image: &BinaryImage, x: usize, y: usize, mode: &AdjacencyMode) -> bool {

}

fn count_components(image: &BinaryImage, mode: &AdjacencyMode) -> u32 {
    let amount = 0;
    let is_checked = BoolMatrix::new(image.width(), image.height(), false);
    let pixels_stack = Vec::new();

    for (x, y) in image.iter() {
        if image.is_white(x, y) || is_checked.check(x, y) {
            continue;
        }

        amount += 1;
        pixels_stack.push((x, y));
        while !pixels_stack.is_empty() {
            let (next_x, next_y) = pixels_stack.pop().unwrap();
            
        }
    }

    amount
}