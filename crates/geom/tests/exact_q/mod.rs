//! The spline layer's exact rational oracle, shared with `geom-core`'s
//! suites: one oracle answers both crates' containment rows.

#[path = "../../../geom-core/tests/exact_q/mod.rs"]
mod shared;

pub(crate) use shared::Q;
