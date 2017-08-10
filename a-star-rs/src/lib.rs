/// we allow dead code to prevent warnings saying the functions are not used
#[allow(dead_code)]
mod lib {

    pub struct Nodes {
        width: u8,
        height: u8,
    }

    impl Nodes {

        /// Constructor for a new Nodes object.
        ///
        /// # Arguments:
        ///
        /// * `width` - the number of nodes per line,
        /// * `height` - the number of lines
        ///
        /// # Returns:
        ///
        /// Created Nodes object.
        pub fn new(
            width: u8,
            height: u8,
        ) -> Nodes {
            Nodes {
                width: width,
                height: height,
            }
        }
    }
}

#[cfg(test)]
mod tests;
