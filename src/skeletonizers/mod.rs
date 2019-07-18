mod eberly_interior_algorithms;
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

fn is_local_articulation_point(image: &BinaryImage, x: usize, y: usize, mode: AdjacencyMode) -> bool {
    let mut around = BinaryImage::new(3, 3);
    fill_around(&mut around, image, x, y);

    let components = count_components(&around, mode);

    around.set_white(1, 1);

    components != count_components(&around, mode)
}

fn fill_around(around: &mut BinaryImage, image: &BinaryImage, mut x: usize, mut y: usize) {
    x -= 1;
    y -= 1;
    for i in 0..9 {
        let delta_x = i % 3;
        let delta_y = i / 3;
        let new_x = x + delta_x;
        let new_y = y + delta_y;

        if new_x > image.width() - 1 || new_y > image.height() - 1 {
            around.set_black(delta_x, delta_y);
        } else {
            if image.is_black(new_x, new_y) {
                around.set_black(delta_x, delta_y);
            } else {
                around.set_white(delta_x, delta_y);
            }
        }
    }
}

fn count_components(image: &BinaryImage, mode: AdjacencyMode) -> u32 {
    let mut amount = 0;
    let mut is_checked = BoolMatrix::new(image.width(), image.height(), false);
    let mut pixels_stack = Vec::new();

    for (x, y) in image.iter() {
        if image.is_white(x, y) || is_checked.check(x, y) {
            continue;
        }

        amount += 1;
        pixels_stack.push((x, y));
        while !pixels_stack.is_empty() {
            let (next_x, next_y) = pixels_stack.pop().unwrap();
            
            is_checked.set(next_x, next_y);

            // TODO: make this look not like shit and optimize
            if next_x != 0 && image.is_black(next_x - 1, next_y) && !is_checked.check(next_x - 1, next_y) {
                pixels_stack.push((next_x - 1, next_y));
            }

            if next_x != image.width() - 1 && image.is_black(next_x + 1, next_y) && !is_checked.check(next_x + 1, next_y) {
                pixels_stack.push((next_x + 1, next_y));
            }

            if next_y != 0 && image.is_black(next_x, next_y - 1) && !is_checked.check(next_x, next_y - 1) {
                pixels_stack.push((next_x, next_y - 1));
            }

            if next_y != image.height() - 1 && image.is_black(next_x, next_y + 1) && !is_checked.check(next_x, next_y + 1) {
                pixels_stack.push((next_x, next_y + 1));
            }

            if mode == AdjacencyMode::Eight {
                if next_x != 0 && next_y != 0 && image.is_black(next_x - 1, next_y - 1) && !is_checked.check(next_x - 1, next_y - 1) {
                    pixels_stack.push((next_x - 1, next_y - 1));
                }

                if next_x != image.width() - 1 && next_y != 0 && image.is_black(next_x + 1, next_y - 1) && !is_checked.check(next_x + 1, next_y - 1) {
                    pixels_stack.push((next_x + 1, next_y - 1));
                }

                if next_x != 0 && next_y != image.height() - 1 && image.is_black(next_x - 1, next_y + 1) && !is_checked.check(next_x - 1, next_y + 1) {
                    pixels_stack.push((next_x - 1, next_y + 1));
                }

                if next_x != image.width() - 1 && next_y != image.height() - 1 && image.is_black(next_x + 1, next_y + 1) && !is_checked.check(next_x + 1, next_y + 1) {
                    pixels_stack.push((next_x + 1, next_y + 1));
                }
            }
        }
    }

    amount
}