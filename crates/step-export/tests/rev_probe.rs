//! REVIEW-ONLY probes (review/m6-6). Not in the `all` aggregator.
//!
//! The print-only rows were retired 2026-08-13
//! ([[review-and-dependency-policy]]'s retirement licence — a row that
//! asserts nothing is never a gate); their claims are owned by
//! `kernel_sidecars.rs` and `m6_6_sense_gate.rs`.
//!
//! What remains here ASSERTS.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;

use geom::Surface;
use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ValidationError, validate_geometric};
use geom_core::Tol;

/// V2: the ball's curved arm is PROVABLY silent (Unencoded, not
/// accidentally-agreeing): flipping EITHER single band stays green.
/// If the arm read an encoded side, exactly one direction would
/// disagree and refuse.
#[test]
fn ball_both_single_band_flips_green() {
    let body = common::ball();
    let bands: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(bands.len(), 2);
    for k in bands {
        let flipped = body.flipped_face_sense_for_tests(k).unwrap();
        let v = validate_geometric(&flipped, Tol::witness());
        let vol = topo::mass_properties(&flipped, Tol::witness()).map(|p| p.volume);
        println!("BALL flip {k:?}: verdict={v:?} vol={vol:?}");
        assert!(v.is_ok(), "either single-band flip must stay exempt");
    }
}

/// V3: nappe adversaries. Report the signed apex-side of each cone
/// face's rims ((p - apex)·axis over boundary sample) and assert
/// honest-green + flip-refuses for every cone face on every body.
fn revolved(pts: &[(f64, f64)], axis_dir: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(axis_dir.0, axis_dir.1),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

#[test]
fn nappe_adversaries() {
    let cases: Vec<(&str, Body<f64>)> = vec![
        ("corpus_cone_apex_up", common::cone()),
        // apex at origin (bottom), base disc on top: material ABOVE apex
        (
            "apex_down_cone",
            revolved(&[(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)], (0.0, 1.0)),
        ),
        // flare bore: conical bore whose virtual apex (0,-1) sits BELOW
        // the face (levels on the +axis side if axis is +y)
        (
            "flare_bore",
            revolved(
                &[(0.5, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)],
                (0.0, 1.0),
            ),
        ),
        // countersink twin (apex at (0,2), ABOVE the face)
        (
            "countersink",
            revolved(
                &[(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.5, 1.0)],
                (0.0, 1.0),
            ),
        ),
    ];
    let mut seen_pos = false;
    let mut seen_neg = false;
    for (name, body) in &cases {
        let ok = validate_geometric(body, Tol::witness());
        println!("NAPPE {name}: honest={:?}", ok.as_ref().map(|_| "green"));
        assert!(ok.is_ok(), "{name}: honest body must be green");
        let cones: Vec<(FaceKey, bool, Point3<f64>, Vec3<f64>)> = body
            .faces()
            .filter_map(|(k, f)| match body.get_surface(f.surface) {
                Some(&Surface::Cone { apex, axis, .. }) => Some((k, f.sense, apex, axis)),
                _ => None,
            })
            .collect();
        assert!(!cones.is_empty(), "{name}: cone faces present");
        for (k, sense, apex, axis) in cones {
            // sample the face's vertices for apex-side sign
            let mut side = 0.0f64;
            for (_, p) in body.points() {
                let d = (*p - apex).dot(axis);
                if d.abs() > side.abs() {
                    side = d;
                }
            }
            if side > 0.0 {
                seen_pos = true;
            } else if side < 0.0 {
                seen_neg = true;
            }
            let flipped = body.flipped_face_sense_for_tests(k).unwrap();
            let errs = validate_geometric(&flipped, Tol::witness())
                .expect_err(&format!("{name}: flipped cone face must refuse"));
            let named = errs
                .iter()
                .any(|e| matches!(e, ValidationError::CurvedSenseInverted { face } if *face == k));
            println!(
                "NAPPE {name} cone sense={sense} apex_side(max|d|)={side:+.3} flip named={named}"
            );
            assert!(
                named,
                "{name}: CurvedSenseInverted must name the cone face: {errs:?}"
            );
        }
    }
    assert!(
        seen_pos && seen_neg,
        "adversary set must exercise BOTH apex sides: pos={seen_pos} neg={seen_neg}"
    );
}

/// V5: the quadrature-owned conic-trimmed cylinder — cut_cylinder's
/// tilted-ellipse wall. Does the kernel gate see a sense flip there,
/// or does the flip slip (PropsError-exempt + winding-derived volume)?
#[test]
fn conic_trimmed_wall_flip_probe() {
    let body = common::cut_cylinder();
    assert!(
        validate_geometric(&body, Tol::witness()).is_ok(),
        "honest cut_cylinder green"
    );
    let walls: Vec<(FaceKey, bool)> = body
        .faces()
        .filter_map(|(k, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. }))
                .then_some((k, f.sense))
        })
        .collect();
    println!("CONIC walls={walls:?}");
    for (k, sense) in walls {
        let flipped = body.flipped_face_sense_for_tests(k).unwrap();
        let v = validate_geometric(&flipped, Tol::witness());
        let vol = topo::mass_properties(&flipped, Tol::witness()).map(|p| p.volume);
        println!("CONIC cut_cylinder wall sense={sense} flip -> {v:?} vol={vol:?}");
    }
}
