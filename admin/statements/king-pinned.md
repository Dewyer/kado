The third day shall bring a third problem:

Crime sometimes doesn't pay.
The exit of the maze you got your boss out of was set up to be a trap, now the standoff continues.
He got into a square room and needs to find the place where to dodge to before he gets shot down.
There are multiple type of hostiles in the room and the best way to describe them would be to compare them 
to black chess pieces.

All of them can only threaten squares they would be able to **move** to according to normal chess rules.
(Excluding the en-passant and castling rules)
All enemy pieces are black pieces, and the mafia boss is a white king.
Your job is to create a list of places for your boss to jump to, in increasing order according to their level ot threat
(if two squares have the same threat level, then sort by their column index, and then their row index).
Each square's threat is equal to the amount of enemy pieces/guys threatening it.

Encoding of chess pieces:
- 1 - Pawn
- 2 - Rook
- 3 - Knight
- 4 - Bishop
- 5 - Queen
- 6 - Mafia boss (White king)
- 7 - Enemy king
- 0 - Empty space

The other parti also brought their mafia boss.  (There is always a black king)

The input json for this problem looks like this:
```json
{
    "room": [
        [0, 6, 0, 0],
        [0, 0, 1, 0],
        [0, 1, 7, 0],
        [0, 0, 0, 0]
    ]
}
```

The output should be a simple JSON array of coordinate pairs (column index and row index, the room is indexed from the top left corner and indexing starts with 0):
```json
{
    "places_to_move_to": [[0,2],[0,3],[3,0],[0,0],[0,1],[2,0],[2,3],[3,1],[3,2],[3,3],[1,1],[1,3]]
}
```