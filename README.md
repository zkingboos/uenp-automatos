This software demonstrates how to create a finite automaton where you can test different states.
It was developed as an activity for the "Introduction to the Theory of Computation" class in the "Licentiate in Computing" course.

# Getting started
To run this code, you need to create two files: `instruction.json` and `input.csv`, before executing the program.

### Example
> instruction.json
```json
{
  "initial": 0,
  "final": [2],
  "transitions": [
    { "from": 0, "to": 0, "read": "a" },
    { "from": 2, "to": 2, "read": "a" },
    { "from": 1, "to": 1, "read": "b" },
    { "from": 1, "to": 2, "read": "a" },
    { "from": 0, "to": 1, "read": "b" }
  ]
}
````
> input.csv
```csv
ba;1
aaaabbbbbaaaaa;1
abababab;0
bbbbbbbb;0
aaaaaaaaaaaa;0
aaaaabaaaaa;1
```

Run the project using `cargo run` or the appropriate command for your environment.
You can also find the program binaries in the [releases](https://github.com/zkingboos/uenp-automatos/releases) section.