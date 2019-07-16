use crate::binary_image::BinaryImage;
use crate::skeletonizers::AdjacencyMode;
use crate::skeletonizers::Skeletonizer;
use crate::bool_matrix::BoolMatrix;

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

impl RosenfeldSkeletonizer {
    pub fn new(mode: AdjacencyMode) -> Self {
        RosenfeldSkeletonizer {
            mode
        }
    }

    fn was_or_is_black(&self, image: &BinaryImage, is_deleted: &BoolMatrix, x: usize, y: usize) -> bool {
        return x >= 0 && x < image.width() && y >= 0 && y < image.height()
            && (image.is_black(x, y) || is_deleted.check(x, y));
    }

    fn process_side(&self, image: &mut BinaryImage, side: &ProcessingSide) -> usize {
        let amount = 0;
        let change_x = Vec::new();
        let change_y = Vec::new();

        let is_deleted = BoolMatrix::new(image.width(), image.height(), false);

        for (x, y) in image.iter() {
            if image.is_white(x, y) {
                continue;
            }

            match side {
                ProcessingSide::Up    if self.was_or_is_black(image, is_deleted, x, y - 1) => continue,
                ProcessingSide::Right if self.was_or_is_black(image, is_deleted, x + 1, y) => continue,
                ProcessingSide::Down  if self.was_or_is_black(image, is_deleted, x, y + 1) => continue,
                ProcessingSide::Left  if self.was_or_is_black(image, is_deleted, x - 1, y) => continue,
                _ => ()
            };

                ProcessingSide::Up    if y != 0                  && (image.is_black(x, y - 1) || is_deleted.check(x, y)) => continue,
                ProcessingSide::Right if x != image.width() - 1  && (image.is_black(x + 1, y) || is_deleted.check(x, y)) => continue,
                ProcessingSide::Down  if y != image.height() - 1 && (image.is_black(x, y + 1) || is_deleted.check(x, y)) => continue,
                ProcessingSide::Left  if x != 0                  && (image.is_black(x - 1, y) || is_deleted.check(x, y)) => continue,
                _ => ()
            };

            let black_count = 0;


        }

        amount
    }
}

impl Skeletonizer for RosenfeldSkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        let sides = [ProcessingSide::Up, ProcessingSide::Right, ProcessingSide::Down, ProcessingSide::Left];

        let i = 0;
        loop {
            let x = 0;
            for side in &sides {
                x += self.process_side(image, side);
            }
        }
    }
}