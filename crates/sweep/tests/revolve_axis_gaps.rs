//! A full revolve of a profile whose outer loop meets the axis in k ≥ 2
//! runs: DEFINED as `revolve(filled) − Σ revolve(pocket)` (crate topo's
//! README, structural conventions), each gap between two runs closing a
//! pocket that revolves to a cavity inserted through the void door with
//! the profile's carried containment.
//!
//! Every row checks mass properties against a closed form derived by
//! hand (cylinders, cones and balls), which is also the orientation
//! oracle for each reversed cavity boundary: a cavity inserted with its
//! winding inverted would ADD its volume instead of removing it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::revolve_common;

use geom::Surface;
use geom_core::{Point2, Tol};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{CavityOrigin, Revolution, Revolved, RevolvedKind, revolve};
use topo::FaceKey;

const PI: f64 = core::f64::consts::PI;

/// Volume and surface area of the built body.
fn props(t: &Revolved<f64>) -> (f64, f64) {
    let tol = Tol::witness();
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&t.body), tol),
        tol,
    )
    .unwrap();
    (p.volume, p.surface_area)
}

fn assert_near(what: &str, got: f64, want: f64) {
    assert!(
        ((got - want) / want).abs() < 1e-12,
        "{what}: got {got}, want {want}"
    );
}

/// The faces of one shell.
fn shell_faces(t: &Revolved<f64>, shell: topo::ShellKey) -> BTreeSet<FaceKey> {
    t.body
        .get_shell(shell)
        .unwrap()
        .faces
        .iter()
        .copied()
        .collect()
}

/// The walls loop 0's canonical segments `segs` sweep, with their π
/// twins.
fn walls_of(t: &Revolved<f64>, segs: &[usize]) -> BTreeSet<FaceKey> {
    let RevolvedKind::Full { pi_walls, .. } = &t.kind else {
        panic!("a full revolve")
    };
    let walls = t.walls();
    segs.iter()
        .flat_map(|&s| [walls[0][s], pi_walls[s]])
        .flatten()
        .collect()
}

/// The cylinder radii among a shell's walls.
fn cylinder_radii(t: &Revolved<f64>, shell: topo::ShellKey) -> Vec<f64> {
    let mut radii: Vec<f64> = shell_faces(t, shell)
        .into_iter()
        .filter_map(|f| {
            let face = t.body.get_face(f).unwrap();
            match *t.body.get_surface(face.surface).unwrap() {
                Surface::Cylinder { radius, .. } => Some(radius),
                _ => None,
            }
        })
        .collect();
    radii.sort_by(f64::total_cmp);
    radii.dedup();
    radii
}

/// A radius-1, height-`h` column against the axis with square notches
/// `(depth, lo, hi)` cut into it from the axis, lowest first: one axis
/// run per notch plus one.
fn notched_column(h: f64, notches: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    let mut pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, h),
        Point2::new(0.0, h),
    ];
    for &(d, lo, hi) in notches.iter().rev() {
        pts.extend([
            Point2::new(0.0, hi),
            Point2::new(d, hi),
            Point2::new(d, lo),
            Point2::new(0.0, lo),
        ]);
    }
    ProfileLoop::polygon(pts)
}

/// **The C profile** (r ∈ [0,1], z ∈ [0,3], notch r < 0.5, z ∈ (1,2)):
/// the radius-1 column of height 3 with a radius-0.5 cylindrical cavity
/// at z ∈ [1,2]. Two shells in one solid, tier-3 valid; the cavity is
/// keyed as gap 0's pocket and its faces are exactly the notch's walls;
/// no crossing-pipeline predicate fires (the degenerate no-crossing
/// arm, as the holed revolve's suite pins it).
#[test]
fn the_c_profile_revolves_to_a_column_with_a_cavity() {
    let vp = validated(vec![notched_column(3.0, &[(0.5, 1.0, 2.0)])]);
    let bracket = geom_core::k_stats::Bracket::open();
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let verdicts = bracket.finish().verdicts;
    let crossing: Vec<_> = verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("bool_"))
        .collect();
    assert!(
        crossing.is_empty(),
        "the pocket is the degenerate no-crossing arm: got {crossing:?}"
    );
    assert!(
        verdicts.iter().any(|v| v.predicate == "axis_gap"),
        "the gap order is decided through the funnel"
    );
    assert_all_tiers(&t.body);

    assert_eq!(t.body.shells().count(), 2, "outer and one cavity");
    assert_eq!(t.cavities.len(), 1);
    let cavity = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    assert_ne!(cavity, t.shell);
    assert_eq!(t.body.get_shell(cavity).unwrap().solid, t.solid);
    // Canonical segments: 0..=2 the outer stretch, 3 the upper run,
    // 4..=6 the notch, 7 the lower run.
    assert_eq!(shell_faces(&t, cavity), walls_of(&t, &[4, 5, 6]));
    assert_eq!(shell_faces(&t, t.shell), walls_of(&t, &[0, 1, 2]));
    let walls = t.walls();
    assert!(
        walls[0][3].is_none() && walls[0][7].is_none(),
        "runs sweep nothing"
    );
    assert_eq!(cylinder_radii(&t, cavity), vec![0.5]);
    assert_eq!(cylinder_radii(&t, t.shell), vec![1.0]);

    // V = π·1²·3 − π·0.5²·1; A = (2π·1·3 + 2π·1²) + (2π·0.5·1 + 2π·0.5²).
    let (v, a) = props(&t);
    assert_near("volume", v, 2.75 * PI);
    assert_near("area", a, 8.0 * PI + 1.5 * PI);
}

