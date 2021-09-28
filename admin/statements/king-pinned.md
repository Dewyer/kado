The third day shall bring a third problem:

Crime sometimes doesn't pay.
The exit of the maze you got your boss out of was set up to be a trap, now the standoff continues.
He got into an 8m x 8m square room and needs to find the place where to dodge to before he gets shot down.
There are multiple type of hostiles in the room and the best way to describe them would be to compare them 
to white chess pieces. (How convenient right?)
- Pawns - Knife wielding enemies
- Kings - Crazy knife wielding enemies who can only jump
- Bishops - Cross facing machine gunners
- Rooks - Machine gunners facing the other way

All of them can only threaten to shoot squares they would be able to move to according to normal chess rules.
The enemy was originally coming from the bottom  (All enemies are white pieces).
Your job is to create a list of places for your boss to jump to, in decreasing order according to their level ot threat.
Each square's threat is equal to the amount of enemy pieces/guys threatening it.

Encoding of chess pieces:
- 1 - Pawn
- 2 - Rook
- 3 - Knight
- 4 - Bishop
- 5 - Queen
- 6 - King
- 0 - Empty space

The other parti also brought their mafia boss. 

The input json for this problem looks like this:
```json
{
    "room": [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]
    ]
}
```
