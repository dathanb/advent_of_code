package org.dathan.aoc.aoc2015;


import lombok.Data;
import org.apache.commons.lang3.StringUtils;
import org.apache.commons.text.StringEscapeUtils;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class Day08 {
    public long part1(List<String> input) {
        long diff = 0;
        for (String line : input) {
            int count = 0;
            char[] chars = line.toCharArray();
            for (int i = 0; i < chars.length; i++) {
                if (chars[i] == '"') {
                    continue;
                } else if (chars[i] == '\\' && chars[i + 1] != 'x') {
                    count++;
                    i += 1;
                } else if (chars[i] == '\\' && chars[i + 1] == 'x') {
                    count++;
                    i += 3;
                } else {
                    count++;
                }
            }

            diff += line.length() - count;
        }
        return diff;
    }

    public long part2(List<String> input) {
        long diff = 0;

        for (String line : input) {
            String escapedValue = String.format("\"%s\"", StringEscapeUtils.escapeJava(line));
            diff += escapedValue.length() - line.length();
        }

        return diff;
    }
}

