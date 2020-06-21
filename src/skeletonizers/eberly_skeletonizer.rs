// eberly_skeletonizer.rs - Skeletonization using the Eberly algorithm
// Copyright (C) 2019 Denis Karpovskiy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::binary_image::BinaryImage;
use crate::bool_matrix::BoolMatrix;
use crate::skeletonizers::{is_local_articulation_point, AdjacencyMode, Skeletonizer};

pub struct EberlySkeletonizer;
struct FourInteriorAlgorithm;
struct ThreeInteriorAlgorithm;
struct TwoInteriorAlgorithm;

trait EberlyInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool;
    fn remove_interiors(image: &mut BinaryImage, is_interior: &BoolMatrix) {
        for (x, y) in image.pixels_iter() {
            if is_interior.check(x, y)
                && !is_local_articulation_point(image, x, y, AdjacencyMode::Eight)
            {
                image.set_bg(x, y);
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ReturnStatus {
    ExitCriteriaNotMet,
    NoMoreInteriorPixels,
    CantRemoveMoreBoundaryPixels,
}

impl Skeletonizer for EberlySkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        while Self::thinning::<FourInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {}

        while Self::thinning::<ThreeInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {}

        while Self::thinning::<TwoInteriorAlgorithm>(image) == ReturnStatus::ExitCriteriaNotMet {}
    }
}

impl EberlySkeletonizer {
    pub fn new() -> Self {
        EberlySkeletonizer {}
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
        let mut is_interior = BoolMatrix::new(image.width(), image.height(), false);
        let mut is_interior_exists = false;

        for (x, y) in image.pixels_iter() {
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
        for (x, y) in image.pixels_iter() {
            if Self::is_boundary(image, x, y, is_interior) {
                image.set_bg(x, y);
                amount += 1;
            }
        }

        amount
    }

    fn is_boundary(image: &BinaryImage, x: usize, y: usize, is_interior: &BoolMatrix) -> bool {
        image.is_fg(x, y)
            && !is_interior.check(x, y)
            && Self::is_adjacent_to_bg(image, x, y)
            && Self::is_adjacent_to_interior(image, x, y, is_interior)
            && !is_local_articulation_point(image, x, y, AdjacencyMode::Eight)
    }

    fn is_adjacent_to_bg(image: &BinaryImage, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x >= image.width() - 1 || y >= image.height() - 1 {
            return true;
        }

        for i in 0..9 {
            if image.is_bg(x + i % 3 - 1, y + i / 3 - 1) {
                return true;
            }
        }

        false
    }

    fn is_adjacent_to_interior(
        image: &BinaryImage,
        x: usize,
        y: usize,
        is_interior: &BoolMatrix,
    ) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if x + j != 0
                    && x + j - 1 < image.width()
                    && y + i != 0
                    && y + i - 1 < image.height()
                    && is_interior.check(x + j - 1, y + i - 1)
                {
                    return true;
                }
            }
        }

        false
    }
}

impl ThreeInteriorAlgorithm {
    fn count_black_neighbours(image: &BinaryImage, x: usize, y: usize) -> usize {
        let mut count = 0;
        let x_offsets = [1, 2, 1, 0];
        let y_offsets = [0, 1, 2, 1];

        for i in 0..x_offsets.len() {
            let new_x = x + x_offsets[i];
            let new_y = y + y_offsets[i];

            if new_x != 0
                && new_x - 1 < image.width()
                && new_y != 0
                && new_y - 1 < image.height()
                && image.is_fg(new_x - 1, new_y - 1)
            {
                count += 1;
            }
        }

        count
    }
}

impl EberlyInteriorAlgorithm for TwoInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool {
        if image.is_fg(x, y) {
            let mut horizontal_black_count = 0;
            let mut vertical_black_count = 0;

            if y != 0 && image.is_fg(x, y - 1) {
                vertical_black_count += 1;
            }

            if x != image.width() - 1 && image.is_fg(x + 1, y) {
                horizontal_black_count += 1;
            }

            if y != image.height() - 1 && image.is_fg(x, y + 1) {
                vertical_black_count += 1
            }

            if x != 0 && image.is_fg(x - 1, y) {
                horizontal_black_count += 1;
            }

            horizontal_black_count == 1 && vertical_black_count == 1
        } else {
            false
        }
    }
}

impl EberlyInteriorAlgorithm for ThreeInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool {
        image.is_fg(x, y) && Self::count_black_neighbours(image, x, y) == 3
    }
}

impl EberlyInteriorAlgorithm for FourInteriorAlgorithm {
    fn is_interior(image: &BinaryImage, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x >= image.width() - 1 || y >= image.height() - 1 {
            false
        } else {
            let x_offset = [1, 2, 0, 1, 1];
            let y_offset = [1, 1, 1, 2, 0];

            for i in 0..x_offset.len() {
                if image.is_bg(x + x_offset[i] - 1, y + y_offset[i] - 1) {
                    return false;
                }
            }

            true
        }
    }

    fn remove_interiors(_image: &mut BinaryImage, _is_interior: &BoolMatrix) {
        // nothing to do
    }
}

impl Default for EberlySkeletonizer {
    fn default() -> Self {
        EberlySkeletonizer::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelColor;

    #[test]
    fn eberly_constructor_test() {
        // Arrange & Act & Assert
        EberlySkeletonizer::new();
    }

    #[test]
    fn eberly_algorithm_test() {
        // Arrange
        let mut image = BinaryImage::new(4, 4, PixelColor::White);
        image.fill(PixelColor::Black);
        let skeletonizer = EberlySkeletonizer::new();

        // Act
        skeletonizer.process(&mut image);

        // Assert
        for (x, y) in image.pixels_iter() {
            if x == 2 && y == 2 {
                assert!(image.is_fg(x, y));
            } else {
                assert!(image.is_bg(x, y));
            }
        }
    }

    #[test]
    fn three_interior_is_interior_true_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.set_fg(1, 1);
        image.set_fg(1, 0);
        image.set_fg(0, 1);
        image.set_fg(2, 1);

        // Act
        let result = ThreeInteriorAlgorithm::is_interior(&image, 1, 1);

        // Assert
        assert_eq!(true, result);
    }

    #[test]
    fn three_interior_is_interior_false_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.set_fg(1, 1);
        image.set_fg(1, 0);
        image.set_fg(2, 1);

        // Act
        let result = ThreeInteriorAlgorithm::is_interior(&image, 1, 1);

        // Assert
        assert_eq!(false, result);
    }

    #[test]
    fn two_interior_is_interior_true_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.set_fg(1, 1);
        image.set_fg(1, 0);
        image.set_fg(0, 1);

        // Act
        let result = TwoInteriorAlgorithm::is_interior(&image, 1, 1);

        assert_eq!(true, result);
    }

    #[test]
    fn two_interior_is_interior_false_test() {
        // Arrange
        let mut image = BinaryImage::new(3, 3, PixelColor::White);
        image.set_fg(1, 1);
        image.set_fg(1, 0);
        image.set_fg(1, 2);

        // Act
        let result = TwoInteriorAlgorithm::is_interior(&image, 1, 1);

        assert_eq!(false, result);
    }
}
