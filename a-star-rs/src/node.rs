//! One node structure and its implementation.

/// One node on the grid.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Node {
    heuristic: u8,
    cost: u8,
    usuable: bool,
}

#[allow(dead_code)]
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

    /// Sets the node as not usuable.
    pub fn set_unusuable(&mut self) {
        self.usuable = false;
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
    /// The node heuristic.
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
    /// The node movement cost.
    pub fn get_cost(&self) -> u8 {
        self.cost
    }
}
