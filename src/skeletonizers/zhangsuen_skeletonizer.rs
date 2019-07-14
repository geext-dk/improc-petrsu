use crate::binary_image::BinaryImage;
use crate::skeletonizers::Skeletonizer;

pub struct ZhangSuenSkeletonizer {
}

impl ZhangSuenSkeletonizer {
    fn step_one(image: &mut BinaryImage) -> u32 {
        let mut count = 0;
        let mut marked_pixels = Vec::new();

        for y in 1..image.height() - 1 {
            for x in 1..image.width() - 1 {
                if image.is_white(x, y) {
                    continue;
                }

                let black_count = ZhangSuenSkeletonizer::count_black_neighbours(image, x, y);
                if black_count < 2 || black_count > 6 {
                    continue;
                }

                if ZhangSuenSkeletonizer::count_w2b_transitions(image, x, y) != 1 {
                    continue;
                }

                if image.is_black(x, y + 1) && image.is_black(x + 1, y)
                        && (image.is_black(x, y - 1) || image.is_black(x - 1, y)) {
                    continue;
                }

                marked_pixels.push((x, y));
                count += 1;
            }
        }

        for (x, y) in marked_pixels {
            image.set_white(x, y);
        }

        count
    }

    fn step_two(image: &mut BinaryImage) -> u32 {
        let mut count = 0;
        let mut marked_pixels = Vec::new();
        for y in 1..image.height() - 1 {
            for x in 1..image.width() - 1 {
                if image.is_white(x, y) {
                    continue;
                }

                let black_count = ZhangSuenSkeletonizer::count_black_neighbours(image, x, y);
                if black_count < 2 || black_count > 6 {
                    continue;
                }

                if ZhangSuenSkeletonizer::count_w2b_transitions(image, x, y) != 1 {
                    continue;
                }

                if image.is_black(x, y - 1) && image.is_black(x - 1, y) 
                        && (image.is_black(x, y + 1) || image.is_black(x + 1, y)) {
                    continue;
                }

                marked_pixels.push((x, y));
                count += 1;
            }
        }

        for (x, y) in marked_pixels {
            image.set_white(x, y);
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
            if image.is_black(x + delta_x[i], y + delta_y[i]) {
                count += 1;
            }
        }

        count
    }

    fn count_w2b_transitions(image: &BinaryImage, mut x: usize, mut y: usize)  -> u32 {
        x -= 1;
        y -= 1;
        let delta_x = [2, 2, 2, 1, 0, 0, 0, 1];
        let delta_y = [0, 1, 2, 2, 2, 1, 0, 0];

        let mut count = 0;
        let mut is_previous_white = image.is_white(x + 1, y);
        for i in 0..8 {
            let is_current_white = image.is_white(x + delta_x[i], y + delta_y[i]);
            if is_previous_white && !is_current_white {
                count += 1;
            }

            is_previous_white = is_current_white;
        }

        count
    }
}

impl Skeletonizer for ZhangSuenSkeletonizer {
    fn process(&self, image: &mut BinaryImage) {
        let mut outer_image = BinaryImage::new(image.width() + 2, image.height() + 2);
        for (x, y) in image.iter() {
            if image.is_black(x, y) {
                outer_image.set_black(x + 1, y + 1);
            } else {
                outer_image.set_white(x + 1, y + 1);
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
            if outer_image.is_black(x + 1, y + 1) {
                image.set_black(x, y);
            } else {
                image.set_white(x, y);
            }
        }
    }
}