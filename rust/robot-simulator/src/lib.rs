// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot {
    direction: Direction,
    position: (i32, i32)
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            direction: d,
            position: (x, y),
        }
    }

    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }

        self
    }

    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::East => self.direction = Direction::North,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
        }

        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.position.1 += 1,
            Direction::East => self.position.0 += 1,
            Direction::South => self.position.1 -= 1,
            Direction::West => self.position.0 -= 1,
        }

        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {

        for instruction in instructions.chars() {
            match instruction {
                'R' => {
                    self = self.turn_right();
                },
                'L' => {
                    self = self.turn_left();
                },
                'A' => {
                    self = self.advance();
                },
                _ => panic!("Invalid instruction"),
            }
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
