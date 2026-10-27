//! Review probes for PR #5823 (ATREST crossing touch): near-contact and
//! turned bodies, read for a decided interference where the two solids
//! only touch, and for the inscribed tetrahedron at every pose.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::meeting::{Pose, posed_box, posed_pyramid, poses};
use geom_core::Tol;
use topo::{Body, ContactRecords, ValidationError, validate_pseudomanifold};

fn t() -> Tol {
    Tol::witness()
}

fn eps() -> f64 {
    t().eps()
}

/// `[decided interference count, TouchInterference count, undecidable
/// count]` and a one-line tally.
fn read(body: &Body<f64>) -> ([usize; 3], String) {
    let errs = validate_pseudomanifold(body, &ContactRecords::default(), t()).err().unwrap_or_default();
    let decided = errs
        .iter()
        .filter(|e| topo::census_interference_solids(body, e).is_some())
        .count();
    let touch = errs
        .iter()
        .filter(|e| format!("{e:?}").starts_with("TouchInterference"))
        .count();
    let undecided: Vec<String> = errs
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusUndecidable { what, .. } => {
                Some(what.chars().take(40).collect())
            }
            _ => None,
        })
        .collect();
    let mut kinds: Vec<String> = errs
        .iter()
        .map(|e| {
            let d = format!("{e:?}");
            let k: String = d.chars().take_while(|c| c.is_alphanumeric()).collect();
            match e {
                ValidationError::UnrecordedContact { contact, .. } => {
                    let c = format!("{contact:?}");
                    format!("{k}/{}", c.chars().take_while(|c| c.is_alphanumeric()).collect::<String>())
                }
                _ => k,
            }
        })
        .collect();
    kinds.sort();
    kinds.dedup();
    (
        [decided, touch, undecided.len()],
        format!("decided={decided} touch={touch} kinds={kinds:?} undecided={undecided:?}"),
    )
}

fn join(a: Body<f64>, b: &topo::AtRestBody<f64>) -> Body<f64> {
    let mut a = a;
    topo::instance::graft_disjoint(&mut a, b).unwrap();
    a
}

/// `pose` after `local`, through the scene's fresh copy of pose `i`.
fn compose(local: &Pose, i: usize) -> Pose {
    poses().into_iter().nth(i).unwrap().after(local)
}

fn local_turn(axis: [f64; 3], angle: f64, c: [f64; 3]) -> Pose {
    let r = Pose::turn("local", axis, angle, [0.0; 3]);
    let rc = r.at(c);
    Pose::turn("local", axis, angle, [c[0] - rc.x, c[1] - rc.y, c[2] - rc.z])
}

const CUBE: [(f64, f64); 3] = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];

