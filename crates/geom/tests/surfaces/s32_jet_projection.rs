//! [`Surface::jet`] is the enum's derivative primitive, and the **five**
//! single-partial accessors plus [`Surface::normal`] are its
//! projections. That is a **shared contract**: one implementation now
//! answers what six doors once answered independently, so the guard
//! obligation of all six sits here.
//!
//! Three obligations:
//!
//! 1. **No fork.** Every accessor must be bit-identical to the jet
//!    field it names. Today that holds by construction — each accessor
//!    body *is* `self.jet(u, v).d…` — so these rows are tautological as
//!    shipped and are stated that way deliberately: they become
//!    load-bearing the moment anyone gives an accessor a body of its
//!    own again, which is exactly the regression this file exists to
//!    catch. Same shape and same reason as
//!    `ders3_agrees_with_ders_bit_for_bit_on_the_shared_block`.
//! 2. **`normal` is `du × dv` normalized**, bit-identical to the same
//!    two fields taken off one jet. Note the exact scope: because the
//!    accessors are projections, two evaluations and one produce the
//!    same bits, so this row pins the **value** and cannot see the
//!    **evaluation count**. No bitwise row can: the count is invisible
//!    to every assertion in this file. It is not, however, invisible to
//!    *every* instrument — `NurbsSurface::ders` allocates, so a
//!    `#[global_allocator]` counter does witness it (measured: `normal`
//!    40 allocations here, 80 with the two-evaluation body restored).
//!    That test is deliberately not added: a global allocator is
//!    per-test-binary and this crate aggregates every suite into one
//!    binary (`autotests = false`, `[[test]] name = "all"`), so pinning
//!    the count costs a second binary. The claim is therefore
//!    "unguarded *here*, and guardable only at that price", not
//!    "unguardable".
//! 3. **`eval` and the jet's point agree.** `eval` is deliberately not
//!    a projection — that is a source fact no bitwise row can witness,
//!    and it is not claimed here. What is pinned is agreement: for
//!    [`Surface::Nurbs`] it runs a dedicated pass (`NurbsSurface::eval`,
//!    order-0 basis — cheaper, and measured so: 2 allocations against
//!    the jet's 40) while the jet's point comes out of
//!    `NurbsSurface::ders`' order-2 tensor pass. What the two owe each
//!    other is agreement to round-off; what they in fact deliver is bit
//!    identity, which is what the row asserts — a future divergence
//!    inside round-off is a legitimate relaxation of this row, not a
//!    regression to chase.
//!
//! Poison is in the corpus, not assumed: the placeholder NURBS payload
//! and two degenerate analytic charts make most fields NaN, and every
//! comparison here is on raw bits, so a NaN must match a NaN *payload*.
//!
//! **One caveat on comparing NaNs by raw bits.** A quiet NaN's *sign*
//! bit is unspecified by IEEE for every arithmetic operation, and it is
//! not a stable function of this source: an adversarial dump found
//! LLVM sinking an `fneg` into an operand and flipping quiet-NaN signs
//! for expressions whose source had not changed at all. Rows 1 and 2
//! are safe from that by construction — each accessor's body *is* the
//! jet field, one computation compared with itself. Row 3 is the
//! exposed one, because it compares two separately compiled copies of
//! one expression; its `Surface::Nurbs` arm is already NaN-sign-
//! agnostic, and if a compiler change ever reddens its analytic arm
//! with a pure sign flip on a NaN, the correct repair is to widen that
//! arm the same way, not to hunt for a numerical regression.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// A bicubic × biquadratic rational patch with irregular weights — the
/// arm where a jet field is genuinely computed rather than read off a
/// constant.
fn rational_patch() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let (nu, nv) = (5usize, 4usize);
    let mut control = Vec::with_capacity(nu * nv);
    let mut weights = Vec::with_capacity(nu * nv);
    for iu in 0..nu {
        for iv in 0..nv {
            let x = iu as f64 * 0.75;
            let y = iv as f64 * 0.6;
            let z = 0.3 * x * x - 0.2 * y * y + 0.45 * x * y + 0.1 * x * x * y;
            control.push(Point3::new(x, y, z));
            weights.push(0.6 + 0.35 * ((iu * 3 + iv * 5) % 7) as f64);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

// These three are the workspace's stock test frame, reproduced here
// rather than shared: the same `axis`/`u_ref`/`origin` triple appears
// in eleven files across `geom` and `geom-brep`, of which this is the
// eleventh. Filed as a class in §D rather than swept from here — the
// sweep is not this unit's to make.
fn axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}
fn u_ref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}
fn origin() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

