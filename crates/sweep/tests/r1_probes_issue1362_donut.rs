//! R1 review probes for PR 1389's disclosed deviation 2: the donut
//! row's `signed_volume` → `signed_volume_lifted` repair.
//!
//! Three questions, each attacked by execution:
//!   1. Is the unlifted fan really a STRUCTURAL zero on this body in
//!      exact arithmetic, or a small genuine value drowned in noise?
//!   2. Is the lifted repair a real orientation oracle, or a new
//!      vacuity?
//!   3. Does the row's sign survive the premises its comment asserts
//!      but nothing enforces?

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Point3, Tol};
use profile::RawLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};
use topo::Body;

fn donut() -> sweep::Revolved<f64> {
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex::new(p2(1.0, 0.5), 1.0),
        profile::ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    let vp = validated(vec![lp]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap()
}

/// The unlifted divergence fan, re-derived with an EXPLICIT anchor so
/// the reviewer can vary it. Over ℝ every anchor gives the same value
/// (each boundary edge is shared by exactly two faces with opposite
/// orientation, so the fan set is a closed polyhedron).
fn fan_at(body: &Body<f64>, o: Point3<f64>) -> f64 {
    let mut six_v = 0.0;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                six_v += (p1 - o).dot((pts[i] - o).cross(pts[i + 1] - o));
            }
        }
    }
    six_v / 6.0
}

/// The same fan, PER FACE, so the claimed mirror cancellation can be
/// looked at directly instead of through its sum.
fn fan_per_face(body: &Body<f64>, o: Point3<f64>) -> Vec<(topo::FaceKey, f64)> {
    let mut out = Vec::new();
    for (fk, face) in body.faces() {
        let mut six_v = 0.0;
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                six_v += (p1 - o).dot((pts[i] - o).cross(pts[i + 1] - o));
            }
        }
        out.push((fk, six_v / 6.0));
    }
    out
}

/// **Q1. Structural zero, or a real value lost to cancellation?**
///
/// The fan is anchor-independent over ℝ. So evaluate it from many
/// widely-separated anchors: a genuine nonzero value reproduces at
/// every anchor; a structural zero returns cancellation noise whose
/// magnitude tracks the anchor's distance and whose sign wanders.
/// This is the discriminator the PR did not run — it measured two
/// anchors, one of which sits exactly on the body's symmetry plane
/// (where the mirror terms negate BITWISE and 0 is guaranteed by the
/// float arithmetic, not by the geometry).
#[test]
fn r1_the_donut_fan_is_a_structural_zero_not_a_lost_value() {
    let t = donut();
    let exact = 2.0 * core::f64::consts::PI.powi(2) * 1.5 * 0.25;
    println!("exact torus volume 2π²Rr² = {exact}");
    // The discriminator: over ℝ the fan is the same from every
    // anchor. A GENUINE value reproduces at every anchor until the
    // f64 noise floor (which grows like ε·d³) swamps it. A STRUCTURAL
    // zero has no value to reproduce, so what comes back IS the noise
    // floor and therefore grows like d³ from the first anchor on.
    let mut vals = Vec::new();
    for (o, d) in [
        (Point3::new(0.0, 0.5, 0.0), 0.0), // the body's own centre
        (Point3::origin(), 0.5),
        (Point3::new(1.0, 0.0, 0.0), 1.1),
        (Point3::new(-3.0, 7.0, 11.0), 13.6),
        (Point3::new(1e3, -1e3, 1e3), 1.7e3),
        (Point3::new(1e6, 1e6, -1e6), 1.7e6),
    ] {
        let v = fan_at(&t.body, o);
        println!("  anchor {o:?} (|d| ≈ {d:e}) -> fan {v:e}");
        vals.push((d, v));
    }
    // At the symmetry-plane anchor the mirror terms negate bitwise, so
    // the answer is an exact 0 — the PR's "loop-anchored 0".
    assert_eq!(vals[0].1, 0.0, "the body-centre anchor should give exact 0");
    // Nothing reproduces: the residue at 1.7e6 is ~1e17 times the
    // residue at 13.6, i.e. it is the noise floor and not a value.
    let (far, near) = (vals[5].1.abs(), vals[3].1.abs());
    println!(
        "far/near residue ratio {:e} (anchor ratio 1.2e5)",
        far / near
    );
    assert!(
        far / near > 1e9,
        "the residue did NOT scale with the anchor distance — it reproduces, \
         so the unlifted fan is measuring a real (tiny) value and the PR's \
         structural-zero argument would be wrong. near {near:e} far {far:e}"
    );
    // And no anchor within the body's own scale recovers anything that
    // could be mistaken for the volume.
    for (d, v) in vals.iter().take(4) {
        assert!(
            v.abs() < 1e-12 * exact,
            "anchor at {d:e} recovered a non-trivial fan value {v:e}"
        );
    }

    // The per-face halves, at an OFF-symmetry anchor where a bitwise
    // negation is not available for free.
    let per = fan_per_face(&t.body, Point3::new(-3.0, 7.0, 11.0));
    println!("per-face fans at an off-symmetry anchor: {per:?}");
    assert_eq!(per.len(), 2);
    let (a, b) = (per[0].1, per[1].1);
    assert!(
        (a + b).abs() < 1e-9 * a.abs().max(1.0),
        "the two half-torus fans did not cancel: {a:e} + {b:e}"
    );
    assert!(
        a.abs() > 1.0,
        "each half's own fan should be O(1) — if it is not, the halves \
         are not 'mirror images that cancel', they are each ~0"
    );
}

