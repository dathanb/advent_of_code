package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day02Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(1606483, new Day02().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(3842356, new Day02().part2(input));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day02.txt"));
    }
}
