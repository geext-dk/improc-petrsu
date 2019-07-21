pub mod rosenfeld_skeletonizer;
pub mod eberly_skeletonizer;
pub mod zhangsuen_skeletonizer;

use crate::binary_image::{ BinaryImage, PixelColor };
use crate::bool_matrix::BoolMatrix;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AdjacencyMode {
    Four,
    Eight
}

pub trait Skeletonizer {
    fn process(&self, binary_image: &mut BinaryImage);
}

fn is_local_articulation_point(image: &BinaryImage, x: usize, y: usize, mode: AdjacencyMode, foreground_color: PixelColor) -> bool {
    let mut around = get_around(image, x, y);

    let components = count_components(&around, mode);

    if foreground_color == PixelColor::White { 
        around.set_white(1, 1);
    } else {
        around.set_black(1, 1);
    }

    components != count_components(&around, mode)
}

fn get_around(image: &BinaryImage, x: usize, y: usize) -> BinaryImage {
    let mut around = BinaryImage::new_with_color(3, 3, PixelColor::Black);
    
    for i in 0..9 {
        let delta_x = i % 3;
        let delta_y = i / 3;
        let new_x = x + delta_x;
        let new_y = y + delta_y;

        if new_x == 0 || new_x - 1 > image.width() - 1 || new_y == 0 || new_y - 1 > image.height() - 1 {
            around.set_black(delta_x, delta_y);
        } else {
            if image.is_black(new_x - 1, new_y - 1) {
                around.set_black(delta_x, delta_y);
            } else {
                around.set_white(delta_x, delta_y);
            }
        }
    }

    around
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_components_three_modefour_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::White);
        image.set_black(0, 0);
        image.set_black(1, 1);
        image.set_black(2, 2);

        // Act
        let count = count_components(&image, AdjacencyMode::Four);

        // Assert
        assert_eq!(3, count);
    }

    #[test]
    fn count_components_one_modeeight_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::White);
        image.set_black(0, 0);
        image.set_black(1, 1);
        image.set_black(2, 2);

        // Act
        let count = count_components(&image, AdjacencyMode::Eight);

        // Assert
        assert_eq!(1, count);
    }

    #[test]
    fn count_components_four_modeeight_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::White);
        image.set_black(0, 0);
        image.set_black(2, 2);
        image.set_black(2, 0);
        image.set_black(0, 2);

        // Act
        let count = count_components(&image, AdjacencyMode::Eight);

        // Assert
        assert_eq!(4, count);
    }

    #[test]
    fn get_around_borders_test() {
        // Arrange
        let image = BinaryImage::new_with_color(1, 1, PixelColor::White);
        
        // Act
        let around = get_around(&image, 0, 0);

        // Assert
        for (x, y) in around.iter() {
            if x == 1 && y == 1 {
                assert!(around.is_white(x, y));
            } else {
                assert!(around.is_black(x, y));
            }
        }
    }

    #[test]
    fn get_around_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(5, 5, PixelColor::Black);
        image.set_white(2, 2);
        image.set_white(1, 2);
        image.set_white(3, 3);
        image.set_white(3, 4);

        // Act
        let around = get_around(&image, 2, 2);

        // Assert
        for (x, y) in around.iter() {
            if x == 1 && y == 1 || x == 0 && y == 1 || x == 2 && y == 2 {
                assert!(around.is_white(x, y));
            } else {
                assert!(around.is_black(x, y));
            }
        }
    }

    #[test]
    fn is_local_articulation_point_true_modefour_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::Black);
        image.set_white(1, 1);
        image.set_white(2, 2);
        image.set_white(2, 1);

        // Act & Assert
        assert_eq!(true, is_local_articulation_point(&image, 2, 1, AdjacencyMode::Four, PixelColor::White));
    }

    #[test]
    fn is_local_articulation_point_false_modefour_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::Black);
        image.set_white(1, 1);
        image.set_white(2, 2);
        image.set_white(2, 1);
        image.set_white(1, 2);

        // Act & Assert
        assert_eq!(false, is_local_articulation_point(&image, 2, 1, AdjacencyMode::Four, PixelColor::White));
    }

    #[test]
    fn is_local_articulation_point_true_modeeight_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::Black);
        image.set_white(1, 1);
        image.set_white(2, 2);
        image.set_white(3, 3);

        // Act & Assert
        assert_eq!(true, is_local_articulation_point(&image, 2, 2, AdjacencyMode::Eight, PixelColor::White));
    }

    #[test]
    fn is_local_articulation_point_false_modeeight_test() {
        // Arrange
        let mut image = BinaryImage::new_with_color(3, 3, PixelColor::Black);
        image.set_white(1, 1);
        image.set_white(2, 2);
        image.set_white(3, 3);
        image.set_white(1, 2);
        image.set_white(2, 3);

        // Act & Assert
        assert_eq!(false, is_local_articulation_point(&image, 2, 2, AdjacencyMode::Eight, PixelColor::White));
    }
}