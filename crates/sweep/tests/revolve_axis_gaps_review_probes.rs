//! Reviewer probes for PR 5707 (full revolve, one cavity per axis gap).
//! Each row checks the built body's volume against a closed form (the
//! winding oracle) and tier validity; exploratory scans print outcomes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane, test_support::bulge_loop};
use revolve_common::*;
use sweep::{CavityOrigin, Revolution, RevolveAxis, Revolved, RevolvedKind, revolve};

const PI: f64 = core::f64::consts::PI;

fn props(t: &Revolved<f64>) -> (f64, f64) {
    let tol = Tol::witness();
    let p = topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(&t.body), tol),
        tol,
    )
    .unwrap();
    (p.volume, p.surface_area)
}

fn near(what: &str, got: f64, want: f64, rel: f64) {
    assert!(
        ((got - want) / want).abs() < rel,
        "{what}: got {got}, want {want}"
    );
}

/// Points of a radius-`w`, height-`h` column with square notches
/// `(depth, lo, hi)` cut from the axis (CCW, starting at the origin).
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

fn notched_volume(w: f64, h: f64, notches: &[(f64, f64, f64)]) -> f64 {
    PI * w * w * h - notches.iter().map(|&(d, lo, hi)| PI * d * d * (hi - lo)).sum::<f64>()
}

fn check(name: &str, t: &Revolved<f64>, cavities: usize, want_v: f64) {
    assert_all_tiers(&t.body);
    assert_eq!(t.cavities.len(), cavities, "{name}: cavity count");
    assert_eq!(t.body.shells().count(), cavities + 1, "{name}: shells");
    let (v, _) = props(t);
    near(&format!("{name} volume"), v, want_v, 1e-10);
    // Every gap's cavity is findable, ranks dense.
    let pockets: Vec<_> = t
        .cavities
        .iter()
        .filter(|c| matches!(c.origin, CavityOrigin::Pocket { .. }))
        .collect();
    for g in 0..pockets.len() {
        assert!(t.cavity(CavityOrigin::Pocket { gap: g }).is_some(), "{name}: gap {g}");
    }
}

#[test]
fn probe_k_up_to_five_with_varying_depths() {
    let rows: Vec<Vec<(f64, f64, f64)>> = vec![
        vec![(0.5, 1.0, 2.0), (0.25, 3.0, 4.0), (0.75, 5.0, 6.0)],
        vec![
            (0.9, 1.0, 1.5),
            (0.1, 2.0, 2.5),
            (0.5, 3.0, 3.5),
            (0.99, 4.0, 4.5),
        ],
    ];
    for notches in rows {
        let h = 7.0;
        let vp = validated(vec![ProfileLoop::polygon(notched_pts(1.0, h, &notches))]);
        let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
        check("k", &t, notches.len(), notched_volume(1.0, h, &notches));
    }
}

/// Rotated start vertex and clockwise authoring must give the same body.
#[test]
fn probe_start_vertex_rotation_and_clockwise_authoring() {
    let notches = [(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)];
    let base = notched_pts(1.0, 5.0, &notches);
    let want = notched_volume(1.0, 5.0, &notches);
    for rot in 0..base.len() {
        for cw in [false, true] {
            let mut pts = base.clone();
            pts.rotate_left(rot);
            if cw {
                pts.reverse();
            }
            let vp = validated(vec![ProfileLoop::polygon(pts)]);
            let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
                .unwrap_or_else(|e| panic!("rot {rot} cw {cw}: {e}"));
            check(&format!("rot {rot} cw {cw}"), &t, 2, want);
            let low = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
            // The lower notch (radius 0.5) is gap 0 regardless.
            let has_half = t.body.get_shell(low).unwrap().faces.iter().any(|&f| {
                let face = t.body.get_face(f).unwrap();
                matches!(*t.body.get_surface(face.surface).unwrap(),
                    geom::Surface::Cylinder { radius, .. } if (radius - 0.5).abs() < 1e-12)
            });
            assert!(has_half, "rot {rot} cw {cw}: gap 0 is the r=0.5 notch");
            // bands[0] in swept order of first segment: strictly
            // monotone positions after the sort.
            let firsts: Vec<usize> = t.bands[0].iter().map(|b| b.segments[0]).collect();
            eprintln!("rot {rot} cw {cw}: band firsts {firsts:?}");
        }
    }
}

