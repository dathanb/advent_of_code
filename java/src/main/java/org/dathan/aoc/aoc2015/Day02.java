package org.dathan.aoc.aoc2015;

import java.util.List;

import static org.dathan.aoc.MathUtil.min;

public class Day02 {
    public long part1(List<String> input) {
        long sum = 0;
        for (var line : input) {
            String[] sides = line.split("x");
            int length = Integer.parseInt(sides[0]);
            int width = Integer.parseInt(sides[1]);
            int height = Integer.parseInt(sides[2]);

            int side1 = length * width;
            int side2 = width * height;
            int side3 = length * height;

            int smallestSide = min(side1, side2, side3);

            sum += 2 * side1 + 2 * side2 + 2 * side3 + smallestSide;
        }
        return sum;
    }

    public long part2(List<String> input) {
        long sum = 0;
        for (var line : input) {
            String[] sides = line.split("x");
            int length = Integer.parseInt(sides[0]);
            int width = Integer.parseInt(sides[1]);
            int height = Integer.parseInt(sides[2]);

            int perimeter1 = 2 * length + 2 * width;
            int perimeter2 = 2 * width + 2 * height;
            int perimeter3 = 2 * length + 2 * height;

            int smallestPerimeter = min(perimeter1, perimeter2, perimeter3);
            int volume = length * width * height;

            sum += smallestPerimeter + volume;
        }
        return sum;
    }
}