/// Every variant, plus the placeholder and two degenerate charts whose
/// fields poison.
fn corpus() -> Vec<(&'static str, Surface<f64>)> {
    vec![
        (
            "plane",
            Surface::Plane {
                origin: origin(),
                normal: axis(),
                u_ref: u_ref(),
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: origin(),
                axis: axis(),
                radius: 2.5,
                u_ref: u_ref(),
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: origin(),
                axis: axis(),
                half_angle: 0.4,
                u_ref: u_ref(),
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: origin(),
                radius: 2.5,
                axis: axis(),
                u_ref: u_ref(),
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: origin(),
                axis: axis(),
                major_radius: 3.0,
                minor_radius: 0.75,
                u_ref: u_ref(),
            },
        ),
        ("nurbs", Surface::Nurbs(Arc::new(rational_patch()))),
        ("nurbs_placeholder", Surface::nurbs_placeholder()),
        (
            "plane_degenerate",
            Surface::Plane {
                origin: origin(),
                normal: axis(),
                u_ref: Vec3::zero(),
            },
        ),
        (
            "sphere_r0",
            Surface::Sphere {
                center: origin(),
                radius: 0.0,
                axis: axis(),
                u_ref: u_ref(),
            },
        ),
    ]
}

/// Span-cell interiors, the clamped ends, the two named chart
/// singularities (cone apex `v = 0`, sphere poles `v = ±π/2`), and the
/// subnormal/huge extremes.
fn params() -> Vec<(f64, f64)> {
    let pi = core::f64::consts::PI;
    let mut ps = Vec::new();
    for i in 0..13 {
        for j in 0..13 {
            ps.push((i as f64 / 12.0, j as f64 / 12.0));
        }
    }
    ps.extend_from_slice(&[
        (0.0, 0.0),
        (0.3, 0.0),
        (0.3, 1e-300),
        (0.3, 1e-160),
        (0.7, pi / 2.0),
        (0.7, -pi / 2.0),
        (-2.0, -1.5),
        (2.5, 3.25),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        (1e15, -1e15),
    ]);
    ps
}

fn bits(v: Vec3<f64>) -> (u64, u64, u64) {
    (v.x.to_bits(), v.y.to_bits(), v.z.to_bits())
}
fn pbits(p: Point3<f64>) -> (u64, u64, u64) {
    (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
}

#[test]
fn every_accessor_is_bit_identical_to_the_jet_field_it_names() {
    for (name, s) in corpus() {
        for (u, v) in params() {
            let j = s.jet(u, v);
            for (field, accessor, from_jet, from_accessor) in [
                ("du", "deriv_u", j.du, s.deriv_u(u, v)),
                ("dv", "deriv_v", j.dv, s.deriv_v(u, v)),
                ("duu", "deriv_uu", j.duu, s.deriv_uu(u, v)),
                ("duv", "deriv_uv", j.duv, s.deriv_uv(u, v)),
                ("dvv", "deriv_vv", j.dvv, s.deriv_vv(u, v)),
            ] {
                assert_eq!(
                    bits(from_jet),
                    bits(from_accessor),
                    "{name}: jet.{field} forked from {accessor} at ({u}, {v})"
                );
            }
        }
    }
}

#[test]
fn normal_is_du_cross_dv_normalized() {
    for (name, s) in corpus() {
        for (u, v) in params() {
            let j = s.jet(u, v);
            assert_eq!(
                bits(s.normal(u, v)),
                bits(j.du.cross(j.dv).normalize()),
                "{name}: normal is not du × dv normalized at ({u}, {v})"
            );
        }
    }
}

#[test]
fn eval_agrees_bitwise_with_the_jets_point() {
    for (name, s) in corpus() {
        let analytic = !matches!(s, Surface::Nurbs(_));
        for (u, v) in params() {
            let (e, p) = (pbits(s.eval(u, v)), pbits(s.jet(u, v).point));
            if analytic {
                assert_eq!(
                    e, p,
                    "{name}: eval and jet.point forked at ({u}, {v}) — the \
                     analytic arms hold two verbatim copies of one point \
                     expression, and this row is what keeps them equal"
                );
            } else {
                // Two certified passes over the same patch, NOT one
                // expression: `eval` accumulates the order-0 basis
                // directly, the jet reads A₀₀/w₀₀ out of the order-2
                // tensor pass. They agree bitwise here (measured, not
                // owed — see this file's header).
                let nan = |a: u64| f64::from_bits(a).is_nan();
                let same = |a: u64, b: u64| a == b || (nan(a) && nan(b));
                assert!(
                    same(e.0, p.0) && same(e.1, p.1) && same(e.2, p.2),
                    "{name}: eval {e:?} and jet.point {p:?} disagree at ({u}, {v})"
                );
            }
        }
    }
}
