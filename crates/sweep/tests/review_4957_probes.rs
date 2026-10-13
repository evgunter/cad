//! Review probes for PR 4957 (bore ruling step 4: the plane–plane band
//! and the chamfer consume a through-feature wholly inside the material
//! they remove).
//!
//! - **p1**: the carve at dihedrals other than a right one. The
//!   chamfer's triangle and the fillet's sliver at a 60° and a 120°
//!   lateral edge, a bore wholly inside each: `ΔV = −(A(θ) − π·a²)`,
//!   `A(θ)` the chamfer's `r²·sin θ / 2` or the fillet's
//!   `r²·(cot(θ/2) − (π − θ)/2)`.
//! - **p2**: a hole in kept material just past a support's plane, in
//!   `Ω` but outside the chamfer's triangle (a C-section whose upper jaw
//!   overhangs the lower jaw's support across a `0.005` gap). It must
//!   never be consumed. This is the row that pins the triangle's rim
//!   sides: on a convex body the rims bound the end face itself, so
//!   no other row can read a cycle past one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Real};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::BlendError;
use sweep::test_support::{assert_naming_totality, extruded, finished};
use topo::{Body, ContactRecords, FaceKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, Verb, edge, tol, volume_enclosure};

fn poly<T: Real>(pts: &[(f64, f64)]) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(
        pts.iter()
            .map(|&(x, y)| (Point2::new(f(x), f(y)), f(0.0)))
            .collect(),
    )
}

fn round<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

/// `outline` with the bore `(x, y, a)` through it, extruded a unit
/// along `z`.
fn bored(outline: &[(f64, f64)], hole: (f64, f64, f64)) -> topo::AtRestBody<f64> {
    finished(
        "probe",
        extruded(
            SketchPlane::xy(),
            vec![poly(outline), round(hole.0, hole.1, hole.2)],
            1.0,
            tol(),
        ),
        tol(),
    )
}

