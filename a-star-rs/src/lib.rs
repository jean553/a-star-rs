#![feature(vec_remove_item)]
#![feature(libc)]

extern crate libc;

/// One node structure and its implementation.
mod node;

/// Contains all nodes.
mod nodes;

/// Main interface that calculates and stores the path into the given C array.
///
/// # Args:
///
/// * `path` - C pointer to the array of indices that are part of the path
/// * `width` - the number of nodes per line (max allowed: 10),
/// * `height` - the number of nodes per column (max allowed: 10),
/// * `departure` - the departure node index,
/// * `arrival` - the arrival node index
#[no_mangle]
pub fn get_path(
    path: *const libc::uint8_t,
    width: u8,
    height: u8,
    departure: u8,
    arrival: u8,
) {
    const C_PATH_ARRAY_SIZE: usize = 100;
    let path: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(
            path as *mut u8,
            C_PATH_ARRAY_SIZE,
        )
    };

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
