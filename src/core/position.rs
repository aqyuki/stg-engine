#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn distance(&self, other: Position) -> i32 {
        let x = (self.x - other.x).pow(2) as f64;
        let y = (self.y - other.y).pow(2) as f64;
        (x + y).sqrt().round() as i32
    }

    pub fn aquire(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn update(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Position::new(0, 0);
        let b = Position::new(3, 4);
        assert_eq!(a.distance(b), 5);
    }
}
