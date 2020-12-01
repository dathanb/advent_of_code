package com.jedaway.adventofcode.aoc2020.day1;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.net.URISyntaxException;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day1Test {
    @Test
    public void testDay1() throws IOException, URISyntaxException {
        Day1.Day1Output result = Day1.load(getClass().getClassLoader().getResource("day1_testinput.txt").toURI()).run();
        assertEquals(514579, result.product);
    }

    @Test
    public void runDay1() throws URISyntaxException, IOException {
        Day1.Day1Output result = Day1.load(getClass().getClassLoader().getResource("day1_input.txt").toURI()).run();
        assertEquals(440979, result.product);
    }
}
