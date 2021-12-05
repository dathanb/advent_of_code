
pub mod day1 {
	use std::fs::File;
	use std::io::{self, BufRead};

	pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<std::path::Path>, {
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines())
	}

//    pub fn parse_lines() -> io::Result<io::Lines<io::BufReader<std::fs::File>>> {
//	}
}
