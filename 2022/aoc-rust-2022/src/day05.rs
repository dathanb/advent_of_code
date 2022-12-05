use anyhow::Result;
use crate::shared::read_lines;
use regex::Regex;

pub fn day05_part1(path: &str) -> Result<String> {
    let lines: Vec<String> = read_lines(path);
    let mut stacks = get_stacks(&lines);
    let starting_line = stacks.iter().map(|s| s.len()).max().unwrap() + 2;

    for line in &lines[starting_line..] {
        let instructions:Vec<&str> = line.split(" ").collect();
        println!("{:?}", instructions);
        let from = instructions[3].parse::<usize>().expect("Couldn't parse as an integer");
        let to = instructions[5].parse::<usize>().expect("Couldn't parse as an integer");
        let num = instructions[1].parse::<usize>().expect("Couldn't parse as an integer");
        println!("move {} from {} to {}", num, from, to);

        for _ in 0..num {
            let string = stacks[from-1].pop().unwrap();
            stacks[to-1].push(string);
        }
    }

    let mut tops = Vec::<String>::new();
    for stack in stacks {
        if stack.len() > 0 {
            tops.push(stack[stack.len()-1].clone());
        }
    }

    Ok(tops.join(""))
}

pub fn day05_part2(path: &str) -> Result<String> {
    let lines = read_lines(path);

    let mut stacks = get_stacks(&lines);
    let starting_line = stacks.iter().map(|s| s.len()).max().unwrap() + 2;

    for line in &lines[starting_line..] {
        let instructions:Vec<&str> = line.split(" ").collect();
        println!("{:?}", instructions);
        let from = instructions[3].parse::<usize>().expect("Couldn't parse as an integer");
        let to = instructions[5].parse::<usize>().expect("Couldn't parse as an integer");
        let num = instructions[1].parse::<usize>().expect("Couldn't parse as an integer");
        println!("move {} from {} to {}", num, from, to);

        let mut temp_stack = Vec::new();
        for _ in 0..num {
            temp_stack.push(stacks[from-1].pop().unwrap());
        }

        for _ in 0..num {
            stacks[to-1].push(temp_stack.pop().unwrap());
        }
    }

    let mut tops = Vec::<String>::new();
    for stack in stacks {
        if stack.len() > 0 {
            tops.push(stack[stack.len()-1].clone());
        }
    }

    Ok(tops.join(""))
}


fn get_stacks(lines: &Vec<String>) -> Vec<Vec<String>> {
    // There are 4*n - 1 characters in the first line, where n is the number of stacks.
    // So n = (len+1)/4
    let num_stacks = (lines[0].len() + 1)/4;
    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    for i in 0..lines.len() {
        if lines[i].contains("1") {
            break;
        }
        let pattern = Regex::new(r"\s{1,4}").expect("Invalid regex");
        // let columns:Vec<&str> = lines[i].split(&pattern).collect();
        let columns:Vec<&str> = pattern.split(&lines[i]).collect();
        println!("{:?}", columns);

        for i in 0..num_stacks {
            if columns[i].len() > 0 {
                stacks[i].insert(0, columns[i].replace("[", "").replace("]", "").to_string());
            }
        }

    }

    stacks
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day05::{day05_part1, day05_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let code = day05_part1("data/day05_test.txt")?;
        println!("{}", code);
        assert_eq!("CMZ", code);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let code = day05_part1("data/day05.txt")?;
        println!("{}", code);
        assert_eq!("ZWHVFWQWW", code);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let code = day05_part2("data/day05_test.txt")?;
        println!("{}", code);
        assert_eq!("MCD", code);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let code = day05_part2("data/day05.txt")?;
        println!("{}", code);
        assert_eq!("HZFZCCWWV", code);
        Ok(())
    }
}
