#[cfg(test)]
mod tests {

    use lib::Nodes;

    #[test]
    fn test_create_nodes() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        let mut nodes = Nodes::new(
            WIDTH,
            HEIGHT,
        );

        const FIRST_INDEX: usize = 0;
        nodes.set_node_usuable(
            FIRST_INDEX,
            false,
        );
    }
}
