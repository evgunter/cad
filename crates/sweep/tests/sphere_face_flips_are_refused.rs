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
        // The door reads the same seam regions without tier 3: it refuses
        // every flip tier 3 does, or measures it inside out, which is
        // check 7's reading of the door's own sign; and it measures every
        // flip tier 3 passes. The sign half holds for a body of one shell
        // only: a whole sphere shell flipped inside a body of several (a
        // ball less a ball the fuzz drew strictly inside it) measures a
        // positive wrong total at the raw door. That body never finishes,
        // so the typed door never reads it, and the hollow-ball row of
        // `the_door_reads_seam_regions_as_tier_3_does` pins the raw door's
        // reading of it
        // (`a-sphere-shell-flipped-whole-in-a-multi-shell-body-measures-wrong-at-the-door`, closed by PR 5590).
        // Raw door: the subject is the door's parity with tier 3 over flips tier 3 may refuse.
        let door = topo::test_support::measure_unvalidated(
            &flipped,
            tol,
            topo::test_support::pinned_resolution(tol),
        )
        .map(|m| m.enclosure.volume);
        let tier3 = topo::validate_geometric(&flipped, tol);
        match (&tier3, &door) {
            (Ok(()), Err(e)) => failures.push(format!(
                "{label}: flipping {set:?} passes tier 3 and the door refuses it: {e:?}"
            )),
            (Err(errs), Ok(v)) if *v > 0.0 && body.shells().count() == 1 => failures.push(format!(
                "{label}: flipping {set:?}: tier 3 refuses ({errs:?}), the door measures {v}"
            )),
            _ => {}
        }
        match tier3 {
            Ok(()) => failures.push(format!(
                "{label}: flipping {set:?} passes tier 3 (measures {:?}, unflipped {:?})",
                // Raw door: a failure message, reading the flip and its unflipped body as the parity above does.
                topo::test_support::measure_unvalidated(
                    &flipped,
                    tol,
                    topo::test_support::pinned_resolution(tol)
                )
                .map(|m| m.enclosure.volume),
                topo::test_support::measure_unvalidated(
                    body,
                    tol,
                    topo::test_support::pinned_resolution(tol)
                )
                .map(|m| m.enclosure.volume),
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
    // The double cover split until each face reads its own side, and the
    // valid ball split so (review of PR 5516, R2's `p2`): the door reads
    // their seam regions as tier 3 does, the bodies themselves included.
    {
        use crate::seam_region_common::{
            split_faces_at_equator, split_meridians_at_equator, split_outside, v,
        };
        let ball = finished(
            "ball",
            ball_poled(1.0, v([0.0; 3]), v([0.0, 0.0, 1.0]), tol),
            tol,
        );
        let mut control = Body::clone(&ball);
        split_meridians_at_equator(&mut control, tol);
        split_faces_at_equator(&mut control, tol);
        let mut cover = Body::clone(&ball);
        assert!(split_outside(&mut cover, v([0.0, 1.0, 0.0]), tol));
        split_meridians_at_equator(&mut cover, tol);
        split_faces_at_equator(&mut cover, tol);
        for (name, body) in [("split ball", &control), ("split double cover", &cover)] {
            total += flips(name, body, &mut failures) + 1;
            let at_rest = topo::AtRestBody::validate(topo::Body::clone(body), tol);
            let tier3 = at_rest.is_ok();
            let door = match &at_rest {
                Ok(finished) => {
                    topo::mass_properties(finished, tol, topo::test_support::pinned_resolution(tol))
                        .is_ok()
                }
                // Raw door: the subject is the door's parity with tier 3, here over the double cover tier 3 refuses.
                Err(_) => topo::test_support::measure_unvalidated(
                    body,
                    tol,
                    topo::test_support::pinned_resolution(tol),
                )
                .is_ok(),
            };
            if tier3 != door {
                failures.push(format!(
                    "{name}: tier 3 passes = {tier3}, the door measures = {door}"
                ));
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
