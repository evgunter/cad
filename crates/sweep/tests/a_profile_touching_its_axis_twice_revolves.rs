//! **A profile touching its axis more than once revolves**: a U whose two
//! feet lie along the axis, a comb with three, and a zigzag that touches
//! it at single vertices, each about the canonical test axis (`y`, the
//! profile in `x ≥ 0`), by a quarter turn, a half turn either way, three
//! quarters and a full turn. One row keeps the reported spelling: the U
//! on `SketchPlane::zx()` about the sketch's `u` line.
//!
//! A partial turn's caps meet along every on-axis run as one axis edge
//! each. At a half turn those caps are coplanar: every axis edge reads
//! the join smooth, and the caps end on the start cap's record with one
//! row. Away from π the caps are two records and every axis edge is
//! their `Intersection`. A zigzag's caps share only its pinned vertices,
//! so no edge decides them one carrier at any angle, and a full turn of
//! it refuses as the pinch it would be.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{
    assert_all_tiers, assert_declared_image_in, assert_half_turn_caps, axis_y, counts, description,
    start_cap_record, validated, validated_on,
};

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_brep::EdgeDescription;
use geom_core::{Point2, Tol, Vec2};
use profile::{ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, RevolveError, Revolved, RevolvedKind, revolve};

/// The U: two feet along the axis, `y ∈ [0, 1]` and `y ∈ [2, 3]`, a
/// notch `[0, 1] × [1, 2]` between them. `∬ x dA = 6 − ½`.
const U: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
    (0.0, 3.0),
    (2.0, 3.0),
    (2.0, 0.0),
];

/// Three feet along the axis, two unit notches between them.
/// `∬ x dA = 10 − 2·½`.
const COMB: [(f64, f64); 12] = [
    (0.0, 0.0),
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
    (0.0, 3.0),
    (1.0, 3.0),
    (1.0, 4.0),
    (0.0, 4.0),
    (0.0, 5.0),
    (2.0, 5.0),
    (2.0, 0.0),
];

/// Touching the axis at two single vertices, `y = 1` and `y = 3`.
/// `∬ x dA = 8 − 4·⅙`.
const ZIGZAG: [(f64, f64); 7] = [
    (1.0, 0.0),
    (0.0, 1.0),
    (1.0, 2.0),
    (0.0, 3.0),
    (1.0, 4.0),
    (2.0, 4.0),
    (2.0, 0.0),
];

const PARTIAL_ANGLES: [f64; 4] = [FRAC_PI_2, PI, -PI, 3.0 * FRAC_PI_2];

fn polygon(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)))
}

fn revolved(
    pts: &[(f64, f64)],
    revolution: Revolution<f64>,
) -> Result<Revolved<f64>, RevolveError> {
    revolve(
        &validated(vec![polygon(pts)]),
        axis_y(),
        revolution,
        Tol::witness(),
    )
}

/// Pappus: the swept volume is `|θ| ∬ x dA`, positive only where the
/// body is wound outward. Pins the shape and its winding; it cannot see
/// which records the faces wear.
fn assert_volume(label: &str, t: &Revolved<f64>, theta: f64, moment: f64) {
    let tol = Tol::witness();
    let got = topo::mass_properties(&t.body, tol, topo::test_support::pinned_resolution(tol))
        .expect("mass properties compute")
        .enclosure
        .volume;
    let want = theta.abs() * moment;
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "{label}: volume {got}, Pappus {want}"
    );
}

/// The axis edges of a partial revolve: one per on-axis run, each a
/// single edge both caps share.
fn axis_edges(label: &str, t: &Revolved<f64>) -> Vec<topo::EdgeKey> {
    let RevolvedKind::Partial {
        start_meridians,
        end_meridians,
        ..
    } = &t.kind
    else {
        panic!("{label}: a partial revolve");
    };
    t.walls()[0]
        .iter()
        .enumerate()
        .filter(|(_, w)| w.is_none())
        .map(|(j, _)| {
            assert_eq!(
                start_meridians[0][j], end_meridians[0][j],
                "{label}: on-axis segment {j} is one edge the caps share"
            );
            start_meridians[0][j]
        })
        .collect()
}

/// The caps' records and the axis edges' descriptions at `theta`: at a
/// half turn, the caps on the start cap's record (the plane the same
/// turn's sign gives the start cap at a quarter turn) with the row in
/// the act's order, and every axis edge an image in that chart; away
/// from π, two records, no row, and every axis edge their
/// `Intersection`.
fn assert_caps(
    label: &str,
    t: &Revolved<f64>,
    theta: f64,
    quarter: impl FnOnce() -> Revolved<f64>,
) {
    let RevolvedKind::Partial {
        start_cap,
        end_cap,
        coplanar_caps,
        ..
    } = &t.kind
    else {
        panic!("{label}: a partial revolve");
    };
    let key = |f: &topo::FaceKey| t.body.get_face(*f).unwrap().surface;
    let edges = axis_edges(label, t);
    if theta.abs() == PI {
        assert_half_turn_caps(t, &start_cap_record(&quarter()));
        for &e in &edges {
            assert_declared_image_in(&t.body, e, key(start_cap));
        }
    } else {
        assert_ne!(key(start_cap), key(end_cap), "{label}: two cap records");
        assert_eq!(*coplanar_caps, None, "{label}: no coincidence");
        for &e in &edges {
            assert!(
                matches!(
                    description(&t.body, e),
                    EdgeDescription::Intersection { .. }
                ),
                "{label}: axis edge {e:?} is the caps' Intersection"
            );
        }
    }
}

