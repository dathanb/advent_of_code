use anyhow::{anyhow, Result};
use crate::coordinate::{Coordinate, Direction, Grid, RowOrientation};
use crate::shared::read_lines;

pub fn day09_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let rows = 1000;
    let cols = 1000;
    let mut grid: Grid<bool> = Grid::new(rows, cols);
    let mut head = Coordinate::new(rows, cols);
    head.row = 250; head.col = 250;
    let mut tail = Coordinate::new(rows, cols);
    tail.row = 250; tail.col = 250;
    grid[&tail] = true;

    for line in lines {
        println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>()?;
        match parts[0] {
            "R" => move_head(&mut head, &mut tail, &mut grid, Direction::Right, count)?,
            "L" => move_head(&mut head, &mut tail, &mut grid, Direction::Left, count)?,
            "U" => move_head(&mut head, &mut tail, &mut grid, Direction::Up, count)?,
            "D" => move_head(&mut head, &mut tail, &mut grid, Direction::Down, count)?,
            _ => return Err(anyhow!("Unrecognized direction {:?}", parts))
        }
    }

    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            let coordinate = Coordinate{row, col, rows, cols, orientation: RowOrientation::Normal};
            if grid[&coordinate] {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

pub fn day09_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    todo!()
}

fn move_head(head: &mut Coordinate, tail: &mut Coordinate, grid: &mut Grid<bool>, direction: Direction, count: usize) -> Result<()> {
    for _ in 0..count {
        head.increment_mut(&direction)?;
        move_tail(&head, tail, grid)?;
        grid[tail] = true;
    }

    Ok(())
}

fn move_tail(head: &Coordinate, tail: &mut Coordinate, grid: &mut Grid<bool>) -> Result<()> {
    let row_diff = head.row as i32 - tail.row as i32;
    let col_diff = head.col as i32 - tail.col as i32;
    match (row_diff, col_diff) {
        (-2, 0) => { tail.row -= 1; }
        (-2, -1) => {
            tail.row -= 1;
            tail.col -= 1;
        }
        (-2, 1) => {
            tail.row -= 1;
            tail.col += 1;
        }
        (-1, -2) => {
            tail.row -= 1;
            tail.col -= 1;
        }
        (-1, 2) => {
            tail.row -= 1;
            tail.col += 1;
        }
        (0, -2) => { tail.col -= 1; }
        (0, 2) => { tail.col += 1; }
        (1, -2) => {
            tail.row += 1;
            tail.col -= 1;
        }
        (1, 2) => { tail.row += 1; tail.col += 1; }
        (2, -1) => {
            tail.row += 1;
            tail.col -= 1;
        }
        (2, 0) => { tail.row += 1; }
        (2, 1) => {
            tail.row += 1;
            tail.col += 1;
        }
        _ => {},// do nothing
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day09::{day09_part1, day09_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day09_part1("data/day09_test.txt")?;
        println!("{}", score);
        assert_eq!(13, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day09_part1("data/day09.txt")?;
        println!("{}", score);
        assert_eq!(6044, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day09_part2("data/day09_test.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day09_part2("data/day09.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