/// **Q2. Is the lifted repair a real orientation oracle?**
///
/// The delivered row measures +2.828 with one lift assignment and
/// −2.828 with the two swapped. That antisymmetry is exactly what a
/// VACUOUS oracle would also show, so it proves nothing on its own.
/// The real question is whether the sign tracks the BODY's winding.
/// Rows here:
///   (a) the value is stable under the choice of interior lift, as the
///       PR claims — swept over the whole off-seam minor circle;
///   (b) the value is anchor-independent (it must be: closed fan set);
///   (c) the magnitude is NOT the torus volume, so `> 0.0` is the only
///       thing this row may read.
#[test]
fn r1_the_lifted_donut_oracle_is_stable_and_is_only_a_sign() {
    let t = donut();
    let RevolvedKind::Full { .. } = &t.kind else {
        panic!("full revolve")
    };
    let (f0, f1) = (t.walls[0][0].unwrap(), t.walls[0][1].unwrap());
    let exact = 2.0 * core::f64::consts::PI.powi(2) * 1.5 * 0.25;

    // (a) sweep the lift azimuth around the torus, off the u = 0 seam.
    // Lower half of the minor circle for f0, upper for f1 — the same
    // half-assignment the delivered row hard-codes.
    let mut seen = Vec::new();
    for k in 1..8 {
        let az = core::f64::consts::PI * f64::from(k) / 8.0;
        let (s, c) = az.sin_cos();
        let lo = Point3::new(1.5 * s, 0.0, 1.5 * c);
        let hi = Point3::new(1.5 * s, 1.0, 1.5 * c);
        let v = signed_volume_lifted(&t.body, &[(f0, lo), (f1, hi)]);
        println!("lift azimuth {az:.4} -> lifted volume {v}");
        seen.push(v);
        assert!(v > 0.0, "lifted oracle went non-positive at azimuth {az}");
    }
    let (mn, mx) = (
        seen.iter().cloned().fold(f64::INFINITY, f64::min),
        seen.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );
    println!("lifted volume over 7 off-seam lift pairs: [{mn}, {mx}]");
    // (c) it is NOT the volume — record how far off, so the row's
    // "only the sign" restriction is a measured fact.
    println!(
        "delivered lift pair value vs exact torus volume: {} vs {exact}",
        signed_volume_lifted(
            &t.body,
            &[
                (f0, Point3::new(0.0, 0.0, 1.5)),
                (f1, Point3::new(0.0, 1.0, 1.5)),
            ]
        )
    );

    // (b) the delivered value, reproduced, and the swap.
    let v = signed_volume_lifted(
        &t.body,
        &[
            (f0, Point3::new(0.0, 0.0, 1.5)),
            (f1, Point3::new(0.0, 1.0, 1.5)),
        ],
    );
    let w = signed_volume_lifted(
        &t.body,
        &[
            (f0, Point3::new(0.0, 1.0, 1.5)),
            (f1, Point3::new(0.0, 0.0, 1.5)),
        ],
    );
    println!("delivered lifts {v}; swapped lifts {w}");
    assert!((v - 2.828).abs() < 5e-3, "PR's +2.828 not reproduced: {v}");
    assert!((w + 2.828).abs() < 5e-3, "PR's −2.828 not reproduced: {w}");
}

