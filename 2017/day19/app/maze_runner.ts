import {Maze} from "./maze";
import {Coordinate} from "./coordinate";

export class MazeRunner {
    constructor(maze: Maze) {
        this.maze = maze;
        this.position = maze.entryPoint;
        this.steps = [this.position];
    }

    maze: Maze;
    steps: Coordinate[];
    position: Coordinate;

    step(): void {
        let possibleSteps = this.maze.possibleMoves(this.position);
        console.log(possibleSteps);
        let nextSteps = possibleSteps.find((value: Coordinate[], index: number, obj: Coordinate[][]) => {
            return this.steps.indexOf(value[0]) > -1;
        });
        console.log(nextSteps);
    }
}