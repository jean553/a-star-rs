/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

    pub struct Node {
        heuristic: u8,
        usuable: bool,
    }

    /// Returns the horizontal and vertical position for the given index.
    ///
    /// # Arguments:
    ///
    /// * `index` - the source index
    /// * `width` - number of nodes per line
    ///
    /// # Returns:
    ///
    /// tuple that contains the horizontal and vertical positions
    fn get_positions(
        index: u8,
        width: u8,
    ) -> (u8, u8) {
        return (
            index % width,
            index / width,
        );
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

        /// Sets the node heuristic.
        ///
        /// # Arguments:
        ///
        /// * `heuristic` - the heuristic to set
        pub fn set_heuristic(
            &mut self,
            heuristic: u8,
        ) {
            self.heuristic = heuristic;
        }

        /// Returns the node heuristic.
        ///
        /// # Returns:
        ///
        /// the node heuristic
        pub fn get_heuristic(&self) -> u8 {
            self.heuristic
        }
    }

    pub struct Nodes {
        width: u8,
        height: u8,
        nodes: Vec<Node>,
        departure: usize,
        arrival: usize,
        open_list: Vec<u8>,
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
                open_list: Vec::new(),
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

        /// Generate the heuristics of every node
        /// according to the departure and arrival indices.
        pub fn generate_heuristics(&mut self) {

            let (
                index_x,
                index_y,
            ) = get_positions(
                self.arrival as u8,
                self.width,
            );

            for (counter, node) in self.nodes.iter_mut().enumerate() {

                let (
                    node_x,
                    node_y,
                ) = get_positions(
                    counter as u8,
                    self.width,
                );

                /* rounded at the integer level */
                let heuristic = (
                    ((index_x as i8 - node_x as i8) as f32).powi(2) +
                    ((index_y as i8 - node_y as i8) as f32).powi(2)
                ).sqrt() as u8;

                (*node).set_heuristic(heuristic);
            }
        }

        /// Returns the heuristic of the given node.
        ///
        /// # Arguments:
        ///
        /// * `index` - index of the concerned node
        ///
        /// # Returns:
        ///
        /// heuristic of the node
        pub fn get_node_heuristic(
            &self,
            index: usize,
        ) -> u8 {
            self.nodes[index].get_heuristic()
        }

        /// Generates the open list of children for the given node.
        ///
        /// # Arguments:
        ///
        /// * `index` - index of the parent node for the generated list
        ///
        /// TODO: partially implemented
        pub fn generate_children_list(
            &mut self,
            index: u8,
        ) {

            let mut children: Vec<u8> = Vec::new();

            let (
                horizontal_position,
                vertical_position,
            ) = get_positions(
                index,
                self.width,
            );

            let node_at_right_border =
                if horizontal_position == self.width - 1 {
                    true
                } else {
                    false
                };

            let node_at_bottom_border =
                if vertical_position == self.height - 1 {
                    true
                } else {
                    false
                };

            if !node_at_right_border {
                children.push(horizontal_position + 1);

                if !node_at_bottom_border {
                    children.push(vertical_position + self.width + 1);
                }
            }

            if vertical_position != self.height - 1 {
                children.push(vertical_position + self.width);
            }

            self.open_list = children;
        }

        /// Returns the children open list.
        ///
        /// # Returns:
        ///
        /// vector that contains the indeces of all the children
        pub fn get_children_open_list(&self) -> Vec<u8> {
            self.open_list.clone()
        }
    }
}

#[cfg(test)]
mod tests;
