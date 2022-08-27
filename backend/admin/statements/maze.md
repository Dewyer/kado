Here goes the second problem for you:

The Mafia boss is in big trouble! He is in the middle of a gun fight and there is only one way he can escape...
The only way is to go into a maze and shake off the pursuers!
Help him solving the maze!

The input json for this problem looks like this:

```json

  {
    "width": 3,
    "height": 3,
    "maze": [
      1, 1, 1,
      1, 0, 1,
      0, 1, 1
    ],
    "startCell": {
      "x": 1,
      "y": 2
    },
    "endCell": {
      "x": 0,
      "y": 0
    }
  }
```

where
`maze.width` is the width of the maze,
`maze.height` it the height of the maze,
`maze.maze` is a 2D plane consisting of walls (0) and tiles (1).

You start from `maze.startCell` and need to provide steps one must make to reach the `maze.endCell`. How you implement that, is up to you...


In the end, you need to provide us with a String of needed steps as the output:

```json
{
	"solution" : "RUULL"
}
```

where "RUULL" encodes: Go right, up, up, left, left.


