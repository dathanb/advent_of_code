package org.dathan.aoc.aoc2015;

import org.dathan.aoc.aoc2023.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day06Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(569999, new Day06().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(17836115, new Day06().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        assertEquals(1_000_000, new Day06().part1(Collections.singletonList("turn on 0,0 through 999,999")));
        assertEquals(1_000, new Day06().part1(Collections.singletonList("toggle 0,0 through 999,0")));
        assertEquals(0, new Day06().part1(Collections.singletonList("turn off 499,499 through 500,500")));
    }

    @Test
    public void part2ProvidedTestData() {
        assertEquals(1, new Day06().part2(Collections.singletonList("turn on 0,0 through 0,0")));
        assertEquals(2_000_000, new Day06().part2(Collections.singletonList("toggle 0,0 through 999,999")));
    }

    private List<String> getProblemInput() {
        return org.dathan.aoc.IOUtil.getLines(org.dathan.aoc.IOUtil.readFromResource("/2015/day06.txt"));
    }

}