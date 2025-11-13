use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Manhattan distance to another point
    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Get all 4 cardinal neighbors (up, down, left, right)
    pub fn neighbors4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y - 1), // up
            Point::new(self.x, self.y + 1), // down
            Point::new(self.x - 1, self.y), // left
            Point::new(self.x + 1, self.y), // right
        ]
    }

    /// Get all 8 neighbors (cardinal + diagonal)
    pub fn neighbors8(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y - 1), // top-left
            Point::new(self.x, self.y - 1),     // top
            Point::new(self.x + 1, self.y - 1), // top-right
            Point::new(self.x - 1, self.y),     // left
            Point::new(self.x + 1, self.y),     // right
            Point::new(self.x - 1, self.y + 1), // bottom-left
            Point::new(self.x, self.y + 1),     // bottom
            Point::new(self.x + 1, self.y + 1), // bottom-right
        ]
    }

    /// Move in a direction by a given distance
    pub fn move_by(&self, dx: i32, dy: i32) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point::new(x, y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point::new(x as i32, y as i32)
    }
}