/// A partial turn of a profile with `runs` on-axis runs at every angle.
fn assert_partials(name: &str, pts: &[(f64, f64)], runs: usize, moment: f64) {
    for theta in PARTIAL_ANGLES {
        let label = format!("{name} by {theta}");
        let t = revolved(pts, Revolution::Partial(theta))
            .unwrap_or_else(|e| panic!("{label}: the wedge builds, got {e}"));
        assert_all_tiers(&t.body);
        assert_volume(&label, &t, theta, moment);
        assert_eq!(
            axis_edges(&label, &t).len(),
            runs,
            "{label}: one axis edge per run"
        );
        let quarter = || revolved(pts, Revolution::Partial(FRAC_PI_2.copysign(theta))).unwrap();
        assert_caps(&label, &t, theta, quarter);
    }
}

#[test]
fn a_u_touching_its_axis_twice_revolves_by_every_partial_angle() {
    assert_partials("U", &U, 2, 5.5);
}

#[test]
fn a_comb_touching_its_axis_three_times_revolves_by_every_partial_angle() {
    assert_partials("comb", &COMB, 3, 9.0);
}

/// The reported spelling: the U drawn in `(u, v)` on `SketchPlane::zx()`,
/// revolved by π about the `u` line with the axis pointing along `−u`.
#[test]
fn the_reported_u_half_revolves() {
    let pts: Vec<(f64, f64)> = U.iter().map(|&(x, y)| (y, x)).collect();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(-1.0, 0.0),
    };
    let build = |theta| {
        revolve(
            &validated_on(SketchPlane::zx(), vec![polygon(&pts)]),
            axis,
            Revolution::Partial(theta),
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("the reported U by {theta} builds, got {e}"))
    };
    let t = build(PI);
    assert_all_tiers(&t.body);
    assert_volume("the reported U", &t, PI, 5.5);
    assert_caps("the reported U", &t, PI, || build(FRAC_PI_2));
}

/// The pinned vertices are the only contact: no axis edge, so the caps
/// stay two records even where they are coplanar.
#[test]
fn a_zigzag_touching_its_axis_at_two_vertices_revolves_by_every_partial_angle() {
    for theta in PARTIAL_ANGLES {
        let label = format!("zigzag by {theta}");
        let t = revolved(&ZIGZAG, Revolution::Partial(theta))
            .unwrap_or_else(|e| panic!("{label}: the wedge builds, got {e}"));
        assert_all_tiers(&t.body);
        assert_volume(&label, &t, theta, 8.0 - 4.0 / 6.0);
        assert!(
            t.walls()[0].iter().all(Option::is_some),
            "{label}: no axis edge"
        );
        assert_eq!(t.poles[0].iter().flatten().count(), 2, "{label}: two poles");
        let RevolvedKind::Partial { coplanar_caps, .. } = &t.kind else {
            panic!("{label}: a partial revolve");
        };
        assert_eq!(*coplanar_caps, None, "{label}: no edge decides the caps");
    }
}

/// A full turn closes each notch into a cavity shell. The counts pin the
/// full turn's topology as it is built today, each curved wall one band
/// closed on its wrap edge; the cavity count and the volume pin what any
/// correct full turn of these profiles is.
#[test]
fn a_full_turn_closes_each_notch_into_a_cavity() {
    for (label, pts, cavities, want_counts, moment) in [
        ("U", &U[..], 1, (4, 6, 6, 0), 5.5),
        ("comb", &COMB[..], 2, (6, 9, 9, 0), 9.0),
    ] {
        let t = revolved(pts, Revolution::Full)
            .unwrap_or_else(|e| panic!("{label}: the full turn builds, got {e}"));
        assert_all_tiers(&t.body);
        assert_eq!(t.cavities.len(), cavities, "{label}: one cavity per notch");
        assert_eq!(counts(&t.body), want_counts, "{label}: (v, e, f, r)");
        assert_volume(label, &t, TAU, moment);
    }
}

#[test]
fn a_full_turn_of_a_zigzag_refuses_the_pinch() {
    let refused = revolved(&ZIGZAG, Revolution::Full).err();
    assert!(
        matches!(refused, Some(RevolveError::NonManifoldAxisContact { .. })),
        "{refused:?}"
    );
}
