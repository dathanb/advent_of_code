package org.dathan.aoc;

import java.util.concurrent.Callable;

public class ExceptionUtil {
    public static <T> T uncheck(Callable<T> delegate) {
        try {
            return delegate.call();
        } catch (Exception ex) {
            throw new RuntimeException(ex);
        }
    }
}
