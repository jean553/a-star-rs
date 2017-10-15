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

Some indices can be set as `not usuable`. By default, they are all usuables.
An unusuable node is not considered at all and
cannot be part of the final path.

```rust
nodes.get_node_by_index(23).set_unusuable();
```

The following code can be used to generate the heuristics. A node heuristic
is the distance (in nodes) from the current node to the arrival node.
The nodes around the destination node have an heuristic of 1.

The heuristics are generated using the distance formula
derived from the Pythagorean theorem.

```rust
nodes.generate_heuristics();
```

At anytime of the iteration, there is always a `current node`,
which is the current position of the research pointer.
The value of the current index can be changed with the method
`set_current()`.

Furthermore, every node has children. The children of one node
are all the nodes around him. The children nodes list is generated
according to the current node index.

```rust
nodes.set_current(0);
nodes.generate_children_list();
nodes.get_children_list(); // [1, 10, 11]
```

The open list contains the indices that have to be considered in order
to find the appropriate path. The open list have to be updated according
to the children list.

```rust
nodes.update_open_list();
```
