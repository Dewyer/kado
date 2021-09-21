here goes the first problem for you:

We define *insane* numbers in a set as the numbers that have the most (or tied for the most)
`1`-s in their binary representation. (4 would be represented as `100`).

Your job is to find all the *insane* numbers in a given set.

The input json for this problem looks like this:
```json
{
    "meta": {
      "set_length": 4
    },
    "set": [
        1,
        233,
        100,
        64
    ]
}
```
`meta.set_length` contains the length of the input set.

The output should be a simple JSON array of numbers containing the insane numbers for example the output for the input above:
```json
{
    "insane_numbers": [
      233
    ]
}
```

Available amount of samples: `2` (so the maximum `sample_index` is 1)

#### Constraints
Each number is larger then `0` and smaller then `18446744073709551615` (they are 64-bit unsigned integers)