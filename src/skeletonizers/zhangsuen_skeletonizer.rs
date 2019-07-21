use crate::binary_image::BinaryImage;
use crate::skeletonizers::Skeletonizer;
use crate::PixelColor;

pub struct ZhangSuenSkeletonizer;

impl Skeletonizer for ZhangSuenSkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        let mut outer_image = BinaryImage::new(image.width() + 2, image.height() + 2, PixelColor::White);
        for (x, y) in image.iter() {
            if image.is_fg(x, y) {
                outer_image.set_fg(x + 1, y + 1);
            }
        }

        loop {
            let mut pixels_changed = 0;

            pixels_changed += ZhangSuenSkeletonizer::step_one(&mut outer_image);
            pixels_changed += ZhangSuenSkeletonizer::step_two(&mut outer_image);

            if pixels_changed == 0 {
                break;
            }
        }

        for (x, y) in image.iter() {
            if outer_image.is_fg(x + 1, y + 1) {
                image.set_fg(x, y);
            } else {
                image.set_bg(x, y);
            }
        }
    }
}

impl ZhangSuenSkeletonizer {
    pub fn new() -> Self {
        ZhangSuenSkeletonizer {}
    }
    
    fn step_one(image: &mut BinaryImage) -> u32 {
        let mut count = 0;
        let mut marked_pixels = Vec::new();

        for y in 1..image.height() - 1 {
            for x in 1..image.width() - 1 {
                if image.is_bg(x, y) {
                    continue;
                }

                let black_count = ZhangSuenSkeletonizer::count_black_neighbours(image, x, y);
                if black_count < 2 || black_count > 6 {
                    continue;
                }

                if ZhangSuenSkeletonizer::count_transitions(image, x, y) != 1 {
                    continue;
                }

                if image.is_fg(x, y + 1) && image.is_fg(x + 1, y)
                        && (image.is_fg(x, y - 1) || image.is_fg(x - 1, y)) {
                    continue;
                }

                marked_pixels.push((x, y));
                count += 1;
            }
        }

        for (x, y) in marked_pixels {
            image.set_bg(x, y);
        }

        count
    }

    fn step_two(image: &mut BinaryImage) -> u32 {
        let mut count = 0;
        let mut marked_pixels = Vec::new();
        for y in 1..image.height() - 1 {
            for x in 1..image.width() - 1 {
                if image.is_bg(x, y) {
                    continue;
                }

                let black_count = ZhangSuenSkeletonizer::count_black_neighbours(image, x, y);
                if black_count < 2 || black_count > 6 {
                    continue;
                }

                if ZhangSuenSkeletonizer::count_transitions(image, x, y) != 1 {
                    continue;
                }

                if image.is_fg(x, y - 1) && image.is_fg(x - 1, y) 
                        && (image.is_fg(x, y + 1) || image.is_fg(x + 1, y)) {
                    continue;
                }

                marked_pixels.push((x, y));
                count += 1;
            }
        }

        for (x, y) in marked_pixels {
            image.set_bg(x, y);
        }

        count
    }

    fn count_black_neighbours(image: &BinaryImage, mut x: usize, mut y: usize) -> u32 {
        let mut count = 0;
        x -= 1;
        y -= 1;
        let delta_x = [1, 2, 2, 2, 1, 0, 0, 0];
        let delta_y = [0, 0, 1, 2, 2, 2, 1, 0];

        for i in 0..8 {
            if image.is_fg(x + delta_x[i], y + delta_y[i]) {
                count += 1;
            }
        }

        count
    }

    fn count_transitions(image: &BinaryImage, x: usize, y: usize)  -> u32 {
        let delta_x = [2, 2, 2, 1, 0, 0, 0, 1];
        let delta_y = [0, 1, 2, 2, 2, 1, 0, 0];

        let mut count = 0;
        let mut is_previous_bg = if y == 0 || y > image.height() || x > image.height() - 1 {
            true
        } else {
            image.is_bg(x, y - 1)
        };

        for i in 0..8 {
            let new_x = x + delta_x[i];
            let new_y = y + delta_y[i];

            let is_current_bg = if new_x == 0 || new_x > image.width() || new_y == 0 || new_y > image.height() {
                true
            } else {
                image.is_bg(new_x - 1, new_y - 1)
            };

            if is_previous_bg && !is_current_bg {
                count += 1;
            }

            is_previous_bg = is_current_bg;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zhangsuen_algorithm_test() {

    }

    #[test]
    fn count_transitions_borders_test() {
        // Arrange
        let mut image = BinaryImage::new(1, 1, PixelColor::White);
        image.set_fg(0, 0);

        // Act
        let count = ZhangSuenSkeletonizer::count_transitions(&image, 0, 0);

        // Assert
        assert_eq!(0, count);
    }

    #[test]
    fn count_transitions_zero_black_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.fill(PixelColor::Black);

        // Act
        let count = ZhangSuenSkeletonizer::count_transitions(&image, 1, 1);

        // Assert
        assert_eq!(0, count);
    }

    #[test]
    fn count_transitions_zero_white_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.set_fg(1, 1);

        // Act
        let count = ZhangSuenSkeletonizer::count_transitions(&image, 1, 1);

        // Assert
        assert_eq!(0, count);
    }

    #[test]
    fn count_transitions_one_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.fill(PixelColor::Black);
        image.set_bg(0, 0);

        // Act
        let count = ZhangSuenSkeletonizer::count_transitions(&image, 1, 1);

        // Assert
        assert_eq!(1, count);
    }
    
    #[test]
    fn count_transitions_four_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.fill(PixelColor::Black);
        image.set_bg(0, 0);
        image.set_bg(2, 0);
        image.set_bg(0, 2);
        image.set_bg(2, 2);

        // Act
        let count = ZhangSuenSkeletonizer::count_transitions(&image, 1, 1);

        // Assert
        assert_eq!(4, count);
    }
}