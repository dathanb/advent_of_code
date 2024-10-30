package org.dathan.aoc;

import java.util.HashMap;
import java.util.Map;

public class DisjointSet<T> {
    private Map<T, T> nodes = new HashMap<>();

    public DisjointSet() {
    }

    void add(T node) {
        nodes.computeIfAbsent(node, n -> n);
    }

    /**
     * Find the representative node ("root") for the given node's set.
     * This operation may perform path compression to make find operations faster in the future.
     */
    public T find(T node) {
        T parent = nodes.get(node);
        if (parent.equals(node)) {
            return node;
        } else {
            T root = find(parent);
            nodes.put(node, root);
            return root;
        }
    }

    public void union(T nodeU, T nodeV) {
        add(nodeU);
        add(nodeV);
        T rootNodeU = find(nodeU);
        T rootNodeV = find(nodeV);

        nodes.put(rootNodeU, rootNodeV);
    }

    public boolean detectCycle(T u, T v) {
        add(u);
        add(v);
        T rootU = find(u);
        T rootV = find(v);
        return rootU.equals(rootV);
    }
}
