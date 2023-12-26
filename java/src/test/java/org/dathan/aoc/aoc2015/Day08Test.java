package org.dathan.aoc.aoc2015;

import org.dathan.aoc.IOUtil;
import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class Day08Test {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(1333, new Day08().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(2046, new Day08().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
""
"abc"
"aaa\\"aaa"
"\\x27"
""");
        assertEquals(12, new Day08().part1(input));
        assertEquals(3, new Day08().part1(Collections.singletonList("\"\\\"")));

    }

    @Test
    public void part2ProvidedTestData() {
//        assertEquals(1, new Day08().part2(Collections.singletonList("turn on 0,0 through 0,0")));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/day08.txt"));
    }

}