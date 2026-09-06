//! Accepting fixtures: closed-form profiles that must validate, with
//! their canonical forms pinned (roles, traversal, starting vertex,
//! classified segment kinds).
//!
//! The `Probe` instantiation smoke test lives in `validate_ok_probe.rs`,
//! gated on the `probe` feature; everything here is f64 and runs in the
//! default build.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{annulus, circle_h, l_profile, lens, profile, rect, rounded_rect, tol};
use geom_core::Sign;
use profile::{LoopRole, SegmentKind, ValidatedProfile};

/// Convenience: validate an f64 profile or panic with the error.
fn ok(p: &profile::Profile<f64>) -> ValidatedProfile<f64> {
    p.validate(tol()).expect("fixture must validate")
}

fn kinds(vp: &ValidatedProfile<f64>, li: usize) -> Vec<char> {
    vp.loops()[li]
        .segments()
        .iter()
        .map(|s| match s.kind {
            SegmentKind::Line => 'L',
            SegmentKind::Arc {
                turn: Sign::Positive,
                ..
            } => '+',
            SegmentKind::Arc {
                turn: Sign::Negative,
                ..
            } => '-',
            SegmentKind::Arc { .. } => '?',
        })
        .collect()
}

#[test]
fn rectangle_canonical_form() {
    let vp = ok(&profile(vec![rect(0.0, 0.0, 2.0, 2.0)]));
    assert_eq!(vp.loops().len(), 1);
    let lp = &vp.loops()[0];
    assert_eq!(lp.role(), LoopRole::Outer);
    // Canonical start = lexicographic minimum = (0, 0); CCW input
    // preserved verbatim.
    let vs = lp.vertices();
    assert_eq!(vs.len(), 4);
    assert_eq!((vs[0].pos().x, vs[0].pos().y), (0.0, 0.0));
    assert_eq!((vs[1].pos().x, vs[1].pos().y), (2.0, 0.0));
    assert_eq!((vs[2].pos().x, vs[2].pos().y), (2.0, 2.0));
    assert_eq!((vs[3].pos().x, vs[3].pos().y), (0.0, 2.0));
    assert_eq!(kinds(&vp, 0), vec!['L'; 4]);
    // Segment chain is coherent: segment k runs vertex k → k+1.
    let segs = lp.segments();
    assert_eq!(segs.len(), 4);
    assert_eq!((segs[3].start.x, segs[3].start.y), (0.0, 2.0));
    assert_eq!((segs[3].end.x, segs[3].end.y), (0.0, 0.0));
}

#[test]
fn clockwise_rectangle_canonicalizes_to_the_same_form() {
    let ccw = ok(&profile(vec![rect(0.0, 0.0, 2.0, 2.0)]));
    let cw = ok(&profile(vec![rect(0.0, 0.0, 2.0, 2.0).reversed()]));
    assert_eq!(format!("{ccw:?}"), format!("{cw:?}"));
}

#[test]
fn l_profile_validates_with_canonical_start() {
    let vp = ok(&profile(vec![l_profile()]));
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    let v0 = vp.loops()[0].vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (0.0, 0.0));
    assert_eq!(kinds(&vp, 0), vec!['L'; 6]);
}

#[test]
fn circle_as_two_arcs_is_the_minimal_closed_carrier() {
    let vp = ok(&profile(vec![circle_h(0.0, 0.0, 2.0)]));
    let lp = &vp.loops()[0];
    assert_eq!(lp.role(), LoopRole::Outer);
    // Canonical start: the lexicographic minimum (−2, 0).
    let v0 = lp.vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (-2.0, 0.0));
    assert_eq!(kinds(&vp, 0), vec!['+', '+']);
    for s in lp.segments() {
        match s.kind {
            SegmentKind::Arc {
                center,
                radius,
                turn,
            } => {
                assert!(center.x.abs() < 1e-12 && center.y.abs() < 1e-12);
                assert!((radius - 2.0).abs() < 1e-12);
                assert_eq!(turn, Sign::Positive);
            }
            SegmentKind::Line => panic!("circle segments must classify as arcs"),
        }
    }
}

