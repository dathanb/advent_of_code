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

    public static long max(long... nums) {
        long max = nums[0];
        for (var num: nums) {
            if (num > max) {
                max = num;
            }
        }
        return max;
    }

    public static int max(int... nums) {
        int max = nums[0];
        for (var num: nums) {
            if (num > max) {
                max = num;
            }
        }
        return max;
    }
}
