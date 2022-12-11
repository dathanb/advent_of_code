use anyhow::{anyhow, Result};
use crate::coordinate::{Coordinate, Direction, Grid, RowOrientation};
use crate::shared::read_lines;

pub fn day09_part1(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let rows = 1000;
    let cols = 1000;
    let mut grid: Grid<bool> = Grid::new(rows, cols);
    let mut knots = Vec::<Coordinate>::new();
    for _ in 0..2 {
        knots.push(Coordinate { rows, cols, row: 250, col: 250, orientation: RowOrientation::Normal });
    }
    grid[&knots[knots.len() - 1]] = true;

    for line in lines {
        println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>()?;
        match parts[0] {
            "R" => move_knots(&mut knots, &mut grid, Direction::Right, count)?,
            "L" => move_knots(&mut knots, &mut grid, Direction::Left, count)?,
            "U" => move_knots(&mut knots, &mut grid, Direction::Up, count)?,
            "D" => move_knots(&mut knots, &mut grid, Direction::Down, count)?,
            _ => return Err(anyhow!("Unrecognized direction {:?}", parts))
        }
    }

    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            let coordinate = Coordinate { row, col, rows, cols, orientation: RowOrientation::Normal };
            if grid[&coordinate] {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

pub fn day09_part2(path: &str) -> Result<i32> {
    let lines = read_lines(path);
    let rows = 500;
    let cols = 500;
    let mut grid: Grid<bool> = Grid::new(rows, cols);
    let mut knots = Vec::<Coordinate>::new();
    for _ in 0..10 {
        knots.push(Coordinate { rows, cols, row: 50, col: 50, orientation: RowOrientation::Normal });
    }
    grid[&knots[knots.len() - 1]] = true;

    for line in lines {
        println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>()?;
        match parts[0] {
            "R" => move_knots(&mut knots, &mut grid, Direction::Right, count)?,
            "L" => move_knots(&mut knots, &mut grid, Direction::Left, count)?,
            "U" => move_knots(&mut knots, &mut grid, Direction::Up, count)?,
            "D" => move_knots(&mut knots, &mut grid, Direction::Down, count)?,
            _ => return Err(anyhow!("Unrecognized direction {:?}", parts))
        }
        print_board(&grid, &knots);
    }

    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            let coordinate = Coordinate { row, col, rows, cols, orientation: RowOrientation::Normal };
            if grid[&coordinate] {
                sum += 1;
            }
        }
    }

    Ok(sum)
}

fn print_board(grid: &Grid<bool>, knots: &Vec<Coordinate>) {
    for row in (0..grid.rows).rev() {
        for col in 0..grid.cols {
            let mut char: String = ".".to_string();
            for i in (1..knots.len()).rev() {
                if row == knots[i].row && col == knots[i].col {
                    char = format!("{}", i);
                }
            }
            if row == knots[0].row && col == knots[0].col {
                char = "H".to_string();
            }
            print!("{}", char);
        }
        println!();
    }
}

fn move_knots(knots: &mut Vec<Coordinate>, grid: &mut Grid<bool>, direction: Direction, count: usize) -> Result<()> {
    for _ in 0..count {
        knots[0].increment_mut(&direction)?;
        for i in 1..knots.len() {
            move_tail(knots, i)?;
        }
        grid[&knots[knots.len()-1]] = true;
    }

    Ok(())
}

fn move_tail(knots: &mut Vec<Coordinate>, tail_index: usize) -> Result<()> {
    let row_diff = knots[tail_index - 1].row as i32 - knots[tail_index].row as i32;
    let col_diff = knots[tail_index - 1].col as i32 - knots[tail_index].col as i32;
    match (row_diff, col_diff) {
        (-2, 0) => { knots[tail_index].row -= 1; }
        (2, 0) => { knots[tail_index].row += 1; }
        (0, -2) => { knots[tail_index].col -= 1; }
        (0, 2) => { knots[tail_index].col += 1; }
        (-2, -1) | (-1, -2) | (-2,-2)  => {
            knots[tail_index].row -= 1;
            knots[tail_index].col -= 1;
        }
        (-2, 1) | (-1, 2) | (-2, 2) => {
            knots[tail_index].row -= 1;
            knots[tail_index].col += 1;
        }
        (1, -2) | (2, -1) | (2,-2) => {
            knots[tail_index].row += 1;
            knots[tail_index].col -= 1;
        }
        (1, 2) | (2,1) | (2,2) => {
            knots[tail_index].row += 1;
            knots[tail_index].col += 1;
        }
        _ => {}// do nothing
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
        assert_eq!(1, score);
        Ok(())
    }

    #[test]
    pub fn test_part2_2() -> Result<()> {
        let score = day09_part2("data/day09_test2.txt")?;
        println!("{}", score);
        assert_eq!(36, score);
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
