#![feature(vec_remove_item)]

/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {


    pub struct Node {
        heuristic: u8,
        cost: u8,
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
        index: usize,
        width: u8,
    ) -> (u8, u8) {

        let index = index as u8;

        return (
            index % width,
            index / width,
        );
    }

    /// Returns the index according to the horizontal and vertical positions.
    ///
    /// # Arguments:
    ///
    /// * `horizontal_position` - the horizontal position
    /// * `vertical_position` - the vertical position
    /// * `width` - nodes per line
    ///
    /// # Returns:
    ///
    /// the index
    fn get_index_from_positions(
        horizontal_position: u8,
        vertical_position: u8,
        width: u8,
    ) -> usize {
        ((vertical_position * width) + horizontal_position) as usize
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
                cost: 0,
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

        /// Sets the cost.
        ///
        /// # Arguments:
        ///
        /// * `cost` - the cost to set
        pub fn set_cost(
            &mut self,
            cost: u8,
        ) {
            self.cost = cost;
        }

        /// Getter of the cost.
        ///
        /// # Returns:
        ///
        /// the node movement cost
        pub fn get_cost(&self) -> u8 {
            self.cost
        }
    }

    pub struct Nodes {
        width: u8,
        height: u8,
        nodes: Vec<Node>,
        current: usize,
        arrival: usize,
        open_list: Vec<usize>,
        closed_list: Vec<usize>,
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
                current: departure,
                arrival: arrival,
                open_list: Vec::new(),
                closed_list: vec![departure],
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
                self.arrival,
                self.width,
            );

            for (counter, node) in self.nodes.iter_mut().enumerate() {

                let (
                    node_x,
                    node_y,
                ) = get_positions(
                    counter,
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

        /// Returns the cost of a node.
        ///
        /// # Arguments:
        ///
        /// * `index` - index of the concerned node
        ///
        /// # Returns:
        ///
        /// cost of the node
        pub fn get_node_cost(
            &self,
            index: usize,
        ) -> u8 {
            self.nodes[index].get_cost()
        }

        /// Generates the open list of children for the given node.
        ///
        /// # Arguments:
        ///
        /// * `index` - index of the parent node for the generated list
        pub fn generate_children_list(
            &mut self,
            index: u8,
        ) {

            let mut children: Vec<usize> = Vec::new();

            let (
                horizontal_position,
                vertical_position,
            ) = get_positions(
                self.current,
                self.width,
            );

            if horizontal_position != 0 {
                children.push(
                    get_index_from_positions(
                        horizontal_position - 1,
                        vertical_position,
                        self.width,
                    )
                );
            }

            if horizontal_position != self.width - 1 {
                children.push(
                    get_index_from_positions(
                        horizontal_position + 1,
                        vertical_position,
                        self.width,
                    )
                );
            }

            if vertical_position != 0 {
                children.push(
                    get_index_from_positions(
                        horizontal_position,
                        vertical_position - 1,
                        self.width,
                    )
                );

                if horizontal_position != self.width - 1 {
                    children.push(
                        get_index_from_positions(
                            horizontal_position + 1,
                            vertical_position - 1,
                            self.width,
                        )
                    );
                }

                if horizontal_position != 0 {
                    children.push(
                        get_index_from_positions(
                            horizontal_position - 1,
                            vertical_position - 1,
                            self.width,
                        )
                    );
                }
            }

            if vertical_position != self.height - 1 {
                children.push(
                    get_index_from_positions(
                        horizontal_position,
                        vertical_position + 1,
                        self.width,
                    )
                );

                if horizontal_position != self.width - 1 {
                    children.push(
                        get_index_from_positions(
                            horizontal_position + 1,
                            vertical_position + 1,
                            self.width,
                        )
                    );
                }

                if horizontal_position != 0 {
                    children.push(
                        get_index_from_positions(
                            horizontal_position - 1,
                            vertical_position + 1,
                            self.width,
                        )
                    );
                }
            }

            self.open_list = children;
        }

        /// Returns the children open list.
        ///
        /// # Returns:
        ///
        /// vector that contains the indeces of all the children
        pub fn get_children_open_list(&self) -> Vec<usize> {
            self.open_list.clone()
        }

        /// Generates the costs of the children of the open list
        pub fn generate_costs(&mut self) {

            let signed_departure = self.current as i8;

            for index in self.open_list.iter() {

                let signed_index = *index as i8;
                let node = &mut self.nodes[*index];

                const HORIZONTAL_MOVE: i8 = 1;
                const VERTICAL_MOVE: i8 = 10;

                if signed_index == signed_departure - HORIZONTAL_MOVE ||
                    signed_index == signed_departure + HORIZONTAL_MOVE ||
                    signed_index == signed_departure - VERTICAL_MOVE ||
                    signed_index == signed_departure + VERTICAL_MOVE {

                    const HORIZONTAL_OR_VERTICAL_MOVE: u8 = 10;
                    node.set_cost(HORIZONTAL_OR_VERTICAL_MOVE);
                    continue;
                }

                const DIAGONAL_MOVE: u8 = 14;
                node.set_cost(DIAGONAL_MOVE);
            }
        }

        /// Getter for the closed list
        ///
        /// # Returns:
        ///
        /// vector containing the closed list
        pub fn get_closed_list(&self) -> Vec<usize> {
            self.closed_list.clone()
        }

        /// Iterate the research to the next node,
        /// remove the target from the open list
        pub fn iterate(&mut self) {

            let mut minimum: u8 = <u8>::max_value();
            let mut target: usize = 0;

            for index in self.open_list.iter() {

                let node = &self.nodes[*index];
                let value = node.heuristic + node.cost;

                if value < minimum {
                    minimum = value;
                    target = *index;
                }
            }

            self.current = target;
            self.open_list.remove_item(&target);
            self.closed_list.push(target);
        }

        /// Getter for the current index
        ///
        /// # Returns:
        ///
        /// the current index
        pub fn get_current(&self) -> usize {
            self.current
        }
    }
}

#[cfg(test)]
mod tests;
