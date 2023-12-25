package org.dathan.aoc.aoc2015;

import com.google.common.collect.Range;

import java.util.*;
import java.util.function.Function;

public class Day06 {
    public long part1(List<String> input) {
        int[][] field = new int[1000][1000];
        for (String line: input) {
            Function<Integer, Integer> operation;
            int nextToken = 0;
            if (line.startsWith("turn on")) {
                operation = v -> 1;
                nextToken = 8;
            } else if (line.startsWith("turn off")) {
                operation = v -> 0;
                nextToken = 9;
            } else if (line.startsWith("toggle")) {
                operation = v -> v == 0 ? 1 : 0;
                nextToken = 7;
            } else {
                throw new RuntimeException(String.format("Unrecognized input %s", line));
            }

            String restOfLine = line.substring(nextToken);
            Scanner scanner = new Scanner(restOfLine).useDelimiter("[ ,]");
            int startX = scanner.nextInt();
            int startY = scanner.nextInt();
            scanner.next(); // consume "through"
            int endX = scanner.nextInt();
            int endY = scanner.nextInt();

            for (int x=startX; x<= endX; x++) {
                for (int y=startY; y<=endY; y++) {
                    field[y][x] = operation.apply(field[y][x]);
                }
            }
        }

        long countOfOnCells = 0;
        for (int x=0; x<1000; x++) {
            for (int y=0; y<1000; y++) {
                countOfOnCells += field[y][x];
            }
        }

        return countOfOnCells;
    }

    public long part2(List<String> input) {
        int[][] field = new int[1000][1000];
        for (String line: input) {
            Function<Integer, Integer> operation;
            int nextToken = 0;
            if (line.startsWith("turn on")) {
                operation = v -> v + 1;
                nextToken = 8;
            } else if (line.startsWith("turn off")) {
                operation = v -> v == 0 ? 0 : v - 1;
                nextToken = 9;
            } else if (line.startsWith("toggle")) {
                operation = v -> v + 2;
                nextToken = 7;
            } else {
                throw new RuntimeException(String.format("Unrecognized input %s", line));
            }

            String restOfLine = line.substring(nextToken);
            Scanner scanner = new Scanner(restOfLine).useDelimiter("[ ,]");
            int startX = scanner.nextInt();
            int startY = scanner.nextInt();
            scanner.next(); // consume "through"
            int endX = scanner.nextInt();
            int endY = scanner.nextInt();

            for (int x=startX; x<= endX; x++) {
                for (int y=startY; y<=endY; y++) {
                    field[y][x] = operation.apply(field[y][x]);
                }
            }
        }

        long countOfOnCells = 0;
        for (int x=0; x<1000; x++) {
            for (int y=0; y<1000; y++) {
                countOfOnCells += field[y][x];
            }
        }

        return countOfOnCells;
    }

    private static boolean hasTriplet(String line) {
        char[] chars = line.toCharArray();
        boolean hasTriplet = false;
        for (int i = 0; i < chars.length - 2; i++) {
            if (chars[i] == chars[i+2]) {
                hasTriplet = true;
                break;
            }
        }
        return hasTriplet;
    }

    private static boolean hasRepeatedPair(String line) {
        Map<String, List<Range<Integer>>> pairs = new HashMap<>();
        boolean hasRepeatedPair = false;

        for (int i = 0; i < line.length() && !hasRepeatedPair; i++) {
            if (i < line.length() - 1) {
                String pair = line.substring(i, i + 2);
                Range<Integer> thisRange = Range.closed(i, i + 1);
                List<Range<Integer>> ranges = pairs.computeIfAbsent(pair, (key) -> new ArrayList<>());
                for (var range : ranges) {
                    if (!range.isConnected(thisRange)) {
                        hasRepeatedPair = true;
                    }
                }
                ranges.add(thisRange);
            }
        }
        return hasRepeatedPair;
    }

    private static class Part1Scanner {
        public boolean scan(String input) {
            int vowelCount = 0;
            char previousChar = 0;
            boolean hasDoubleLetter = false;

            for (char ch: input.toCharArray()) {
                if ((previousChar == 'a' && ch == 'b') ||
                        (previousChar == 'c' && ch == 'd') ||
                        (previousChar == 'p' && ch == 'q') ||
                        (previousChar == 'x' && ch == 'y')) {
                    return false;
                }

                if (ch == previousChar) {
                    hasDoubleLetter = true;
                }

                if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') {
                    vowelCount += 1;
                }

                previousChar = ch;
            }

            return vowelCount >= 3 && hasDoubleLetter;
        }
    }
}

