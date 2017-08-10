/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

    pub struct Node {
        heuristic: u8,
        usuable: bool,
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
                usuable: true,
            }
        }

        /// Sets if the node is usuable or not.
        ///
        /// # Arguments:
        ///
        /// * `usuable` - true if the node is usuable
        pub fn set_is_usuable(
            &mut self,
            usuable: bool,
        ) {
            self.usuable = usuable;
        }
    }

    pub struct Nodes {
        width: u8,
        height: u8,
        nodes: Vec<Node>,
        departure: usize,
        arrival: usize,
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
            departure: usize,
            arrival: usize,
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
                departure: departure,
                arrival: arrival,
            }
        }

        /// Specifies usuable nodes.
        ///
        /// # Arguments:
        ///
        /// * `index` - the index of the node to set
        /// * `usuable` - is the node usuable or not
        pub fn set_node_usuable(
            &mut self,
            index: usize,
            usuable: bool,
        ) {
            self.nodes[index].set_is_usuable(usuable);
        }
    }
}

#[cfg(test)]
mod tests;