#[test]
fn annulus_roles_and_hole_reorientation() {
    let vp = ok(&annulus());
    assert_eq!(vp.loops().len(), 2);
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
    // Outer stays counterclockwise (positive bulges), the hole is
    // re-oriented clockwise (bulges negated by the reversal).
    assert_eq!(kinds(&vp, 0), vec!['+', '+']);
    assert_eq!(kinds(&vp, 1), vec!['-', '-']);
    assert_eq!(vp.loops()[1].vertices()[0].bulge(), -1.0);
    // Canonical starts: lexicographic minima of each circle.
    let o0 = vp.loops()[0].vertices()[0].pos();
    let h0 = vp.loops()[1].vertices()[0].pos();
    assert_eq!((o0.x, o0.y), (-2.0, 0.0));
    assert_eq!((h0.x, h0.y), (-1.0, 0.0));
}

#[test]
fn annulus_loop_discovery_order_puts_outer_first() {
    // Same annulus with the hole listed first: the outer is hoisted to
    // the front, and the canonical form is identical byte-for-byte.
    let base = ok(&annulus());
    let swapped = ok(&profile(vec![
        circle_h(0.0, 0.0, 1.0),
        circle_h(0.0, 0.0, 2.0),
    ]));
    assert_eq!(format!("{base:?}"), format!("{swapped:?}"));
}

#[test]
fn rounded_rectangle_alternates_lines_and_ccw_arcs() {
    let vp = ok(&profile(vec![rounded_rect(4.0, 3.0, 0.5)]));
    let lp = &vp.loops()[0];
    assert_eq!(lp.role(), LoopRole::Outer);
    // Lexicographic minimum vertex: (0, 0.5) — the input's closing
    // vertex — so the canonical chain starts with its corner arc.
    let v0 = lp.vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (0.0, 0.5));
    assert_eq!(kinds(&vp, 0), vec!['+', 'L', '+', 'L', '+', 'L', '+', 'L']);
    // Corner arc geometry: the first canonical segment is the corner
    // about (0.5, 0.5) with radius 0.5.
    match lp.segments()[0].kind {
        SegmentKind::Arc { center, radius, .. } => {
            assert!((center.x - 0.5).abs() < 1e-12);
            assert!((center.y - 0.5).abs() < 1e-12);
            assert!((radius - 0.5).abs() < 1e-12);
        }
        SegmentKind::Line => panic!("corner must classify as an arc"),
    }
}

#[test]
fn lens_two_vertex_loop_with_distinct_carriers() {
    let vp = ok(&profile(vec![lens()]));
    let lp = &vp.loops()[0];
    assert_eq!(lp.role(), LoopRole::Outer);
    assert_eq!(kinds(&vp, 0), vec!['+', '-']);
    let v0 = lp.vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (0.0, 0.0));
}

#[test]
fn validated_profile_exposes_the_plane_unchanged() {
    let p = profile(vec![rect(0.0, 0.0, 1.0, 1.0)]);
    let vp = ok(&p);
    // Identity placement passes through untouched.
    let world = vp.plane().to_world(geom_core::Point2::new(0.25, 0.5));
    assert_eq!((world.x, world.y, world.z), (0.25, 0.5, 0.0));
}

#[test]
fn reversal_is_a_bit_exact_involution() {
    let loops = [
        rect(0.0, 0.0, 2.0, 1.0),
        circle_h(1.0, -2.0, 0.75),
        rounded_rect(4.0, 3.0, 0.5),
        common::bracket(),
        lens(),
    ];
    for lp in &loops {
        let back = lp.reversed().reversed();
        assert_eq!(back.vertices().len(), lp.vertices().len());
        // Declared-tangent joints round-trip exactly too (the reversal
        // remap is an involution).
        assert_eq!(back.tangent_joints(), lp.tangent_joints());
        for (a, b) in lp.vertices().iter().zip(back.vertices().iter()) {
            assert_eq!(a.pos().x.to_bits(), b.pos().x.to_bits());
            assert_eq!(a.pos().y.to_bits(), b.pos().y.to_bits());
            assert_eq!(a.bulge().to_bits(), b.bulge().to_bits());
        }
    }
}

#[test]
fn reversed_loop_validates_to_the_identical_canonical_form() {
    // The reversal involution seen through validation: winding is
    // invisible, so the reversed input produces the same canonical
    // bytes.
    let base = ok(&annulus());
    let reversed = ok(&profile(vec![
        circle_h(0.0, 0.0, 2.0).reversed(),
        circle_h(0.0, 0.0, 1.0).reversed(),
    ]));
    assert_eq!(format!("{base:?}"), format!("{reversed:?}"));
}
