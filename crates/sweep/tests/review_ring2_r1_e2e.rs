//! **The NURBS-walled loft prism, certified end to end**: validate,
//! mass properties, tessellation — the whole path a ring bound travels
//! before it reaches a user, walked through the public doors.
//!
//! The prism's volume is exactly `9` in ℝ, which is what makes the row
//! a gate rather than a print: the certified enclosure
//! `[volume − pad, volume + pad]` must contain it, at whatever ε the
//! leg runs. A bound that tightened past the truth breaks that, and no
//! digest in the tree states it about this body through these doors.
//!
//! The bits are printed as well as asserted, because RING-3 dissolves
//! the ring into `Interval` and re-takes exactly this comparison; the
//! numbers here move with ε and are therefore not pinned.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::loft_prism;

/// The prism is `3 × 3 × 1` in meters.
const EXACT_VOLUME: f64 = 9.0;

#[test]
fn ring2_r1_end_to_end_certificate_bits() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    topo::validate_geometric(&body, tol).expect("the prism is tier-3 valid");
    let props = topo::mass_properties(&body, tol).expect("mass properties");
    println!(
        "R1E2E volume={:.17e} bits={:#018x} pad={:.17e} pad_bits={:#018x}",
        props.volume,
        props.volume.to_bits(),
        props.volume_pad,
        props.volume_pad.to_bits()
    );
    println!(
        "R1E2E area={:.17e} bits={:#018x} apad={:.17e} apad_bits={:#018x}",
        props.surface_area,
        props.surface_area.to_bits(),
        props.area_pad,
        props.area_pad.to_bits()
    );
    // The certificate is an enclosure, so the one thing it may never
    // do is exclude the answer.
    assert!(
        props.volume_pad > 0.0 && props.volume_pad.is_finite(),
        "the volume pad is a width, not a refusal: {props:?}"
    );
    assert!(
        (props.volume - EXACT_VOLUME).abs() <= props.volume_pad,
        "the certified volume enclosure [{} +/- {}] excludes the prism's exact {EXACT_VOLUME}",
        props.volume,
        props.volume_pad
    );
    assert!(
        props.area_pad > 0.0 && props.area_pad.is_finite() && props.surface_area > 0.0,
        "the area certificate is a width around a positive area: {props:?}"
    );

    // Tessellation is the consumer at the far end of those bounds: a
    // tighter chordal tolerance may never ask for FEWER triangles.
    let mut coarser = 0usize;
    for chordal in [1e-1, 1e-2, 1e-3] {
        let mesh = mesh::tessellate(&body, chordal, tol).expect("tessellates");
        let triangles: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        println!(
            "R1E2E chordal={chordal:e} triangles={triangles} positions={} patches={}",
            mesh.positions.len(),
            mesh.patches.len()
        );
        assert!(
            triangles > coarser,
            "chordal {chordal:e} asked for {triangles} triangles, \
             no more than the coarser run's {coarser}"
        );
        coarser = triangles;
    }
}
