//! **`point_in_solid` inside a tilted-cut cylinder cavity** — a brick
//! with a rod subtracted (a cavity whose wall has reversed sense), split
//! by a plane at tilt 1.0, built through the public doors.
//!
//! The plane crosses the cavity, so each half's section is two planar
//! faces, one either side of the bore, each bounded by lines and an
//! ellipse arc. A section face is charted with its outward normal, so
//! its sense bit is `true`; one of the lower half's two section faces
//! used to carry `false`, inherited from the reversed cavity wall the
//! split's null face was carved from, and the ray lane read every
//! crossing of it with the wrong sign.
//!
//! Every probe is a grid point whose analytic distance from each
//! boundary surface (the six brick planes, the rod's cylinder and the
//! cut plane) is at least `1e3·ε`; the region it is judged by is
//! `(brick − rod) ∩ {the kept side of the cut}`, in closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::pis_arc_capped_poses::poses;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, prism_at};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// The cut's normal: tilted 1.0 rad about `y`, through `(0, 0, 1.25)`.
fn cut_normal() -> Vec3<f64> {
    Vec3::new(1.0f64.sin(), 0.0, 1.0f64.cos())
}

fn cut_origin() -> Point3<f64> {
    Point3::new(0.0, 0.0, 1.25)
}

/// Which body a row reads.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Part {
    Below,
    Above,
    Uncut,
}

/// The brick `[−2, 2]² × [0, 2.5]` less the unit rod about `z`, through
/// the subtract door; the rod runs past both caps.
fn cavity() -> Body<f64> {
    let block: Body<f64> = brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.5), tol());
    let rod: Body<f64> = prism_at(
        vec![(Point2::new(-1.0, 0.0), 1.0), (Point2::new(1.0, 0.0), 1.0)],
        -0.5,
        3.5,
        tol(),
    );
    match topo::subtract(&block, &rod, tol()) {
        Ok(topo::BooleanResult::Body(b)) => b.body,
        other => panic!("the rod subtracts: {:?}", other.err()),
    }
}

fn fixture(part: Part) -> Body<f64> {
    let whole = cavity();
    if part == Part::Uncut {
        return whole;
    }
    let result = split(
        &whole,
        &SplitPlane {
            origin: cut_origin(),
            normal: cut_normal(),
        },
        tol(),
    )
    .expect("the cut splits");
    let kept = if part == Part::Below {
        result.below
    } else {
        result.above
    };
    let SplitPart::Body(half) = kept else {
        panic!("material on both sides of the cut");
    };
    half
}

/// The closed form, or `None` within `clear` of a boundary surface.
fn truth(part: Part, p: Point3<f64>, clear: f64) -> Option<bool> {
    let elevation = (p - cut_origin()).dot(cut_normal());
    let walls = [
        p.x + 2.0,
        2.0 - p.x,
        p.y + 2.0,
        2.0 - p.y,
        p.z,
        2.5 - p.z,
        p.x.hypot(p.y) - 1.0,
    ];
    let side = match part {
        Part::Below => -elevation,
        Part::Above => elevation,
        Part::Uncut => 1.0,
    };
    if walls.iter().chain([&side]).any(|d| d.abs() < clear) {
        return None;
    }
    Some(walls.iter().all(|d| *d > 0.0) && side > 0.0)
}

/// The 9³ and 11³ grids over the brick, each clear of every boundary.
fn probes(part: Part) -> Vec<(Point3<f64>, bool)> {
    let clear = 1e3 * tol().eps();
    let mut out = Vec::new();
    for n in [9usize, 11] {
        // The per-axis shift keeps the grid off the fixture's planes of
        // symmetry.
        let at = |c: usize, lo: f64, hi: f64, axis: usize| {
            lo + (hi - lo) * (c as f64 + 0.5 + 0.13 * axis as f64) / n as f64
        };
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let p =
                        Point3::new(at(i, -2.0, 2.0, 0), at(j, -2.0, 2.0, 1), at(k, 0.0, 2.5, 2));
                    if let Some(t) = truth(part, p, clear) {
                        out.push((p, t));
                    }
                }
            }
        }
    }
    out
}

