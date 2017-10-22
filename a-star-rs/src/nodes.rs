//! Module that contains the grid structure and its implementation.

use node::Node;

/// Grid that contains all the nodes.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Nodes {
    width: u8,
    height: u8,
    nodes: Vec<Node>,
    departure_index: usize,
    current_index: usize,
    arrival_index: usize,
    open_list: Vec<usize>,
    closed_list: Vec<usize>,
    children_list: Vec<usize>,
}

#[allow(dead_code)]
impl Nodes {

    /// Constructor for a new Nodes object.
    ///
    /// # Arguments:
    ///
    /// * `width` - the number of nodes per line,
    /// * `height` - the number of lines
    /// * `departure` - the departure cell index
    /// * `arrival` - the arrival cell index
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
            departure_index: departure,
            current_index: departure,
            arrival_index: arrival,
            open_list: Vec::new(),
            closed_list: vec![departure],
            children_list: Vec::new(),
        }
    }

    /// Main research method
    pub fn research_path(&mut self) -> Vec<usize> {

        let mut final_index: Option<usize> = None;

        self.generate_heuristics();
        self.generate_children_list();

        while final_index.is_none() {

            self.update_open_list();
            self.generate_costs();

            final_index = self.iterate();

            self.generate_children_list();
            self.generate_backward_movement();
        }

        let final_index = final_index.unwrap();

        let arrival_index = self.arrival_index;
        self.get_node_by_index(arrival_index)
            .set_backward_movement(final_index as i8 - arrival_index as i8);

        let mut path = Vec::new();
        path.push(self.arrival_index);

        let mut current_index = self.arrival_index;

        loop {

            current_index = (
                current_index as i8 +
                self.get_node_by_index(current_index)
                    .get_backward_movement()
            ) as usize;

            if current_index == self.departure_index {
                break;
            }

            path.push(current_index);
        }

        path.reverse();
        path
    }

    /// Generate the heuristics of every node from departure and arrival.
    pub fn generate_heuristics(&mut self) {

        let (
            index_x,
            index_y,
        ) = self.get_positions(self.arrival_index);

        let nodes_copy = self.clone();

        for (counter, node) in self.nodes.iter_mut().enumerate() {

            let (
                node_x,
                node_y,
            ) = nodes_copy.get_positions(counter);

            /* rounded at the integer level */
            let heuristic = (
                ((index_x as i8 - node_x as i8) as f32).powi(2) +
                ((index_y as i8 - node_y as i8) as f32).powi(2)
            ).sqrt() as u8;

            (*node).set_heuristic(heuristic);
        }
    }

    /// Generates the list of children for the current index.
    pub fn generate_children_list(&mut self) {

        let mut children: Vec<usize> = Vec::new();

        let (
            horizontal_position,
            vertical_position,
        ) = self.get_positions(self.current_index);

        if horizontal_position != 0 {
            children.push(
                self.get_index_from_positions(
                    horizontal_position - 1,
                    vertical_position,
                )
            );
        }

        if horizontal_position != self.width - 1 {
            children.push(
                self.get_index_from_positions(
                    horizontal_position + 1,
                    vertical_position,
                )
            );
        }

        if vertical_position != 0 {
            children.push(
                self.get_index_from_positions(
                    horizontal_position,
                    vertical_position - 1,
                )
            );

            if horizontal_position != self.width - 1 {
                children.push(
                    self.get_index_from_positions(
                        horizontal_position + 1,
                        vertical_position - 1,
                    )
                );
            }

            if horizontal_position != 0 {
                children.push(
                    self.get_index_from_positions(
                        horizontal_position - 1,
                        vertical_position - 1,
                    )
                );
            }
        }

        if vertical_position != self.height - 1 {
            children.push(
                self.get_index_from_positions(
                    horizontal_position,
                    vertical_position + 1,
                )
            );

            if horizontal_position != self.width - 1 {
                children.push(
                    self.get_index_from_positions(
                        horizontal_position + 1,
                        vertical_position + 1,
                    )
                );
            }

            if horizontal_position != 0 {
                children.push(
                    self.get_index_from_positions(
                        horizontal_position - 1,
                        vertical_position + 1,
                    )
                );
            }
        }

        self.children_list = children;
    }

    /// Returns the children open list.
    ///
    /// # Returns:
    ///
    /// Vector that contains the indices of all the children.
    pub fn get_open_list(&self) -> Vec<usize> {
        self.open_list.clone()
    }

    /// Generates the costs of the open list nodes.
    pub fn generate_costs(&mut self) {

        let signed_current = self.current_index as i8;
        let current_cost = self.nodes[self.current_index].get_cost();

        for index in self.open_list.iter() {

            let signed_index = *index as i8;
            let node = &mut self.nodes[*index];

            if node.get_cost() != 0 {
                continue;
            }

            const DIAGONAL_MOVE: u8 = 14;
            let mut cost = DIAGONAL_MOVE;

            const HORIZONTAL_MOVE: i8 = 1;
            let map_width = self.width as i8;

            if signed_index == signed_current - HORIZONTAL_MOVE ||
                signed_index == signed_current + HORIZONTAL_MOVE ||
                signed_index == signed_current - map_width ||
                signed_index == signed_current + map_width {

                const HORIZONTAL_OR_VERTICAL_MOVE: u8 = 10;
                cost = HORIZONTAL_OR_VERTICAL_MOVE;
            }

            node.set_cost(current_cost + cost);
        }
    }

    /// Getter for the closed list.
    ///
    /// # Returns:
    ///
    /// Vector containing the closed list.
    pub fn get_closed_list(&self) -> Vec<usize> {
        self.closed_list.clone()
    }

    /// Iterates to the next node and remove the target from the open list.
    ///
    /// Returns:
    ///
    /// The index of the final node if found, or None
    pub fn iterate(&mut self) -> Option<usize> {

        // FIXME: #55 limits the capacities of the algorithm,
        // check if there is a better way to handle this `initial` value
        let mut minimum: u8 = <u8>::max_value();

        let mut target: usize = 0;

        for index in self.open_list.iter() {

            let node = &self.nodes[*index];
            let heuristic = node.get_heuristic();

            if heuristic == 1 {
                self.current_index = *index;
                return Some(*index);
            }

            let value = heuristic + node.get_cost();

            if value < minimum {
                minimum = value;
                target = *index;
            }
        }

        // FIXME: #55 incorrect behaviour if no path is found
        self.current_index = target;

        // FIXME: #60 check if going to an open list node from the new current
        // node is faster than going from the previous current node
        // to this open list node

        self.open_list.remove_item(&target);
        self.closed_list.push(target);

        None
    }

    /// Getter for the current index.
    ///
    /// # Returns:
    ///
    /// The current index.
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    /// Setter of the current index.
    ///
    /// # Arguments:
    ///
    /// `index` - the current index
    pub fn set_current_index(
        &mut self,
        current: usize,
    ) {
        self.current_index = current;
    }

    /// Sets the open list as the children list.
    pub fn update_open_list(&mut self) {

        for index in self.children_list.iter() {

            if
                self.open_list.contains(index) ||
                self.closed_list.contains(index) ||
                !self.nodes[*index].is_usuable()
            {
                continue;
            }

            self.open_list.push(*index);
        }
    }

    /// Returns the children list.
    ///
    /// # Returns:
    ///
    /// List of the current children.
    pub fn get_children_list(&self) -> Vec<usize> {
        self.children_list.clone()
    }

    /// Returns a node reference of a node for read and write access.
    ///
    /// # Arguments:
    ///
    /// `index` - the index of the node to get
    ///
    /// # Returns:
    ///
    /// The node to read or write.
    pub fn get_node_by_index(
        &mut self,
        index: usize,
    ) -> &mut Node {
        &mut self.nodes[index]
    }

    /// Generates the backward movement of the current index.
    pub fn generate_backward_movement(&mut self) {

        let current_index = self.current_index as i8;
        let mut selected_index = self.departure_index as i8;

        let children_list = self.children_list.clone();
        if !children_list.contains(&self.departure_index) {

            let mut minimum_cost = <u8>::max_value();

            for child in children_list.iter() {

                let child_node = self.get_node_by_index(*child);
                let child_cost = child_node.get_cost();

                if
                    child_cost != 0 &&
                    child_cost < minimum_cost
                {
                    minimum_cost = child_cost;
                    selected_index = *child as i8;
                }
            }
        }

        let index = self.current_index;
        let mut current_node = self.get_node_by_index(index);

        let backward_movement = selected_index - current_index;
        current_node.set_backward_movement(backward_movement);
    }

    /// Returns the horizontal and vertical position for the given index.
    ///
    /// # Arguments:
    ///
    /// * `index` - the source index
    ///
    /// # Returns:
    ///
    /// Tuple that contains the horizontal and vertical positions.
    fn get_positions(
        &self,
        index: usize,
    ) -> (u8, u8) {

        let index = index as u8;

        (
            index % self.width,
            index / self.width,
        )
    }

    /// Returns the index according to the horizontal and vertical positions.
    ///
    /// # Arguments:
    ///
    /// * `horizontal_position` - the horizontal position
    /// * `vertical_position` - the vertical position
    ///
    /// # Returns:
    ///
    /// The index from the given positions.
    fn get_index_from_positions(
        &self,
        horizontal_position: u8,
        vertical_position: u8,
    ) -> usize {
        (vertical_position * self.width + horizontal_position) as usize
    }
}
