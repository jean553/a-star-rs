//! Module that contains the node structure and its implementation.
//! A node is a step on the map.

/// as this is a library, there is no explicit call
/// of the methods from the library itself
#[allow(dead_code)]
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