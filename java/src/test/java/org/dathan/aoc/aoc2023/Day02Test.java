package org.dathan.aoc.aoc2023;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day02Test {
    @Test
    public void part1() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day02.txt"));
        assertEquals(2149, new Day02().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day02.txt"));
        assertEquals(71274, new Day02().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day02-part1-test.txt"));
        assertEquals(8, new Day02().part1(input));
    }

    @Test
    public void part2ProvidedTestData() {
        List<String> input = IOUtil.getLines(IOUtil.readFromResource("/2023/day02-part1-test.txt"));
        assertEquals(2286, new Day02().part2(input));
    }


}