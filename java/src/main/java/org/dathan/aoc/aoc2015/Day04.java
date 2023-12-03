package org.dathan.aoc.aoc2015;

import org.apache.commons.codec.binary.Hex;
import org.dathan.aoc.Coordinate;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HashMap;

import static org.dathan.aoc.ExceptionUtil.uncheck;

public class Day04 {
    public long part1(String secretKey) {
        for (long i = 0; ; i++) {
            String hash = computeHash(secretKey, i);
            if (hash.startsWith("00000")) {
                return i;
            }
        }
    }

    public long part2(String secretKey) {
        for (long i = 0; ; i++) {
            String hash = computeHash(secretKey, i);
            if (hash.startsWith("000000")) {
                return i;
            }
        }
    }

    private String computeHash(String secret, long saltInt) {
        String input = String.format("%s%d", secret, saltInt);

        MessageDigest md = null;
        md = uncheck(() -> MessageDigest.getInstance("MD5"));
        md.update(input.getBytes(StandardCharsets.UTF_8));
        byte[] digest = md.digest();
        return Hex.encodeHexString(digest);
    }

}

