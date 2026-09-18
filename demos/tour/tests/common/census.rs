//! The Euler–Poincaré census as the tour's suites ask it — a built
//! body's ring count and its genus, through
//! `pncad::topo::readback::euler_counts`. Shared by every suite in this
//! cargo root through `#[path]`, as `rim_select` is: one home, so a
//! suite that asks a body its genus imports the question rather than
//! spelling the identity.

use pncad::topo::Body;
use pncad::topo::readback::euler_counts;

/// The body's ring count — every face's rings, summed.
pub fn rings(body: &Body<f64>) -> i64 {
    euler_counts(body).r
}

/// The body's whole-body genus. An odd census is a torn store, and the
/// row fails on the door's typed refusal in its own words.
pub fn genus(body: &Body<f64>) -> i64 {
    euler_counts(body)
        .genus()
        .unwrap_or_else(|refusal| panic!("{refusal}"))
}
