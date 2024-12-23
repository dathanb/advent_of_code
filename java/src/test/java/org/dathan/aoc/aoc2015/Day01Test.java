package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day01Test {
    @Test
    public void part1() {
        String input = getProblemInput().get(0);
        assertEquals(280, new Day01().part1(input));
    }

    @Test
    public void part2() {
        String input = getProblemInput().get(0);
        assertEquals(1797, new Day01().part2(input));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day01.txt"));
    }
}
