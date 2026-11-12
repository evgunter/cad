//! **A ring crossed only at its vertices is divided there, or the
//! division refuses.** A plate drilled through by a diamond, its
//! corners at `(±1, 0)` and `(0, ±1)`, is cut along `x = d` by a
//! subtract and an intersect with the prism `x ≥ d`, and by the split
//! at `x = d`. The cut crosses the ring at its two corners `(0, ±1)`
//! and nowhere else.
//!
//! This is the fact `chord_join::paths_rung`'s premise rests on: a ring
//! left a bystander of a division does not cross it, so every vertex
//! reads one side. A crossing at a vertex is never left to a bystander's
//! side reading:
//!
//! - **`d = 0` and `d` inside the zero band**: the division identifies
//!   both corners and divides the ring there. Each side keeps half the
//!   diamond, `17 m³` within the offset's sliver, and subtract plus
//!   intersect is the plate.
//! - **`d` in the band**: the corners' coincidence with the cut is
//!   undecided, and the operation refuses there (the boolean's
//!   `VertexOnFace`, the split's `SliverVertex`).
//! - **`d` past the band**: the cut misses the corners and crosses two
//!   of the ring's edges, which it divides.
//!
//! It guards the division, not the side ladders: a division that skipped
//! a corner would leave a ring that crosses the cut. The operation's
//! at-rest finish refuses that ring by check 9, so the row turns red on
//! the refused result, before any volume is read.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common;
use common::{
    FaceGeometry, describe_as_intersections, drill_hole, finished, identity_map, plane_every_face,
    prism_ops, prism_z, split_plane,
};
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{
    AtRestBody, Body, BooleanDecision, BooleanError, BooleanResult, SplitPart, intersect,
    mass_properties, split, subtract,
};

fn diamond_plate(tol: Tol) -> AtRestBody<f64> {
    let pt = identity_map::<f64>;
    let mut body = Body::<f64>::new();
    let ops = prism_ops(
        &mut body,
        &[(-3.0, -3.0), (3.0, -3.0), (3.0, 3.0), (-3.0, 3.0)],
        (0.0, 1.0),
        pt,
        FaceGeometry::Declined,
        tol,
    );
    let entry = ops.sides[0].he_plus;
    let diamond = [(1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)];
    let rim: Vec<_> = diamond.iter().map(|&(x, y)| pt(x, y, 1.0)).collect();
    let drops: Vec<_> = diamond.iter().map(|&(x, y)| pt(x, y, 0.0)).collect();
    drill_hole(&mut body, entry, ops.bottom.face, &rim, &drops, tol);
    plane_every_face(&mut body, tol);
    describe_as_intersections(&mut body, tol);
    finished("the diamond plate", body, tol)
}

fn volume(b: &AtRestBody<f64>, tol: Tol) -> f64 {
    mass_properties(b, tol, common::pinned_resolution(tol))
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume
}

/// The volume of the plate's part at `x ≥ d`: the half plate less the
/// diamond's half, `18 − 1`, moved by the strip `d` wide (`6·d` of
/// plate, less the diamond's `2·d` across its corners).
fn half(d: f64) -> f64 {
    17.0 - 4.0 * d
}

#[test]
fn a_ring_crossed_at_its_vertices_is_divided_or_refused() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let plate = diamond_plate(tol);
    let whole = volume(&plate, tol);
    assert!((whole - 34.0).abs() < 1e-12, "the plate: {whole}");
    // Each row: the offset, whether the ring divides, and whether the
    // cut is identified with the ring's corners (decided Zero) rather
    // than dividing its edges where they are.
    for (what, d, divides, identified) in [
        ("on the corners", 0.0, true, true),
        ("in the zero band", 0.5 * band.zero(), true, true),
        ("in the band", 3.0 * band.zero(), false, false),
        ("past the band", 3.0 * band.escalate(), true, false),
    ] {
        let tool = finished(
            "the prism x ≥ d",
            prism_z::<f64>(
                &[(d, -4.0), (4.0, -4.0), (4.0, 4.0), (d, 4.0)],
                -1.0,
                2.0,
                tol,
            )
            .body,
            tol,
        );
        let sub = subtract(&plate, &tool, tol);
        let int = intersect(&plate, &tool, tol);
        let cut = split(
            &plate,
            &split_plane(Point3::new(d, 0.0, 0.5), Vec3::unit_x(), tol),
            tol,
        );
        if divides {
            let body = |what: &str, r: Result<BooleanResult<f64>, BooleanError>| match r {
                Ok(BooleanResult::Body(b)) => volume(&b.body, tol),
                other => panic!("{what}: {:?}", other.map(|_| ())),
            };
            let (kept, taken) = (body("subtract", sub), body("intersect", int));
            // An identified cut's sliver: the strip `d` wide across the
            // plate's 6 m section, which the corners identified at `x = 0`
            // leave the cut face's fan to read either side of. A cut past
            // the band divides the ring's edges exactly and has none.
            let sliver = if identified { 6.0 * d } else { 0.0 };
            assert!(
                (taken - half(d)).abs() < 1e-9 + sliver && (kept + taken - whole).abs() < 1e-9,
                "{what}: intersect {taken} is the half at x ≥ d, subtract {kept} the rest"
            );
            let r = cut.unwrap_or_else(|e| panic!("{what}: the split refused: {e:?}"));
            let side = |s: &SplitPart<f64>| match s {
                SplitPart::Body(b) => volume(b, tol),
                SplitPart::Empty => panic!("{what}: a side of the split is empty"),
            };
            let (above, below) = (side(&r.above), side(&r.below));
            assert!(
                (above - half(d)).abs() < 1e-9 + sliver && (above + below - whole).abs() < 1e-9,
                "{what}: the split's sides {above} + {below}"
            );
        } else {
            for (op, r) in [("subtract", sub), ("intersect", int)] {
                assert!(
                    matches!(
                        r,
                        Err(BooleanError::Escalated {
                            decision: BooleanDecision::Coincidence(..),
                            ..
                        })
                    ),
                    "{what}: {op} refuses on the corners' coincidence: {:?}",
                    r.map(|_| ())
                );
            }
            assert!(
                matches!(
                    cut,
                    Err(topo::SplitError::Reduce(
                        topo::SplitReduceError::SliverVertex { .. }
                    ))
                ),
                "{what}: the split refuses on a corner's side: {:?}",
                cut.map(|_| ())
            );
        }
    }
}
