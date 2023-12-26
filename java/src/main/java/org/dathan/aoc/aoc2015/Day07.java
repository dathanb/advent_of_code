package org.dathan.aoc.aoc2015;

import org.apache.commons.lang3.StringUtils;
import org.apache.commons.lang3.function.TriFunction;

import java.util.*;

public class Day07 {
    public long part1(List<String> input) {
        List<String> data = new ArrayList<>(input);
        Map<String, Wire> wires = new HashMap<>();

        for (String line : data) {
            Wire wire = Wire.parse(line);
            wires.put(wire.getOutput(), wire);
        }

        return wires.get("a").evaluate(wires);
    }

    public long part2(List<String> input) {
        List<String> data = new ArrayList<>(input);
        Map<String, Wire> wires = new HashMap<>();

        for (String line : data) {
            Wire wire = Wire.parse(line);
            wires.put(wire.getOutput(), wire);
        }

        long aValue = wires.get("a").evaluate(wires);
        wires.put("b", Wire.parse(String.format("%d -> b", aValue)));

        wires.forEach((key, value) -> value.clearMemory());

        return wires.get("a").evaluate(wires);
    }

    /**
     * Defines how a wire gets its value (some inputs and an operation on them);
     */
    static class Wire {
        private final String code;
        String leftOperand;
        String rightOperand;
        TriFunction<String, String, Map<String, Wire>, Long> operation;
        String outputRef;
        private Long memory = null;

        public static Wire parse(String input) {
            String leftOperand;
            String rightOperand = null;
            TriFunction<String, String, Map<String, Wire>, Long> operation;
            String outputRef;
            String[] parts = input.split(" ");
            if (parts.length == 3) { // e.g., 1 -> a; a -> b
                leftOperand = parts[0];
                rightOperand = null;
                operation = (left, right, wires) -> resolveReference(left, wires);
                outputRef = parts[2];
            } else if (parts.length == 4) { // unary operation: NOT [constant|reference] -> a; NOT a -. b
                leftOperand = parts[1];
                operation = (left, right, wires) -> ~resolveReference(left, wires);
                outputRef = parts[3];
            } else if (parts.length == 5) { // binary operation: [constant|reference] [AND|OR|LSHIFT|RSHIFT] [constant|reference] -> [reference]
                leftOperand = parts[0];
                rightOperand = parts[2];
                switch (parts[1]) {
                    case "AND" -> operation = (left, right, wires) -> resolveReference(left, wires) & resolveReference(right, wires);
                    case "OR" -> operation = (left, right, wires) -> resolveReference(left, wires) | resolveReference(right, wires);
                    case "LSHIFT" -> operation = (left, right, wires) -> resolveReference(left, wires) << resolveReference(right, wires);
                    case "RSHIFT" -> operation = (left, right, wires) -> resolveReference(left, wires) >> resolveReference(right, wires);
                    default -> throw new RuntimeException(String.format("Unrecognized operation: %s", input));
                }
                outputRef = parts[4];
            } else {
                throw new RuntimeException(String.format("Unrecognized operation: %s", input));
            }

            return new Wire(input, leftOperand, rightOperand, operation, outputRef);
        }

        private static Long resolveReference(String ref, Map<String, Wire> wires) {
            System.out.printf("Resolving reference %s\n", ref);
            if (StringUtils.isNumeric(ref)) {
                return Long.parseLong(ref);
            } else {
                Wire wire = wires.get(ref);
                return wire.evaluate(wires);
            }
        }
        public Wire(String code, String leftOperand, String rightOperand, TriFunction<String, String, Map<String, Wire>, Long> operation, String outputRef) {
            this.code = code;
            this.leftOperand = leftOperand;
            this.rightOperand = rightOperand;
            this.operation = operation;
            this.outputRef = outputRef;
        }

        String getOutput() {
            return outputRef;
        }

        long evaluate(Map<String, Wire> wires) {
            if (this.memory != null) {
                System.out.printf("Returning value for \"%s\" from memory: %d\n", code, memory);
                return this.memory;
            }
            System.out.printf("Evaluating code: %s\n", code);
            long value = this.operation.apply(leftOperand, rightOperand, wires);
            memory = value;
            return value;
        }

        public void clearMemory() {
            this.memory = null;
        }
    }
}

