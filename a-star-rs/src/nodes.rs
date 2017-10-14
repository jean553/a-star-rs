//! Module that contains the map structure and its implementation. 

use utils;
use node::Node;

/// as this is a library, there is no explicit call
/// of the methods from the library itself
#[allow(dead_code)]
#[derive(Clone)]
pub struct Nodes {
    width: u8,
    height: u8,
    nodes: Vec<Node>,
    current: usize,
    arrival: usize,
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
            current: departure,
            arrival: arrival,
            open_list: Vec::new(),
            closed_list: vec![departure],
            children_list: Vec::new(),
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
        ) = self.get_positions(self.arrival);

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

    /// Generates the open list of children for the given node
    /// for the current index.
    pub fn generate_children_list(&mut self) {

        let mut children: Vec<usize> = Vec::new();

        let (
            horizontal_position,
            vertical_position,
        ) = utils::get_positions(
            self.current,
            self.width,
        );

        if horizontal_position != 0 {
            children.push(
                utils::get_index_from_positions(
                    horizontal_position - 1,
                    vertical_position,
                    self.width,
                )
            );
        }

        if horizontal_position != self.width - 1 {
            children.push(
                utils::get_index_from_positions(
                    horizontal_position + 1,
                    vertical_position,
                    self.width,
                )
            );
        }

        if vertical_position != 0 {
            children.push(
                utils::get_index_from_positions(
                    horizontal_position,
                    vertical_position - 1,
                    self.width,
                )
            );

            if horizontal_position != self.width - 1 {
                children.push(
                    utils::get_index_from_positions(
                        horizontal_position + 1,
                        vertical_position - 1,
                        self.width,
                    )
                );
            }

            if horizontal_position != 0 {
                children.push(
                    utils::get_index_from_positions(
                        horizontal_position - 1,
                        vertical_position - 1,
                        self.width,
                    )
                );
            }
        }

        if vertical_position != self.height - 1 {
            children.push(
                utils::get_index_from_positions(
                    horizontal_position,
                    vertical_position + 1,
                    self.width,
                )
            );

            if horizontal_position != self.width - 1 {
                children.push(
                    utils::get_index_from_positions(
                        horizontal_position + 1,
                        vertical_position + 1,
                        self.width,
                    )
                );
            }

            if horizontal_position != 0 {
                children.push(
                    utils::get_index_from_positions(
                        horizontal_position - 1,
                        vertical_position + 1,
                        self.width,
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
    /// vector that contains the indices of all the children
    pub fn get_open_list(&self) -> Vec<usize> {
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

            if self.closed_list.contains(index) {
                continue;
            }

            let node = &self.nodes[*index];
            let value = node.get_heuristic() + node.get_cost();

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

    /// Setter of the current index
    ///
    /// # Arguments:
    ///
    /// `index` - the current index
    pub fn set_current(
        &mut self,
        current: usize,
    ) {
        self.current = current;
    }

    /// Sets the open list as the children list
    pub fn update_open_list(&mut self) {

        let open_list = &mut self.open_list;

        for child in self.children_list.iter() {

            if open_list.contains(child) || self.closed_list.contains(child) {
                continue;
            }

            open_list.push(*child);
        }
    }

    /// Returns the children list
    ///
    /// # Returns:
    ///
    /// list of the current children
    pub fn get_children_list(&self) -> Vec<usize> {
        self.children_list.clone()
    }
}
