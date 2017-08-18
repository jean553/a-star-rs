#[cfg(test)]
mod tests {

    use nodes::Nodes;

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

        assert_eq!(
            nodes.get_closed_list(),
            [0],
            "unexpected closed list",
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

        const FIRST_INDEX: usize = 0;
        nodes.set_current(FIRST_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [1, 10, 11],
            "unexpected children",
        );

        const SECOND_INDEX: usize = 1;
        nodes.set_current(SECOND_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [0, 2, 10, 11, 12],
            "unexpected children",
        );

        const THIRD_INDEX: usize = 15;
        nodes.set_current(THIRD_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [4, 5, 6, 14, 16, 24, 25, 26],
            "unexpected children",
        );

        const FOURTH_INDEX: usize = 95;
        nodes.set_current(FOURTH_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [84, 85, 86, 94, 96],
            "unexpected children",
        );

        const FIFTH_INDEX: usize = 99;
        nodes.set_current(FIFTH_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
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

        nodes.generate_children_list();
        nodes.update_open_list();
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

    #[test]
    fn test_iterate_once() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        const FIRST_DEPARTURE_INDEX: usize = 0;
        const FIRST_ARRIVAL_INDEX: usize = 25;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.generate_heuristics();
        nodes.generate_children_list();
        nodes.update_open_list();
        nodes.generate_costs();

        assert_eq!(
            nodes.get_open_list(),
            [1, 10, 11],
            "unexpected open list",
        );

        nodes.iterate();

        assert_eq!(
            nodes.get_current(),
            1,
            "unexpected current",
        );

        assert_eq!(
            nodes.get_closed_list(),
            [0, 1],
            "unexpected closed list",
        );

        assert_eq!(
            nodes.get_open_list(),
            [10, 11],
            "unexpected open list",
        );
    }

    #[test]
    fn test_iterate_twice() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        const FIRST_DEPARTURE_INDEX: usize = 0;
        const FIRST_ARRIVAL_INDEX: usize = 25;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.generate_heuristics();
        nodes.generate_children_list();
        nodes.update_open_list();
        nodes.generate_costs();
        nodes.iterate();

        assert_eq!(
            nodes.get_current(),
            1,
            "unexpected current",
        );

        nodes.generate_children_list();
        nodes.update_open_list();
        nodes.generate_costs();
        nodes.iterate();

        assert_eq!(
            nodes.get_current(),
            2,
            "unexpected current",
        );
    }

    #[test]
    fn test_open_list_update() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        const FIRST_DEPARTURE_INDEX: usize = 0;
        const FIRST_ARRIVAL_INDEX: usize = 25;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.generate_heuristics();
        nodes.generate_children_list();
        nodes.update_open_list();

        assert_eq!(
            nodes.get_closed_list(),
            [0],
            "unexpected closed list",
        );

        assert_eq!(
            nodes.get_open_list(),
            [1, 10, 11],
            "unexpected open list",
        );

        nodes.generate_costs();
        nodes.iterate();

        nodes.generate_children_list();
        nodes.update_open_list();

        assert_eq!(
            nodes.get_closed_list(),
            [0, 1],
            "unexpected closed list",
        );

        let mut open_list = nodes.get_open_list();
        open_list.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            open_list,
            [2, 10, 11, 12],
            "unexpected open list",
        );
    }
}
