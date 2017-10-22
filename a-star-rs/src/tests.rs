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
        nodes.get_node_by_index(FIRST_INDEX)
            .set_unusuable();

        nodes.generate_heuristics();

        const FIRST_INDEX_EXPECTED_HEURISTIC: u8 = 1;
        assert_eq!(
            nodes.get_node_by_index(FIRST_INDEX)
                .get_heuristic(),
            FIRST_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic.",
        );

        const SECOND_INDEX: usize = 2;
        const SECOND_INDEX_EXPECTED_HEURISTIC: u8 = 2;
        assert_eq!(
            nodes.get_node_by_index(SECOND_INDEX)
                .get_heuristic(),
            SECOND_INDEX_EXPECTED_HEURISTIC,
            "Unexpected heuristic.",
        );

        const LAST_INDEX: usize = 99;
        const LAST_INDEX_EXPECTED_HEURISTIC: u8 = 12;
        assert_eq!(
            nodes.get_node_by_index(LAST_INDEX)
                .get_heuristic(),
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
        nodes.set_current_index(FIRST_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [1, 10, 11],
            "unexpected children",
        );

        const SECOND_INDEX: usize = 1;
        nodes.set_current_index(SECOND_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [0, 2, 10, 11, 12],
            "unexpected children",
        );

        const THIRD_INDEX: usize = 15;
        nodes.set_current_index(THIRD_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [4, 5, 6, 14, 16, 24, 25, 26],
            "unexpected children",
        );

        const FOURTH_INDEX: usize = 95;
        nodes.set_current_index(FOURTH_INDEX);
        nodes.generate_children_list();
        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [84, 85, 86, 94, 96],
            "unexpected children",
        );

        const FIFTH_INDEX: usize = 99;
        nodes.set_current_index(FIFTH_INDEX);
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
            nodes.get_node_by_index(0)
                .get_cost(),
            0,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_by_index(1)
                .get_cost(),
            10,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_by_index(10)
                .get_cost(),
            10,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_by_index(11)
                .get_cost(),
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
            nodes.get_current_index(),
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
            nodes.get_current_index(),
            1,
            "unexpected current",
        );

        nodes.generate_children_list();
        nodes.update_open_list();
        nodes.generate_costs();
        nodes.iterate();

        assert_eq!(
            nodes.get_current_index(),
            10,
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

        const UNUSUABLE_NODE_INDEX: usize = 1;
        nodes.get_node_by_index(UNUSUABLE_NODE_INDEX)
            .set_unusuable();

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
            [10, 11],
            "unexpected open list",
        );
    }

    #[test]
    fn test_complete_research() {

        // TODO: #58 implement the complete search test

        const WIDTH: u8 = 6;
        const HEIGHT: u8 = 6;
        const FIRST_DEPARTURE_INDEX: usize = 13;
        const FIRST_ARRIVAL_INDEX: usize = 34;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.get_node_by_index(15)
            .set_unusuable();

        nodes.get_node_by_index(25)
            .set_unusuable();

        nodes.generate_heuristics();
        nodes.generate_children_list();
        nodes.update_open_list();
        nodes.generate_costs();

        let mut open_list = nodes.get_open_list();
        open_list.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            open_list,
            [6, 7, 8, 12, 14, 18, 19, 20],
            "unexpected open list",
        );

        assert_eq!(
            nodes.get_closed_list(),
            [13],
            "unexpected closed list",
        );

        assert_eq!(
            nodes.get_node_by_index(14)
                .get_cost(),
            10,
            "unexpected cost",
        );

        // first iteration

        nodes.iterate();

        assert_eq!(
            nodes.get_current_index(),
            14,
            "unexpected current",
        );

        let mut open_list = nodes.get_open_list();
        open_list.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            open_list,
            [6, 7, 8, 12, 18, 19, 20],
            "unexpected open list",
        );

        assert_eq!(
            nodes.get_closed_list(),
            [13, 14],
            "unexpected closed list",
        );

        nodes.generate_children_list();

        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [7, 8, 9, 13, 15, 19, 20, 21],
            "unexpected children",
        );

        nodes.update_open_list();

        let mut open_list = nodes.get_open_list();
        open_list.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            open_list,
            [6, 7, 8, 9, 12, 18, 19, 20, 21],
            "unexpected open list",
        );

        nodes.generate_costs();

        assert_eq!(
            nodes.get_node_by_index(21)
                .get_cost(),
            24,
            "unexpected cost",
        );

        assert_eq!(
            nodes.get_node_by_index(9)
                .get_cost(),
            24,
            "unexpected cost",
        );

        nodes.iterate();

        assert_eq!(
            nodes.get_current_index(),
            19,
            "unexpected current",
        );

        nodes.generate_children_list();

        let mut children = nodes.get_children_list();
        children.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            children,
            [12, 13, 14, 18, 20, 24, 25, 26],
            "unexpected children",
        );

        nodes.update_open_list();

        let mut open_list = nodes.get_open_list();
        open_list.sort_by(|a, b| a.cmp(b));

        assert_eq!(
            open_list,
            [6, 7, 8, 9, 12, 18, 20, 21, 24, 26],
            "unexpected open list",
        );

        assert_eq!(
            nodes.get_closed_list(),
            [13, 14, 19],
            "unexpected closed list",
        );
    }

    #[test]
    fn test_backward_movement() {

        const WIDTH: u8 = 6;
        const HEIGHT: u8 = 6;
        const FIRST_DEPARTURE_INDEX: usize = 13;
        const FIRST_ARRIVAL_INDEX: usize = 34;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.get_node_by_index(15)
            .set_unusuable();

        nodes.get_node_by_index(25)
            .set_unusuable();

        nodes.get_node_by_index(27)
            .set_unusuable();

        nodes.get_node_by_index(33)
            .set_unusuable();

        // research method simulation
        nodes.generate_heuristics();
        nodes.generate_children_list();

        // perform height iterations
        for _ in 0..11 {
            nodes.update_open_list();
            nodes.generate_costs();
            nodes.iterate();
            nodes.generate_children_list();
            nodes.generate_backward_movement();
        }

        assert_eq!(
            nodes.get_node_by_index(14)
                .get_backward_movement(),
            -1,
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(19)
                .get_backward_movement(),
            -6,
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(7)
                .get_backward_movement(),
            6,
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(12)
                .get_backward_movement(),
            1,
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(20)
                .get_backward_movement(),
            -7, // - 6 - 1
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(8)
                .get_backward_movement(),
            5, // 6 - 1
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(18)
                .get_backward_movement(),
            -5, // - 6 + 1
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(6)
                .get_backward_movement(),
            7, // + 6 + 1
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(1)
                .get_backward_movement(),
            6,
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(21)
                .get_backward_movement(),
            -7, // - 6 - 1
            "unexpected backward movement",
        );

        assert_eq!(
            nodes.get_node_by_index(28)
                .get_backward_movement(),
            -7, // - 6 - 1
            "unexpected backward movement",
        );
    }

    #[test]
    fn test_research_api() {

        const WIDTH: u8 = 6;
        const HEIGHT: u8 = 6;
        const FIRST_DEPARTURE_INDEX: usize = 13;
        const FIRST_ARRIVAL_INDEX: usize = 34;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
            FIRST_DEPARTURE_INDEX,
            FIRST_ARRIVAL_INDEX,
        );

        nodes.get_node_by_index(15)
            .set_unusuable();

        nodes.get_node_by_index(25)
            .set_unusuable();

        nodes.get_node_by_index(27)
            .set_unusuable();

        nodes.get_node_by_index(33)
            .set_unusuable();

        assert_eq!(
            nodes.research_path(),
            28,
            "unexpected final node index",
        );

        assert_eq!(
            nodes.get_node_by_index(34)
                .get_backward_movement(),
            -6,
            "unexpected backward movement",
        );
    }
}
