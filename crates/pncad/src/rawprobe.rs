//! REVIEW PROBE (lane rawtarget-rv): does a non-test consumer crate
//! have any route to either raw mint with `test-support` off?
#![allow(dead_code, unused_imports)]

use editor_core::{MeshPick, PickTarget};

pub fn probe_meshpick_build(m: &mesh::Mesh) -> Result<MeshPick, editor_core::MeshPickError> {
    MeshPick::build(m)
}

pub fn probe_picktarget_new<'a, T: geom_core::predicate::Decide>(
    eval: &editor_core::Evaluation<T>,
    node: editor_core::RecipeNodeId,
    body: u32,
    pick: &'a MeshPick,
) -> PickTarget<'a> {
    PickTarget::new(eval, node, body, pick)
}
