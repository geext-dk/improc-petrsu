use crate::binary_image::BinaryImage;
use crate::skeletonizers::{ is_local_articulation_point, Skeletonizer, AdjacencyMode };
use crate::bool_matrix::BoolMatrix;

#[derive(PartialEq, Eq)]
pub enum ProcessingSide
{
    Up,
    Right,
    Down,
    Left
}

pub struct RosenfeldSkeletonizer {
    mode: AdjacencyMode
}

impl Skeletonizer for RosenfeldSkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        let sides = [ProcessingSide::Up, ProcessingSide::Right, ProcessingSide::Down, ProcessingSide::Left];

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

    fn was_or_is_black(&self, image: &BinaryImage, is_deleted: &BoolMatrix, x: usize, y: usize) -> bool {
        return x < image.width() && y < image.height()
            && (image.is_fg(x, y) || is_deleted.check(x, y));
    }

    fn process_side(&self, image: &mut BinaryImage, side: &ProcessingSide) -> usize {
        let mut amount = 0;

        let mut is_deleted = BoolMatrix::new(image.width(), image.height(), false);

        for (x, y) in image.iter() {
            if image.is_bg(x, y) {
                continue;
            }

            match side {
                ProcessingSide::Up    if self.was_or_is_black(image, &is_deleted, x, y - 1) => continue,
                ProcessingSide::Right if self.was_or_is_black(image, &is_deleted, x + 1, y) => continue,
                ProcessingSide::Down  if self.was_or_is_black(image, &is_deleted, x, y + 1) => continue,
                ProcessingSide::Left  if self.was_or_is_black(image, &is_deleted, x - 1, y) => continue,
                _ => ()
            };
            
            let mut black_count = 0;

            if *side != ProcessingSide::Up && y != 0 && (image.is_fg(x, y - 1) || is_deleted.check(x, y - 1)) {
                black_count += 1;
            }

            if *side != ProcessingSide::Right && x != image.width() - 1 && (image.is_fg(x + 1, y) || is_deleted.check(x + 1, y)) {
                black_count += 1;
            }

            if *side != ProcessingSide::Down  && y != image.height() - 1 && (image.is_fg(x, y + 1) || is_deleted.check(x, y + 1)) {
                black_count += 1;
            }

            if *side != ProcessingSide::Left  && x != 0 && (image.is_fg(x - 1, y) || is_deleted.check(x - 1, y)) {
                black_count += 1;
            }
            
            if self.mode == AdjacencyMode::Eight {
                if x != 0 && y != 0 && (image.is_fg(x - 1, y - 1) || is_deleted.check(x - 1, y - 1)) {
                    black_count += 1;
                }
    
                if x != image.width() - 1 && y != 0 && (image.is_fg(x + 1, y - 1) || is_deleted.check(x + 1, y - 1)) {
                    black_count += 1;
                }
    
                if x != image.width() - 1 && y != image.height() - 1 && (image.is_fg(x + 1, y + 1) || is_deleted.check(x + 1, y + 1)) {
                    black_count += 1;
                }
    
                if x != 0 && y != image.height() - 1 && (image.is_fg(x - 1, y + 1) || is_deleted.check(x - 1, y + 1)) {
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