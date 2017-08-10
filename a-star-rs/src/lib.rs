/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

    pub struct Node {
        heuristic: u8,
    }

    impl Node {

        /// Constructor for a new Node object.
        ///
        /// # Returns:
        ///
        /// Created Node object.
        pub fn new() -> Node {
            Node {
                heuristic: 0,
            }
        }
    }

    pub struct Nodes {
        width: u8,
        height: u8,
        nodes: Vec<Node>,
    }

    impl Nodes {

        /// Constructor for a new Nodes object.
        ///
        /// # Arguments:
        ///
        /// * `width` - the number of nodes per line,
        /// * `height` - the number of lines
        ///
        /// # Returns:
        ///
        /// Created Nodes object.
        pub fn new(
            width: u8,
            height: u8,
        ) -> Nodes {

            let mut nodes: Vec<Node> = Vec::new();
            let indices = width * height;

            for _ in 0..indices {
                nodes.push(Node::new());
            }

            Nodes {
                width: width,
                height: height,
                nodes: nodes,
            }
        }
    }
}

#[cfg(test)]
mod tests;
