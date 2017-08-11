#[cfg(test)]
mod tests {

    use lib::Nodes;

    #[test]
    fn test_create_nodes() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        const DEPARTURE_INDEX: usize = 0;
        const ARRIVAL_INDEX: usize = 10;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            DEPARTURE_INDEX,
            ARRIVAL_INDEX,
        );

        const FIRST_INDEX: usize = 0;
        nodes.set_node_usuable(
            FIRST_INDEX,
            false,
        );

        nodes.generate_heuristics();
    }
}
