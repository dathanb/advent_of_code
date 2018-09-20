const {Maze} = require("./maze");

Maze.fromStdin().then(maze => { console.log(maze.toString()); });
