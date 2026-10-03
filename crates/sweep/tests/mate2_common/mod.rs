//! Shared scene builders for the MATE-2 suites (issue 1032): the unit's
//! own fixtures and both review lanes' adversarial probes.
//!
//! The three suites were written independently and grew near-identical
//! copies of these builders — including two `peg_at`s whose argument
//! ORDERS disagreed, which is the kind of duplicate that reads fine in
//! each file and is a trap across them. One copy, one order.

#![allow(
    dead_code,
    unreachable_pub,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]

use crate::common::three_arc;
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::{extruded, sketch_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FaceKey, FacePairDeclaration,
    mass_properties,
};

/// An annulus of bore `r` and outer radius `outer`, `z ∈ [z0, z0 + len]`,
/// both rims three 120° arcs starting at `deg0` — the bore wall is 3
/// faces.
pub fn collar_of(r: f64, outer: f64, deg0: f64, z0: f64, len: f64) -> Body<f64> {
    let o = Point2::new(0.0, 0.0);
    extruded(
        sketch_at(z0),
        vec![three_arc(o, outer, deg0), three_arc(o, r, deg0)],
        len,
        Tol::witness(),
    )
}

/// The collar: [`collar_of`] with bore 0.5, outer 1.5, `z ∈ [1, 2]`.
pub fn collar_at(deg0: f64) -> Body<f64> {
    collar_of(0.5, 1.5, deg0, 1.0, 1.0)
}

pub fn collar() -> Body<f64> {
    collar_at(0.0)
}

/// A three-arc peg of radius 0.5 split at `deg0`, z ∈ [z0, z0 + h].
///
/// **Argument order is `(deg0, z0, h)`** — the azimuth first, then the
/// span. The two suites that grew their own copy disagreed about this.
pub fn peg_at(deg0: f64, z0: f64, h: f64) -> Body<f64> {
    peg_of(0.5, deg0, z0, h)
}

/// A three-arc peg of radius `r` split at `deg0`, `z ∈ [z0, z0 + h]`.
pub fn peg_of(r: f64, deg0: f64, z0: f64, h: f64) -> Body<f64> {
    extruded(
        sketch_at(z0),
        vec![three_arc(Point2::new(0.0, 0.0), r, deg0)],
        h,
        Tol::witness(),
    )
}

/// The rectangle `ρ ∈ [bore, outer]`, `y ∈ [y0, y1]` revolved a full
/// turn about `y`: its bore is ONE face with a self-mated seam ruling at
/// azimuth 0 (from `+x`).
pub fn full_turn_collar(bore: f64, outer: f64, (y0, y1): (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(bore, y0),
        Point2::new(outer, y0),
        Point2::new(outer, y1),
        Point2::new(bore, y1),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// `body` turned a quarter about `x`, taking its `z` axis to `y` and a
/// sketch azimuth `θ` (from `+x` toward `+y`) to the azimuth `θ` from
/// `+x` toward `−z`, which is the revolve's own sense about `+y`: a peg
/// set on [`full_turn_collar`]'s axis.
pub fn onto_y(body: &Body<f64>) -> Body<f64> {
    let up = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -core::f64::consts::FRAC_PI_2,
    );
    topo::transform_rigid(body, &up, Tol::witness()).unwrap()
}

pub fn peg(z0: f64, h: f64) -> Body<f64> {
    peg_at(0.0, z0, h)
}

/// The cylinder faces of `body` at radius ≈ `r`.
pub fn walls_at(body: &Body<f64>, r: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The sphere faces of `body` at radius ≈ `r`.
pub fn spheres_at(body: &Body<f64>, r: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Sphere { radius, .. }) if (radius - r).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// Every pair of `a_faces` × `b_faces` declared `Rest` into `decls`.
pub fn declare_rest(decls: &mut BooleanDeclarations, a_faces: &[FaceKey], b_faces: &[FaceKey]) {
    for &fa in a_faces {
        for &fb in b_faces {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
}

/// The planar face at height `z` facing `up`.
pub fn plane_face(body: &Body<f64>, z: f64, up: bool) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - z).abs() < 1e-12 && (normal.z > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
    };
    f
}

/// Every (bore wall × peg wall) pair declared `Rest` — the mate's only
/// contact unless a caller adds one — plus every continuation the two
/// parts have (a peg end flush with the collar's face), which a union
/// refuses undeclared.
pub fn wall_decls(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    wall_decls_at(a, b, 0.5)
}

/// [`wall_decls`] for walls of radius `r`.
pub fn wall_decls_at(a: &Body<f64>, b: &Body<f64>, r: f64) -> BooleanDeclarations {
    let mut decls = continuations(a, b);
    declare_rest(&mut decls, &walls_at(a, r), &walls_at(b, r));
    decls
}

/// Every continuation the flush detector finds between `a` and `b`
/// (one carrier, aligned senses), declared.
pub fn continuations(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let found = topo::flush::find_flush_candidates(a, b, Tol::witness())
        .expect("a fixture's flush pairs decide definitely");
    let picked: Vec<_> = found
        .into_iter()
        .filter(|f| f.class == topo::BooleanCoincidence::Continuation)
        .collect();
    topo::flush::declare_all(&picked)
}

pub fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

pub fn boolean_body(r: BooleanResult<f64>) -> topo::BooleanBody<f64> {
    match r {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("a threaded mate cannot be empty"),
    }
}

pub fn body_of(r: BooleanResult<f64>) -> Body<f64> {
    boolean_body(r).body
}

/// The additivity distance in ULPs of the sum — the raw number both
/// probe suites report and [`assert_additive`] bounds.
pub fn additivity_ulps(v: f64, sum: f64) -> f64 {
    if v == sum {
        return 0.0;
    }
    (v - sum).abs() / (f64::EPSILON * sum.abs())
}

/// Exact additivity, to the arithmetic these volumes are computed in:
/// the operand interiors are disjoint, so the union's volume is their
/// sum. The comparison is relative rather than bitwise because both
/// sides carry irrational (π) terms that no rearrangement cancels —
/// fixture (i)'s bitwise oracle exists only because its peg and bore
/// π-terms cancel against an integer.
///
/// **The bound is EMPIRICAL and says so.** There is no derivation
/// available for it: `v`, `vp` and `vq` are three independent
/// divergence-theorem summations over three different face sets, and a
/// per-face error budget for those sums is not something a fixture can
/// honestly bound from the outside. So the number is a MEASUREMENT with
/// headroom, and the measurement lives in the tree rather than in this
/// comment — `mate2_r1_probes::probe_reports_actual_additivity_ulps`
/// and `mate2_r2_probes::r2_measure_additivity_ulp_gap` print the
/// distance every run, so the headroom is visible instead of asserted.
/// Measured: 1.528 ULP (partial engagement) and 1.132 ULP (full). The
/// bound is 4 — the next power of two above twice the worst case, which
/// keeps the assert meaningful (it is not a decade of slack) while not
/// tracking a number nobody derived. It was 8 and is tightened here
/// because the probes made the real distance visible; if a future
/// configuration exceeds 4, the answer is to read WHY that
/// configuration sums differently, not to raise the bound again.
pub fn assert_additive(v: f64, vp: f64, vq: f64) {
    let sum = vp + vq;
    let ulps = additivity_ulps(v, sum);
    assert!(
        ulps <= 4.0,
        "exactly additive to 4 ULP: {v} vs {vp} + {vq} = {sum} ({ulps:.3} ULP)"
    );
}
