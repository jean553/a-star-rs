#[cfg(test)]
mod tests {

    use lib::Nodes;

    #[test]
    fn test_create_nodes() {

        const WIDTH: u8 = 10;
        const HEIGHT: u8 = 10;
        let nodes = Nodes::new(
            WIDTH,
            HEIGHT,
        );
    }
}
