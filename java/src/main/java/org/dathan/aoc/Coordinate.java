package org.dathan.aoc;

public record Coordinate(int x, int y) {
    public static final Coordinate ORIGIN = new Coordinate(0, 0);

    public Coordinate left() {
        return new Coordinate(x - 1, y);
    }

    public Coordinate right() {
        return new Coordinate(x + 1, y);
    }

    public Coordinate up() {
        return new Coordinate(x, y + 1);
    }

    public Coordinate down() {
        return new Coordinate(x, y - 1);
    }

    public Coordinate move(char direction) {
        return switch (direction) {
            case '<' -> left();
            case '>' -> right();
            case '^' -> up();
            case 'V', 'v' -> down();
            default -> throw new RuntimeException("Unrecognized direction " + direction);
        };
    }
}
