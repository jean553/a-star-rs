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
