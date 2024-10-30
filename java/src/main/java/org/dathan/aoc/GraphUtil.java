package org.dathan.aoc;

import com.google.common.graph.EndpointPair;
import com.google.common.graph.MutableValueGraph;
import com.google.common.graph.ValueGraph;
import com.google.common.graph.ValueGraphBuilder;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class GraphUtil {
    public static ValueGraph<String, Long> minimumSpanningTree(ValueGraph<String, Long> graph) {
        List<EndpointPair<String>> edgeList = new ArrayList<>(graph.edges());

        edgeList.sort(Comparator.comparing(e -> graph.edgeValue(e).get()));

        int totalNodes = graph.nodes().size();

        MutableValueGraph<String, Long> spanningTree = ValueGraphBuilder.undirected().build();
        DisjointSet<String> nodeSet = new DisjointSet<>();
        for (EndpointPair<String> edge : edgeList) {
            if (nodeSet.detectCycle(edge.nodeU(), edge.nodeV())) {
                continue;
            }
            System.out.printf("Adding to MST: %s -> %s = %d\n", edge.nodeU(), edge.nodeV(), graph.edgeValue(edge).get());
            spanningTree.putEdgeValue(edge.nodeU(), edge.nodeV(), graph.edgeValue(edge).get());
            nodeSet.union(edge.nodeU(), edge.nodeV());
            if (spanningTree.edges().size() == totalNodes - 1) {
                break;
            }
        }
        return spanningTree;
    }
}
