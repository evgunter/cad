//! **A wrap edge's tip reads its face's full turn in the Boolean.** The
//! unit hemisphere whose curved face reaches its pole by one wrap edge,
//! walked out and back (DESIGN `[topology]`'s seam clause), cut by boxes
//! whose faces pass through the pole: the half-spaces bounded by the
//! planes `x = 0` (square to the wrap edge's plane) and `z = 0` (holding
//! the wrap edge), and the four quarters. At the tip the face's one corner is a full turn round
//! the one edge, as a slit's tip is (`topo::sector_shape::SlitTips`, read
//! off the loop by `topo::surface_boundary::tips`), and a germ along the
//! edge faces the strut by the run's order (`boolean::insert`'s
//! `strut_faces_first`). Each op, either way round, holds its closed form
//! and passes tiers 3 and 3′. (A plane through the pole tilted off the
//! axis cuts the sphere in a circle through the pole, which mints no
//! pcurve with or without the tip:
//! `work/charttail/a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve.md`.)
//!
//! The cone of the same height, its apex reached by one wrap edge, splits
//! through the apex at its closed forms; its Boolean through the apex
//! refuses `NormalAtConeApex`, the Boolean's apex sector algebra not yet
//! built.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::wrap_tip::{cone, hemisphere, one_wrap_edge};
use core::f64::consts::PI;
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{
    AtRestBody, BooleanResult, ContactRecords, SplitPart, intersect, split, subtract,
    validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

/// The hemisphere's volume.
const HEMI: f64 = 2.0 * PI / 3.0;

fn holds(what: &str, b: &AtRestBody<f64>, want: f64) {
    let mp = topo::test_support::pinned_volume(what, b, tol());
    let rounding = 64.0 * f64::EPSILON * want;
    assert!(
        (mp.volume - want).abs() <= mp.volume_pad + rounding,
        "{what}: holds {} ± {:e}, not {want}",
        mp.volume,
        mp.volume_pad
    );
    assert_eq!(validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(b, &ContactRecords::default(), tol()),
        Ok(()),
        "{what}: tier 3′"
    );
}

fn body_of(r: Result<BooleanResult<f64>, topo::BooleanError>, what: &str) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// The hemisphere with its pole a wrap tip.
fn tipped_hemisphere() -> AtRestBody<f64> {
    finished(
        "the tipped hemisphere",
        one_wrap_edge(hemisphere(), true).0,
        tol(),
    )
}

/// The box over `x × y × z`.
fn brick(x: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "a box",
        topo::test_support::brick(x, (-0.5, 2.0), z, tol()),
        tol(),
    )
}

/// Holds `body ∩ tool`, `tool ∩ body`, `body ∖ tool` and `tool ∖ body`
/// to their closed forms, `inside` the volume the two share and `size`
/// the tool's.
fn cuts(what: &str, body: &AtRestBody<f64>, (tool, size): (&AtRestBody<f64>, f64), inside: f64) {
    for (op, r, want) in [
        ("body ∩ tool", intersect(body, tool, tol()), inside),
        ("tool ∩ body", intersect(tool, body, tol()), inside),
        ("body ∖ tool", subtract(body, tool, tol()), HEMI - inside),
        ("tool ∖ body", subtract(tool, body, tol()), size - inside),
    ] {
        let what = format!("{what}, {op}");
        holds(&what, &body_of(r, &what), want);
    }
}

/// **The body is at rest at its closed form.**
#[test]
fn the_tipped_hemisphere_is_at_rest() {
    holds("the tipped hemisphere", &tipped_hemisphere(), HEMI);
}

/// **The fast row**: the hemisphere less the half-space square to the
/// wrap edge's plane (the tip's corner a full turn), and less the one
/// holding the wrap edge (a germ along the tip's edge).
#[test]
fn the_tipped_hemisphere_less_a_half_space_through_the_pole() {
    let body = tipped_hemisphere();
    for (what, x, z) in [
        ("x > 0", (0.0, 2.0), (-2.0, 2.0)),
        ("z > 0", (-2.0, 2.0), (0.0, 2.0)),
    ] {
        let what = format!("the hemisphere less {what}");
        holds(
            &what,
            &body_of(subtract(&body, &brick(x, z), tol()), &what),
            HEMI / 2.0,
        );
    }
}

/// **A half-space through the axis**, either side of either plane:
/// square to the wrap edge's plane, and holding the wrap edge.
#[test]
fn a_half_space_through_the_pole_cuts_the_tipped_hemisphere() {
    let body = tipped_hemisphere();
    for (what, x, z) in [
        ("x > 0", (0.0, 2.0), (-2.0, 2.0)),
        ("x < 0", (-2.0, 0.0), (-2.0, 2.0)),
        ("z > 0", (-2.0, 2.0), (0.0, 2.0)),
        ("z < 0", (-2.0, 2.0), (-2.0, 0.0)),
    ] {
        cuts(what, &body, (&brick(x, z), 2.5 * 2.0 * 4.0), HEMI / 2.0);
    }
}

/// **A quarter through the axis**, each of the four.
#[test]
fn a_quarter_through_the_pole_cuts_the_tipped_hemisphere() {
    let body = tipped_hemisphere();
    for (x, z) in [
        ((0.0, 2.0), (0.0, 2.0)),
        ((-2.0, 0.0), (-2.0, 0.0)),
        ((0.0, 2.0), (-2.0, 0.0)),
        ((-2.0, 0.0), (0.0, 2.0)),
    ] {
        cuts(
            &format!("the quarter x ∈ {x:?}, z ∈ {z:?}"),
            &body,
            (&brick(x, z), 2.5 * 2.0 * 2.0),
            HEMI / 4.0,
        );
    }
}

/// **The cone's apex reached by one wrap edge splits** through planes
/// holding the axis, each side half the cone; its Boolean through the
/// apex refuses by name.
#[test]
fn the_tipped_cones_apex_splits_and_the_boolean_refuses_it_by_name() {
    let body = finished("the tipped cone", one_wrap_edge(cone(), true).0, tol());
    for n in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)] {
        let what = format!("the split along {n:?}");
        let r = split(
            &body,
            &topo::test_support::split_plane(Point3::origin(), n, tol()),
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
        for (side, part) in [("above", &r.above), ("below", &r.below)] {
            let SplitPart::Body(b) = part else {
                panic!("{what}: nothing {side}");
            };
            holds(&format!("{what}, {side}"), b, PI / 6.0);
        }
    }
    match subtract(&body, &brick((0.0, 2.0), (-2.0, 2.0)), tol()) {
        Err(topo::BooleanError::NormalAtConeApex { .. }) => {}
        other => panic!("the tipped cone less x > 0: {other:?}"),
    }
}
