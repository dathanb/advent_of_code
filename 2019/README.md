# Advent of Code 2019

This year, it's all Rust!

## Day 01

How to read a file line by line:

```
let file = File::open("resources/input.txt").unwrap();

let reader = BufReader::new(file);

let mut total: i32 = 0;

for line_result in reader.lines() {
  let line = line_result.expect("Error getting next line from file.");
  let num: i32 = line.parse().unwrap();
  let weight = (num /3)-2;
  total += weight;
}
```

## Day 02

How to read a file all at once:
```
    let contents = fs::read_to_string("resources/input.txt").unwrap();
```

## Day 03

We're going to be reading in coordinates for two wires. It doens't look offhand like we need to validate anything about
the wires, so the simplest thing to do seems to be to be to 
1. calculate coordinates as we read the wire positions, and add all those coordinates to a set. 
2. Do that for the second wire in a second set
3. Calculate the intersection of the two sets.
4. Iterate over that result, and pick the one with the lowest Manhattan distance.

We could encode the coordinates into a string or soemthing, but let's see what it takes to define a type we can stick
into a set in Rust. We're here to learn after all!


