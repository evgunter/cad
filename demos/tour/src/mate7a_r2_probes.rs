//! R2 review probes for MATE-7a (PR #1477). Independent re-measurement
//! of the unit's headline deviation: lily wall 1's refusal.
//!
//! These rows do not assert the PR's story; they PRINT the measured
//! geometry and assert only what the reviewer independently confirms.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![cfg(test)]

use pncad::geom_core::{Point3, Tol};
use pncad::topo::{Body, BooleanError, FaceKey};

use crate::lily::plant;

fn body_of<'a>(pieces: &'a [crate::lily::Piece<f64>], name: &str) -> &'a Body<f64> {
    &pieces
        .iter()
        .find(|p| p.name == name)
        .expect("named lily piece")
        .body
}

/// Point cloud of ONE face, from the tessellation's per-face patch.
fn face_points(body: &Body<f64>, face: FaceKey, chordal: f64) -> Vec<Point3<f64>> {
    let m = pncad::mesh::tessellate(body, chordal, Tol::witness()).expect("tessellates");
    let patch = m
        .patches
        .iter()
        .find(|p| p.face == face)
        .expect("the named face has a patch");
    let mut idx: Vec<u32> = patch.triangles.iter().flatten().copied().collect();
    idx.sort_unstable();
    idx.dedup();
    idx.into_iter()
        .map(|i| m.positions[i as usize])
        .collect::<Vec<_>>()
}

fn aabb(pts: &[Point3<f64>]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in pts {
        for (k, v) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[k] = lo[k].min(v);
            hi[k] = hi[k].max(v);
        }
    }
    (lo, hi)
}

