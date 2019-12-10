use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
fn part1() -> Result<String, String> {
    let mut orbits = get_input()?;

    let mut keys: Vec<String> = Vec::new();
    for satellite in orbits.keys() {
        keys.push(satellite.clone());
    }

    for satellite in &keys {
        record_depth(&mut orbits, &satellite);
    }

    let mut total = 0;
    for satellite in &keys {
        total += orbits[satellite].depth.unwrap();
    }

    Ok(format!("{}", total))
}

#[allow(dead_code)]
fn part2() -> Result<String, String> {
    let orbits = get_input()?;

    let mut you_path : Vec<String> = Vec::new();
    path_to_root(&mut you_path, &orbits, String::from("YOU"));
    let mut santa_path: Vec<String> = Vec::new();
    path_to_root(&mut santa_path, &orbits, String::from("SAN"));

    while you_path[you_path.len()-1] == santa_path[santa_path.len()-1] {
        you_path.remove(you_path.len()-1);
        santa_path.remove(santa_path.len()-1);
    }

    print_vec(&you_path);
    print_vec(&santa_path);


    Ok(format!("{}", you_path.len() + santa_path.len() - 2))
}

fn get_input() -> Result<HashMap<String, Orbit>, String> {
    let file = match File::open("resources/day06.txt") {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening file: {}", err)),
    };

    let mut orbits: HashMap<String, Orbit> = HashMap::new();
    let reader  = BufReader::new(file);
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(s) => s,
            Err(n) => return Err(format!("{}", n)),
        };

        let parts:Vec<String> = line.trim()
            .split(")")
            .map(|s| s.trim().to_string())
            .collect();

        orbits.insert(parts[1].clone(), Orbit::new(parts[0].clone(), parts[1].clone()));
    }

    Ok(orbits)
}

fn record_depth(orbits: &mut HashMap<String, Orbit>, satellite: &String) -> i32 {
    if satellite == "COM" {
        return 0;
    }

    let orbit = &orbits[satellite];
    if orbit.depth.is_some() {
        return orbit.depth.unwrap();
    }

    let mut orbit = orbit.clone();
    let depth = record_depth(orbits, &orbit.center)+1;
    orbit.depth = Option::Some(depth);
    orbits.insert(satellite.clone(), orbit);

    return depth;
}

fn path_to_root(path: &mut Vec<String>, orbits: &HashMap<String, Orbit>, satellite: String) {
    path.push(satellite.clone());

    if satellite == "COM" {
        return;
    }

    path_to_root(path, orbits, orbits[&satellite].center.clone());
}

fn print_vec(path: &Vec<String>) {
    for c in 0..path.len() {
        print!("{},", path[c]);
    }
    println!();
}

#[derive(Clone)]
struct Orbit {
    center: String,
    satellite: String,
    depth: Option<i32>,
}

impl Orbit {
    fn new(center: String, satellite: String) -> Orbit {
        Orbit {
            center,
            satellite,
            depth: Option::None,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let value = part1().unwrap();
        assert_eq!(value, "314702");
    }

    #[test]
    fn test_part2() {
        let value = part2().unwrap();
        assert_eq!(value, "1");
    }
}
