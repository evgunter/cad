//! The cell-dimension witness ladder (`topo`'s `boolean::shell_witness`)
//! passes over a witness that reads too near the other boundary to say,
//! and answers at the next.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{brick, extruded, finished, sketch_from_axes};
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanOp, BooleanResult, SweepStrategy};

fn tol() -> Tol {
    Tol::witness()
}

fn volume_of(r: BooleanResult<f64>) -> Option<f64> {
    match r {
        BooleanResult::Body(bb) => Some(topo::mass_properties(&bb.body, tol()).unwrap().volume),
        BooleanResult::Empty => None,
    }
}

/// A 0.6 × 0.4 × 0.4 box tilted 45° about `x`, its lowest edge laid
/// along the top rim `y = 1, z = 1` of the unit cube at `0.9·zero`
/// above it, the box leaning away from the cube. The operands touch
/// along that edge only, so the containment witness of each shell meets
/// a corner that reads in-band against the other operand's rim before
/// one that decides. Passing over it, every op answers the touching
/// result; a ladder that takes an in-band witness as a refusal refuses
/// all six.
#[test]
fn a_box_edge_resting_on_a_rim_answers_past_its_in_band_corner() {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let g = 0.9 * tol().eps();
    let a: Body<f64> = extruded(
        sketch_from_axes(
            Point3::new(0.2, 1.0, 1.0 + g),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, s, s),
            tol(),
        ),
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(0.6, 0.0), 0.0),
            (Point2::new(0.6, 0.4), 0.0),
            (Point2::new(0.0, 0.4), 0.0),
        ])],
        0.4,
        tol(),
    );
    let a = finished("the box", a, tol());
    let b: AtRestBody<f64> = finished(
        "the unit box",
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let va = 0.6 * 0.4 * 0.4;
    for (op, ab, ba) in [
        (BooleanOp::Union, Some(1.0 + va), Some(1.0 + va)),
        (BooleanOp::Subtract, Some(va), Some(1.0)),
        (BooleanOp::Intersect, None, None),
    ] {
        for (label, (p, q), want) in [("ab", (&a, &b), ab), ("ba", (&b, &a), ba)] {
            let got = topo::boolean_op_with(
                op,
                p,
                q,
                &BooleanDeclarations::none(),
                SweepStrategy::Realized,
                tol(),
            )
            .map(volume_of)
            .unwrap_or_else(|e| panic!("{op:?} {label}: {e:?}"));
            assert!(
                match (got, want) {
                    (Some(v), Some(w)) => (v - w).abs() < 1e-9,
                    (None, None) => true,
                    _ => false,
                },
                "{op:?} {label}: volume {got:?}, want {want:?}"
            );
        }
    }
}
