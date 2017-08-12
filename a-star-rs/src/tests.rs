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

        const FIRST_INDEX_EXPECTED_HEURISTIC: u8 = 1;
        assert_eq!(
            nodes.get_node_heuristic(FIRST_INDEX),
            FIRST_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic."
        );

        const SECOND_INDEX: usize = 2;
        const SECOND_INDEX_EXPECTED_HEURISTIC: u8 = 2;
        assert_eq!(
            nodes.get_node_heuristic(SECOND_INDEX),
            SECOND_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic."
        );

        const LAST_INDEX: usize = 99;
        const LAST_INDEX_EXPECTED_HEURISTIC: u8 = 12;
        assert_eq!(
            nodes.get_node_heuristic(LAST_INDEX),
            LAST_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic."
        );
    }
}