/// The profile on the axis's other side, with the axis direction
/// reversed (and an x-directed axis), still builds the right body.
#[test]
fn probe_reflected_frames() {
    let notches = [(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)];
    let want = notched_volume(1.0, 5.0, &notches);
    let base = notched_pts(1.0, 5.0, &notches);
    // Mirror x -> -x, axis dir (0,-1) so r = -x.
    let mirrored: Vec<_> = base.iter().map(|p| Point2::new(-p.x, p.y)).collect();
    let axis_down = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, -1.0),
    };
    let vp = validated(vec![ProfileLoop::polygon(mirrored)]);
    let t = revolve(&vp, axis_down, Revolution::Full, Tol::witness()).unwrap();
    check("mirrored", &t, 2, want);
    // Gap 0 is lowest along d̂ = -y: the r=0.25 notch at y in [3,4].
    let g0 = t.cavity(CavityOrigin::Pocket { gap: 0 }).unwrap();
    let r_of = |shell| -> Vec<f64> {
        t.body
            .get_shell(shell)
            .unwrap()
            .faces
            .iter()
            .filter_map(|&f| {
                let face = t.body.get_face(f).unwrap();
                match *t.body.get_surface(face.surface).unwrap() {
                    geom::Surface::Cylinder { radius, .. } => Some(radius),
                    _ => None,
                }
            })
            .collect()
    };
    eprintln!("mirrored gap0 radii {:?}", r_of(g0));
    // Same profile, axis dir (0,-1) without mirroring: r = -x < 0 refused.
    let vp = validated(vec![ProfileLoop::polygon(base.clone())]);
    let e = revolve(&vp, axis_down, Revolution::Full, Tol::witness());
    eprintln!("unmirrored with down axis: {:?}", e.as_ref().err());
    // x-directed axis: rotate (x,y) -> (y,-x)?  r = (p-o).perp_dot(d)
    // with d = (1,0): r = -y. Profile point (x,y) -> (y, -x).
    let rotated: Vec<_> = base.iter().map(|p| Point2::new(p.y, -p.x)).collect();
    let axis_x = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(1.0, 0.0),
    };
    let vp = validated(vec![ProfileLoop::polygon(rotated)]);
    let t = revolve(&vp, axis_x, Revolution::Full, Tol::witness()).unwrap();
    check("x axis", &t, 2, want);
    // Off-origin axis.
    let shifted: Vec<_> = base.iter().map(|p| Point2::new(p.x + 3.0, p.y - 7.0)).collect();
    let axis_off = RevolveAxis {
        origin: Point2::new(3.0, 11.0),
        dir: Vec2::new(0.0, 2.0),
    };
    let vp = validated(vec![ProfileLoop::polygon(shifted)]);
    let t = revolve(&vp, axis_off, Revolution::Full, Tol::witness()).unwrap();
    check("shifted", &t, 2, want);
}

/// Collinear runs of segments inside a pocket stretch, and an axis run
/// of several collinear on-axis segments.
#[test]
fn probe_collinear_runs_in_pocket_and_in_axis_run() {
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.5), // upper run in two segments
        Point2::new(0.0, 2.0),
        Point2::new(0.25, 2.0), // notch top in two collinear segments
        Point2::new(0.5, 2.0),
        Point2::new(0.5, 1.7), // notch wall in three
        Point2::new(0.5, 1.2),
        Point2::new(0.5, 1.0),
        Point2::new(0.0, 1.0),
        Point2::new(0.0, 0.5), // lower run in two
    ];
    let vp = validated(vec![ProfileLoop::polygon(pts)]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    check("collinear", &t, 1, 3.0 * PI - 0.25 * PI);
    let walls = t.walls();
    // The three notch-wall segments 8, 9, 10 read one face; the top
    // 6, 7 one face.
    assert_eq!(walls[0][7], walls[0][8]);
    assert_eq!(walls[0][8], walls[0][9]);
    assert_eq!(walls[0][5], walls[0][6]);
    for b in &t.bands[0] {
        eprintln!("band segs {:?}", b.segments);
    }
}

