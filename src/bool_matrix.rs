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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_false_test() {
        // Arrange & Act
        let matrix = BoolMatrix::new(3, 3, false);

        // Assert
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(false, matrix.data[y][x]);
            }
        }
    }

    #[test]
    fn constructor_true_test() {
        // Arrange & Act
        let matrix = BoolMatrix::new(3, 3, true);

        // Assert
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(true, matrix.data[y][x]);
            }
        }
    }

    #[test]
    fn check_true_test() {
        // Arrange
        let mut matrix = BoolMatrix::new(3, 3, false);
        matrix.data[1][1] = true;

        // Act & Assert
        assert_eq!(true, matrix.check(1, 1));
    }
    
    #[test]
    fn check_false_test() {
        // Arrange
        let mut matrix = BoolMatrix::new(3, 3, false);
        matrix.data[1][1] = false;
        
        // Act & Assert
        assert_eq!(false, matrix.check(1, 1));
    }
    
    #[test]
    fn set_test() {
        // Arrange
        let mut matrix = BoolMatrix::new(3, 3, false);

        // Act
        matrix.set(1, 1);

        // Assert
        assert_eq!(true, matrix.data[1][1]);
    }
    
    #[test]
    fn unset_test() {
        // Arrange
        let mut matrix = BoolMatrix::new(3, 3, true);

        // Act
        matrix.unset(1, 1);

        // Assert
        assert_eq!(false, matrix.data[1][1]);
    }
}