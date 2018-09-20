export class Coordinate {
    constructor(row: number, col: number) {
        this.row = row;
        this.col = col;
    }

    row: number;
    col: number;

    up(): Coordinate {
        return new Coordinate(this.row - 1, this.col);
    }

    down(): Coordinate {
        return new Coordinate(this.row + 1, this.col);
    }

    left(): Coordinate {
        return new Coordinate(this.row, this.col-1);
    }

    right(): Coordinate {
        return new Coordinate(this.row, this.col + 1);
    }

    move(direction: string): Coordinate {
        var map: { [direction: string]: () => Coordinate} = {
            "up": this.up,
            "down": this.down,
            "left": this.left,
            "right": this.right
        };

        return map[direction]();
    }

    toString(): string {
        return `(${this.row}, ${this.col})`;
    }
}