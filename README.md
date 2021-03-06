[![Build Status](https://travis-ci.org/jean553/a-star-rs.svg?branch=master)](https://travis-ci.org/jean553/a-star-rs)

# a-star-rs

Library that implements A-star algorithm.

- [Development](#development)
- [Usage](#usage)
    * [Grid creation](#grid-creation)
    * [Open list and closed list](#open-list-and-closed-list)
    * [Usuable and unusuable nodes](#usuable-and-unusuable-nodes)
    * [Heurtistics generation](#heuristics-generation)
    * [Children nodes](#children-nodes)
    * [Open list update](#open-list-update)
    * [Costs generation](#costs-generation)
    * [Iteration](#iteration)
    * [Backward movement generation](#backward-movement-generation)

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

## Simple usage

Simply specifies the width and height of your map (in nodes).
Set the departure index and the expected arrival index.

The indices are set by row, like this:

```
0  1  2  3  4
5  6  7  8  9
10 11 12 13 14
```

In the following example, we build a map with a width of 5 nodes,
a height of 10 nodes, the departure node is at the index 3
and the arrival node is at the index 21.

```rust
let mut nodes = Nodes::new(5, 10, 3, 21);
```

Some nodes can be set as unusuable (that means the node cannot be part
of the final path):

```rust
nodes.get_node_by_index(10).set_unusuable();
```

Use the function `research_path` to generate the path:

```rust
let path = nodes.generate_path();
```

## Public methods

Public methods without name mangling for library usage:

```rust
#[no_mangle]
pub fn get_path(
    path: &mut [u8],
    width: u8,
    height: u8,
    departure: u8,
    arrival: u8,
)

#[no_mangle]
pub fn get_positions(
    width: u8,
    index: u8,
) -> (u8, u8)
```

## Implementation details

### Grid creation

Create a nodes grid by giving the dimensions, the departure cell index
and the arrival cell index.

The following constructor call creates a grid of 25 cells ( 5 x 5 ),
the departure cell index is 8 and the arrival cell index is 20.

```rust
let mut nodes = Nodes::new(5, 5, 8, 20);
```

### `Open list` and `Closed list`

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

### Usuable and unusuable nodes

Some indices can be set as `not usuable`. By default, they are all usuables.
An unusuable node is not considered at all and
cannot be part of the final path.

```rust
nodes.get_node_by_index(1).set_unusuable();
```

Note: an unusuable node is never part of the closed list.

### Heuristics generation

The following code can be used to generate the heuristics. A node heuristic
is the distance (in nodes) from the current node to the arrival node.
The nodes around the destination node have an heuristic of 1.

The heuristics are generated using the distance formula
derived from the Pythagorean theorem.

```rust
nodes.generate_heuristics();
```

### Children nodes

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

### `Open list` update

The open list contains the indices that have to be considered in order
to find the appropriate path. The open list have to be updated according
to the children list.

```rust
nodes.update_open_list(); // [10, 11]
```

### Costs generation

Every move from the current node to a child node has a cost.
In this example, the cost is 10 for a horizontal or vertical movement,
and 14 for a diagonal movement.

The following line of code generates the costs of every children
of the current node.

```rust
nodes.generate_costs();
```

### Iteration

During an iteration, one movement occurs. In order to choose one which node
the movement is done, the score of each child node is calculated.
The score is the sum of the cost and the heuristic.
The node (or the first node) with the smallest score is chosen for the move.

```rust
let last_node_index = nodes.iterate();
```

The `iterate` function returns an `Option` object.
It returns `None` if the destination has not been found yet
and the iterations must continue.
It returns the index of a node with an heuristic equal to 1
(child of the destination node, so the destination has been found).

### Backward movement generation

After every iteration, the current node backward movement argument value
is updated. This value is the movement to perform within the path
to go from the destination node to the departure node. This value is used
at the end of the process once the destination has been found.

If the current node is a child of the departure node, the backward movement
value is the one required to go directly to the departure node.

If the current node is not a child of the departure node,
then the child node of the current one with the smaller cost value
is considered.
