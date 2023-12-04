package org.dathan.aoc.aoc2023;

import org.dathan.aoc.Coordinate;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day03 {
    public long part1(List<String> input) {
        HashMap<Coordinate, Integer> partNumberLocation = new HashMap<>();
        HashMap<Integer, Integer> partNumbers = new HashMap<>();

        int partNumberCount = 0;

        // extract all the numbers
        for (int lineNumber = 0; lineNumber < input.size(); lineNumber++) {
            char[] chars = input.get(lineNumber).toCharArray();
            for (int x = 0; x < chars.length; x++) {
                String numString;
                if ((numString = getNumber(chars, x)) != null) {
                    int partNumber = Integer.parseInt(numString);
                    partNumbers.put(partNumberCount, partNumber);
                    for (int n = x; n < x + numString.length(); n++) {
                        partNumberLocation.put(new Coordinate(n, lineNumber), partNumberCount);
                    }
                    partNumberCount += 1;
                    x += numString.length()-1;
                }
            }
        }

        long sum = 0;
        // process all the symbols
        for (int lineNumber = 0; lineNumber < input.size(); lineNumber++) {
            char[] chars = input.get(lineNumber).toCharArray();
            for (int x = 0; x < chars.length; x++) {
                if ((chars[x] >= '0' && chars[x] <= '9') || (chars[x] == '.')) {
                    continue;
                }

                // we have a symbol, let's get all the part numbers it's adjacent to.
                Set<Integer> adjacentPartNumbers = new HashSet<>();
                for (int dx = -1; dx < 2; dx++) {
                    for (int dy = -1; dy < 2; dy++) {
                        // if the adjacent square is out of bounds, ignore it and try the next one
                        if (lineNumber + dy < 0 || lineNumber + dy >= input.size() || x + dx < 0 || x + dx >= chars.length) {
                            continue;
                        }
                        Coordinate adjacentCoordinate = new Coordinate(x + dx, lineNumber + dy);
                        if (partNumberLocation.containsKey(adjacentCoordinate)) {
                            adjacentPartNumbers.add(partNumberLocation.get(adjacentCoordinate));
                        }

                    }
                }
                for (int partNumberIndex: adjacentPartNumbers) {
                    sum += partNumbers.get(partNumberIndex);
                }
            }
        }

        return sum;
    }

    public long part2(List<String> input) {
        HashMap<Coordinate, Integer> partNumberLocation = new HashMap<>();
        HashMap<Integer, Integer> partNumbers = new HashMap<>();

        int partNumberCount = 0;

        // extract all the numbers
        for (int lineNumber = 0; lineNumber < input.size(); lineNumber++) {
            char[] chars = input.get(lineNumber).toCharArray();
            for (int x = 0; x < chars.length; x++) {
                String numString;
                if ((numString = getNumber(chars, x)) != null) {
                    int partNumber = Integer.parseInt(numString);
                    partNumbers.put(partNumberCount, partNumber);
                    for (int n = x; n < x + numString.length(); n++) {
                        partNumberLocation.put(new Coordinate(n, lineNumber), partNumberCount);
                    }
                    partNumberCount += 1;
                    x += numString.length()-1;
                }
            }
        }

        long sum = 0;
        // process all the symbols
        for (int lineNumber = 0; lineNumber < input.size(); lineNumber++) {
            char[] chars = input.get(lineNumber).toCharArray();
            for (int x = 0; x < chars.length; x++) {
                if (chars[x] != '*') {
                    continue;
                }

                // we have a symbol, let's get all the part numbers it's adjacent to.
                Set<Integer> adjacentPartNumbers = new HashSet<>();
                for (int dx = -1; dx < 2; dx++) {
                    for (int dy = -1; dy < 2; dy++) {
                        // if the adjacent square is out of bounds, ignore it and try the next one
                        if (lineNumber + dy < 0 || lineNumber + dy >= input.size() || x + dx < 0 || x + dx >= chars.length) {
                            continue;
                        }
                        Coordinate adjacentCoordinate = new Coordinate(x + dx, lineNumber + dy);
                        if (partNumberLocation.containsKey(adjacentCoordinate)) {
                            adjacentPartNumbers.add(partNumberLocation.get(adjacentCoordinate));
                        }
                    }
                }

                if (adjacentPartNumbers.size() != 2) {
                    continue;
                }

                long gearRatio = adjacentPartNumbers.stream()
                        .mapToLong(n -> partNumbers.get(n))
                        .reduce(1, (accum, element) -> accum * element);
                sum += gearRatio;

            }
        }

        return sum;
    }

    private String getNumber(char[] chars, int startIndex) {
        if (chars[startIndex] < '0' || chars[startIndex] > '9') {
            return null;
        }
        StringBuilder sb = new StringBuilder();
        int index = startIndex;
        while (index < chars.length && chars[index] >= '0' && chars[index] <= '9') {
            sb.append(chars[index]);
            index += 1;
        }

        return sb.toString();
    }
}