/// **Q2b. What the lifted donut oracle actually measures.**
///
/// `signed_volume_lifted`'s per-face fan wraps, so its contribution is
/// `(q − o) · Σᵢ (pᵢ − o) × (pᵢ₊₁ − o)` — LINEAR in the lift `q`. For
/// this body the two faces' boundary loops are the same cycle in
/// opposite senses, so the two contributions collapse to
/// `(q₀ − q₁) · A₀ / 6`: the answer depends on the lifts ONLY through
/// their difference, and not at all on their being interior surface
/// points (which `signed_volume_lifted`'s docs require and the row's
/// comment argues for at length).
///
/// Predictions, all checked: translating BOTH lifts by an arbitrary
/// vector — including far outside the body — changes nothing; and the
/// magnitude is exactly the two rims' OCTAGONAL projected area
/// (`2√2·2² − 2√2·1² = 6√2`) times the lift separation, `2√2` after
/// the `/6` — a number about the rim polygonalization, not the torus.
#[test]
fn r1_the_lifted_donut_value_depends_only_on_the_lift_difference() {
    let t = donut();
    let (f0, f1) = (t.walls[0][0].unwrap(), t.walls[0][1].unwrap());
    let base = signed_volume_lifted(
        &t.body,
        &[
            (f0, Point3::new(0.0, 0.0, 1.5)),
            (f1, Point3::new(0.0, 1.0, 1.5)),
        ],
    );
    for shift in [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -1.5),  // both lifts onto the AXIS
        Point3::new(50.0, 0.0, 0.0),  // both lifts far OUTSIDE the body
        Point3::new(0.0, 400.0, 7.0), // nowhere near the surface
    ] {
        let v = signed_volume_lifted(
            &t.body,
            &[
                (f0, Point3::new(shift.x, shift.y, 1.5 + shift.z)),
                (f1, Point3::new(shift.x, 1.0 + shift.y, 1.5 + shift.z)),
            ],
        );
        println!("both lifts shifted by {shift:?} -> {v}");
        assert!(
            (v - base).abs() < 1e-12 * base.abs(),
            "the lifted oracle DID depend on where the lifts sit, not \
             only on their difference: {v} vs {base}"
        );
    }
    // The magnitude is the rim octagons' area times the separation.
    let octagonal_annulus = 2.0 * 2f64.sqrt() * (4.0 - 1.0); // 2√2(R₂² − R₁²)
    println!(
        "delivered value {base} vs (octagonal annulus {octagonal_annulus}) × \
         (lift separation 1.0) / 3 = {}",
        octagonal_annulus / 3.0
    );
    assert!((base - octagonal_annulus / 3.0).abs() < 1e-12);
}

