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

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A circle at the origin as three 120° arcs, first joint at `deg0`.
pub fn three_arc(radius: f64, deg0: f64) -> ProfileLoop<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(radius * th.cos(), radius * th.sin())
    };
    ProfileLoop::new(vec![
        ProfileVertex::new(at(deg0), b120),
        ProfileVertex::new(at(deg0 + 120.0), b120),
        ProfileVertex::new(at(deg0 + 240.0), b120),
    ])
}

pub fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, loops).validate(Tol::witness()).unwrap();
    sweep::extrude(&profile, sweep::Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The collar: an annulus (outer r = 1.5, bore r = 0.5), z ∈ [1, 2],
/// both rims three 120° arcs starting at `deg0` — bore wall is 3 faces.
pub fn collar_at(deg0: f64) -> Body<f64> {
    extruded(vec![three_arc(1.5, deg0), three_arc(0.5, deg0)], 1.0, 1.0)
}

pub fn collar() -> Body<f64> {
    collar_at(0.0)
}

/// A three-arc peg of radius 0.5 split at `deg0`, z ∈ [z0, z0 + h].
///
/// **Argument order is `(deg0, z0, h)`** — the azimuth first, then the
/// span. The two suites that grew their own copy disagreed about this.
pub fn peg_at(deg0: f64, z0: f64, h: f64) -> Body<f64> {
    extruded(vec![three_arc(0.5, deg0)], z0, h)
}

pub fn peg(z0: f64, h: f64) -> Body<f64> {
    peg_at(0.0, z0, h)
}

/// The cylinder faces of `body` at radius ≈ `r`.
pub fn walls_at(body: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
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
/// contact unless a caller adds one.
pub fn wall_decls(a: &Body<f64>, b: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(a, 0.5) {
        for &fb in &walls_at(b, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    decls
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
