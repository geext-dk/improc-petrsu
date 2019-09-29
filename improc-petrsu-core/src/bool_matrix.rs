// bool_matrix.rs - A helper struct to work with binary matrices
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

pub struct BoolMatrix {
    data: Vec<Vec<bool>>,
}

impl BoolMatrix {
    pub fn new(width: usize, height: usize, default_value: bool) -> BoolMatrix {
        BoolMatrix {
            data: vec![vec![default_value; width]; height],
        }
    }

    pub fn check(&self, x: usize, y: usize) -> bool {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.data[y][x] = true;
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
}
