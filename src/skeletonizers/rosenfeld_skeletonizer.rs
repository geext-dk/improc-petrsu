use crate::binary_image::BinaryImage;
use crate::skeletonizers::{ is_local_articulation_point, Skeletonizer, AdjacencyMode };
use crate::bool_matrix::BoolMatrix;

#[derive(PartialEq, Eq)]
pub enum ProcessingSide
{
    North,
    East,
    South,
    West
}

pub struct RosenfeldSkeletonizer {
    mode: AdjacencyMode
}

impl Skeletonizer for RosenfeldSkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        let sides = [ProcessingSide::North, ProcessingSide::South, ProcessingSide::West, ProcessingSide::East];

        loop {
            let mut x = 0;
            for side in &sides {
                x += self.process_side(image, side);
            }

            if x == 0 {
                break;
            }
        }
    }
}

impl RosenfeldSkeletonizer {
    pub fn new(mode: AdjacencyMode) -> Self {
        RosenfeldSkeletonizer {
            mode
        }
    }

    fn process_side(&self, image: &mut BinaryImage, side: &ProcessingSide) -> usize {
        let mut amount = 0;

        let mut is_deleted = BoolMatrix::new(image.width(), image.height(), false);

        for (x, y) in image.iter() {
            if image.is_bg(x, y) {
                continue;
            }

            let mut is_fg = [[false; 3]; 3];
            for i in 0..9 {
                let new_x = x + i % 3;
                let new_y = y + i / 3;
                is_fg[i / 3][i % 3] = if new_x == 0 || new_x > image.width() || new_y == 0 || new_y > image.height() 
                        || (image.is_bg(new_x - 1, new_y - 1) && !is_deleted.check(new_x - 1, new_y - 1)) {
                    false
                } else {
                    true
                };
            }

            match side {
                ProcessingSide::North if is_fg[0][1] => continue,
                ProcessingSide::East  if is_fg[1][2] => continue,
                ProcessingSide::South if is_fg[2][1] => continue,
                ProcessingSide::West  if is_fg[1][0] => continue,
                _ => ()
            };
            
            let mut black_count = 0;

            if *side != ProcessingSide::North && is_fg[0][1] {
                black_count += 1;
            }

            if *side != ProcessingSide::East && is_fg[1][2] {
                black_count += 1;
            }

            if *side != ProcessingSide::South  && is_fg[2][1] {
                black_count += 1;
            }

            if *side != ProcessingSide::West  && is_fg[1][0] {
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
        for (x, y) in image.iter() {
            if x == 2 && y == 2 || x == 1 && y == 2 {
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
        for (x, y) in image.iter() {
            if x == 2 && y == 2 || x == 1 && y == 2 {
                assert!(image.is_fg(x, y));
            } else {
                assert!(image.is_bg(x, y));
            }
        }
    }
}