package org.dathan.aoc.aoc2023;

import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day04 {
    public long part1(List<String> input) {
        Pattern numberPattern = Pattern.compile("([0-9]+)");

        int sum = 0;
        for (String line : input) {
            Set<Integer> winners = new HashSet<>();
            Set<Integer> numbers = new HashSet<>();
            String withoutCardNum = line.substring(line.indexOf(':') + 2);

            String[] parts = withoutCardNum.split("\\|");

            Matcher matcher = numberPattern.matcher(parts[0]);
            while (matcher.find()) {
                winners.add(Integer.parseInt(matcher.group(1)));
            }

            matcher = numberPattern.matcher(parts[1]);
            while (matcher.find()) {
                numbers.add(Integer.parseInt(matcher.group(1)));
            }

            numbers.retainAll(winners);
            sum += (int) Math.pow(2, numbers.size() - 1);
        }

        return sum;
    }

    public long part2(List<String> input) {
        Pattern numberPattern = Pattern.compile("([0-9]+)");

        HashMap<Integer, Scratcher> cards = new HashMap<>();
        List<Scratcher> queue = new ArrayList<>();
        for (int i = 0; i < input.size(); i++) {
            String line = input.get(i);
            Set<Integer> winners = new HashSet<>();
            Set<Integer> numbers = new HashSet<>();
            String withoutCardNum = line.substring(line.indexOf(':') + 2);

            String[] parts = withoutCardNum.split("\\|");

            Matcher matcher = numberPattern.matcher(parts[0]);
            while (matcher.find()) {
                winners.add(Integer.parseInt(matcher.group(1)));
            }

            matcher = numberPattern.matcher(parts[1]);
            while (matcher.find()) {
                numbers.add(Integer.parseInt(matcher.group(1)));
            }

            Scratcher card = new Scratcher(i+1, winners, numbers);
            cards.put(i + 1, card);
            queue.add(card);
        }

        int sum = 0;
        while (!queue.isEmpty()) {
            sum += 1;
            Scratcher card = queue.remove(queue.size()-1);
            Set<Integer> winningPicks = new HashSet<>(card.picks());
            winningPicks.retainAll(card.winners());
            int score = winningPicks.size();
            for (int i = 1; i < score+1; i++) {
                Scratcher newCard = cards.get(card.index() + i);
                queue.add(newCard);
            }
        }

        return sum;
    }
}

record Scratcher(int index, Set<Integer> winners, Set<Integer> picks) {
}