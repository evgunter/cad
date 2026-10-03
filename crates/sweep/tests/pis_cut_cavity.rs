//! **Plane cuts through a bore: `point_in_solid` and each section
//! face's sense.** Each fixture is built through the public doors: a
//! brick with a rod subtracted (a cavity whose wall has reversed sense),
//! or the bored cylinder, split by one plane.
//!
//! A split's section face is charted with the outward normal of its
//! side, and its sense is its loop's winding about that normal, the
//! reading tier 3's check 6 falsifies the bit with. Where the plane
//! crosses the bore's wall along a line, the section is two faces either
//! side of the bore, bounded by lines and an ellipse arc, and each winds
//! counter-clockwise: sense `true`. Where it crosses the bore all round,
//! the section is one face over the whole outline with the bore's
//! section as its ring, its outer loop counter-clockwise: sense `true`.
//!
//! Every probe is a grid point whose analytic distance from each
//! boundary surface is at least `1e3·ε`; the region it is judged by is
//! the fixture's closed form intersected with the kept side of the cut.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::{bored_brick, section_faces, tilted};
use crate::common::poses::poses;
use geom_core::{Band, Point3, Tol};
use sweep::test_support::bored_cylinder;
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{Body, PointInSolidError, SolidContainment, point_in_solid, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// A solid in closed form: its body, the signed distances of `p` from
/// its boundary surfaces (positive on the material side of each; the
/// solid is where all are positive), and its bounding box.
struct Solid {
    body: Body<f64>,
    walls: fn(Point3<f64>) -> Vec<f64>,
    lo: [f64; 3],
    hi: [f64; 3],
}

fn brick_walls(p: Point3<f64>) -> Vec<f64> {
    vec![p.x + 2.0, 2.0 - p.x, p.y + 2.0, 2.0 - p.y, p.z, 2.5 - p.z]
}

fn cavity() -> Solid {
    Solid {
        body: bored_brick(0.0, 0.0, 1.0),
        walls: |p| {
            let mut w = brick_walls(p);
            w.push(p.x.hypot(p.y) - 1.0);
            w
        },
        lo: [-2.0, -2.0, 0.0],
        hi: [2.0, 2.0, 2.5],
    }
}

/// The rod off the brick's centre, at `(0.8, 0)`.
fn off_centre_cavity() -> Solid {
    Solid {
        body: bored_brick(0.8, 0.0, 1.0),
        walls: |p| {
            let mut w = brick_walls(p);
            w.push((p.x - 0.8).hypot(p.y) - 1.0);
            w
        },
        lo: [-2.0, -2.0, 0.0],
        hi: [2.0, 2.0, 2.5],
    }
}

/// `bored_cylinder(0.3, 0.2, 0.37)`: the unit cylinder of height 1,
/// bored at radius 0.3 about `(0.2, 0)`.
fn bored() -> Solid {
    Solid {
        body: bored_cylinder(0.3, 0.2, 0.37, tol()),
        walls: |p| {
            vec![
                1.0 - p.x.hypot(p.y),
                (p.x - 0.2).hypot(p.y) - 0.3,
                p.z,
                1.0 - p.z,
            ]
        },
        lo: [-1.0, -1.0, 0.0],
        hi: [1.0, 1.0, 1.0],
    }
}

/// A cutting plane.
type Cut = SplitPlane<f64>;

/// `p`'s signed distance from the cut, positive above.
fn elevation(cut: &Cut, p: Point3<f64>) -> f64 {
    (p - cut.origin).dot(cut.normal.get())
}

/// Which half of a cut a row reads.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Part {
    Below,
    Above,
}

/// One fixture and one cut through it, with what was measured of it
/// (identically at ε = 1e-9, 1e-6 and 1e-12 unless noted), per body in
/// [`parts`] order.
struct Case {
    name: &'static str,
    solid: fn() -> Solid,
    cut: Option<Cut>,
    /// The fewest probes answered over the six poses, over the three ε
    /// rows: a change that turns answers into refusals reds here.
    floor: [usize; 2],
    /// The most in-band escalations over the six poses.
    escalations: usize,
}

fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "cavity cut at tilt 1.0",
            solid: cavity,
            cut: Some(tilted(1.25, 1.0, false)),
            floor: [5015, 11428],
            escalations: 0,
        },
        Case {
            name: "cavity uncut",
            solid: cavity,
            cut: None,
            floor: [12360, 0],
            escalations: 0,
        },
        Case {
            name: "cavity cut flat",
            solid: cavity,
            cut: Some(tilted(1.25, 0.0, false)),
            floor: [12360, 12360],
            escalations: 0,
        },
        Case {
            name: "cavity cut at tilt 0.3",
            solid: cavity,
            cut: Some(tilted(1.25, 0.3, false)),
            floor: [5518, 9696],
            escalations: 0,
        },
        Case {
            name: "off-centre cavity cut flat",
            solid: off_centre_cavity,
            cut: Some(tilted(1.25, 0.0, false)),
            floor: [12360, 12360],
            escalations: 0,
        },
        Case {
            name: "off-centre cavity cut at tilt 1.4, flipped",
            solid: off_centre_cavity,
            cut: Some(tilted(1.25, 1.4, true)),
            floor: [11263, 5958],
            escalations: 0,
        },
        Case {
            name: "bored cylinder cut flat",
            solid: bored,
            cut: Some(tilted(0.5, 0.0, false)),
            floor: [12360, 12360],
            escalations: 0,
        },
        Case {
            name: "bored cylinder cut at tilt 0.3",
            solid: bored,
            cut: Some(tilted(0.5, 0.3, false)),
            floor: [5200, 8936],
            escalations: 3,
        },
        Case {
            name: "bored cylinder cut at tilt -1, flipped",
            solid: bored,
            cut: Some(tilted(0.5, -1.0, true)),
            floor: [5909, 9560],
            escalations: 0,
        },
    ]
}

/// The bodies a case reads: each half of the cut, or the whole solid.
fn parts(case: &Case, solid: &Solid) -> Vec<(Option<Part>, Body<f64>)> {
    let Some(cut) = case.cut else {
        return vec![(None, solid.body.clone())];
    };
    let result = split(&solid.body, &cut, tol())
        .unwrap_or_else(|e| panic!("{}: the cut splits: {e:?}", case.name));
    [(Part::Below, result.below), (Part::Above, result.above)]
        .into_iter()
        .map(|(part, kept)| {
            let SplitPart::Body(half) = kept else {
                panic!("{}: material on both sides of the cut", case.name);
            };
            (Some(part), half)
        })
        .collect()
}

/// The closed form, or `None` within `1e3·ε` of a boundary surface.
fn truth(solid: &Solid, cut: Option<Cut>, part: Option<Part>, p: Point3<f64>) -> Option<bool> {
    let clear = 1e3 * tol().eps();
    let mut walls = (solid.walls)(p);
    if let (Some(cut), Some(part)) = (cut, part) {
        let e = elevation(&cut, p);
        walls.push(if part == Part::Below { -e } else { e });
    }
    if walls.iter().any(|d| d.abs() < clear) {
        return None;
    }
    Some(walls.iter().all(|d| *d > 0.0))
}

/// The 9³ and 11³ grids over the solid's box, each clear of every
/// boundary.
fn probes(solid: &Solid, cut: Option<Cut>, part: Option<Part>) -> Vec<(Point3<f64>, bool)> {
    let mut out = Vec::new();
    for n in [9usize, 11] {
        // The per-axis shift keeps the grid off the fixtures' planes of
        // symmetry.
        let at = |c: usize, axis: usize| {
            solid.lo[axis]
                + (solid.hi[axis] - solid.lo[axis]) * (c as f64 + 0.5 + 0.13 * axis as f64)
                    / n as f64
        };
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let p = Point3::new(at(i, 0), at(j, 1), at(k, 2));
                    if let Some(t) = truth(solid, cut, part, p) {
                        out.push((p, t));
                    }
                }
            }
        }
    }
    out
}

/// A refusal on a margin strictly inside the band's gap `(ε, K·ε)`: a
/// probe's ray landing within the band of a boundary edge or wall trim,
/// a typed refusal and not an answer.
fn in_the_band(diag: &geom_core::Indeterminate) -> bool {
    let (eps, k) = (tol().eps(), tol().k());
    matches!(diag.margin.diagnostic_f64_for_error_text(), geom_core::ErrorTextReading::Value(v) if v.abs() > eps && v.abs() < k * eps)
}

