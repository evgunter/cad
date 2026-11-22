//! **A profile touching its axis more than once revolves**: a U whose two
//! feet lie along the axis, a comb with three, and a zigzag that touches
//! it at single vertices, each on `SketchPlane::zx()` about the sketch's
//! `u` line, by a quarter turn, a half turn either way, three quarters
//! and a full turn.
//!
//! A partial turn's caps meet along every on-axis run as one axis edge
//! each. At a half turn those caps are coplanar: every axis edge reads
//! the join smooth, and the caps end on one record with one row. Away
//! from π the caps are two records and every axis edge is their
//! `Intersection`. A zigzag's caps share only its pinned vertices, so no
//! edge decides them one carrier at any angle, and a full turn of it
//! refuses as the pinch it would be.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{assert_all_tiers, assert_declared_image_in, counts, description};

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_brep::EdgeDescription;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, RevolveError, Revolved, RevolvedKind, revolve};

/// The U: two feet along the axis, `u ∈ [0, 1]` and `u ∈ [2, 3]`, a
/// notch `[1, 2] × [0, 1]` between them. `∬ v dA = 6 − ½`.
const U: [(f64, f64); 8] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (2.0, 1.0),
    (2.0, 0.0),
    (3.0, 0.0),
    (3.0, 2.0),
    (0.0, 2.0),
];

/// Three feet along the axis, two unit notches between them.
/// `∬ v dA = 10 − 2·½`.
const COMB: [(f64, f64); 12] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (2.0, 1.0),
    (2.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (4.0, 1.0),
    (4.0, 0.0),
    (5.0, 0.0),
    (5.0, 2.0),
    (0.0, 2.0),
];

/// Touching the axis at two single vertices, `u = 1` and `u = 3`.
/// `∬ v dA = 8 − 4·⅙`.
const ZIGZAG: [(f64, f64); 7] = [
    (0.0, 1.0),
    (1.0, 0.0),
    (2.0, 1.0),
    (3.0, 0.0),
    (4.0, 1.0),
    (4.0, 2.0),
    (0.0, 2.0),
];

const PARTIAL_ANGLES: [f64; 4] = [FRAC_PI_2, PI, -PI, 3.0 * FRAC_PI_2];

fn revolved(
    pts: &[(f64, f64)],
    revolution: Revolution<f64>,
) -> Result<Revolved<f64>, RevolveError> {
    let tol = Tol::witness();
    let lp = ProfileLoop::polygon(pts.iter().map(|&(u, v)| Point2::new(u, v)));
    let vp = Profile::new(SketchPlane::zx(), vec![lp])
        .validate(tol)
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(-1.0, 0.0),
    };
    revolve(&vp, axis, revolution, tol)
}

/// Pappus: the swept volume is `|θ| ∬ v dA`, and positive only where the
/// body is wound outward.
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

/// A partial turn of a profile with `runs` on-axis runs: valid at every
/// tier, one axis edge per run shared by the caps, the caps one record
/// with one row at a half turn, two records elsewhere with each axis edge
/// their `Intersection`.
fn assert_partial(label: &str, pts: &[(f64, f64)], theta: f64, runs: usize, moment: f64) {
    let t = revolved(pts, Revolution::Partial(theta))
        .unwrap_or_else(|e| panic!("{label}: the wedge builds, got {e}"));
    assert_all_tiers(&t.body);
    assert_volume(label, &t, theta, moment);
    let RevolvedKind::Partial {
        start_cap,
        end_cap,
        start_meridians,
        end_meridians,
        coplanar_caps,
        ..
    } = &t.kind
    else {
        panic!("{label}: a partial revolve");
    };
    let axis_edges: Vec<_> = t.walls()[0]
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
        .collect();
    assert_eq!(axis_edges.len(), runs, "{label}: one axis edge per run");
    let key = |f: &topo::FaceKey| t.body.get_face(*f).unwrap().surface;
    if theta.abs() == PI {
        assert_eq!(
            key(start_cap),
            key(end_cap),
            "{label}: the caps wear one record"
        );
        for &e in &axis_edges {
            assert_declared_image_in(&t.body, e, key(start_cap));
        }
        let row = coplanar_caps.expect("the caps' coincidence is recorded");
        let mut cells = row.cells.map(|c| match c {
            topo::RowCell::Result {
                cell: topo::Cell::Face(f),
            } => Some(f),
            _ => None,
        });
        cells.sort();
        let mut caps = [Some(*start_cap), Some(*end_cap)];
        caps.sort();
        assert_eq!(cells, caps, "{label}: the row names the two caps");
    } else {
        assert_ne!(key(start_cap), key(end_cap), "{label}: two cap records");
        assert_eq!(*coplanar_caps, None, "{label}: no coincidence");
        for &e in &axis_edges {
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

#[test]
fn a_u_touching_its_axis_twice_revolves_by_every_partial_angle() {
    for theta in PARTIAL_ANGLES {
        assert_partial(&format!("U by {theta}"), &U, theta, 2, 5.5);
    }
}

#[test]
fn a_comb_touching_its_axis_three_times_revolves_by_every_partial_angle() {
    for theta in PARTIAL_ANGLES {
        assert_partial(&format!("comb by {theta}"), &COMB, theta, 3, 9.0);
    }
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
/// full turn's topology as it is built today.
#[test]
fn a_full_turn_closes_each_notch_into_a_cavity() {
    for (label, pts, cavities, want_counts, moment) in [
        ("U", &U[..], 1, (8, 12, 8, 0), 5.5),
        ("comb", &COMB[..], 2, (12, 18, 12, 0), 9.0),
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