/// **Q4 (style): sibling vacuity sweep.** The donut's structural zero
/// was found because the anchor fix made it deterministic. Any OTHER
/// body whose `signed_volume` is a structural zero is passing
/// `> 0.0` on a coin flip. The signature: a fan that does not
/// reproduce across two far-apart anchors (over ℝ it must).
///
/// The bodies whose committed rows ALREADY use `signed_volume_lifted`
/// (ball, cone) are expected suspects — that is why they are lifted.
/// The finding would be a body still asserted with the UNLIFTED
/// oracle that shows the signature.
#[test]
fn r1_sweeps_the_revolve_fixtures_for_more_structural_zeros() {
    let rect = || {
        validated(vec![profile::ProfileLoop::polygon([
            p2(1.0, 0.0),
            p2(2.0, 0.0),
            p2(2.0, 1.0),
            p2(1.0, 1.0),
        ])])
    };
    let circle = || {
        validated(vec![profile::ProfileLoop::new(vec![
            profile::ProfileVertex::new(p2(1.0, 0.5), 1.0),
            profile::ProfileVertex::new(p2(2.0, 0.5), 1.0),
        ])])
    };
    let half_disc = || {
        validated(vec![profile::ProfileLoop::new(vec![
            profile::ProfileVertex::new(p2(0.0, -1.0), 1.0),
            profile::ProfileVertex::new(p2(0.0, 1.0), 0.0),
        ])])
    };
    let triangle = || {
        validated(vec![profile::ProfileLoop::polygon([
            p2(0.0, 0.0),
            p2(1.0, 0.0),
            p2(0.0, 1.0),
        ])])
    };
    let quarter = Revolution::Partial(core::f64::consts::FRAC_PI_2);
    // (name, body, does its committed row use the LIFTED oracle?)
    let cases: Vec<(&str, sweep::Revolved<f64>, bool)> = vec![
        (
            "washer full",
            revolve(&rect(), axis_y(), Revolution::Full, Tol::witness()).unwrap(),
            false,
        ),
        (
            "washer quarter",
            revolve(&rect(), axis_y(), quarter, Tol::witness()).unwrap(),
            false,
        ),
        ("donut full", donut(), true),
        (
            "donut quarter",
            revolve(&circle(), axis_y(), quarter, Tol::witness()).unwrap(),
            false,
        ),
        (
            "ball full",
            revolve(&half_disc(), axis_y(), Revolution::Full, Tol::witness()).unwrap(),
            true,
        ),
        (
            "ball quarter",
            revolve(&half_disc(), axis_y(), quarter, Tol::witness()).unwrap(),
            false,
        ),
        (
            "cone full",
            revolve(&triangle(), axis_y(), Revolution::Full, Tol::witness()).unwrap(),
            true,
        ),
        (
            "cone quarter",
            revolve(&triangle(), axis_y(), quarter, Tol::witness()).unwrap(),
            false,
        ),
    ];
    let mut unlifted_suspects = Vec::new();
    for (name, t, lifted) in &cases {
        let o = probe_anchor(&t.body);
        let near = fan_at(&t.body, o);
        // Body scale, for a relative floor: the bbox of its probes.
        let mut d2: f64 = 0.0;
        for (_, face) in t.body.faces() {
            for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
                for p in loop_probe_points(&t.body, lk) {
                    d2 = d2.max((p - o).dot(p - o));
                }
            }
        }
        let scale = d2.sqrt().powi(3).max(f64::MIN_POSITIVE);
        // A structural zero is a fan indistinguishable from f64 noise
        // on terms of the body's own size.
        let structural_zero = near.abs() < 1e-12 * scale;
        println!(
            "{name:16} lifted={lifted:5}  fan {near:13.6e}  body scale {scale:9.3e}  \
             structural zero {structural_zero}"
        );
        if structural_zero && !lifted {
            unlifted_suspects.push(*name);
        }
    }
    println!("UNLIFTED rows showing the structural-zero signature: {unlifted_suspects:?}");
    // FINDING, pinned: the class is wider than `signed_volume_lifted`'s
    // doc says ("faces whose boundary probes are COPLANAR — the
    // two-band sphere/cone patches"). The donut's halves are not
    // coplanar; they are MIRROR-PAIRED, a second mechanism, and PR
    // 1389 documented it only at the one call site it repaired. A
    // partially-revolved ball is a third instance. No committed row
    // asserts `signed_volume(..) > 0.0` on that body today, so this is
    // latent rather than live — but it is the sibling the fix pass
    // owes a look, not the one that happened to go red.
    assert_eq!(
        unlifted_suspects,
        vec!["ball quarter"],
        "the structural-zero census over the revolve fixtures moved"
    );
}
