import {Coordinate} from "./coordinate";

export class Maze {
    static fromStdin = () => {
        let resolverSlot:((list: String[][])=>void)[] = [];
        let list: string[][] = [];

        const stdin = process.openStdin();
        stdin.on("data", (d) => {
            d.toString().split(/\r\n|\r|\n/).forEach((line) => {
                list.push(line.split(''));
            });
        });

        stdin.on('end', () => {
            if (resolverSlot[0] != null) {
                resolverSlot[0](list);
            }
        });

        return new Promise<String[][]>((resolve, _) => {
            resolverSlot.push(resolve);
        }).then((list) => { return new Maze(list); });
    };

    static findEntryPoint(list: String[][]): Coordinate {
        let col = list[0].findIndex((str) => { return str === "|"; });
        return new Coordinate(0, col);
    }

    maze: String[][];
    entryPoint: Coordinate;

    constructor(list: String[][]) {
        this.maze = list;
        this.entryPoint = Maze.findEntryPoint(list);
    }

    // TODO: DRY this out
    possibleMoves(position: Coordinate): Coordinate[][] {
        let coordinates: Coordinate[][] = [];
        let line = this.getLine(position);
        // in the "jump" case, we make the assumption that "jumps" will still end up on the maze
        if(line == "|") {
            let up = this.clip(position.up());
            if (up != null) {
                if(this.getLine(up) == "|") {
                    coordinates.push([up]);
                } else if (this.getLine(up) == "-") {
                    coordinates.push([up, up.up()]);
                }
            }

            let down = this.clip(position.down());
            if (down != null) {
                if(this.getLine(down) == "|") {
                    coordinates.push([down]);
                } else if (this.getLine(down) == "-") {
                    coordinates.push([down, down.down()]);
                }
            }
        } else if (line == "-") {
            let left = this.clip(position.left());
            if (left != null) {
                if(this.getLine(left) == "-") {
                    coordinates.push([left]);
                } else if (this.getLine(left) == "|") {
                    coordinates.push([left, left.left()]);
                }
            }

            let right = this.clip(position.right());
            if (right != null) {
                if(this.getLine(right) == "-") {
                    coordinates.push([right]);
                } else if (this.getLine(right) == "|") {
                    coordinates.push([right, right.right()]);
                }
            }
        }

        return coordinates;
    }

    getLine(p: Coordinate): String {
        return this.maze[p.row][p.col];
    }

    clip(p: Coordinate): Coordinate | null {
        if(p.row < 0) {
            return null;
        } else if (p.row >= this.maze.length) {
            return null;
        } else if (p.col < 0) {
            return null;
        } else if (p.col >= this.maze[p.row].length) {
            return null;
        }
        return p;
    }

    toString(): String {
        let reducer = (prev: String, curr: String[], index: number, array: String[][]) => {
            return prev + curr.join('') + "\n";
        };
        return this.maze.reduce(reducer, '');
    }
}