/// **The inscribed tetrahedron at every pose** (the PR's witness, turned),
/// and grown or shrunk about the cube's centre by `k ε`.
#[test]
fn probe_inscribed_tetrahedron_at_every_pose() {
    let mut bad = Vec::new();
    for pose in poses() {
        for k in [0.0, -0.25, 0.25, -1e3, 1e3] {
            let s = 1.0 + k * eps();
            let m = |p: [f64; 3]| p.map(|x| 0.5 + (x - 0.5) * s);
            let cube = posed_box(pose.label, CUBE, &pose, t()).into_body();
            let tet = posed_pyramid(
                &[m([1.0, 1.0, 0.0]), m([1.0, 0.0, 1.0]), m([0.0, 1.0, 1.0])],
                m([0.0, 0.0, 0.0]),
                &pose,
                t(),
            );
            let body = join(cube, &tet);
            let ([decided, touch, undecided], line) = read(&body);
            println!("INSCRIBED {} k={k}: {line}", pose.label);
            if k == 0.0 && !(decided == 1 && touch == 1 && undecided == 0) {
                bad.push(format!("{} k={k}: {line}", pose.label));
            }
            if decided == 0 && undecided == 0 {
                bad.push(format!("CLEARED {} k={k}: {line}", pose.label));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A pyramid's apex on the cube's top** (touch only), the apex moved
/// by `s ε` along z, and flattened to `h`: never a decided interference
/// in band or outside the cube.
#[test]
fn probe_apex_on_a_face_is_no_interference() {
    let mut bad = Vec::new();
    for pose in poses() {
        for h in [1.0, 1e-3, 1e-5] {
            for s in [0.0, 0.25, 0.5, -0.25, -0.5, 4.0] {
                let z = 1.0 + s * eps();
                let cube = posed_box(pose.label, CUBE, &pose, t()).into_body();
                let base = [
                    [0.1, 0.2, 1.0 + h],
                    [0.9, 0.1, 1.0 + h],
                    [0.8, 0.9, 1.0 + h],
                    [0.2, 0.7, 1.0 + h],
                ];
                // Counterclockwise seen from the apex (below): reverse.
                let base: Vec<_> = base.iter().rev().copied().collect();
                let Ok(pyr) = std::panic::catch_unwind(|| {
                    posed_pyramid(&base, [0.45, 0.5, z], &pose, t())
                }) else {
                    println!("APEX {} h={h} s={s}: pyramid did not build", pose.label);
                    continue;
                };
                let body = join(cube, &pyr);
                let ([decided, touch, _], line) = read(&body);
                println!("APEX {} h={h} s={s}: {line}", pose.label);
                if (decided > 0 || touch > 0) && s >= -0.5 {
                    bad.push(format!("{} h={h} s={s}: {line}", pose.label));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **A box resting on the cube's top on one bottom edge, its far side
/// lifted by `r ε`** (touch only, a thin wedge of air under it), and
/// the same box resting flat turned about z (coplanar faces opposed,
/// edges crossing in the plane): never a decided interference.
#[test]
fn probe_thin_wedges_and_coplanar_sheets_are_no_interference() {
    let mut bad = Vec::new();
    for (i, pose) in poses().into_iter().enumerate() {
        // Lifted on its edge y = 0.25 (inside the cube's top), far side y = 0.75.
        for r in [0.25, 0.5, 1.0, 2.0, 8.0, 1e3, 1e6] {
            for dir in [1.0, -1.0] {
                let angle = dir * (r * eps() / 0.5).atan();
                let local = local_turn([1.0, 0.0, 0.0], angle, [0.0, 0.25, 1.0]);
                let p = compose(&local, i);
                let cube = posed_box(pose.label, CUBE, &pose, t()).into_body();
                let b = posed_box(pose.label, [(0.25, 0.75), (0.25, 0.75), (1.0, 1.5)], &p, t());
                let body = join(cube, &b);
                let ([decided, touch, _], line) = read(&body);
                println!("LIFT {} r={r} dir={dir}: {line}", pose.label);
                // dir = 1 lifts the far side (y grows, z grows): touch only.
                if dir > 0.0 && (decided > 0 || touch > 0) {
                    bad.push(format!("LIFT {} r={r}: {line}", pose.label));
                }
            }
        }
        // Flat, turned about z through the top's centre; and overhanging
        // the cube's edges so edges cross in the plane.
        for (what, b) in [
            ("inside", [(0.25, 0.75), (0.25, 0.75), (1.0, 1.5)]),
            ("overhang", [(-0.3, 1.3), (0.3, 0.7), (1.0, 1.5)]),
        ] {
            for s in [0.0, 0.25, 0.5] {
                let local = Pose::turn("shift", [0.0, 0.0, 1.0], 0.0, [0.0, 0.0, -s * eps()])
                    .after(&local_turn([0.0, 0.0, 1.0], 0.4, [0.5, 0.5, 1.0]));
                let p = compose(&local, i);
                let cube = posed_box(pose.label, CUBE, &pose, t()).into_body();
                let bx = posed_box(pose.label, b, &p, t());
                let body = join(cube, &bx);
                let ([decided, touch, _], line) = read(&body);
                println!("FLAT {what} {} s={s}: {line}", pose.label);
                if decided > 0 || touch > 0 {
                    bad.push(format!("FLAT {what} {} s={s}: {line}", pose.label));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}
