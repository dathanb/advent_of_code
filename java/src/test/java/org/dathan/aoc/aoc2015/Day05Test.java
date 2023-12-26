package org.dathan.aoc.aoc2015;

import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day05Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(236, new Day05().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(51, new Day05().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        assertEquals(1, new Day05().part1(Collections.singletonList("ugknbfddgicrmopn")));
        assertEquals(1, new Day05().part1(Collections.singletonList("aaa")));
        assertEquals(0, new Day05().part1(Collections.singletonList("jchzalrnumimnmhp")));
        assertEquals(0, new Day05().part1(Collections.singletonList("haegwjzuvuyypxyu")));
        assertEquals(0, new Day05().part1(Collections.singletonList("dvszwmarrgswjxmb")));
    }

    @Test
    public void part2ProvidedTestData() {
        assertEquals(1, new Day05().part2(Collections.singletonList("qjhvhtzxzqqjkmpb")));
        assertEquals(1, new Day05().part2(Collections.singletonList("xxyxx")));
        assertEquals(0, new Day05().part2(Collections.singletonList("uurcxstgmygtbstg")));
        assertEquals(0, new Day05().part2(Collections.singletonList("ieodomkazucvgmuy")));
    }

    private List<String> getProblemInput() {
        return org.dathan.aoc.IOUtil.getLines(org.dathan.aoc.IOUtil.readFromResource("/2015/day05.txt"));
    }

}