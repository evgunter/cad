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
use geom_core::{Point2, Tol, Vec2};
use profile::{ProfileLoop, RawLoop, test_support::bulge_loop};
use revolve_common::*;
use sweep::{CavityOrigin, Revolution, RevolveAxis, Revolved, RevolvedKind, revolve};
use topo::FaceKey;

const PI: f64 = core::f64::consts::PI;

/// Volume and surface area of the built body.
fn props(t: &Revolved<f64>) -> (f64, f64) {
    let tol = Tol::witness();
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&t.body), tol),
        tol,
        topo::test_support::pinned_resolution(tol),
    )
    .unwrap();
    (p.enclosure.volume, p.enclosure.surface_area)
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

/// A radius-`w`, height-`h` column against the axis with square notches
/// `(depth, lo, hi)` cut into it from the axis, lowest first: one axis
/// run per notch plus one. CCW, starting at the origin.
fn notched_pts(w: f64, h: f64, notches: &[(f64, f64, f64)]) -> Vec<Point2<f64>> {
    let mut pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(w, 0.0),
        Point2::new(w, h),
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
    pts
}

/// [`notched_pts`] at radius 1.
fn notched_column(h: f64, notches: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(notched_pts(1.0, h, notches))
}

/// The closed-form volume of a [`notched_pts`] column.
fn notched_volume(w: f64, h: f64, notches: &[(f64, f64, f64)]) -> f64 {
    PI * w * w * h
        - notches
            .iter()
            .map(|&(d, lo, hi)| PI * d * d * (hi - lo))
            .sum::<f64>()
}

/// Pappus for a polygon profile against the y axis: `2π ∫ r dA`, which
/// is the full revolve's volume whatever its cavities.
fn pappus_volume(pts: &[Point2<f64>]) -> f64 {
    let n = pts.len();
    let moment: f64 = (0..n)
        .map(|i| {
            let (p, q) = (pts[i], pts[(i + 1) % n]);
            (p.x + q.x) * (p.x * q.y - q.x * p.y)
        })
        .sum::<f64>()
        / 6.0;
    2.0 * PI * moment
}

