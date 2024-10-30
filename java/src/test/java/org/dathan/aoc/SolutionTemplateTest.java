package org.dathan.aoc;

import org.junit.jupiter.api.Test;

import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class SolutionTemplateTest {
    @Test
    public void part1() {
        List<String> input = getProblemInput();
        assertEquals(251, new SolutionTemplate().part1(input));
    }

    @Test
    public void part2() {
        List<String> input = getProblemInput();
        assertEquals(2046, new SolutionTemplate().part2(input));
    }

    @Test
    public void part1ProvidedTestData() {
        List<String> input = IOUtil.getLines("""
""
"abc"
"aaa\\"aaa"
"\\x27"
""");
        assertEquals(12, new SolutionTemplate().part1(input));
        assertEquals(3, new SolutionTemplate().part1(Collections.singletonList("\"\\\\\"")));
    }

    @Test
    public void part2ProvidedTestData() {
//        assertEquals(1, new Template().part2(Collections.singletonList("turn on 0,0 through 0,0")));
    }

    private List<String> getProblemInput() {
        return IOUtil.getLines(IOUtil.readFromResource("/2015/template.txt"));
    }

}