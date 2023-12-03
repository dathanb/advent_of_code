package org.dathan.aoc.aoc2015;

import org.dathan.aoc.Coordinate;

import java.util.HashMap;

import static org.dathan.aoc.MathUtil.min;

public class Day03 {
    public long part1(String input) {
        long multipleCount = 0;
        HashMap<Coordinate, Integer> visits = new HashMap<>();
        Coordinate position = Coordinate.ORIGIN;
        visits.put(position, 1);

        for (char ch: input.toCharArray()) {
            position = position.move(ch);
            visits.compute(position, (p, c) -> c == null ? 0 : c + 1);
        }

        return visits.size();
    }

    public long part2(String input) {
        long multipleCount = 0;
        HashMap<Coordinate, Integer> visits = new HashMap<>();
        Coordinate currentPosition = Coordinate.ORIGIN;
        Coordinate otherPosition = Coordinate.ORIGIN;
        visits.put(Coordinate.ORIGIN, 2);

        for (char ch: input.toCharArray()) {
            currentPosition = currentPosition.move(ch);
            visits.compute(currentPosition, (p, c) -> c == null ? 0 : c + 1);

            Coordinate temp = currentPosition;
            currentPosition = otherPosition;
            otherPosition = temp;
        }

        return visits.size();
    }
}

