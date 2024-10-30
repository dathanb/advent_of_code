package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day09Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(251, new Day09().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(898, new Day09().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
""");
        assertEquals(605, new Day09().part1(input));
    }

    @Test
    public void part2ProvidedTestData() {
//        assertEquals(1, new Day09().part2(Collections.singletonList("turn on 0,0 through 0,0")));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day09.txt"));
    }

}