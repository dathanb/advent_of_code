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
}