/// Pockets of two segments meeting the axis at cone apexes at both run
/// ends, arcs in a pocket, and a hole with two gaps.
#[test]
fn probe_cone_tips_arcs_and_holes() {
    // Diamond notch: (0,2) -> (0.5,1.5) -> (0,1): two cones.
    let pts = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 3.0),
        Point2::new(0.0, 3.0),
        Point2::new(0.0, 2.0),
        Point2::new(0.5, 1.5),
        Point2::new(0.0, 1.0),
    ];
    let vp = validated(vec![ProfileLoop::polygon(pts)]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    check("diamond", &t, 1, 3.0 * PI - PI / 12.0);
    assert!(t.poles[0][4].is_some() && t.poles[0][6].is_some());

    // Two round notches (balls r=0.25 at y=1.25 and y=2.25) beside a
    // square one, plus a hole.
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
    let vp = validated(vec![lp, hole]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let ball = 4.0 / 3.0 * PI * 0.25f64.powi(3);
    let want = 16.0 * PI - 0.25 * PI * 0.5 - 2.0 * ball - 2.0 * PI * 1.25 * 0.5;
    check("arcs+hole", &t, 4, want);
    assert!(t.cavity(CavityOrigin::Hole { loop_index: 1 }).is_some());
    for g in 0..3 {
        assert!(t.cavity(CavityOrigin::Pocket { gap: g }).is_some());
    }

    // An arc mixed with lines in one pocket: quarter disc notch
    // (0,2) -> (0.5,2) line, (0.5,2) -> (0,1.5) quarter arc about (0,2)
    // bulging down-right, then the axis.  Quarter arc CW about (0,2):
    // from angle 0 to -90°, bulge = tan(-90°/4).
    let b = (-PI / 8.0).tan();
    let lp = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 3.0), 0.0),
        (Point2::new(0.0, 3.0), 0.0),
        (Point2::new(0.0, 2.0), 0.0),
        (Point2::new(0.5, 2.0), b),
        (Point2::new(0.0, 1.5), 0.0),
    ]);
    match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Ok(vp) => {
            let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
            // Half ball of radius 0.5.
            check("quarter", &t, 1, 3.0 * PI - 2.0 / 3.0 * PI * 0.125);
        }
        Err(e) => eprintln!("quarter notch did not validate: {e}"),
    }
}

fn try_build(pts: Vec<Point2<f64>>) -> String {
    match Profile::new(SketchPlane::xy(), vec![ProfileLoop::polygon(pts)]).validate(Tol::witness())
    {
        Err(e) => format!("validate refused: {e}"),
        Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
            Err(e) => format!("revolve refused: {e:?}"),
            Ok(t) => {
                let tiers = (
                    topo::validate(&t.body).is_ok(),
                    topo::validate_closed(&t.body).is_ok(),
                    topo::validate_geometric(&t.body, Tol::witness()).is_ok(),
                );
                format!("built: cavities {} tiers {tiers:?} vol {:.15}", t.cavities.len(), props(&t).0)
            }
        },
    }
}

/// Thin gaps, thin walls and short runs near the band: what validates
/// must build tier-valid, and nothing may reach `AxisGapEscalated` at
/// the profile's own tolerance.
#[test]
fn probe_thin_scan() {
    let eps = eps();
    eprintln!("eps {eps}");
    for c in [0.25, 0.5, 0.9, 1.0, 1.1, 1.5, 2.0, 3.0, 5.0, 10.0, 30.0, 100.0] {
        let d = c * eps;
        // thin notch height d (gap between runs is d)
        let a = try_build(notched_pts(1.0, 3.0, &[(0.5, 1.0, 1.0 + d)]));
        // thin wall between notch and outer wall
        let b = try_build(notched_pts(1.0, 3.0, &[(1.0 - d, 1.0, 2.0)]));
        // short axis run between two notches
        let c2 = try_build(notched_pts(1.0, 3.0, &[(0.5, 0.5, 1.0), (0.5, 1.0 + d, 2.0)]));
        // shallow notch depth d
        let e = try_build(notched_pts(1.0, 3.0, &[(d, 1.0, 2.0)]));
        // short bottom run (filled pole near the pocket pole)
        let f = try_build(notched_pts(1.0, 3.0, &[(0.5, d, 2.0)]));
        eprintln!("c={c}\n  gap: {a}\n  wall: {b}\n  run: {c2}\n  depth: {e}\n  bottom-run: {f}");
    }
}

