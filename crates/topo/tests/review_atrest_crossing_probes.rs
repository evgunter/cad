//! Review probes for PR #5823 (ATREST crossing touch). Each row prints
//! the census's verdict classes; the asserts are the claims: a gap or
//! an exact touch decides no interference (claim 2), and a touch in
//! band is never a `TouchInterference` (claim 1). Written to compile
//! on the merge-base too (findings classed by their `Debug` name).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use common::meeting::{Pose, posed_box, posed_pyramid, poses};
use geom_core::Tol;
use topo::{Body, ContactRecords, ValidationError, validate_pseudomanifold};

fn classes(body: &Body<f64>) -> (Vec<String>, bool) {
    let errs = validate_pseudomanifold(body, &ContactRecords::default(), Tol::witness())
        .err()
        .unwrap_or_default();
    let decided = errs
        .iter()
        .any(|e| topo::census_interference_solids(body, e).is_some());
    let out = errs
        .iter()
        .map(|e: &ValidationError| {
            let d = format!("{e:?}");
            let name: String = d.chars().take_while(|c| c.is_alphanumeric()).collect();
            match e {
                ValidationError::CensusUndecidable { what, .. } => {
                    format!("{name}[{}]", what.chars().take(40).collect::<String>())
                }
                ValidationError::UnrecordedContact { contact, .. } => {
                    let c = format!("{contact:?}");
                    format!("{name}[{}]", c.chars().take_while(|c| c.is_alphanumeric()).collect::<String>())
                }
                _ => name,
            }
        })
        .collect();
    (out, decided)
}

