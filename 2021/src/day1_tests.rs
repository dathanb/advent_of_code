#[cfg(test)]
mod day1_tests {
	use crate::day1::day1::read_lines;

    #[test]
    pub fn it_works() {
		println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        if let Ok(_lines) = read_lines("./src/day1_input.txt") {
		} else {
			panic!("Failed to read lines");
		}
    }
}