/// A notch whose round wall comes within the band of the outer wall
/// (arc/line) and concentric thin-shell balls (arc/arc).
#[test]
fn probe_round_pocket_near_outer_wall() {
    let eps = eps();
    for c in [0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 100.0, 1e3, 1e4] {
        let d = c * eps;
        // round notch radius 0.5 about (0, 1.5), outer wall at r = 0.5 + d
        let w = 0.5 + d;
        let lp = bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(w, 0.0), 0.0),
            (Point2::new(w, 3.0), 0.0),
            (Point2::new(0.0, 3.0), 0.0),
            (Point2::new(0.0, 2.0), -1.0),
            (Point2::new(0.0, 1.0), 0.0),
        ]);
        let arc_line = match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
            Err(e) => format!("validate refused: {e}"),
            Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
                Err(e) => format!("revolve refused: {e:?}"),
                Ok(t) => format!(
                    "built tiers {:?} vol {:.15} want {:.15}",
                    (
                        topo::validate_closed(&t.body).is_ok(),
                        topo::validate_geometric(&t.body, Tol::witness()).is_ok()
                    ),
                    props(&t).0,
                    PI * w * w * 3.0 - 4.0 / 3.0 * PI * 0.125
                ),
            },
        };
        // hollow ball radii 1 and 1 + d
        let r2 = 1.0 + d;
        let lp = bulge_loop(vec![
            (Point2::new(0.0, -r2), 1.0),
            (Point2::new(0.0, r2), 0.0),
            (Point2::new(0.0, 1.0), -1.0),
            (Point2::new(0.0, -1.0), 0.0),
        ]);
        let arc_arc = match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
            Err(e) => format!("validate refused: {e}"),
            Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
                Err(e) => format!("revolve refused: {e:?}"),
                Ok(t) => format!(
                    "built tiers {:?} vol {:.15} want {:.15}",
                    (
                        topo::validate_closed(&t.body).is_ok(),
                        topo::validate_geometric(&t.body, Tol::witness()).is_ok()
                    ),
                    props(&t).0,
                    4.0 / 3.0 * PI * (r2.powi(3) - 1.0)
                ),
            },
        };
        eprintln!("c={c}\n  arc/line: {arc_line}\n  arc/arc: {arc_arc}");
    }
}

/// Every name-bearing map of loop 0 has the canonical loop's length.
#[test]
fn probe_maps_sized_and_pocket_handles_disjoint() {
    let notches = [(0.5, 1.0, 2.0), (0.25, 3.0, 4.0)];
    let pts = notched_pts(1.0, 5.0, &notches);
    let n = pts.len();
    let vp = validated(vec![ProfileLoop::polygon(pts)]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
        ..
    } = &t.kind
    else {
        panic!()
    };
    assert_eq!(meridians[0].len(), n);
    assert_eq!(pi_walls.len(), n);
    assert_eq!(pi_meridians.len(), n);
    assert_eq!(pi_rims.len(), n);
    assert_eq!(t.rims[0].len(), n);
    assert_eq!(t.poles[0].len(), n);
    // Every face of the body is some band's face or a π wall.
    let mut named: std::collections::BTreeSet<_> = t.bands[0].iter().map(|b| b.face).collect();
    named.extend(pi_walls.iter().flatten().copied());
    let all: std::collections::BTreeSet<_> = t.body.faces().map(|(f, _)| f).collect();
    assert_eq!(named, all, "every face is named by loop 0's maps");
    // Every vertex is a pole or a rim's vertex: count poles = 2 per shell
    // where a cone/curve meets the axis; here plane discs everywhere -> 0.
    eprintln!("poles {:?}", t.poles[0]);
}

