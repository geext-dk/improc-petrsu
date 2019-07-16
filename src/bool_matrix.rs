pub struct BoolMatrix {
    data: Vec<Vec<bool>>
}

impl BoolMatrix {
    pub fn new(width: usize, height: usize, default_value: bool) -> BoolMatrix {
        BoolMatrix {
            data: vec![vec![default_value; width]; height]
        }
    }

    pub fn check(&self, x: usize, y: usize) -> bool {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.data[y][x] = true;
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        self.data[y][x] = false;
    }
}