/// **The cavity reads its truth at every pose, and both controls answer
/// as they did.** Every answer is the truth. The one refusal admitted is
/// the at-infinity side on a point OUTSIDE the body: a cut body's
/// tilted-section walls carry no closed-form volume, so a probe whose
/// first ray meets nothing is `VolumeUncertified`, which the props lane
/// owns. A point inside is always answered.
///
/// Measured over the six poses, identically at ε = 1e-9, 1e-6 and
/// 1e-12: below 5015 answered (3741 on the base, which also answered
/// 1274 wrong, all false `Out`) and 7345 at infinity (the same on the
/// base); above 11428 answered and 932 at infinity; uncut 12360
/// answered. The floor is those counts, so a change that turns answers
/// into refusals reds here.
#[test]
fn the_cut_cavity_reads_its_truth_at_every_pose() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut problems = Vec::new();
    for (part, floor) in [
        (Part::Below, 5015),
        (Part::Above, 11428),
        (Part::Uncut, 12360),
    ] {
        let body = fixture(part);
        let probes = probes(part);
        assert!(
            probes.iter().filter(|(_, t)| *t).count() >= 100,
            "{part:?}: the grid must reach the material"
        );
        let mut answered = 0;
        for (pose, map) in poses() {
            let posed = transform_rigid(&body, &map, tol()).unwrap();
            for &(p, inside) in &probes {
                let want = if inside {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                    Ok(got) if got == want => answered += 1,
                    Ok(got) => problems.push(format!("{part:?} | {pose} | {p:?}: {got:?}")),
                    Err(PointInSolidError::VolumeUncertified) if !inside => {}
                    Err(e) => problems.push(format!("{part:?} | {pose} | {p:?}: {e:?}")),
                }
            }
        }
        eprintln!("MEASURE {part:?}: answered {answered}");
        if answered < floor {
            problems.push(format!("{part:?}: answered {answered}, floor {floor}"));
        }
    }
    assert!(
        problems.is_empty(),
        "{} problems:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

/// **The traced probe.** `(−16/9, 0.9467, 1.8778)` lies in the lower
/// half, beside the bore. The first schedule ray, `+x`, leaves through
/// the section face on the `+y` side of the bore; with that face's sense
/// read `false`, the crossing read as an entry and the point as `Out`.
/// The row's own example column, `x = −1.422, y = 1.113`, rides along.
/// Both halves validate: the lower half used to fail tier 3's winding
/// check on that section face (`LoopRoleInverted`), and every section
/// face is sense `true`.
#[test]
fn the_traced_probe_reads_in_and_every_section_face_faces_out() {
    let band = Band::linear(tol()).expect("the witness band");
    let below = fixture(Part::Below);
    let traced = Point3::new(-16.0 / 9.0, 0.9466666666666667, 1.8777777777777778);
    let mut column = vec![traced];
    column.extend([0.128, 0.628, 1.128, 1.628].map(|z| Point3::new(-1.422, 1.113, z)));
    for p in column {
        assert_eq!(truth(Part::Below, p, 1e-3), Some(true), "{p:?} is inside");
        assert!(
            matches!(
                point_in_solid(&below, p, band, tol()),
                Ok(SolidContainment::In)
            ),
            "{p:?}"
        );
    }
    for part in [Part::Below, Part::Above] {
        let half = fixture(part);
        topo::validate_geometric(&half, tol())
            .unwrap_or_else(|e| panic!("{part:?} validates: {e:?}"));
        let sections: Vec<bool> = half
            .faces()
            .filter(|(_, f)| {
                matches!(
                    half.get_surface(f.surface),
                    Some(geom::Surface::Plane { normal, .. })
                        if normal.cross(cut_normal()).norm() < 1e-12
                )
            })
            .map(|(_, f)| f.sense)
            .collect();
        assert_eq!(
            sections,
            vec![true, true],
            "{part:?}: two section faces, both facing out"
        );
    }
}
