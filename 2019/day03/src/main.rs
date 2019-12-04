use std::collections::{HashMap, HashSet};
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

    part2(&lines);

    Ok(())
}

fn part1(lines: &Vec<&str>) {
    let first_wire_points = get_point_set(lines[0]);
    let second_wire_points = get_point_set(lines[1]);

    let first_wire_coordinates: HashSet<&Coordinate> = first_wire_points.keys().collect();
    let second_wire_coordinates: HashSet<&Coordinate> = second_wire_points.keys().collect();
    let intersection_points = first_wire_coordinates.intersection(&second_wire_coordinates);

    println!("{}", intersection_points.map(|c| Coordinate { x: 0, y: 0 }.distance(**c)).min().unwrap());
}

fn part2(lines: &Vec<&str>) {
    let first_wire = get_point_set(lines[0]);
    let second_wire = get_point_set(lines[1]);

    let mut min_distance: i64 = std::i32::MAX.into();
    for coordinate in first_wire.keys() {
        let distance = first_wire.get(coordinate)
            .map_or(std::i32::MAX.into(), |d| d + second_wire.get(coordinate).unwrap_or(&std::i32::MAX.into()));
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("{}", min_distance);
}

fn get_point_set(line: &str) -> HashMap<Coordinate, i64> {
    let mut all_points: HashMap<Coordinate, i64> = HashMap::new();
    let movements: Vec<Movement> = line.split(",")
        .map(|n| n.trim())
        .map(|d| Movement::parse(d))
        .collect();

    let mut current_coordinate = Coordinate { x: 0, y: 0 };
    let mut steps = 0;
    movements.iter()
        .flat_map(|m| m.to_units())
        .for_each(|m| {
            steps += 1;
            current_coordinate = current_coordinate.apply(m);
            all_points.insert(current_coordinate, steps);
        });

    all_points
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Coordinate {
    x: i64,
    y: i64,
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

    fn distance(&self, other: Coordinate) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Movement<'a> {
    direction: &'a str,
    distance: i64,
}

impl<'a> Movement<'a> {
    fn parse(text: &str) -> Movement {
        let mut movement: Movement = Movement { direction: "", distance: 0 };
        match text.chars().nth(0) {
            Some('U') => movement.direction = "U",
            Some('D') => movement.direction = "D",
            Some('L') => movement.direction = "L",
            Some('R') => movement.direction = "R",
            _ => panic!("Unrecognized direction in string {}", text),
        }

        movement.distance = text[1..text.len()].parse::<i64>().unwrap();

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