/// Claim 4: an axial gap in band with every validator decision definite,
/// using pinned run ends on opposite sides of the axis (|r| < ε) so the
/// Euclidean run-end separation clears the band while Δz does not.
#[test]
fn probe_axis_gap_escalation_at_own_tolerance() {
    let eps = eps();
    for (ra, rb, dz) in [
        (0.99, -0.99, 9.95),
        (0.9, -0.9, 9.9),
        (0.5, -0.5, 9.99),
        (0.0, 0.0, 9.99),
        (0.0, 0.0, 10.01),
    ] {
        let a = Point2::new(ra * eps, 1.0);
        let b = Point2::new(rb * eps, 1.0 + dz * eps);
        // arc from b to a bulging to +r (a near-full circle so it is no
        // sliver): bulge -10 ~ CW sweep of ~ -337 degrees.
        for bulge in [-1.0, 0.0] {
            let mut chain = vec![
                (Point2::new(-ra * eps, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 3.0), 0.0),
                (Point2::new(-rb * eps, 3.0), 0.0),
                (b, bulge),
            ];
            if bulge == 0.0 {
                chain.push((Point2::new(0.5, 1.0 + 0.5 * dz * eps), 0.0));
            }
            chain.push((a, 0.0));
            let lp = bulge_loop(chain);
            let out = match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
                Err(e) => format!("validate refused: {e}"),
                Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
                    Err(e) => format!("REVOLVE REFUSED: {e:?}"),
                    Ok(t) => format!("built, cavities {}", t.cavities.len()),
                },
            };
            eprintln!("ra {ra} rb {rb} dz {dz} bulge {bulge}: {out}");
        }
    }
}

/// The pocket re-collapses its stretch in FORWARD order while the outer
/// loop collapsed it swept (reversed); the line/line cosurface margin
/// reads the next chord's far end against the previous chord's line, so
/// it is direction-dependent. Two kinks are built so that the two
/// directions decide differently.
#[test]
fn probe_pocket_collapse_direction_asymmetry() {
    let eps = eps();
    let wall = |a_end: f64, s: f64| {
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 3.0),
            Point2::new(0.0, 3.0),
            Point2::new(0.0, 2.2),
            Point2::new(0.5, 2.2),
            Point2::new(0.5, a_end),
            Point2::new(0.5 + s, 1.0),
            Point2::new(0.0, 1.0),
        ]
    };
    for (what, a_end, s) in [
        ("X: outer corner, pocket in band", 1.2, 2.5 * eps),
        ("Y: outer merges, pocket corner", 2.15, 15.0 * eps),
        ("Y2", 2.19, 60.0 * eps),
        ("Y3", 2.195, 150.0 * eps),
        ("Y4", 2.199, 500.0 * eps),
        ("control, same kink single run (no pocket): X", 1.2, 2.5 * eps),
    ] {
        let mut pts = wall(a_end, s);
        if what.starts_with("control") {
            // Close the notch's bottom away from the axis: one run only.
            pts.truncate(8);
            pts.push(Point2::new(0.2, 1.0));
            pts.push(Point2::new(0.2, 0.5));
            pts.push(Point2::new(0.0, 0.5));
            // reorder: drop (0,1)
        }
        let out = match Profile::new(SketchPlane::xy(), vec![ProfileLoop::polygon(pts)])
            .validate(Tol::witness())
        {
            Err(e) => format!("validate refused: {e}"),
            Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
                Err(e) => format!("REVOLVE REFUSED: {e}"),
                Ok(t) => {
                    let w = t.walls();
                    format!(
                        "built, cavities {}, walls of segs 6,7 equal: {}, bands {:?}, tiers {:?}",
                        t.cavities.len(),
                        w[0][6] == w[0][7],
                        t.bands[0].iter().map(|b| b.segments.clone()).collect::<Vec<_>>(),
                        (
                            topo::validate_closed(&t.body).is_ok(),
                            topo::validate_geometric(&t.body, Tol::witness()).is_ok()
                        )
                    )
                }
            },
        };
        eprintln!("{what}: {out}");
    }
}

/// Control for Y2's Euler refusal: the same kink (short then long, in
/// SWEPT order) on a single-run outer shell, no pocket.
#[test]
fn probe_kink_control_on_outer_shell() {
    let eps = eps();
    for c in [15.0, 60.0, 150.0] {
        let pts = vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0 + c * eps, 0.0),
            Point2::new(1.0, 1.19),
            Point2::new(1.0, 1.2),
            Point2::new(0.0, 1.2),
        ];
        let out = match Profile::new(SketchPlane::xy(), vec![ProfileLoop::polygon(pts)])
            .validate(Tol::witness())
        {
            Err(e) => format!("validate refused: {e}"),
            Ok(vp) => match revolve(&vp, axis_y(), Revolution::Full, Tol::witness()) {
                Err(e) => format!("REVOLVE REFUSED: {e}"),
                Ok(t) => format!(
                    "built, bands {:?}",
                    t.bands[0].iter().map(|b| b.segments.clone()).collect::<Vec<_>>()
                ),
            },
        };
        eprintln!("CONTROL c={c}: {out}");
    }
}
