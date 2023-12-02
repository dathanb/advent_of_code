package org.dathan.aoc;

public class MathUtil {
    public static long min(long... nums) {
        long min = nums[0];
        for (var num: nums) {
            if (num < min) {
                min = num;
            }
        }
        return min;
    }

    public static int min(int... nums) {
        int min = nums[0];
        for (var num: nums) {
            if (num < min) {
                min = num;
            }
        }
        return min;
    }
}
