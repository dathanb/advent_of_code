package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day03Test {
    @Test
    public void part1() {
        String input = getProblemInput().get(0);
        assertEquals(2081, new Day03().part1(input));
    }

    @Test
    public void part1Test1() {
        assertEquals(2, new Day03().part1(">"));
        assertEquals(4, new Day03().part1("^>v<"));
        assertEquals(2, new Day03().part1("^v^v^v^v^v"));
    }

    @Test
    public void part2() {
        String input = getProblemInput().get(0);
        assertEquals(2341, new Day03().part2(input));
    }

    @Test
    public void part2TestData() {
        Day03 solution = new Day03();
        assertEquals(3, solution.part2("^v"));
        assertEquals(3, solution.part2("^>v<"));
        assertEquals(11, solution.part2("^v^v^v^v^v"));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day03.txt"));
    }

}
