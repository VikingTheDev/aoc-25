use super::point::Point;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub cells: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Grid<T> {
    /// Create a new grid with a default value
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            cells: vec![vec![default; width]; height],
            width,
            height,
        }
    }

    /// Create a grid from a 2D vector
    pub fn from_vec(cells: Vec<Vec<T>>) -> Self {
        let height = cells.len();
        let width = if height > 0 { cells[0].len() } else { 0 };
        Self {
            cells,
            width,
            height,
        }
    }

    /// Get a value at a point (returns None if out of bounds)
    pub fn get(&self, point: &Point) -> Option<&T> {
        if point.x < 0 || point.y < 0 {
            return None;
        }
        let x = point.x as usize;
        let y = point.y as usize;
        self.cells.get(y).and_then(|row| row.get(x))
    }

    /// Get a mutable reference to a value at a point
    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if point.x < 0 || point.y < 0 {
            return None;
        }
        let x = point.x as usize;
        let y = point.y as usize;
        self.cells.get_mut(y).and_then(|row| row.get_mut(x))
    }

    /// Set a value at a point (returns false if out of bounds)
    pub fn set(&mut self, point: &Point, value: T) -> bool {
        if let Some(cell) = self.get_mut(point) {
            *cell = value;
            true
        } else {
            false
        }
    }

    /// Check if a point is within bounds
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && (point.x as usize) < self.width
            && (point.y as usize) < self.height
    }

    /// Get all valid neighbors (4-directional) of a point
    pub fn neighbors4(&self, point: &Point) -> Vec<Point> {
        point
            .neighbors4()
            .into_iter()
            .filter(|p| self.contains(p))
            .collect()
    }

    /// Get all valid neighbors (8-directional) of a point
    pub fn neighbors8(&self, point: &Point) -> Vec<Point> {
        point
            .neighbors8()
            .into_iter()
            .filter(|p| self.contains(p))
            .collect()
    }

    /// Iterate over all points in the grid
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Point::new(x as i32, y as i32)))
    }

    /// Iterate over all cells with their positions
    pub fn cells_with_points(&self) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.points()
            .filter_map(move |p| self.get(&p).map(|cell| (p, cell)))
    }

    /// Find all positions where a predicate is true
    pub fn find_all<F>(&self, predicate: F) -> Vec<Point>
    where
        F: Fn(&T) -> bool,
    {
        self.cells_with_points()
            .filter(|(_, cell)| predicate(cell))
            .map(|(point, _)| point)
            .collect()
    }

    /// Find the first position where a predicate is true
    pub fn find<F>(&self, predicate: F) -> Option<Point>
    where
        F: Fn(&T) -> bool,
    {
        self.cells_with_points()
            .find(|(_, cell)| predicate(cell))
            .map(|(point, _)| point)
    }
}

impl Grid<char> {
    /// Create a character grid from a string (lines separated by newlines)
    pub fn from_string(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        Self::from_vec(cells)
    }

    /// Convert grid back to a string
    pub fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(3, 2, 0);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.get(&Point::new(0, 0)), Some(&0));
    }

    #[test]
    fn test_char_grid() {
        let input = "abc\ndef";
        let grid = Grid::from_string(input);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.get(&Point::new(0, 0)), Some(&'a'));
        assert_eq!(grid.get(&Point::new(2, 1)), Some(&'f'));
    }
}
