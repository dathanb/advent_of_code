package org.dathan.aoc.aoc2023;

import java.util.Arrays;
import java.util.List;
import java.util.regex.MatchResult;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

import static org.dathan.aoc.MathUtil.max;

public class Day02 {
    public long part1(List<String> input) {
        Pattern redPattern = Pattern.compile("([0-9]+) red");
        Pattern greenPattern = Pattern.compile("([0-9]+) green");
        Pattern bluePattern = Pattern.compile("([0-9]+) blue");
        int matchingGameSum = 0;
        for (int index = 0; index < input.size(); index++){
            int gameId = index + 1;

            String gameInput = input.get(index);
            Matcher redMatcher = redPattern.matcher(gameInput);
            int maxRed = 0;
            while (redMatcher.find()) {
                maxRed = max(maxRed, Integer.parseInt(redMatcher.group(1)));
            }
            if (maxRed > 12) {
                continue;
            }

            Matcher greenMatcher = greenPattern.matcher(gameInput);
            int maxGreen = 0;
            while (greenMatcher.find()) {
                maxGreen = max(maxGreen, Integer.parseInt(greenMatcher.group(1)));
            }
            if (maxGreen > 13) {
                continue;
            }

            Matcher blueMatcher = bluePattern.matcher(gameInput);
            int maxBlue = 0;
            while (blueMatcher.find()) {
                maxBlue = max(maxBlue, Integer.parseInt(blueMatcher.group(1)));
            }
            if (maxBlue > 14) {
                continue;
            }

            matchingGameSum += gameId;

        }
        return matchingGameSum;
    }

    public long part2(List<String> input) {
        Pattern redPattern = Pattern.compile("([0-9]+) red");
        Pattern greenPattern = Pattern.compile("([0-9]+) green");
        Pattern bluePattern = Pattern.compile("([0-9]+) blue");
        int gamePowerSum = 0;
        for (int index = 0; index < input.size(); index++){
            int gameId = index + 1;

            String gameInput = input.get(index);
            Matcher redMatcher = redPattern.matcher(gameInput);
            int maxRed = 0;
            while (redMatcher.find()) {
                maxRed = max(maxRed, Integer.parseInt(redMatcher.group(1)));
            }

            Matcher greenMatcher = greenPattern.matcher(gameInput);
            int maxGreen = 0;
            while (greenMatcher.find()) {
                maxGreen = max(maxGreen, Integer.parseInt(greenMatcher.group(1)));
            }

            Matcher blueMatcher = bluePattern.matcher(gameInput);
            int maxBlue = 0;
            while (blueMatcher.find()) {
                maxBlue = max(maxBlue, Integer.parseInt(blueMatcher.group(1)));
            }

            int gamePower = maxRed * maxGreen * maxBlue;
            gamePowerSum += gamePower;

        }
        return gamePowerSum;
    }


}