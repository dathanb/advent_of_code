package org.dathan.aoc;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class DisjointSetTest {
    @Test
    @DisplayName("smoke tests")
    public void smokeTests() {
        DisjointSet<String> set = new DisjointSet<>();
        set.add("hi");
        set.add("there");
        assertNotEquals(set.find("hi"), set.find("there"));
        set.union("hi", "there");
        assertEquals(set.find("hi"), set.find("there"));
        set.add("dathan");
        set.add("bennett");
        assertNotEquals(set.find("hi"), set.find("dathan"));
        assertNotEquals(set.find("dathan"), set.find("bennett"));
        set.union("dathan", "bennett");
        assertEquals(set.find("dathan"), set.find("bennett"));
        assertNotEquals(set.find("hi"), set.find("bennett"));
        set.union("hi", "dathan");
        assertEquals(set.find("hi"), set.find("bennett"));
    }

}