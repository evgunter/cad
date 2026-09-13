//! The Euler–Poincaré census as the suites ask it — a built body's ring
//! count and its genus, through `topo::readback::euler_counts`. One home
//! per test crate: a suite that asks a body its genus imports the
//! question here rather than spelling the identity again.

use topo::Body;
use topo::readback::euler_counts;

/// The body's ring count — every face's rings, summed.
pub fn rings_of(body: &Body<f64>) -> i64 {
    euler_counts(body).r
}

/// The body's whole-body genus. An odd census is a torn store, and the
/// row fails on the door's typed refusal in its own words.
pub fn genus_of(body: &Body<f64>) -> i64 {
    euler_counts(body)
        .genus()
        .unwrap_or_else(|refusal| panic!("{refusal}"))
}
