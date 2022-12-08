use anyhow::Result;
use crate::shared::read_lines;

pub fn day08_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let rows = lines.len();
    let nums = to_height_map(lines)?;
    let cols = nums.len() / rows;

    let mut visibility_map: Vec<bool> = Vec::from_iter(std::iter::repeat(false).take(nums.len()));

    // left-to-right
    for row in 0..rows {
        let mut tallest = i32::MIN;
        for col in 0..cols {
            let index = row * cols + col;
            let height = nums[index];
            if height > tallest {
                visibility_map[index] = true;
                tallest = std::cmp::max(tallest, height);
            }
        }
    }

    // right-to-left
    for row in 0..rows {
        let mut tallest = i32::MIN;
        for col in (0..cols).rev() {
            let index = row * cols + col;
            let height = nums[index];
            if height > tallest {
                visibility_map[index] = true;
                tallest = std::cmp::max(tallest, height);
            }
        }
    }

    // top-to-bottom
    for col in 0..cols {
        let mut tallest = i32::MIN;
        for row in 0..rows {
            let index = row * cols + col;
            let height = nums[index];
            if height > tallest {
                visibility_map[index] = true;
                tallest = std::cmp::max(tallest, height);
            }
        }
    }

    // bottom-to-top
    for col in 0..cols {
        let mut tallest = i32::MIN;
        for row in (0..rows).rev() {
            let index = row * cols + col;
            let height = nums[index];
            if height > tallest {
                visibility_map[index] = true;
                tallest = std::cmp::max(tallest, height);
            }
        }
    }

    Ok(visibility_map.iter().filter(|b| **b).count() as i32)
}


pub fn day08_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let rows = lines.len();
    let nums = to_height_map(lines)?;
    let cols = nums.len() / rows;

    let mut max_visibility_score = i32::MIN;
    for row in 0..rows {
        for col in 0..cols {
            max_visibility_score = std::cmp::max(calculate_visibility_score(&nums, row, col, rows, cols), max_visibility_score);
        }
    }

    Ok(max_visibility_score)
}

fn to_height_map(lines: Vec<String>) -> Result<Vec<i32>> {
    let mut nums: Vec<i32> = Vec::new();
    for line in lines {
        for ch in line.chars() {
            nums.push(ch.to_string().parse::<i32>()?);
        }
    }
    Ok(nums)
}

fn calculate_visibility_score(nums: &Vec<i32>, row: usize, col: usize, rows: usize, cols: usize) -> i32 {
    let coordinate = Coordinate{row, col, rows, cols};
    let scores = vec![calculate_directional_visibility_score(nums, coordinate.clone(), Direction::Left),
         calculate_directional_visibility_score(nums, coordinate.clone(), Direction::Right),
         calculate_directional_visibility_score(nums, coordinate.clone(), Direction::Up),
         calculate_directional_visibility_score(nums, coordinate.clone(), Direction::Down)];
    scores.iter().product()
}

fn calculate_directional_visibility_score(heights: &Vec<i32>, coordinate: Coordinate, direction: Direction) -> i32 {
    let mut coordinate = coordinate;
    let mut score = 0;
    let height = heights[coordinate.index()];
    while let Some(new_coordinate) = coordinate.increment(&direction) {
        coordinate = new_coordinate;
        score += 1;
        if heights[coordinate.index()] >= height {
            break;
        }
    }
    score
}

#[derive(Clone, Copy)]
struct Coordinate {
    pub row: usize,
    pub col: usize,
    pub rows: usize,
    pub cols: usize
}

impl Coordinate {
    pub fn spawn(&self, new_row: usize, new_col: usize) -> Option<Coordinate> {
        if new_row >= self.rows || new_col >= self.cols {
            return None;
        }
        Some(Coordinate {row: new_row, col: new_col, rows: self.rows, cols: self.cols})
    }

    fn increment(&self, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::Left => if self.col == 0 { None } else { self.spawn(self.row, self.col-1) },
            Direction::Right => self.spawn(self.row, self.col+1),
            Direction::Up => if self.row == 0 { None } else { self.spawn(self.row-1, self.col) },
            Direction::Down => self.spawn(self.row+1, self.col)
        }
    }

    fn index(&self) -> usize {
        self.row * self.cols + self.col
    }
}

enum Direction {
    Left,
    Right,
    Down,
    Up
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day08::{day08_part1, day08_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day08_part1("data/day08_test.txt")?;
        println!("{}", score);
        assert_eq!(21, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day08_part1("data/day08.txt")?;
        println!("{}", score);
        assert_eq!(1782, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day08_part2("data/day08_test.txt")?;
        println!("{}", score);
        assert_eq!(8, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day08_part2("data/day08.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
