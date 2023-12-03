package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day04Test {
    @Test
    public void part1() {
        String input = getProblemInput().get(0);
        assertEquals(254575, new Day04().part1(input));
    }

    @Test
    public void part1Test1() {
        assertEquals(609043L, new Day04().part1("abcdef"));
        assertEquals(1048970L, new Day04().part1("pqrstuv"));
    }

    @Test
    public void part2() {
        String input = getProblemInput().get(0);
        assertEquals(1038736, new Day04().part2(input));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day04.txt"));
    }
}
