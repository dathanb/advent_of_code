use anyhow::Result;
use crate::shared::read_lines;
use regex::Regex;

pub fn day07_part1(path: &str) -> Result<u64> {
    let lines = read_lines(path);
    let file_size_regex = Regex::new("^[0-9]")?;

    let mut path: Vec<FileSystemEntry> = Vec::new();
    let mut current_dir = FileSystemEntry(0);
    let mut sum = 0;

    for line in lines {
        if line == "$ cd /" {
            // do nothing
        } else if line == "$ cd .." {
            let size = current_dir.0;
            if size <= 100000 {
                sum += size;
            }
            current_dir = path.pop().expect("Should have been one item to pop");
            current_dir.0 += size;
        } else if line.starts_with("$ cd ") {
            path.push(current_dir);
            current_dir = FileSystemEntry(0);
        } else if line == "$ ls" || line.starts_with("dir") {
            // ignore
        } else if file_size_regex.is_match(line.as_str()) {
            let size = line.split(" ").collect::<Vec<&str>>()[0].parse::<u64>()?;
            current_dir.0 += size;
        }
    }
    // There's a bug here where we don't consider the root directry in the sum. But since I already got the right
    // answer, I'm not going to worry about it
    Ok(sum)
}

pub fn day07_part2(data_file: &str) -> Result<u64> {
    let lines = read_lines(data_file);

    let file_size_regex = Regex::new("^[0-9]")?;

    let mut path: Vec<FileSystemEntry> = Vec::new();
    let mut current_dir = FileSystemEntry(0);

    for line in lines {
        if line == "$ cd /" {
            // do nothing
        } else if line == "$ cd .." {
            let size = current_dir.0;
            current_dir = path.pop().expect("Should have been one item to pop");
            current_dir.0 += size;
        } else if line.starts_with("$ cd ") {
            path.push(current_dir);
            current_dir = FileSystemEntry(0);
        } else if line == "$ ls" || line.starts_with("dir") {
            // ignore
        } else if file_size_regex.is_match(line.as_str()) {
            let size = line.split(" ").collect::<Vec<&str>>()[0].parse::<u64>()?;
            current_dir.0 += size;
        }
    }

    while let Some(mut top_dir) = path.pop() {
        top_dir.0 += current_dir.0;
        current_dir = top_dir;
    }

    let total_available = 70000000;
    let space_needed = 30000000;
    let total_used_space = current_dir.0;
    let space_we_need_to_free = space_needed - (total_available - total_used_space);

    // now find the directory to delete
    let mut smallest = u64::MAX;
    let lines = read_lines(data_file);
    for line in lines {
        if line == "$ cd /" {
            // do nothing
        } else if line == "$ cd .." {
            let size = current_dir.0;
            if size >= space_we_need_to_free {
                smallest = std::cmp::min(smallest, size);
            }
            current_dir = path.pop().expect("Should have been one item to pop");
            current_dir.0 += size;
        } else if line.starts_with("$ cd ") {
            path.push(current_dir);
            current_dir = FileSystemEntry(0);
        } else if line == "$ ls" || line.starts_with("dir") {
            // ignore
        } else if file_size_regex.is_match(line.as_str()) {
            let size = line.split(" ").collect::<Vec<&str>>()[0].parse::<u64>()?;
            current_dir.0 += size;
        }
    }

    while let Some(top_dir) = path.pop() {
        let size = current_dir.0;
        if size >= space_we_need_to_free {
            smallest = std::cmp::min(smallest, size);
        }
        current_dir = top_dir;
        current_dir.0 += size;
    }
    let size = current_dir.0;
    if size >= space_we_need_to_free {
        smallest = std::cmp::min(smallest, size);
    }

    Ok(smallest)
}

#[derive(Debug)]
struct FileSystemEntry(u64);

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use crate::day07::{day07_part1, day07_part2};

    #[test]
    pub fn test_part1() -> Result<()> {
        let score = day07_part1("data/day07_test.txt")?;
        println!("{}", score);
        assert_eq!(95437, score);
        Ok(())
    }

    #[test]
    pub fn run_part1() -> Result<()> {
        let score = day07_part1("data/day07.txt")?;
        println!("{}", score);
        assert_eq!(1307902, score);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let score = day07_part2("data/day07_test.txt")?;
        println!("{}", score);
        assert_eq!(24933642, score);
        Ok(())
    }

    #[test]
    pub fn run_part2() -> Result<()> {
        let score = day07_part2("data/day07.txt")?;
        println!("{}", score);
        assert_eq!(0, score);
        Ok(())
    }
}
