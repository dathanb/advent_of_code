package org.dathan.aoc;

import org.apache.commons.io.IOUtils;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;

public class IOUtil {
    public static String readFromResource(String filename) {
        try {
            return IOUtils.toString(IOUtil.class.getResourceAsStream(filename), StandardCharsets.UTF_8);
        } catch (IOException ex) {
            throw new RuntimeException(ex);
        }
    }

    public static List<String> getLines(String input) {
        return Arrays.stream(input.split("\n")).toList();
    }
}
