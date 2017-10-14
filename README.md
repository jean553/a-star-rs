[![Build Status](https://travis-ci.org/jean553/a-star-rs.svg?branch=master)](https://travis-ci.org/jean553/a-star-rs)

# a-star-rs

Library that implements A-star algorithm.

## Development

Start the docker container.

```bash
vagrant up
```

Connect to the container.

```bash
vagrant ssh
```

Run tests.

```bash
cargo test
```

Generate documentation.

```bash
cargo rustdoc -- --no-defaults
```

## Usage

Create a nodes grid by giving the dimensions, the departure cell index
and the arrival cell index.

The following constructor call creates a grid of 25 cells ( 5 x 5 ),
the departure cell index is 8 and the arrival cell index is 20.

```rust
let mut nodes = Nodes::new(5, 5, 8, 20);
```

`A*` works by using an `open list` and a `closed list`. The `open list`
contains the indices of the nodes that have to be considered
in order to find the shortest path; the `closed list` contains
the indices that don't need to be considered anymore as they won't be used
for sure.

During the creation of the grid, the departure index is directly
added to the closed list. The following code returns the closed list content.

```rust
nodes.get_closed_list(); // [5]
```
