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
        for (String line: input) {
            String unescapedValue = unescape(line);
            int thisLineDiff = line.length() - unescapedValue.length();
            diff += thisLineDiff;
            System.out.printf("%s; %d; %s; %d; %d; %d\n", line, line.length(), unescapedValue, unescapedValue.length(), thisLineDiff, diff);
        }
        return diff;
    }

    public long part2(List<String> input) {
        long diff = 0;

        for (String line: input) {
            String escapedValue = String.format("\"%s\"", StringEscapeUtils.escapeJava(line));
            diff += escapedValue.length() - line.length();
        }

        return diff;
    }

    private String unescape(String line) {
        line = StringUtils.removeStart(line, "\"");
        line = StringUtils.removeEnd(line, "\"");

        Parser parser = new Parser();

        char[] charArray = line.toCharArray();
        for (char ch: charArray) {
            parser.consume(ch);
        }

        return parser.build();
    }

    private static class Parser {
        List<Character> chars = new ArrayList<>();
        EscapedToken currentToken = null;

        public Parser() {
        }

        public void consume(char ch) {
            if (currentToken == null && ch != '\\') {
                chars.add(ch);
            } else if (currentToken == null && ch == '\\') {
                currentToken = new EscapedToken();
                currentToken.consume(ch);
            } else {
                Optional<Character> newCh = currentToken.consume(ch);
                if (newCh.isPresent()) {
                    chars.add(newCh.get());
                    currentToken = null;
                }
            }
        }

        public String build() {
            return StringUtils.join(chars, "");
        }

        @Data
        private static class EscapedToken {
            private List<Character> chars = new ArrayList<>();

            public Optional<Character> consume(char ch) {
                if (chars.size() == 0) { //
                    chars.add(ch);
                    return Optional.empty();
                } else if (chars.size() == 1) { // only /
                    if (ch == 'x') {
                        chars.add(ch);
                        return Optional.empty();
                    } else {
                        chars.add(ch);
                        return Optional.of(ch);
                    }
                } else if (chars.size() == 2) {
                    chars.add(ch);
                    return Optional.empty();
                } else {
                    chars.add(ch);
                    return Optional.of((char)Integer.parseInt(String.format("%c%c", chars.get(2), chars.get(3)), 16));
                }
            }
        }
    }
}

