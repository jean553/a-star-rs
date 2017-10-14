//! TODO: #37 the `nodes` module uses Vec::remove_item which is part
//! of the nightly experimental API; this flag should be removed
//! as soon as the feature becomes stable
#![feature(vec_remove_item)]

mod node;
mod nodes;

#[cfg(test)]
mod tests;