fn curved_walls(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            !matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The section `verb` at size `D` removes from a convex dihedral of
/// interior angle `theta`.
fn section(verb: Verb, theta: f64) -> f64 {
    match verb {
        Verb::Chamfer => D * D * theta.sin() / 2.0,
        Verb::Fillet => D * D * (1.0 / (theta / 2.0).tan() - (PI - theta) / 2.0),
    }
}

/// **p1.** A bore wholly inside the sliver at a 60° and a 120° lateral
/// edge at `(0, 0)` goes with it: tiers 3 and 3′, naming totality, both
/// half-walls gone and retired, no ring left, genus 0, and `ΔV` at the
/// closed form to `1e-9`.
#[test]
fn p1_a_bore_inside_the_sliver_of_a_non_right_dihedral_goes_at_the_closed_form() {
    let s3 = 3f64.sqrt();
    let acute: &[(f64, f64)] = &[(0.0, 0.0), (1.0, 0.0), (0.5, s3 / 2.0)];
    let obtuse: &[(f64, f64)] = &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (-0.5, s3 / 2.0)];
    // The bisector's unit direction at `(0, 0)`, and each case's bore:
    // centre `t` along it, radius `a`.
    let cases = [
        // 60°: the chamfer's triangle is equilateral of side `D`, its
        // incentre `D/√3` along the bisector, inradius `D/(2√3)`.
        ("60° chamfer", acute, PI / 3.0, Verb::Chamfer, D / s3, 0.01),
        // 60°: the arc's centre `2D` along the bisector, nearest `V` at
        // `D`; a bore `0.05` along, `0.025` off each side.
        ("60° fillet", acute, PI / 3.0, Verb::Fillet, 0.05, 0.01),
        // 120°: the triangle's incentre `r_in/sin 60°` along the
        // bisector, `r_in = A/s`.
        (
            "120° chamfer",
            obtuse,
            2.0 * PI / 3.0,
            Verb::Chamfer,
            {
                let area = D * D * (2.0 * PI / 3.0).sin() / 2.0;
                let semi = (2.0 * D + D * s3) / 2.0;
                (area / semi) / (PI / 3.0).sin()
            },
            0.006,
        ),
        // 120°: the arc's centre `D/sin 60°` along the bisector, nearest
        // `V` at `≈ 0.0155`; a bore `0.008` along, radius `0.003`.
        (
            "120° fillet",
            obtuse,
            2.0 * PI / 3.0,
            Verb::Fillet,
            0.008,
            0.003,
        ),
    ];
    for (what, outline, theta, verb, t, a) in cases {
        let half = theta / 2.0;
        let (x, y) = (t * half.cos(), t * half.sin());
        let body = bored(outline, (x, y, a));
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3 {e:?}"));
        let walls = curved_walls(&body);
        assert_eq!(walls.len(), 2, "{what}: the bore's two half-walls");
        let e = edge(&body, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        let out = verb
            .run_finished(&body, &[e])
            .unwrap_or_else(|err| panic!("{what}: the bore goes with the sliver, got {err}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
        validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3′ {e:?}"));
        assert_naming_totality(&body, &out, &[e], what);
        let rec = out.naming.as_ref().expect("births");
        assert!(
            walls
                .iter()
                .all(|w| out.body.get_face(*w).is_none() && rec.dead.faces.contains(w)),
            "{what}: the walls are consumed and retired"
        );
        assert!(
            out.body.faces().all(|(_, f)| f.rings.is_empty()),
            "{what}: no ring is left"
        );
        let c = topo::readback::euler_counts(&out.body);
        assert_eq!((c.s, c.genus()), (1, Ok(0)), "{what}: one genus-0 shell");
        let want = section(verb, theta) - PI * a * a;
        let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
        assert!(
            (dv - want).abs() < 1e-9,
            "{what}: ΔV {dv} vs the closed form {want}"
        );
    }
}

/// **p2.** A C-section: the lower jaw `[0, 1] × [0, 1]`, the upper jaw
/// `[0, 1] × [1.005, 1.2]` joined to it at `x ≤ 0.2`, so the lower
/// jaw's top `y = 1` (`x ∈ [0.2, 1]`) is the slot's floor; and the same
/// section transposed (`x ↔ y`), so each of the crease's two supports
/// in turn is the one kept material lies just past. The crease at
/// `(1, 1)` is chamfered and filleted at `D`. A bore of radius `0.003`
/// at `(0.97, 1.01)` (transposed `(1.01, 0.97)`) runs through the upper
/// jaw — kept material, past the support's plane — inside the region
/// `Ω` about the chord's midpoint, on `V`'s side of the chord and of
/// the other rim. It is outside the material the band removes, so the
/// request either builds and keeps both its walls at `ΔV = −A`, or
/// refuses as a bound: never consumes it, never reads it (or anything
/// else in kept material) inside, and never measures a crossing. Each
/// of the chamfer triangle's two rim sides is load-bearing here: with
/// either deleted, one of the two sections reads a crossing.
#[test]
fn p2_a_bore_in_kept_material_past_a_rim_in_omega_is_never_consumed() {
    let c: &[(f64, f64)] = &[
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.2, 1.0),
        (0.2, 1.005),
        (1.0, 1.005),
        (1.0, 1.2),
        (0.0, 1.2),
    ];
    let transposed: Vec<(f64, f64)> = c.iter().rev().map(|&(x, y)| (y, x)).collect();
    for (what, outline, hole) in [
        ("C", c.to_vec(), (0.97, 1.01, 0.003)),
        ("transposed C", transposed, (1.01, 0.97, 0.003)),
    ] {
        let body = bored(&outline, hole);
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3 {e:?}"));
        let walls = curved_walls(&body);
        assert_eq!(walls.len(), 2, "{what}: the bore's two half-walls");
        let e = edge(&body, [1.0, 1.0, 0.0], [1.0, 1.0, 1.0]);
        for verb in [Verb::Chamfer, Verb::Fillet] {
            match verb.run_finished(&body, &[e]) {
                Ok(out) => {
                    validate_geometric(&out.body, tol())
                        .unwrap_or_else(|err| panic!("{what} {verb:?}: tier 3 {err:?}"));
                    assert!(
                        walls.iter().all(|w| out.body.get_face(*w).is_some()),
                        "{what} {verb:?}: the bore in kept material survives"
                    );
                    let dv = volume_enclosure(&body).0 - volume_enclosure(&out.body).0;
                    let want = Verb::section(verb);
                    assert!(
                        (dv - want).abs() < 1e-9,
                        "{what} {verb:?}: ΔV {dv} vs the band's section alone {want}"
                    );
                }
                Err(err) => {
                    assert!(
                        matches!(
                            err,
                            BlendError::RingClearance { bounded: true, .. }
                                | BlendError::FaceClearance { bounded: true, .. }
                        ),
                        "{what} {verb:?}: kept material refuses only as a bound, got {err:?}"
                    );
                    eprintln!("p2 {what} {verb:?}: refuses {err:?}");
                }
            }
        }
    }
}
