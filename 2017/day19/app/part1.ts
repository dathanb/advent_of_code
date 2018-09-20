import {Maze} from "./maze";
import {MazeRunner} from "./maze_runner";

Maze.fromStdin()
    .then(maze => {
        console.log(maze.toString());
        let runner = new MazeRunner(maze);
        runner.step();
    });
