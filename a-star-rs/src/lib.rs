#![feature(vec_remove_item)]

/// One node structure and its implementation.
mod node;

/// Contains all nodes.
mod nodes;

/// Main interface that returns the path.
///
/// # Args:
///
/// * `width` - the number of nodes per line (max allowed: 10),
/// * `height` - the number of nodes per column (max allowed: 10),
/// * `departure` - the departure node index,
/// * `arrival` - the arrival node index
#[no_mangle]
pub fn get_path(
    path: &mut [u8],
    width: u8,
    height: u8,
    departure: u8,
    arrival: u8,
) {

    let mut nodes = nodes::Nodes::new(
        width,
        height,
        departure as usize,
        arrival as usize,
    );

    let path_indices = nodes.research_path();

    for (counter, index) in path_indices.iter().enumerate() {
        path[counter] = *index as u8;
    }
}

/// Returns positions of an index according to the width.
///
/// # Arguments:
///
/// * `width` - the width of the map
/// * `index` - the source index
///
/// # Returns:
///
/// Tuple that contains the horizontal and vertical positions.
#[no_mangle]
pub fn get_positions(
    width: u8,
    index: u8,
) -> (u8, u8) {

    (
        index % width,
        index / width,
    )
}

#[cfg(test)]
mod tests;