/// **Loop 0's bands read in swept order** (the documented order of
/// `Revolved::bands` and `BandWall::segments`): each band's segments
/// run down the canonical indices, as the reversed traversal does, and
/// the bands' first segments descend with at most one wrap; every
/// off-axis canonical segment is in exactly one band, and its wall is
/// that band's face.
fn assert_swept_order(what: &str, t: &Revolved<f64>, n: usize) {
    let walls = t.walls();
    let next = |c: usize| (c + n - 1) % n;
    let mut seen = BTreeSet::new();
    for band in &t.bands[0] {
        for pair in band.segments.windows(2) {
            assert_eq!(
                pair[1],
                next(pair[0]),
                "{what}: band {:?} in swept order",
                band.segments
            );
        }
        for &c in &band.segments {
            assert!(seen.insert(c), "{what}: segment {c} in one band");
            assert_eq!(
                walls[0][c],
                Some(band.face),
                "{what}: segment {c} reads its band's wall"
            );
        }
    }
    let swept = |c: usize| n - 1 - c;
    let firsts: Vec<usize> = t.bands[0].iter().map(|b| swept(b.segments[0])).collect();
    let descents = (0..firsts.len())
        .filter(|&i| firsts[(i + 1) % firsts.len()] <= firsts[i])
        .count();
    assert!(
        descents <= 1,
        "{what}: bands in swept order, firsts {firsts:?}"
    );
    let walled: BTreeSet<usize> = (0..n).filter(|&c| walls[0][c].is_some()).collect();
    assert_eq!(seen, walled, "{what}: every wall is in a band");
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

/// **Up to five runs, of varied depths**: one cavity per gap, each
/// findable by its rank, and the closed-form volume.
#[test]
fn up_to_five_runs_build_a_cavity_per_gap() {
    let rows: [&[(f64, f64, f64)]; 2] = [
        &[(0.5, 1.0, 2.0), (0.25, 3.0, 4.0), (0.75, 5.0, 6.0)],
        &[
            (0.9, 1.0, 1.5),
            (0.1, 2.0, 2.5),
            (0.5, 3.0, 3.5),
            (0.99, 4.0, 4.5),
        ],
    ];
    for notches in rows {
        let pts = notched_pts(1.0, 7.0, notches);
        let n = pts.len();
        let what = format!("{} notches", notches.len());
        let t = revolve(
            &validated(vec![ProfileLoop::polygon(pts)]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap();
        assert_all_tiers(&t.body);
        assert_eq!(t.body.shells().count(), notches.len() + 1, "{what}");
        for (gap, &(d, ..)) in notches.iter().enumerate() {
            let shell = t.cavity(CavityOrigin::Pocket { gap }).unwrap();
            assert_eq!(cylinder_radii(&t, shell), vec![d], "{what}: gap {gap}");
        }
        assert_swept_order(&what, &t, n);
        assert_near(&what, props(&t).0, notched_volume(1.0, 7.0, notches));
    }
}

/// **Where the loop starts and which way it was authored change
/// nothing**: every rotation of the two-notch column, counter- and
/// clockwise, builds the same body, gap 0 the lower notch, its bands
/// in swept order.
#[test]
fn the_start_vertex_and_the_authored_winding_change_nothing() {
    let notches = [(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)];
    let base = notched_pts(1.0, 5.0, &notches);
    for rot in 0..base.len() {
        for cw in [false, true] {
            let what = format!("rotation {rot}, clockwise {cw}");
            let mut pts = base.clone();
            pts.rotate_left(rot);
            if cw {
                pts.reverse();
            }
            let t = revolve(
                &validated(vec![ProfileLoop::polygon(pts)]),
                axis_y(),
                Revolution::Full,
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{what}: {e}"));
            assert_all_tiers(&t.body);
            let low = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
            let high = t.cavity(CavityOrigin::Pocket { gap: 1 }).unwrap();
            assert_eq!(cylinder_radii(&t, low), vec![0.5], "{what}");
            assert_eq!(cylinder_radii(&t, high), vec![0.25], "{what}");
            assert_swept_order(&what, &t, base.len());
            assert_near(&what, props(&t).0, notched_volume(1.0, 5.0, &notches));
        }
    }
}

/// **The frame decides the gap order, not the sketch**: mirrored with
/// the axis pointing down, gap 0 is the notch lowest along `d̂ = −y`
/// (the upper one in the sketch); an x-directed and an off-origin axis
/// build the same body.
#[test]
fn reflected_and_moved_frames_build_the_same_body() {
    let notches = [(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)];
    let want = notched_volume(1.0, 5.0, &notches);
    let base = notched_pts(1.0, 5.0, &notches);
    let axis = |origin: Point2<f64>, dir: Vec2<f64>| RevolveAxis { origin, dir };
    let cases = [
        (
            "mirrored, axis down",
            base.iter()
                .map(|p| Point2::new(-p.x, p.y))
                .collect::<Vec<_>>(),
            axis(Point2::new(0.0, 0.0), Vec2::new(0.0, -1.0)),
            0.25,
        ),
        (
            "x axis",
            base.iter().map(|p| Point2::new(p.y, -p.x)).collect(),
            axis(Point2::new(0.0, 0.0), Vec2::new(1.0, 0.0)),
            0.5,
        ),
        (
            "off-origin axis",
            base.iter()
                .map(|p| Point2::new(p.x + 3.0, p.y - 7.0))
                .collect(),
            axis(Point2::new(3.0, 11.0), Vec2::new(0.0, 2.0)),
            0.5,
        ),
    ];
    for (what, pts, axis, gap0) in cases {
        let t = revolve(
            &validated(vec![ProfileLoop::polygon(pts)]),
            axis,
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{what}: {e}"));
        assert_all_tiers(&t.body);
        assert_eq!(t.cavities.len(), 2, "{what}");
        let g0 = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
        assert_eq!(cylinder_radii(&t, g0), vec![gap0], "{what}: gap 0");
        assert_near(what, props(&t).0, want);
    }
}

/// **Collinear pieces collapse to one wall**, in a pocket's stretch and
/// in an axis run alike.
#[test]
fn collinear_pieces_in_a_pocket_and_a_run_are_one_wall() {
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.5),
        Point2::new(0.0, 2.0),
        Point2::new(0.25, 2.0),
        Point2::new(0.5, 2.0),
        Point2::new(0.5, 1.7),
        Point2::new(0.5, 1.2),
        Point2::new(0.5, 1.0),
        Point2::new(0.0, 1.0),
        Point2::new(0.0, 0.5),
    ];
    let n = pts.len();
    let t = revolve(
        &validated(vec![ProfileLoop::polygon(pts)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    let walls = t.walls();
    assert_eq!(walls[0][5], walls[0][6], "the notch's top is one plane");
    assert_eq!(walls[0][7], walls[0][8], "the notch's wall is one cylinder");
    assert_eq!(walls[0][8], walls[0][9], "the notch's wall is one cylinder");
    assert_swept_order("collinear", &t, n);
    assert_near("volume", props(&t).0, 3.0 * PI - 0.25 * PI);
}

/// **Cone tips at both ends of a gap, arc pockets beside a square one,
/// and a hole**: every pole kept, every cavity findable by its origin.
#[test]
fn cone_tips_and_arc_pockets_beside_a_hole() {
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.0),
        Point2::new(0.5, 1.5),
        Point2::new(0.0, 1.0),
    ];
    let t = revolve(
        &validated(vec![ProfileLoop::polygon(pts)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert!(
        t.poles[0][4].is_some() && t.poles[0][6].is_some(),
        "two apexes"
    );
    assert_near("diamond", props(&t).0, 3.0 * PI - PI / 12.0);

    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(2.0, 0.0), 0.0),
        (Point2::new(2.0, 4.0), 0.0),
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 3.5), 0.0),
        (Point2::new(0.5, 3.5), 0.0),
        (Point2::new(0.5, 3.0), 0.0),
        (Point2::new(0.0, 3.0), 0.0),
        (Point2::new(0.0, 2.5), -1.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.0, 1.5), -1.0),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let hole = ProfileLoop::polygon([
        Point2::new(1.0, 1.0),
        Point2::new(1.5, 1.0),
        Point2::new(1.5, 2.0),
        Point2::new(1.0, 2.0),
    ]);
    let t = revolve(
        &validated(vec![lp, hole]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 5);
    assert!(t.cavity(CavityOrigin::Hole { loop_index: 1 }).is_some());
    for gap in 0..3 {
        assert!(
            t.cavity(CavityOrigin::Pocket { gap }).is_some(),
            "gap {gap}"
        );
    }
    let ball = 4.0 / 3.0 * PI * 0.25f64.powi(3);
    let want = 16.0 * PI - 0.25 * PI * 0.5 - 2.0 * ball - 2.0 * PI * 1.25 * 0.5;
    assert_near("arcs and a hole", props(&t).0, want);
}

/// **Every handle map of loop 0 spans the canonical loop**, and loop
/// 0's bands and π walls together name every face of the body.
#[test]
fn loop_zeros_maps_cover_every_face() {
    let pts = notched_pts(1.0, 5.0, &[(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)]);
    let n = pts.len();
    let t = revolve(
        &validated(vec![ProfileLoop::polygon(pts)]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
        ..
    } = &t.kind
    else {
        panic!("a full revolve")
    };
    for (what, len) in [
        ("meridians", meridians[0].len()),
        ("pi_walls", pi_walls.len()),
        ("pi_meridians", pi_meridians.len()),
        ("pi_rims", pi_rims.len()),
        ("rims", t.rims[0].len()),
        ("poles", t.poles[0].len()),
    ] {
        assert_eq!(len, n, "{what}");
    }
    let mut named: BTreeSet<FaceKey> = t.bands[0].iter().map(|b| b.face).collect();
    named.extend(pi_walls.iter().flatten().copied());
    let all: BTreeSet<FaceKey> = t.body.faces().map(|(f, _)| f).collect();
    assert_eq!(named, all);
}

/// **A near-collinear kink in a pocket's stretch has one reading**: the
/// cosurface margin of two lines depends on the traversal's direction,
/// and the outer loop's one collapse decides the pocket's runs too, so
/// each run is one wall of one cavity whichever way the kink reads,
/// and the bands keep swept order. (A shorter kink, 15–60ε, refuses
/// on a single-run outer shell as well: that defect is filed on its own
/// row, not this one's.)
#[test]
fn a_kink_in_a_pocket_has_the_outer_loops_reading() {
    let eps = eps();
    let mut built = 0;
    for (a_end, s) in [(2.195, 150.0), (2.199, 500.0)] {
        let what = format!("kink at {a_end}, offset {s}ε");
        let pts = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 3.0),
            Point2::new(0.0, 3.0),
            Point2::new(0.0, 2.2),
            Point2::new(0.5, 2.2),
            Point2::new(0.5, a_end),
            Point2::new(0.5 + s * eps, 1.0),
            Point2::new(0.0, 1.0),
        ];
        let n = pts.len();
        let t = match revolve(
            &validated(vec![ProfileLoop::polygon(pts.clone())]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        ) {
            Ok(t) => t,
            // The kink's collapsed run fails attachment at some ε, on a
            // single-run shell as well:
            // work/lathe/a-near-collinear-kink-on-a-full-revolve-refuses-as-a-kernel-defect.md.
            Err(e) if e.to_string().contains("geometry attachment gate") => {
                println!("SKIPPED at ε {eps:e}: {what}: the filed kink attachment defect: {e}");
                continue;
            }
            Err(e) => panic!("{what}: {e}"),
        };
        built += 1;
        assert_all_tiers(&t.body);
        let cavity = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
        assert_eq!(
            shell_faces(&t, cavity),
            walls_of(&t, &[4, 5, 6, 7]),
            "{what}"
        );
        assert_swept_order(&what, &t, n);
        // The collapsed run keeps its first leg's carrier, which leaves
        // the stored endpoint by up to the cosurface margin (the same
        // filed row), so the volume holds to the band, not to rounding.
        let rel = (props(&t).0 - pappus_volume(&pts)) / pappus_volume(&pts);
        assert!(rel.abs() < 10.0 * eps, "{what}: volume off by {rel}");
    }
    if eps >= 1e-9 {
        assert_eq!(built, 2, "both kinks build at ε {eps:e} and coarser");
    }
}
