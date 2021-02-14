// rosenfeld_skeletonizer.rs - Skeletonization using the Rosenfeld algorithm.
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
use std::cmp::max;

#[derive(PartialEq, Eq)]
pub enum ProcessingSide {
    North,
    East,
    South,
    West,
}

pub struct RosenfeldSkeletonizer {
    mode: AdjacencyMode,
}

impl Skeletonizer for RosenfeldSkeletonizer {
    fn process_with_progress<F>(&self, image: &mut BinaryImage, report_progress: F)
    where
        F: Fn(i32, i32),
    {
        let mut current_progress = 0;
        let max_progress = Self::compute_max_progress(image.width(), image.height());

        let sides = [
            ProcessingSide::North,
            ProcessingSide::South,
            ProcessingSide::West,
            ProcessingSide::East,
        ];

        loop {
            let mut x = 0;
            for side in &sides {
                x += self.process_side(image, side);
            }

            current_progress += 1;

            report_progress(current_progress, max_progress as i32);

            if x == 0 {
                break;
            }
        }

        report_progress(max_progress as i32, max_progress as i32);
    }
}

impl RosenfeldSkeletonizer {
    pub fn new(mode: AdjacencyMode) -> Self {
        RosenfeldSkeletonizer { mode }
    }

    fn compute_max_progress(width: usize, height: usize) -> usize {
        max(width, height) / 2
    }

    fn process_side(&self, image: &mut BinaryImage, side: &ProcessingSide) -> usize {
        let mut amount = 0;

        let mut is_deleted = BoolMatrix::new(image.width(), image.height(), false);

        for (x, y) in image.pixels_iter() {
            if image.is_bg(x, y) {
                continue;
            }

            let mut is_fg = [[false; 3]; 3];
            for i in 0..9 {
                let new_x = x + i % 3;
                let new_y = y + i / 3;
                is_fg[i / 3][i % 3] = !(new_x == 0
                    || new_x > image.width()
                    || new_y == 0
                    || new_y > image.height()
                    || (image.is_bg(new_x - 1, new_y - 1)
                        && !is_deleted.check(new_x - 1, new_y - 1)));
            }

            match side {
                ProcessingSide::North if is_fg[0][1] => continue,
                ProcessingSide::East if is_fg[1][2] => continue,
                ProcessingSide::South if is_fg[2][1] => continue,
                ProcessingSide::West if is_fg[1][0] => continue,
                _ => (),
            };

            let mut black_count = 0;

            if *side != ProcessingSide::North && is_fg[0][1] {
                black_count += 1;
            }

            if *side != ProcessingSide::East && is_fg[1][2] {
                black_count += 1;
            }

            if *side != ProcessingSide::South && is_fg[2][1] {
                black_count += 1;
            }

            if *side != ProcessingSide::West && is_fg[1][0] {
                black_count += 1;
            }

            if self.mode == AdjacencyMode::Eight {
                if is_fg[0][0] {
                    black_count += 1;
                }

                if is_fg[0][2] {
                    black_count += 1;
                }

                if is_fg[2][2] {
                    black_count += 1;
                }

                if is_fg[2][0] {
                    black_count += 1;
                }
            }

            if black_count < 2 {
                continue;
            }

            if !is_local_articulation_point(image, x, y, self.mode) {
                is_deleted.set(x, y);
                image.set_bg(x, y);
                amount += 1;
            }
        }

        amount
    }
}

impl Default for RosenfeldSkeletonizer {
    fn default() -> Self {
        RosenfeldSkeletonizer::new(AdjacencyMode::Four)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelColor;

    #[test]
    fn rosenfeld_algorithm_eightmode_test() {
        // Arrange
        let mut image = BinaryImage::new(4, 4, PixelColor::White);
        image.fill(PixelColor::Black);
        let skeletonizer = RosenfeldSkeletonizer::new(AdjacencyMode::Eight);

        // Act
        skeletonizer.process(&mut image);

        // Assert
        for (x, y) in image.pixels_iter() {
            if (x == 2 || x == 1) && y == 2 {
                assert!(image.is_fg(x, y));
            } else {
                assert!(image.is_bg(x, y));
            }
        }
    }

    #[test]
    fn rosenfeld_algorithm_fourmode_test() {
        // Arrange
        let mut image = BinaryImage::new(4, 4, PixelColor::White);
        image.fill(PixelColor::Black);
        let skeletonizer = RosenfeldSkeletonizer::new(AdjacencyMode::Four);

        // Act
        skeletonizer.process(&mut image);

        // Assert
        for (x, y) in image.pixels_iter() {
            if (x == 2 || x == 1) && y == 2 {
                assert!(image.is_fg(x, y));
            } else {
                assert!(image.is_bg(x, y));
            }
        }
    }
}