fn pair(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut body = a.clone();
    topo::graft_disjoint(&mut body, b).unwrap();
    body
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// Offsets: exact, inside the band either way, and past it.
fn offsets() -> Vec<(&'static str, f64)> {
    let e = eps();
    vec![
        ("0", 0.0),
        ("+0.3e", 0.3 * e),
        ("-0.3e", -0.3 * e),
        ("+3e", 3.0 * e),
        ("-3e", -3.0 * e),
        ("+30e", 30.0 * e),
        ("-30e", -30.0 * e),
        ("-300e", -300.0 * e),
        ("+1e-3", 1e-3),
        ("-1e-3", -1e-3),
    ]
}

fn extra_poses() -> Vec<Pose> {
    let mut v = poses();
    v.push(Pose::turn("odd", [0.3, -0.7, 0.2], 2.3, [0.11, 0.07, -0.19]));
    v
}

/// Print one row; flag a decided interference where `gap` (the
/// solids only touch or stand apart), and a `TouchInterference` in band.
fn row(what: &str, body: &Body<f64>, gap: bool, in_band: bool, bad: &mut Vec<String>) {
    let (c, decided) = classes(body);
    let ti = c.iter().any(|s| s.starts_with("TouchInterference"));
    println!("{what}: decided={decided} {c:?}");
    if gap && decided {
        bad.push(format!("FALSE INTERFERENCE {what}: {c:?}"));
    }
    if in_band && ti {
        bad.push(format!("IN-BAND TOUCHINTERFERENCE {what}: {c:?}"));
    }
}

fn band_of(d: f64) -> bool {
    d.abs() > 0.0 && d.abs() <= 100.0 * eps()
}

#[test]
fn probe_inscribed_tetrahedron_turned() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for pose in extra_poses() {
        let cube = posed_box("cube", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], &pose, tol);
        let tet = posed_pyramid(
            &[[1.0, 1.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
            [0.0, 0.0, 0.0],
            &pose,
            tol,
        );
        let body = pair(&cube, &tet);
        let (c, decided) = classes(&body);
        println!("tet {}: decided={decided} {c:?}", pose.label);
        if !decided {
            bad.push(format!("tet {} not decided: {c:?}", pose.label));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Two unit cubes meeting on a face, an edge or a corner: rests, every
/// pose.
#[test]
fn probe_cubes_touching_rest() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for pose in extra_poses() {
        let a = posed_box("a", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], &pose, tol);
        for (what, b) in [
            ("face", [(1.0, 2.0), (0.2, 0.8), (0.1, 0.9)]),
            ("face-full", [(1.0, 2.0), (0.0, 1.0), (0.0, 1.0)]),
            ("edge", [(1.0, 2.0), (1.0, 2.0), (0.2, 0.7)]),
            ("corner", [(1.0, 2.0), (1.0, 2.0), (1.0, 2.0)]),
        ] {
            let b = posed_box("b", b, &pose, tol);
            row(&format!("cubes {what} {}", pose.label), &pair(&a, &b), true, false, &mut bad);
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A pyramid's apex at `(0.5, 0.5, 1 + d)` pointing down onto a cube's
/// top, and a box pressed `d` above (or into) the top.
#[test]
fn probe_apex_and_box_near_a_face() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for pose in extra_poses() {
        let a = posed_box("a", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], &pose, tol);
        for (label, d) in offsets() {
            let z = 1.0 + d;
            let pyr = posed_pyramid(
                &[[0.1, 0.1, 2.0], [0.1, 0.9, 2.0], [0.9, 0.9, 2.0], [0.9, 0.1, 2.0]],
                [0.5, 0.5, z],
                &pose,
                tol,
            );
            row(
                &format!("apex {label} {}", pose.label),
                &pair(&a, &pyr),
                d >= 0.0,
                band_of(d),
                &mut bad,
            );
            // An edge of a wedge lying on the top: a tetrahedron with
            // its lowest edge along y at height z.
            let tet = posed_pyramid(
                &[[0.5, 0.2, z], [0.2, 0.5, 1.8], [0.8, 0.5, 1.8]],
                [0.5, 0.8, z],
                &pose,
                tol,
            );
            row(
                &format!("edge-on-face {label} {}", pose.label),
                &pair(&a, &tet),
                d >= 0.0,
                band_of(d),
                &mut bad,
            );
            let b = posed_box("b", [(0.2, 0.8), (0.3, 0.7), (z, z + 1.0)], &pose, tol);
            row(
                &format!("box {label} {}", pose.label),
                &pair(&a, &b),
                d >= 0.0,
                band_of(d),
                &mut bad,
            );
            // An apex on the cube's top edge (y = 1, z = 1), the pyramid
            // lying along +y above: touch only for d >= 0.
            let edge_apex = posed_pyramid(
                &[[0.2, 1.8, 1.4 + d], [0.8, 1.8, 1.4 + d], [0.8, 1.8, 2.0 + d], [0.2, 1.8, 2.0 + d]],
                [0.5, 1.0 + d, 1.0 + d],
                &pose,
                tol,
            );
            row(
                &format!("apex-on-edge {label} {}", pose.label),
                &pair(&a, &edge_apex),
                d >= 0.0,
                band_of(d),
                &mut bad,
            );
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// A box tilted about its edge resting on the cube's top: its far
/// bottom edge dips `0.75·sin θ` into the cube (θ > 0) or lifts.
#[test]
fn probe_thin_wedge() {
    let tol = Tol::witness();
    let mut bad = Vec::new();
    for scene in 0..extra_poses().len() {
        for (label, d) in offsets() {
            let theta = (-d / 0.75_f64).asin();
            let pose = || extra_poses().swap_remove(scene);
            let a = posed_box("a", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], &pose(), tol);
            let tilt = Pose::turn("tilt", [0.0, 1.0, 0.0], theta, [0.25, 0.0, 1.0]);
            let b = posed_box(
                "b",
                [(0.0, 1.0), (0.2, 0.8), (0.0, 1.0)],
                &pose().after(&tilt),
                tol,
            );
            row(
                &format!("wedge {label} {}", pose().label),
                &pair(&a, &b),
                d >= 0.0,
                band_of(d),
                &mut bad,
            );
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// **`placement_findings` sees a pierce-only overlap.** A box tilted
/// 1e-3 into a cube's top along an edge it rests on: the overlap
/// shows only as pierces (no vertex strictly inside, no coplanar face
/// pair). Rows that assert a pose clears read `placement_findings(..)
/// .is_empty()`; red where that reads such a pose as clear.
#[test]
fn probe_placement_findings_sees_a_pierced_pair() {
    let tol = Tol::witness();
    let theta = (1e-3_f64 / 0.75).asin();
    let a = posed_box("a", [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)], &Pose::rest(), tol);
    let tilt = Pose::turn("tilt", [0.0, 1.0, 0.0], theta, [0.25, 0.0, 1.0]);
    let b = posed_box("b", [(0.0, 1.0), (0.2, 0.8), (0.0, 1.0)], &tilt, tol);
    let body = pair(&a, &b);
    let errs = validate_pseudomanifold(&body, &ContactRecords::default(), tol)
        .err()
        .unwrap_or_default();
    assert!(
        errs.iter().any(|e| topo::census_interference_solids(&body, e).is_some()),
        "the census decides the overlap: {errs:?}"
    );
    assert!(
        !common::placement_findings(&errs).is_empty(),
        "placement_findings reads the decided overlap as a clear: {errs:?}"
    );
}
