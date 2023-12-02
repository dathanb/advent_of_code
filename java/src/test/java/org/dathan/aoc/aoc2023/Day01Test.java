package org.dathan.aoc.aoc2023;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day01Test {
    @Test
    public void part1TestData() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day01-part1-test.txt"));

        assertEquals(142, new Day01().part1(input));
    }

    @Test
    public void part1() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day01.txt"));

        assertEquals(54667, new Day01().part1(input));
    }

    @Test
    public void part2TestData() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day01-part2-test.txt"));

        assertEquals(281, new Day01().part2(input));
    }

    @Test
    public void part2ExtraTestData() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day01-part2-extratest.txt"));

        assertEquals(312, new Day01().part2(input));
    }

    @Test
    public void part2() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day01.txt"));

        assertEquals(54203, new Day01().part2(input));
    }

}
