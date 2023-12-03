package org.dathan.aoc;

import org.apache.commons.codec.binary.Hex;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;

import static org.dathan.aoc.ExceptionUtil.uncheck;

public class HashUtil {
    public static String computeMd5Hash(String input) {
        MessageDigest md = null;
        md = uncheck(() -> MessageDigest.getInstance("MD5"));
        md.update(input.getBytes(StandardCharsets.UTF_8));
        byte[] digest = md.digest();
        return Hex.encodeHexString(digest);
    }
}
