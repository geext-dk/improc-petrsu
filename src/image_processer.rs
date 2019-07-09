use std::path::Path;

pub trait ImageProcesser {
    fn process(image_path: &Path);
}