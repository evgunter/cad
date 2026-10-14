//! Every sphere face of a boolean of balls, or of a slab and a ball,
//! flipped alone and with each other face on its surface: tier 3
//! refuses every such body, and names each flipped face whose own loop
//! encodes no side by the side of its seam region, where the region has
//! a boundary: a whole sphere is the volume checks' to read.
//!
//! A counterexample search (the PR 4573 flip review's harness, which
//! found six of its 568 flips silent at one seed).
#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

test_utils::gated_to![
    "crates/topo/src/validate.rs",
    "crates/topo/src/props.rs",
    "crates/geom-brep/src/props/curved.rs",
    "crates/sweep/tests/seam_region_common/",
];

use crate::seam_region_common::{flipped, seam_region, unencoded};
use geom::Surface;
use geom_core::{Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};
use test_utils::fuzz;
use topo::{Body, FaceKey, ValidationError};

fn dir(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.1 && n < 1.0 {
            return v / n;
        }
    }
}

/// Flips the sphere faces of `body` alone and in same-surface pairs;
/// returns the number of flips, and every one tier 3 passed or did not
/// name as it should.
fn flips(label: &str, body: &Body<f64>, failures: &mut Vec<String>) -> usize {
    let tol = Tol::witness();
    let sphere: Vec<_> = body
        .faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, f)| (k, f.surface))
        .collect();
    let mut sets: Vec<Vec<FaceKey>> = sphere.iter().map(|(k, _)| vec![*k]).collect();
    for (i, (a, sa)) in sphere.iter().enumerate() {
        for (b, sb) in &sphere[i + 1..] {
            if sa == sb {
                sets.push(vec![*a, *b]);
            }
        }
    }
    let count = sets.len();
    for set in sets {
        let flipped = flipped(body, &set);
        match topo::validate_geometric(&flipped, tol) {
            Ok(()) => failures.push(format!(
                "{label}: flipping {set:?} passes tier 3 (measures {:?}, unflipped {:?})",
                topo::mass_properties(&flipped, tol).map(|m| m.volume),
                topo::mass_properties(body, tol).map(|m| m.volume),
            )),
            Err(errs) => {
                for &face in &set {
                    let named = errs.iter().any(|e| {
                        matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == face)
                    });
                    // A region with no boundary is a whole sphere,
                    // whose side the volume checks read.
                    if unencoded(body, face, tol) && seam_region(body, face).1 && !named {
                        failures.push(format!(
                            "{label}: flipping {set:?} does not name {face:?}: {errs:?}"
                        ));
                    }
                }
            }
        }
    }
    count
}

#[test]
fn every_sphere_face_flip_is_refused_and_named() {
    let tol = Tol::witness();
    let mut rng = fuzz::start("sphere_face_flips");
    let mut failures = Vec::new();
    let mut total = 0;
    for i in 0..fuzz::scaled(10) {
        let rb = rng.range(0.5, 1.2);
        let d = rng.range(0.4, 1.0 + rb - 0.2);
        let off = dir(&mut rng) * d;
        let a = finished(
            "a",
            ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), dir(&mut rng), tol),
            tol,
        );
        let b = finished("b", ball_poled(rb, off, dir(&mut rng), tol), tol);
        for (name, r) in [
            ("union", topo::boolean::union(&a, &b, tol)),
            ("inter", topo::boolean::intersect(&a, &b, tol)),
            ("a-b", topo::boolean::subtract(&a, &b, tol)),
        ] {
            if let Some(res) = r.ok().as_ref().and_then(|r| r.body()) {
                total += flips(&format!("balls #{i} {name}"), &res.body, &mut failures);
            }
        }
    }
    for i in 0..fuzz::scaled(5) {
        let r = rng.range(0.3, 0.9);
        let dip = rng.range(0.05, 0.9 * r);
        let c = Vec3::new(rng.range(1.2, 2.8), rng.range(1.2, 2.8), 1.0 + r - dip);
        let slab = finished("slab", brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), tol), tol);
        let pole = Vec3::new(0.0, 0.0, if i % 2 == 0 { 1.0 } else { -1.0 });
        let ball = finished("ball", ball_poled(r, c, pole, tol), tol);
        for (name, res) in [
            ("slab-ball", topo::boolean::subtract(&slab, &ball, tol)),
            ("slab+ball", topo::boolean::union(&slab, &ball, tol)),
            ("slab^ball", topo::boolean::intersect(&slab, &ball, tol)),
            ("ball-slab", topo::boolean::subtract(&ball, &slab, tol)),
        ] {
            if let Some(res) = res.ok().as_ref().and_then(|r| r.body()) {
                total += flips(&format!("slab #{i} {name}"), &res.body, &mut failures);
            }
        }
    }
    println!("{total} flips, {} failures", failures.len());
    assert!(
        failures.is_empty(),
        "{} of {total} flips: {:#?} {}",
        failures.len(),
        failures,
        fuzz::replay()
    );
}