/// **Three runs, two cavities**, keyed lowest gap first: the lower
/// notch (radius 0.5) is gap 0 and the upper one (radius 0.25) gap 1.
#[test]
fn three_runs_close_two_cavities_in_axial_order() {
    let vp = validated(vec![notched_column(
        5.0,
        &[(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)],
    )]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 3);
    assert_eq!(t.cavities.len(), 2);
    let low = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    let high = t.cavity(CavityOrigin::Pocket { gap: 1 }).unwrap();
    assert_ne!(low, high);
    assert_eq!(
        cylinder_radii(&t, low),
        vec![0.5],
        "gap 0 is the lower notch"
    );
    assert_eq!(
        cylinder_radii(&t, high),
        vec![0.25],
        "gap 1 is the upper notch"
    );
    let (v, _) = props(&t);
    assert_near("volume", v, 5.0 * PI - 0.25 * PI - PI / 16.0);
}

/// **A half-annulus between radii 1 and 2**: a hollow ball, each shell
/// one sphere in two π-bands with its poles on the axis.
#[test]
fn a_half_annulus_revolves_to_a_hollow_ball() {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -2.0), 1.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.0), -1.0),
        (Point2::new(0.0, -1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 2);
    let cavity = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    // Two sphere shells: two poles, two meridians, two bands each.
    assert_eq!(counts(&t.body), (4, 4, 4, 0));
    assert_eq!(
        t.poles[0].iter().flatten().count(),
        4,
        "every run end a pole"
    );
    for (shell, radius) in [(t.shell, 2.0), (cavity, 1.0)] {
        for f in shell_faces(&t, shell) {
            let face = t.body.get_face(f).unwrap();
            let Surface::Sphere { radius: r, .. } = *t.body.get_surface(face.surface).unwrap()
            else {
                panic!("a ball's wall is a sphere")
            };
            assert!((r - radius).abs() < 1e-12, "{r} vs {radius}");
        }
    }
    let (v, a) = props(&t);
    assert_near("volume", v, 4.0 / 3.0 * PI * (8.0 - 1.0));
    assert_near("area", a, 4.0 * PI * (4.0 + 1.0));
}

/// **A hole loop and two runs together**: the hole's toroidal cavity
/// and the gap's cylindrical one, each keyed by its origin.
#[test]
fn a_hole_and_an_axis_gap_compose() {
    let outer = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.0),
        Point2::new(0.5, 2.0),
        Point2::new(0.5, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let hole = ProfileLoop::polygon([
        Point2::new(1.0, 1.0),
        Point2::new(1.5, 1.0),
        Point2::new(1.5, 2.0),
        Point2::new(1.0, 2.0),
    ]);
    let vp = validated(vec![outer, hole]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 3);
    let ring = t.cavity(CavityOrigin::Hole { loop_index: 1 }).unwrap();
    let pocket = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    assert_ne!(ring, pocket);
    assert_eq!(cylinder_radii(&t, ring), vec![1.0, 1.5]);
    assert_eq!(cylinder_radii(&t, pocket), vec![0.5]);
    assert_eq!(shell_faces(&t, pocket), walls_of(&t, &[4, 5, 6]));
    // V = π·2²·3 − π·0.5²·1 − 2π·1.25·(0.5·1) (Pappus for the hole).
    let (v, _) = props(&t);
    assert_near("volume", v, 12.0 * PI - 0.25 * PI - 1.25 * PI);
}

/// **A pocket with a plane tip and a cone tip**: the triangular notch
/// (0,2) → (0.5,2) → (0,1) revolves to a conical cavity whose top is a
/// disc (its centre no vertex) and whose apex is a pole on the axis.
#[test]
fn a_pocket_ends_in_a_disc_and_an_apex() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.0),
        Point2::new(0.5, 2.0),
        Point2::new(0.0, 1.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let cavity = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    // Canonical vertex 6 is (0, 1), the apex; vertex 4, (0, 2), is the
    // disc's centre, and the plane disc takes it.
    let apex = t.poles[0][6].expect("the cone's apex is a pole");
    assert!(t.poles[0][4].is_none(), "a disc's centre is no vertex");
    let p = t.body.vertex_points().find(|&(v, _)| v == apex).unwrap().1;
    assert!(p.distance(geom_core::Point3::new(0.0, 1.0, 0.0)) < 1e-12);
    assert_eq!(shell_faces(&t, cavity), walls_of(&t, &[4, 5]));
    // V = π·1²·3 − π·0.5²·1/3.
    let (v, a) = props(&t);
    assert_near("volume", v, 3.0 * PI - PI / 12.0);
    // A = 8π + (π·0.5² + π·0.5·√(0.5² + 1²)).
    assert_near("area", a, 8.0 * PI + 0.25 * PI + 0.5 * PI * 1.25_f64.sqrt());
}

/// **A round notch**: an arc from axis to axis revolves to a spherical
/// cavity — the pocket's one wall a sphere with both its poles.
#[test]
fn a_round_notch_revolves_to_a_ball_cavity() {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 3.0), 0.0),
        (Point2::new(0.0, 3.0), 0.0),
        (Point2::new(0.0, 2.0), -1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let cavity = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    assert_eq!(shell_faces(&t, cavity), walls_of(&t, &[4]));
    assert!(t.poles[0][4].is_some() && t.poles[0][5].is_some());
    let (v, a) = props(&t);
    assert_near("volume", v, 3.0 * PI - 4.0 / 3.0 * PI * 0.125);
    assert_near("area", a, 8.0 * PI + PI);
}

/// The same C profile revolves PARTIALLY as before: on-axis runs are
/// ordinary cap boundary there, so a part turn has no cavity.
#[test]
fn a_two_run_profile_part_turn_has_no_cavity() {
    let vp = validated(vec![notched_column(3.0, &[(0.5, 1.0, 2.0)])]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 1);
    assert!(t.cavities.is_empty());
}
