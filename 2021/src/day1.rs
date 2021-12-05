
pub mod day1 {
	use std::fs::File;
	use std::io::{self, BufRead};

    /// Returns the lines from a file;
    ///
    /// This isn't day1-specific and should be moved to a shared library when we start working on a
    /// new day.
	pub fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
	where P: AsRef<std::path::Path>, {
		let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        reader.lines().collect::<Result<_, _>>()
	}

    /// Reads the lines from a file containing a single integer per line and returns them as a
    /// collection of integers
    pub fn read_depths<P>(filename: P) -> Result<Vec<i32>, io::Error>
    where P: AsRef<std::path::Path>, {
        let lines = read_lines(filename)?;
        Ok(lines.iter().map(|s| s.parse::<i32>().unwrap()).collect())
    }


//    pub fn parse_lines() -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
//	}
}
