//! Routines for library common and general purposes.

/// Returns the index according to the horizontal and vertical positions.
///
/// As this is a library, there is no explicit call
/// of the methods from the library itself
///
/// # Arguments:
///
/// * `horizontal_position` - the horizontal position
/// * `vertical_position` - the vertical position
/// * `width` - nodes per line
///
/// # Returns:
///
/// the index
#[allow(dead_code)]
pub fn get_index_from_positions(
    horizontal_position: u8,
    vertical_position: u8,
    width: u8,
) -> usize {
    ((vertical_position * width) + horizontal_position) as usize
}