/// **Every cut through a bore reads its truth at every pose.** Every
/// answer is the truth. The one refusal admitted is the at-infinity side
/// on a point OUTSIDE the body: a body with a tilted-section wall carries
/// no closed-form volume, so a probe whose first ray meets nothing is
/// `VolumeUncertified`, which the props lane owns. A point inside is
/// answered, or refused as an in-band escalation: at ε = 1e-6 three
/// probes per half of the bored cylinder cut at tilt 0.3 escalate on the
/// wall trim.
#[test]
fn every_cut_through_a_bore_reads_its_truth_at_every_pose() {
    let band = Band::linear(tol()).expect("the witness band");
    let mut problems = Vec::new();
    for case in cases() {
        let solid = (case.solid)();
        for (i, (part, body)) in parts(&case, &solid).into_iter().enumerate() {
            let probes = probes(&solid, case.cut, part);
            assert!(
                probes.iter().filter(|(_, t)| *t).count() >= 50,
                "{} {part:?}: the grid must reach the material",
                case.name
            );
            let (mut answered, mut at_infinity, mut escalated) = (0, 0, 0);
            for (pose, map) in poses() {
                let posed = transform_rigid(&body, &map, tol()).unwrap();
                for &(p, inside) in &probes {
                    let want = if inside {
                        SolidContainment::In
                    } else {
                        SolidContainment::Out
                    };
                    let at = format!("{} {part:?} | {pose} | {p:?}", case.name);
                    match point_in_solid(&posed, map.transform_point(p), band, tol()) {
                        Ok(got) if got == want => answered += 1,
                        Ok(got) => problems.push(format!("{at}: {got:?}")),
                        Err(PointInSolidError::VolumeUncertified) if !inside => at_infinity += 1,
                        Err(
                            PointInSolidError::Escalated { diag, .. }
                            | PointInSolidError::Loop(topo::PointInLoopError::Escalated {
                                diag, ..
                            }),
                        ) if in_the_band(&diag) => escalated += 1,
                        Err(e) => problems.push(format!("{at}: {e:?}")),
                    }
                }
            }
            eprintln!(
                "MEASURE {} {part:?}: answered {answered}, at infinity {at_infinity}, \
                 escalated {escalated}",
                case.name
            );
            let floor = case.floor[i];
            if escalated > case.escalations {
                problems.push(format!(
                    "{} {part:?}: escalated {escalated}, cap {}",
                    case.name, case.escalations
                ));
            }
            if answered < floor {
                problems.push(format!(
                    "{} {part:?}: answered {answered}, floor {floor}",
                    case.name
                ));
            }
        }
    }
    assert!(
        problems.is_empty(),
        "{} problems:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

/// **Every half passes tier 3, and every section face winds
/// counter-clockwise.** A section face's sense is the one reading check
/// 6 makes, at every ε row.
#[test]
fn every_section_face_passes_check_6() {
    let mut problems = Vec::new();
    for case in cases() {
        let Some(cut) = case.cut else { continue };
        let solid = (case.solid)();
        for (part, half) in parts(&case, &solid) {
            let sections = section_faces(&half, &cut);
            assert!(
                !sections.is_empty(),
                "{} {part:?}: the cut makes section faces",
                case.name
            );
            if let Err(errors) = topo::validate_geometric(&half, tol()) {
                problems.push(format!("{} {part:?}: tier 3 {errors:?}", case.name));
            }
            for f in &sections {
                if !half.get_face(*f).unwrap().sense {
                    problems.push(format!(
                        "{} {part:?}: section {f:?} is clockwise",
                        case.name
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// **The traced probe.** `(−16/9, 0.9467, 1.8778)` lies in the lower
/// half of the cavity cut at tilt 1.0, beside the bore. The first
/// schedule ray, `+x`, leaves through the section face on the `+y` side
/// of the bore, which faces out of material. The issue's example column
/// `x = −1.422, y = 1.113` rides along.
#[test]
fn the_traced_probe_reads_in() {
    let band = Band::linear(tol()).expect("the witness band");
    let case = &cases()[0];
    let solid = (case.solid)();
    let (_, below) = parts(case, &solid).remove(0);
    let traced = Point3::new(-16.0 / 9.0, 0.9466666666666667, 1.8777777777777778);
    let mut column = vec![traced];
    column.extend([0.128, 0.628, 1.128, 1.628].map(|z| Point3::new(-1.422, 1.113, z)));
    for p in column {
        assert_eq!(
            truth(&solid, case.cut, Some(Part::Below), p),
            Some(true),
            "{p:?} is inside"
        );
        assert!(
            matches!(
                point_in_solid(&below, p, band, tol()),
                Ok(SolidContainment::In)
            ),
            "{p:?}"
        );
    }
}
