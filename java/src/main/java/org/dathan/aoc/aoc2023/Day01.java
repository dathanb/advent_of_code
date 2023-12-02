package org.dathan.aoc.aoc2023;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day01 {
    public long part1(List<String> input) {
        return input.stream()
                .map(s -> s.replaceAll("[a-z]", ""))
                .map(s -> String.format("%s%s", s.substring(0, 1), s.substring(s.length()-1)))
                .mapToLong(Long::parseLong)
                .sum();
    }

    public long part2(List<String> immutableInput) {
        return immutableInput.stream()
                .map(s -> String.format("%s%s", firstNum(s), lastNum(s)))
                .mapToLong(Long::parseLong)
                .sum();
    }

    private String replaceAt(String originalString, int startIndex, String expectedSubstring, String replacementSubstring) {
        if (startIndex + expectedSubstring.length() > originalString.length()) {
            return originalString;
        }

        if (expectedSubstring.equals(originalString.substring(startIndex, startIndex+expectedSubstring.length()))) {
            return String.format("%s%s%s", originalString.substring(0, startIndex), replacementSubstring, originalString.substring(startIndex + expectedSubstring.length()));
        }
        return originalString;
    }

    private int firstNum(String s) {
        int index = Arrays.stream(new String[]{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"})
                .mapToInt(s::indexOf)
                .filter(n -> n != -1)
                .min()
                .getAsInt();
        return getNumAt(s, index);
    }

    private int lastNum(String s) {
        int index = Arrays.stream(new String[]{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"})
                .mapToInt(s::lastIndexOf)
                .filter(n -> n != -1)
                .max()
                .getAsInt();
        return getNumAt(s, index);
    }


    private int getNumAt(String originalString, int startIndex) {
        return Arrays.stream(new String[]{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"})
                .filter(s -> originalString.indexOf(s, startIndex) == startIndex)
                .findFirst()
                .map(this::toInt)
                .get();
    }

    private int toInt(String s) {
        return switch(s) {
            case "one", "1" -> 1;
            case "two", "2" -> 2;
            case "three", "3" -> 3;
            case "four", "4" -> 4;
            case "five", "5" -> 5;
            case "six", "6" -> 6;
            case "seven", "7" -> 7;
            case "eight", "8" -> 8;
            case "nine", "9" -> 9;
            case "0" -> 0;
            default -> throw new RuntimeException("unrecognized int");
        };
    }
}
