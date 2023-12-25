package org.dathan.aoc.aoc2015;

import com.google.common.collect.Range;
import org.apache.commons.codec.binary.Hex;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.*;

import static org.dathan.aoc.ExceptionUtil.uncheck;

public class Day05 {
    public long part1(List<String> input) {
        Part1Scanner scanner = new Part1Scanner();
        long niceCount = 0;
        for (String line: input) {
            if (scanner.scan(line)) {
                niceCount++;
            }
        }

        return niceCount;
    }

    public long part2(List<String> input) {

        long count = 0;

        for (String line: input) {
            if (hasRepeatedPair(line) && hasTriplet(line)) {
                count += 1;
            }
        }

        return count;
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

