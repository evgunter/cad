//! REVIEW PROBE (review/kernel): print the certified volume enclosure
//! (midpoint, pad) at ambient eps for the quadrature bodies, mm^3.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;

#[test]
fn print_enclosures() {
    let eps = geom_core::Tolerance::get().eps;
    for (name, body) in common::fixture_corpus() {
        let p = topo::mass_properties(&body).unwrap();
        println!(
            "PROBE eps={eps:e} {name}: vol={} pad={}",
            p.volume * 1e9,
            p.volume_pad * 1e9
        );
    }
}
