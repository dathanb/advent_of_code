package com.jedaway.adventofcode.aoc2020.day1;

import java.io.File;
import java.io.IOException;
import java.net.URI;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.HashSet;
import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;

/**
 * https://adventofcode.com/2020/day/1
 */
public class Day1 {

    private final Set<Integer> integers;

    public static Day1 load(URI uri) throws IOException {
        List<Integer> integers = Files.lines(new File(uri).toPath(), StandardCharsets.UTF_8)
                .map(Integer::parseInt)
                .collect(Collectors.toList());
        return new Day1(integers);
    }

    public Day1(List<Integer> integers) {
        this.integers = new HashSet<>(integers);
    }

    public Day1Output run() {
        Integer n = integers.stream().filter(i -> integers.contains(2020 - i)).findFirst().get();
        return new Day1Output(n*(2020-n));
    }

    public static class Day1Output {

        public int product;

        public Day1Output(int product) {
            this.product = product;
        }
    }
}
