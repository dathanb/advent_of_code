use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};

fn main() -> io::Result<()> {
    let file = File::open("resources/input.txt").unwrap();
    let reader = BufReader::new(file);

    let line_strings: Vec<String> = reader.lines()
        .map(|l| l.unwrap())
        .collect();
    let lines: Vec<&str> = line_strings.iter()
        .map(String::as_str).collect();

    part1(&lines);

//    println!("{}", part2(lines));

    Ok(())
}

fn part1(lines: &Vec<&str>) {
    let set1 = get_point_set(lines[0]);
    let set2 = get_point_set(lines[1]);

    let intersection = set1.intersection(&set2);

    println!("{}", intersection.map(|c| Coordinate{x:0, y:0}.distance(*c)).min().unwrap());
}

//fn part2(_lines: &Vec<&str>) -> &'static str {
////    let set1 = get_point_set(lines[0]);
////    let set2 = get_point_set(lines[1]);
//
//    return "";
//}

fn get_point_set(line: &str) -> HashSet<Coordinate> {
    let mut all_points: HashSet<Coordinate> = HashSet::new();
    let movements: Vec<Movement> = line.split(",")
        .map(|n| n.trim())
        .map(|d| Movement::parse(d))
        .collect();

    let mut current_coordinate = Coordinate { x: 0, y: 0 };
    movements.iter()
        .flat_map(|m| m.to_units())
        .for_each(|m| {
            current_coordinate = current_coordinate.apply(m);
            all_points.insert(current_coordinate);
        });

    all_points
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn apply(&self, movement: Movement) -> Coordinate {
        match movement {
            Movement { direction: "U", distance: n } => Coordinate { x: self.x, y: self.y + n },
            Movement { direction: "D", distance: n } => Coordinate { x: self.x, y: self.y - n },
            Movement { direction: "L", distance: n } => Coordinate { x: self.x - n, y: self.y },
            Movement { direction: "R", distance: n } => Coordinate { x: self.x + n, y: self.y },
            _ => panic!("Unrecognized direction: {}", movement.direction)
        }
    }

    fn distance(&self, other: Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Movement<'a> {
    direction: &'a str,
    distance: i32,
}

impl<'a> Movement<'a> {
    fn parse(text: &str) -> Movement {
        let mut movement: Movement = Movement {direction: "", distance: 0};
        match text.chars().nth(0) {
            Some('U') => movement.direction = "U",
            Some('D') => movement.direction = "D",
            Some('L') => movement.direction = "L",
            Some('R') => movement.direction = "R",
            _ => panic!("Unrecognized direction in string {}", text),
        }

        movement.distance = text[1..text.len()].parse::<i32>().unwrap();

        movement
    }

    fn unit(&self) -> Movement {
        Movement { direction: self.direction, distance: 1 }
    }

    fn to_units(&self) -> Vec<Movement> {
        let mut units: Vec<Movement> = Vec::new();

        for _ in 0..self.distance {
            units.push(self.unit());
        }

        units
    }
}

