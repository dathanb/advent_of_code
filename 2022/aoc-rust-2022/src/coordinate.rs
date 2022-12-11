use std::ops::{Index, IndexMut};
use anyhow::{anyhow, Result};

#[derive(Clone, Copy)]
pub struct Coordinate {
    pub orientation: RowOrientation,
    pub row: usize,
    pub col: usize,
    pub rows: usize,
    pub cols: usize,
}

impl Coordinate {
    /// Initializes a new coordinate at the origin of a plane with normal orientation
    pub fn new(rows: usize, cols: usize) -> Coordinate {
        Coordinate {
            row: 0,
            col: 0,
            rows,
            cols,
            orientation: RowOrientation::Normal
        }
    }

    pub fn new_inverted(rows: usize, cols: usize) -> Coordinate {
        Coordinate {
            row: 0,
            col: 0,
            rows,
            cols,
            orientation: RowOrientation::Inverted
        }
    }

    pub fn increment(&self, direction: &Direction) -> Result<Coordinate> {
        self.clone().increment_mut(direction).map(|c| *c)
    }

    pub fn increment_mut(&mut self, direction: &Direction) -> Result<&Coordinate> {
        match (direction, self.orientation) {
            (Direction::Left, _) => if self.col == 0 { Err(anyhow!("Attempted to move to column less than 0")) } else {
                self.col -= 1;
                Ok(self)
            }
            (Direction::Right, _) => if self.col >= self.cols - 1 { Err(anyhow!("Attempted to move past greatest column")) } else {
                self.col += 1;
                Ok(self)
            }
            (Direction::Up, RowOrientation::Normal) | (Direction::Down, RowOrientation::Inverted) => if self.row >= self.rows - 1 { Err(anyhow!("Attempted to move past greatest row")) } else {
                self.row += 1;
                Ok(self)
            }
            (Direction::Up, RowOrientation::Inverted) | (Direction::Down, RowOrientation::Normal) => if self.row == 0 { Err(anyhow!("Attempted to move to row less than 0")) } else {
                self.row -= 1;
                Ok(self)
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum RowOrientation {
    /// Direction::Up means incrementing row
    Normal,
    /// Direction::Up means decrementing row
    Inverted
}

pub enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Index<&Coordinate> for Vec<i32> {
    type Output = i32;

    #[inline]
    fn index(&self, index: &Coordinate) -> &Self::Output {
        let index = index.row * index.cols + index.col;
        Index::index(&**self, index)
    }
}

pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    grid: Vec<T>,
}

impl<T> Grid<T> where T: Default + Clone {
    pub fn new(rows: usize, cols: usize) -> Grid<T> {
        Grid {
            rows,
            cols,
            grid: Vec::<T>::from_iter(std::iter::repeat(T::default()).take(rows*cols)),
        }
    }
}

impl<T> Index<&Coordinate> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Coordinate) -> &Self::Output {
        let index = index.row * self.cols + index.col;
        Index::index(&self.grid, index)
    }
}

impl<T> IndexMut<&Coordinate> for Grid<T>{
    fn index_mut(&mut self, index: &Coordinate) -> &mut Self::Output {
        let index = index.row * self.cols + index.col;
        &mut self.grid[index]
    }
}
