package org.dathan.aoc.aoc2015;

import java.util.Arrays;
import java.util.List;

public class Day01 {
    public long part1(String input) {
        int floor = 0;
        for (char ch: input.toCharArray()) {
            floor += switch(ch) {
                case '(' -> 1;
                case ')' -> -1;
                default -> 0;
            };
        }

        return floor;
    }

    public long part2(String input) {
        int floor = 0;

        char[] charArray = input.toCharArray();
        for (int index = 0; index < charArray.length; index++) {
            floor += switch(charArray[index]) {
                case '(' -> 1;
                case ')' -> -1;
                default -> 0;
            };
            if (floor < 0) {
                return index + 1;
            }
        }

        return 0;
    }
}
