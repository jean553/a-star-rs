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
            "Unexpected heuristic.",
        );

        const SECOND_INDEX: usize = 2;
        const SECOND_INDEX_EXPECTED_HEURISTIC: u8 = 2;
        assert_eq!(
            nodes.get_node_heuristic(SECOND_INDEX),
            SECOND_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic.",
        );

        const LAST_INDEX: usize = 99;
        const LAST_INDEX_EXPECTED_HEURISTIC: u8 = 12;
        assert_eq!(
            nodes.get_node_heuristic(LAST_INDEX),
            LAST_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic.",
        );
    }

    #[test]
    fn test_generated_children_open_list() {

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

        const FIRST_INDEX: u8 = 0;
        nodes.generate_children_list(FIRST_INDEX);
        let mut children = nodes.get_children_open_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [1, 10, 11],
            "unexpected children",
        );

        const SECOND_INDEX: u8 = 1;
        nodes.generate_children_list(SECOND_INDEX);
        let mut children = nodes.get_children_open_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [0, 2, 10, 11, 12],
            "unexpected children",
        );

        const THIRD_INDEX: u8 = 15;
        nodes.generate_children_list(THIRD_INDEX);
        let mut children = nodes.get_children_open_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [4, 5, 6, 14, 16, 24, 25, 26],
            "unexpected children",
        );

        const FOURTH_INDEX: u8 = 95;
        nodes.generate_children_list(FOURTH_INDEX);
        let mut children = nodes.get_children_open_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [84, 85, 86, 94, 96],
            "unexpected children",
        );

        const FIFTH_INDEX: u8 = 99;
        nodes.generate_children_list(FIFTH_INDEX);
        let mut children = nodes.get_children_open_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [88, 89, 98],
            "unexpected children",
        );
    }

    #[test]
    fn test_generated_costs() {

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

        const FIRST_INDEX: u8 = 0;
        nodes.generate_children_list(FIRST_INDEX);
        nodes.generate_costs();

        assert_eq!(
            nodes.get_node_cost(0),
            0,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_cost(1),
            10,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_cost(10),
            10,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_cost(11),
            14,
            "unexpected cost",
        );
    }
}