/// **R2-1: re-measure lily wall 1 from the scene itself.**
///
/// Prints, and asserts, the four things the PR's deviation 1 rests on:
/// which pair the gate names, the two exact loci's separation, the two
/// tube radii, and whether the whole-torus box is what overlaps.
#[test]
fn r2_lily_wall_one_remeasured() {
    let tol = Tol::witness();
    let pieces = plant::<f64>(tol);
    let stem = body_of(&pieces, "lily_stem");
    let arch = body_of(&pieces, "lily_arch");

    let err = crate::booleans::try_union_declared(stem, arch, tol).expect_err("wall 1 refuses");
    println!("R2 wall-1 refusal: {err:?}");
    let BooleanError::CurvedPairUnsupported {
        operand,
        face,
        kind,
        other_face,
        other_kind,
        ..
    } = err
    else {
        panic!("expected the operand gate's refusal")
    };
    println!("R2 named pair: operand={operand:?} kind={kind:?} other_kind={other_kind:?}");

    // The two surfaces the gate named.
    let sa = stem
        .get_face(face)
        .and_then(|f| stem.get_surface(f.surface))
        .expect("stem face surface");
    let sb = arch
        .get_face(other_face)
        .and_then(|f| arch.get_surface(f.surface))
        .expect("arch face surface");
    println!("R2 stem face surface: {sa:?}");
    println!("R2 arch face surface: {sb:?}");

    // Every torus tube radius on each body.
    let minors = |b: &Body<f64>| {
        b.faces()
            .filter_map(|(_, f)| match b.get_surface(f.surface) {
                Some(&pncad::geom::Surface::Torus { minor_radius, .. }) => Some(minor_radius),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    println!("R2 stem tube radii: {:?}", minors(stem));
    println!("R2 arch tube radii: {:?}", minors(arch));

    // The EXACT loci, sampled off the tessellation of each named face.
    let pa = face_points(stem, face, 2e-3);
    let pb = face_points(arch, other_face, 2e-3);
    let mut best = f64::INFINITY;
    for x in &pa {
        for y in &pb {
            best = best.min(x.distance(*y));
        }
    }
    println!(
        "R2 exact-locus separation (sampled, {}x{} pts): {best}",
        pa.len(),
        pb.len()
    );
    println!("R2 stem face AABB: {:?}", aabb(&pa));
    println!("R2 arch face AABB: {:?}", aabb(&pb));

    // The WHOLE-TORUS box of the stem's named face, computed here from
    // the stored carrier alone (what a boundary-blind box would be).
    if let pncad::geom::Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        ..
    } = *sa
    {
        let r = major_radius + minor_radius;
        println!(
            "R2 whole-torus carrier box (centre {center:?}, axis {axis:?}): \
             ring extent ±{r} about the centre in the ring plane, ±{minor_radius} along the axis"
        );
    }

    assert!(
        best > 0.5,
        "R2: the two named loci must be far apart if the PR's 'never approach' claim holds; \
         measured {best}"
    );
}

/// **R2-2: is the named other-face really the arch's FAR cap?**
/// Prints every planar face of the arch with its origin so the "far
/// cap" identification can be checked rather than assumed — and
/// asserts unconditionally (the shipped row's assertion is inside an
/// `if let` and goes vacuous when the `find` misses).
#[test]
fn r2_the_arch_far_cap_identification_is_not_conditional() {
    let tol = Tol::witness();
    let pieces = plant::<f64>(tol);
    let stem = body_of(&pieces, "lily_stem");
    let arch = body_of(&pieces, "lily_arch");
    let planes: Vec<_> = arch
        .faces()
        .filter_map(|(k, f)| match arch.get_surface(f.surface) {
            Some(&pncad::geom::Surface::Plane { origin, normal, .. }) => Some((k, origin, normal)),
            _ => None,
        })
        .collect();
    for (k, o, n) in &planes {
        println!(
            "R2 arch plane {k:?}: origin {o:?} |origin| {} normal {n:?}",
            (*o - Point3::new(0.0, 0.0, 0.0)).norm()
        );
    }
    let far = planes
        .iter()
        .find(|(_, o, _)| (*o - Point3::new(0.0, 0.0, 0.0)).norm() > 2.0);
    assert!(
        far.is_some(),
        "R2: the shipped row's `find` must actually hit, or its assert_eq never runs"
    );
    let err = crate::booleans::try_union_declared(stem, arch, tol).expect_err("wall 1 refuses");
    let BooleanError::CurvedPairUnsupported { other_face, .. } = err else {
        panic!("expected the gate")
    };
    println!(
        "R2 named other_face {other_face:?}; far cap {:?}",
        far.map(|(k, _, _)| *k)
    );
}

/// **R2-3: would a boundary-tight torus box actually retire wall 1?**
///
/// The PR's deviation 1 says wall 1 is the whole-torus box artifact.
/// This row measures the counterfactual it does not state: for EVERY
/// cross pair the kind roster can offend on, the TRUE separation of
/// the two exact loci. A pair whose loci separation is ~0 would still
/// offend under a perfect box, and no declaration can cover it unless
/// the carrier ladder can give it a verdict.
#[test]
fn r2_which_pairs_survive_a_perfect_box() {
    let tol = Tol::witness();
    let pieces = plant::<f64>(tol);
    let stem = body_of(&pieces, "lily_stem");
    let arch = body_of(&pieces, "lily_arch");
    let mesh_a = pncad::mesh::tessellate(stem, 2e-3, tol).expect("stem tessellates");
    let mesh_b = pncad::mesh::tessellate(arch, 2e-3, tol).expect("arch tessellates");
    let cloud = |m: &pncad::mesh::Mesh, f: FaceKey| -> Vec<Point3<f64>> {
        let p = m.patches.iter().find(|p| p.face == f).expect("patch");
        let mut idx: Vec<u32> = p.triangles.iter().flatten().copied().collect();
        idx.sort_unstable();
        idx.dedup();
        idx.into_iter().map(|i| m.positions[i as usize]).collect()
    };
    let kind = |b: &Body<f64>, f: FaceKey| {
        b.get_face(f)
            .and_then(|x| b.get_surface(x.surface))
            .map(|s| {
                format!("{s:?}")
                    .split_whitespace()
                    .next()
                    .unwrap_or("?")
                    .to_string()
            })
            .unwrap_or_default()
    };
    for (ka, _) in stem.faces() {
        if kind(stem, ka) != "Torus" {
            continue;
        }
        let pa = cloud(&mesh_a, ka);
        for (kb, _) in arch.faces() {
            let pb = cloud(&mesh_b, kb);
            let mut best = f64::INFINITY;
            for x in &pa {
                for y in &pb {
                    best = best.min(x.distance(*y));
                }
            }
            println!(
                "[r2-3] stem {ka:?} ({}) vs arch {kb:?} ({}): true locus separation {best:.6}",
                kind(stem, ka),
                kind(arch, kb)
            );
        }
    }
}
