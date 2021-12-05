#[cfg(test)]
mod day1_tests {
	use crate::day1::day1::read_lines;
	use crate::day1::day1::read_depths;

    #[test]
    pub fn read_lines_works() {
        let lines = read_lines("./src/read_lines_test_input.txt").unwrap();
        assert_eq!("500", lines[0]);
        assert_eq!("1000", lines[1]);
    }

    #[test]
    pub fn read_depths_works() {
        let depths = read_depths("./src/read_lines_test_input.txt").unwrap();
        assert_eq!(500, depths[0]);
        assert_eq!(1000, depths[1]);
    }

}
