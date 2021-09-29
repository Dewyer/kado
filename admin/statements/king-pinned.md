The third day shall bring a third problem:

Crime sometimes doesn't pay.
The exit of the maze you got your boss out of was set up to be a trap, now the standoff continues.
He got into a square room and needs to find the place where to dodge to before he gets shot down.
There are multiple type of hostiles in the room and the best way to describe them would be to compare them 
to black chess pieces.

All of them can only threaten squares they would be able to **move** to according to [normal](https://en.wikipedia.org/wiki/Rules_of_chess) chess rules with the following changes:
- The en-passant rule doesn't exist
- You can't castle
- Kings can be captured
- The board can be bigger than 8x8, but it will always be a square

All enemy pieces are black pieces, and the mafia boss is a white king (but can be captured).
Your job is to create a list of places for your boss to jump to, in increasing order according to their level ot threat
(if two squares have the same threat level, then sort by their column index, and then their row index).
Your boss can move to any unoccupied square in the room. 
Each square's threat is equal to the amount of enemy pieces/guys threatening it.
The other parti also brought their mafia boss.  (There is always a black king).

We use the following encoding for chess pieces:
- 1 - Pawn
- 2 - Rook
- 3 - Knight
- 4 - Bishop
- 5 - Queen
- 6 - Mafia boss (A white king that can be captured)
- 7 - Enemy king
- 0 - Empty space


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
There is only one sample available and its this one.

The output should be a simple JSON array of coordinate pairs (column index and row index, the room is indexed from the top left corner and indexing starts with 0):
```json
{
    "places_to_move_to": [[0,0],[0,1],[0,2],[0,3],[2,0],[3,0],[1,1],[2,3],[3,1],[3,2],[3,3],[1,3]]
}
```

Explanation:
All the squares listed are unoccupied.

- (0, 0) - Isn't threatened at all
- (0, 1) - Isn't threatened at all
- (0, 2) - Isn't threatened at all
- (0, 3) - Isn't threatened at all
- (2, 0) - Isn't threatened at all
- (3, 0) - Isn't threatened at all
- (1, 1) - Isn't threatened at all
- (2, 3) - Threatened by the enemy king
- (3, 1) - Threatened by the enemy king
- (3, 2) - Threatened by the enemy king
- (3, 3) - Threatened by the enemy king
- (1, 3) - Threatened by both the enemy king and the (1, 2) pawn
#### Constraints

The room's size will always be between 8 and a 100 (including 8 and 100).