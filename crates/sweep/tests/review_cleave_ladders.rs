//! Reviewer rows for PR 3716 (CLEAVE `cleave/ladders`): the
//! cell-dimension witness ladder answered past an in-band witness, and
//! the role resolution that ships mergedoor scenes A and B, read at
//! points rather than through volume alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{collar, collar_at, peg_at, wall_decls};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use sweep::test_support::{brick, extruded, sketch_from_axes};
use topo::{Body, BooleanDeclarations, BooleanOp, BooleanResult, SolidContainment, SweepStrategy};

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
    let b: Body<f64> = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
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

/// Mergedoor scenes A and B (`curved_mergedoor.rs` row 2) union to a
/// collar whose bore holds the peg exactly where the peg is: each
/// section loop's role is read at points of the result, which an
/// additive volume alone does not pin.
#[test]
fn mergedoor_scenes_a_and_b_hold_the_peg_where_it_is() {
    use SolidContainment::{In, Out};
    let band = Band::linear(tol()).unwrap();
    for (label, a, b, pts) in [
        (
            "A",
            collar(),
            peg_at(0.0, 1.5, 1.0),
            vec![
                ((0.0, 0.0, 1.25), Out),
                ((0.0, 0.0, 1.75), In),
                ((0.0, 0.0, 2.25), In),
                ((1.0, 0.0, 1.5), In),
                ((1.0, 0.0, 2.25), Out),
            ],
        ),
        (
            "B",
            collar_at(0.0),
            peg_at(0.0, 0.5, 1.0),
            vec![
                ((0.0, 0.0, 0.75), In),
                ((0.0, 0.0, 1.25), In),
                ((0.0, 0.0, 1.75), Out),
                ((1.0, 0.0, 1.5), In),
                ((1.0, 0.0, 0.75), Out),
            ],
        ),
    ] {
        let d = wall_decls(&a, &b);
        let BooleanResult::Body(bb) =
            topo::union_with(&a, &b, &d, tol()).unwrap_or_else(|e| panic!("{label}: {e:?}"))
        else {
            panic!("{label}: a union of two solids is not empty");
        };
        assert_eq!(bb.body.solids().count(), 1, "{label}: one solid");
        for ((x, y, z), want) in pts {
            assert_eq!(
                topo::point_in_solid(&bb.body, Point3::new(x, y, z), band, tol()).ok(),
                Some(want),
                "{label} at ({x}, {y}, {z})"
            );
        }
    }
}
