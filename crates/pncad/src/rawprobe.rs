//! REVIEW PROBE rawtarget-rv P3: bypass hunt — can a consumer reach a
//! `MeshPick` without either gated mint?
#![allow(dead_code, unused_imports)]
use editor_core::{MeshPick, NodePick, PickTarget};

pub fn p3a(m: &mesh::Mesh) -> Result<MeshPick, editor_core::MeshPickError> {
    MeshPick::build_every_table(m) // pub(crate)
}
pub fn p3b(n: &NodePick) -> &MeshPick {
    n.pick // private field
}
pub fn p3c() -> MeshPick {
    MeshPick::default() // no Default
}
pub fn p3d<'a>(t: PickTarget<'a>) -> &'a MeshPick {
    t.pick // private field
}
