# Getting started
To run this code you should create the `instruction.json` and the `input.csv` before executing this

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

Run the project using `cargo run` or the respective file.