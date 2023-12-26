package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day07Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(3176, new Day07().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(14710, new Day07().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
123 -> x
456 -> y
x AND y -> a
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
""");
        assertEquals(72, new Day07().part1(input));
    }

    @Test
    public void part2ProvidedTestData() {
        assertEquals(1, new Day07().part2(Collections.singletonList("turn on 0,0 through 0,0")));
        assertEquals(2_000_000, new Day07().part2(Collections.singletonList("toggle 0,0 through 999,999")));
    }

    private List<String> getProblemInput() {
        return org.dathan.aoc.IOUtil.getLines(org.dathan.aoc.IOUtil.readFromResource("/2015/day07.txt"));
    }

}