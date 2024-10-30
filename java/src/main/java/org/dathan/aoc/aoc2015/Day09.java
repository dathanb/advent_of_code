package org.dathan.aoc.aoc2015;


import com.google.common.graph.MutableValueGraph;
import com.google.common.graph.ValueGraph;
import com.google.common.graph.ValueGraphBuilder;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

@SuppressWarnings("UnstableApiUsage")
public class Day09 {
    public long part1(List<String> input) {
        MutableValueGraph<String, Long> graph = buildGraph(input);

        ValueGraph<String, Long> mst = minimumHamiltonian(graph);
        long sum = mst.edges().stream().mapToLong(e -> mst.edgeValue(e).get()).sum();

        return sum;
    }

    public long part2(List<String> input) {
        return 0L;
    }

    MutableValueGraph<String, Long> buildGraph(List<String> input) {
        MutableValueGraph<String, Long> graph = ValueGraphBuilder.undirected().build();
        for (String line : input) {
            String[] parts = line.split(" ");
            String source = parts[0];
            String destination = parts[2];
            long length = Long.parseLong(parts[4]);

            graph.addNode(source);
            graph.addNode(destination);
            graph.putEdgeValue(source, destination, length);
        }
        return graph;
    }

    private ValueGraph<String, Long> minimumHamiltonian(MutableValueGraph<String, Long> graph) {
        // A naive approach to identifying a minimal Hamiltonian path: depth-first search
        ValueGraph<String, Long> min = null;
        long minCost = Long.MAX_VALUE;

        for (String node : graph.nodes()) {
            Set<String> visitedNodes = new HashSet<>();

        }
    }
}

