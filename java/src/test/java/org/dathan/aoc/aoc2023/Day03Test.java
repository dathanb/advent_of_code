package org.dathan.aoc.aoc2023;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day03Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(546312, new Day03().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(87449461, new Day03().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
""");
        assertEquals(4361, new Day03().part1(input));
    }

    @Test
    public void part2ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
""");
        assertEquals(467835, new Day03().part2(input));
    }

    private List<String> getProblemInput() {
        return org.dathan.aoc.IOUtil.getLines(org.dathan.aoc.IOUtil.readFromResource("/2023/day03.txt"));
    }

}