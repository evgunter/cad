//! Adversarial e2e review artifact for M2 PR 1 — the interval /
//! `Dual<Interval>` lanes (2026-07-17), promoted from the reviewer's
//! session scratchpad into CI per the standing convention. These are
//! **independent derivations** — the hand-derived mean-value criterion
//! for the floor jump enclosure with a 20k-box empirical attack,
//! containment sweeps, decoration-chain probes, the tier-3 certified
//! residual dry run — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! Runs in CI's interval lane (`--features interval`; the workspace's
//! x86-64-v3 floor applies). Promotion adaptations (mechanical only):
//! the standard test-lint allows below; everything else is verbatim.
//! Placement rationale: see `review_m2_pr1.rs`.
#![cfg(feature = "interval")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]
// The range-shaped assertions are the reviewer's derivation shapes —
// kept verbatim per the do-not-simplify rule.
#![allow(clippy::manual_range_contains)]

use core::f64::consts::{FRAC_PI_6, TAU};

use geom::Curve3;
use geom::Surface;
use geom_core::{Bounds, Dual, DualInterval, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

fn iv(lo: f64, hi: f64) -> Interval {
    Interval::from_bounds(lo, hi)
}

fn contains(e: Interval, x: f64) -> bool {
    e.lo() <= x && x <= e.hi()
}

// ---------------------------------------------------------------------
// 1(a). The floor mean-value enclosure, falsified empirically
// ---------------------------------------------------------------------

/// Hand-derived requirement: for the derivative-enclosure J of floor
/// over a box B to be mean-value sound, floor(b) − floor(a) ∈ J·(b − a)
/// for ALL a, b ∈ B. Across an integer step the quotients are ≥ 0 and
/// unbounded, so J must be ⊇ [0, huge] — [0, +∞] is the only honest
/// finite-description enclosure; on a one-plateau box (including one
/// whose ENDPOINT is an integer k with the box in [k, k+1)) floor is
/// constant so J = [0, 0] is exact. This test hammers both claims
/// through the public Dual<Interval> surface (tangent seeded [1, 1]).
#[test]
fn floor_mean_value_enclosure_empirical() {
    let mut rng = fuzz::start("review_m2_pr1_interval::floor_mean_value");
    for _ in 0..fuzz::scaled(2_500) {
        let lo = (rng.unit() - 0.5) * 40.0;
        let wid = rng.unit() * 3.0;
        let hi = lo + wid;
        let bx = iv(lo, hi);
        let d = Dual::new(bx, Interval::from_f64(1.0)).floor();
        let j = d.deriv;
        // Sample point pairs inside the box.
        for _ in 0..fuzz::scaled(1) {
            let a = lo + rng.unit() * wid;
            let b = lo + rng.unit() * wid;
            let fa = <f64 as Real>::floor(a);
            let fb = <f64 as Real>::floor(b);
            // f(b) − f(a) must lie in J·(b − a), evaluated exactly:
            // J = [0,0] ⇒ difference must be 0; J = [0,∞] ⇒ the
            // difference and (b − a) must not have strictly opposite
            // signs (quotient ≥ 0).
            if j.hi() == 0.0 {
                assert_eq!(
                    fa,
                    fb,
                    "J = [0,0] on [{lo}, {hi}] but floor moves: {a} → {b} — {}",
                    fuzz::replay()
                );
            } else {
                assert!(
                    j.lo() == 0.0 && j.hi() == f64::INFINITY,
                    "unexpected J = [{}, {}] — {}",
                    j.lo(),
                    j.hi(),
                    fuzz::replay()
                );
                assert!(
                    (fb - fa) * (b - a) >= 0.0,
                    "quotient sign violates J ≥ 0 — {}",
                    fuzz::replay()
                );
            }
        }
        // The value channel must equal the endpoint-floor hull.
        assert_eq!(
            d.value.lo(),
            <f64 as Real>::floor(lo),
            "value channel lo != floor(lo) — {}",
            fuzz::replay()
        );
        assert_eq!(
            d.value.hi(),
            <f64 as Real>::floor(hi),
            "value channel hi != floor(hi) — {}",
            fuzz::replay()
        );
    }
}

/// Endpoint-touching boxes, exhaustively around several integers: the
/// [0,0]-vs-[0,∞] classification is exactly "does the box's floor image
/// span two plateaus".
#[test]
fn floor_jacobian_endpoint_touching_integers() {
    for k in [-3.0f64, 0.0, 1.0, 7.0] {
        let up = f64::next_up(k);
        let dn = f64::next_down(k);
        let cases: [(Interval, bool); 6] = [
            (iv(k, k), true),        // point AT the integer: plateau
            (iv(k, k + 0.9), true),  // [k, k+0.9): one plateau
            (iv(dn, k), false),      // touches k from below: jump
            (iv(k - 0.5, k), false), // jump
            (iv(up, k + 0.5), true), // strictly inside (k, k+1)
            (iv(dn, up), false),     // ulp-straddle: jump
        ];
        for (bx, plateau) in cases {
            let d = Dual::new(bx, Interval::from_f64(1.0)).floor();
            if plateau {
                assert_eq!(
                    (d.deriv.lo(), d.deriv.hi()),
                    (0.0, 0.0),
                    "expected plateau J at k={k}, box=[{}, {}]",
                    bx.lo(),
                    bx.hi()
                );
            } else {
                assert_eq!(
                    (d.deriv.lo(), d.deriv.hi()),
                    (0.0, f64::INFINITY),
                    "expected jump J at k={k}, box=[{}, {}]",
                    bx.lo(),
                    bx.hi()
                );
            }
        }
    }
}

// ---------------------------------------------------------------------
// 1(c)/1(d). reduce_periodic at Interval / Dual<Interval>
// ---------------------------------------------------------------------

/// Containment across magnitudes/periods: the interval reduction of a
/// point enclosure contains the f64 reduction of the same input (exact
/// ops compose containment), including huge and negative θ.
#[test]
fn reduce_periodic_interval_contains_f64_everywhere() {
    for x in [
        -1e15, -12345.678, -1.5, -1e-9, 0.0, 0.3, 7.7, 1e6, 1e15, 1e18,
    ] {
        for p in [0.7, 1.0, TAU, 12.25, 1e-3] {
            let rf = <f64 as Real>::reduce_periodic(x, p);
            let ri = Interval::from_f64(x).reduce_periodic(Interval::from_f64(p));
            assert!(
                contains(ri, rf),
                "x={x}, p={p}: f64 {rf} not in [{}, {}]",
                ri.lo(),
                ri.hi()
            );
            // And the TRUE reduced value is contained: check via the
            // defining identity x − r = k·p with k integer, using the
            // enclosure: (x − ri)/p must contain an integer.
            let k = (Interval::from_f64(x) - ri) / Interval::from_f64(p);
            let any_integer_inside = k.lo().ceil() <= k.hi();
            assert!(
                any_integer_inside,
                "x={x}, p={p}: no integer k in [{}, {}]",
                k.lo(),
                k.hi()
            );
        }
    }
}

/// Dual<Interval> value channel is BIT-identical (endpoint bits) to the
/// plain Interval run — the compositional-body cross-instantiation
/// claim at the certified tier.
#[test]
fn reduce_periodic_dual_interval_value_bit_identity() {
    for (lo, hi) in [(0.3, 0.3), (5.9, 6.4), (-8.0, -7.2), (1e9, 1e9 + 2.0)] {
        for p in [0.7, TAU] {
            let plain = iv(lo, hi).reduce_periodic(Interval::from_f64(p));
            let dual = Real::reduce_periodic(
                Dual::new(iv(lo, hi), Interval::from_f64(1.0)),
                Dual::constant(Interval::from_f64(p)),
            );
            assert_eq!(plain.lo().to_bits(), dual.value.lo().to_bits());
            assert_eq!(plain.hi().to_bits(), dual.value.hi().to_bits());
        }
    }
}

/// A seam-straddling box reduces with the honest widening and the
/// derivative enclosure [−∞, 1]-ish (1 − p·[0,∞]/p): the mean-value
/// form of the seam jump.
#[test]
fn reduce_periodic_seam_box_derivative() {
    let theta = iv(6.2, 6.4); // straddles 2π
    let d = Real::reduce_periodic(
        Dual::new(theta, Interval::from_f64(1.0)),
        Dual::constant(Interval::tau()),
    );
    // Value: contains both 6.2 and 6.4 − 2π ≈ 0.117.
    assert!(contains(d.value, 6.2) && contains(d.value, 6.4 - TAU));
    // Tangent: must contain 1 (the a.e. slope) and be unbounded below
    // (the seam jump has slope −∞).
    assert!(contains(d.deriv, 1.0));
    assert_eq!(d.deriv.lo(), f64::NEG_INFINITY);
}

// ---------------------------------------------------------------------
// 2. copysign at Interval: containment sweep incl. signed zeros
// ---------------------------------------------------------------------

/// Point-enclosure copysign contains the f64 result for every sign
/// class INCLUDING both signed zeros in either slot; and the
/// zero-straddling hull is the tightest interval containing the true
/// image {±|x| : x ∈ X} (i.e. exactly [−sup|X|, sup|X|]).
#[test]
fn copysign_interval_containment_and_tightness() {
    let xs = [-3.5, -1.0, -0.0, 0.0, 0.25, 2.0];
    let ss = [-2.0, -0.0, 0.0, 1.5];
    for x in xs {
        for s in ss {
            let rf = <f64 as Real>::copysign(x, s);
            let ri = Interval::from_f64(x).copysign(Interval::from_f64(s));
            assert!(
                contains(ri, rf),
                "copysign({x}, {s}) = {rf} not in [{}, {}]",
                ri.lo(),
                ri.hi()
            );
        }
    }
    // Tightness of the straddle hull: for X = [2, 3], S ∋ 0 the true
    // image is {−3..−2} ∪ {2..3}; the tightest INTERVAL is [−3, 3].
    let hull = iv(2.0, 3.0).copysign(iv(-1.0, 1.0));
    assert_eq!((hull.lo(), hull.hi()), (-3.0, 3.0));
    // Sign-definite tightness: exactly ±|X|.
    let pos = iv(-3.0, -2.0).copysign(iv(0.5, 1.0));
    assert_eq!((pos.lo(), pos.hi()), (2.0, 3.0));
}

// ---------------------------------------------------------------------
// 3. The basis at Interval: containment of the f64 frames
// ---------------------------------------------------------------------

/// Point-enclosure basis contains the f64 basis componentwise (the
/// division makes exact bit-identity impossible; containment with the
/// outward-rounded enclosure is the certified-tier statement).
#[test]
fn basis_interval_contains_f64() {
    for n in [
        Vec3::new(0.6, 0.8, 1e-12),
        Vec3::new(0.6, 0.8, -1e-12),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
    ] {
        let (b1, b2) = n.orthonormal_basis();
        let ni = Vec3::new(
            Interval::from_f64(n.x),
            Interval::from_f64(n.y),
            Interval::from_f64(n.z),
        );
        let (i1, i2) = ni.orthonormal_basis();
        for (f, e) in [
            (b1.x, i1.x),
            (b1.y, i1.y),
            (b1.z, i1.z),
            (b2.x, i2.x),
            (b2.y, i2.y),
            (b2.z, i2.z),
        ] {
            // f64 result within the enclosure inflated by 1 ulp (the
            // f64 run rounds to nearest; the enclosure brackets the
            // true value, and every op here is within 1 ulp of true).
            let pad = 2.0 * f64::EPSILON * (1.0 + f.abs());
            assert!(
                e.lo() - pad <= f && f <= e.hi() + pad,
                "n = {n:?}: {f} vs [{}, {}]",
                e.lo(),
                e.hi()
            );
        }
    }
}

// ---------------------------------------------------------------------
// 5. Evaluators: enclosure containment sweeps + decoration chains
// ---------------------------------------------------------------------

fn lift_surface(s: &Surface<f64>) -> Surface<Interval> {
    let ip = |p: Point3<f64>| {
        Point3::new(
            Interval::from_f64(p.x),
            Interval::from_f64(p.y),
            Interval::from_f64(p.z),
        )
    };
    let ivc = |v: Vec3<f64>| {
        Vec3::new(
            Interval::from_f64(v.x),
            Interval::from_f64(v.y),
            Interval::from_f64(v.z),
        )
    };
    match *s {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Surface::Plane {
            origin: ip(origin),
            normal: ivc(normal),
            u_ref: ivc(u_ref),
        },
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: ip(origin),
            axis: ivc(axis),
            radius: Interval::from_f64(radius),
            u_ref: ivc(u_ref),
        },
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => Surface::Cone {
            apex: ip(apex),
            axis: ivc(axis),
            half_angle: Interval::from_f64(half_angle),
            u_ref: ivc(u_ref),
        },
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => Surface::Sphere {
            center: ip(center),
            radius: Interval::from_f64(radius),
            axis: ivc(axis),
            u_ref: ivc(u_ref),
        },
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => Surface::Torus {
            center: ip(center),
            axis: ivc(axis),
            major_radius: Interval::from_f64(major_radius),
            minor_radius: Interval::from_f64(minor_radius),
            u_ref: ivc(u_ref),
        },
        // The corpus below is analytic; neither spline kind has an
        // interval fixture to lift.
        Surface::Nurbs(_) | Surface::Approx(_) => Surface::nurbs_placeholder(),
    }
}

fn surfaces() -> Vec<(&'static str, Surface<f64>)> {
    let c = Point3::new(-0.5, 4.0, 1.25);
    let axis = Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0);
    let uref = Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    vec![
        (
            "plane",
            Surface::Plane {
                origin: c,
                normal: axis,
                u_ref: uref,
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: c,
                axis,
                radius: 2.5,
                u_ref: uref,
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: c,
                axis,
                half_angle: FRAC_PI_6,
                u_ref: uref,
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: c,
                radius: 2.5,
                axis,
                u_ref: uref,
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: c,
                axis,
                major_radius: 3.0,
                minor_radius: 1.25,
                u_ref: uref,
            },
        ),
    ]
}

/// Wide-box inclusion at every evaluator of every variant: the image of
/// a (u, v) box must contain the f64 images of sampled inner points, up
/// to the libm-vs-true 1-ulp pad. Boxes deliberately straddle the seam
/// and the poles.
#[test]
fn wide_box_images_contain_sampled_f64_images() {
    let pad = 1e-12; // generous: libm ≤ 1 ulp of true on O(10) values
    let boxes = [
        (iv(-0.5, 0.5), iv(0.2, 0.4)), // seam-straddling u
        (iv(3.0, 3.2), iv(-1.5, 1.5)), // pole-straddling v (sphere)
        (iv(6.0, 6.6), iv(0.0, 0.1)),  // 2π-straddling u
    ];
    for (name, s) in surfaces() {
        let si = lift_surface(&s);
        for (ub, vb) in boxes {
            let img = si.eval(ub, vb);
            let du = si.deriv_u(ub, vb);
            let dvv = si.deriv_vv(ub, vb);
            for i in 0..=4 {
                for j in 0..=4 {
                    let uu = ub.lo() + (ub.hi() - ub.lo()) * f64::from(i) / 4.0;
                    let vv = vb.lo() + (vb.hi() - vb.lo()) * f64::from(j) / 4.0;
                    let q = s.eval(uu, vv);
                    for (e, f) in [(img.x, q.x), (img.y, q.y), (img.z, q.z)] {
                        assert!(
                            e.lo() - pad <= f && f <= e.hi() + pad,
                            "{name} eval ({uu}, {vv}): {f} vs [{}, {}]",
                            e.lo(),
                            e.hi()
                        );
                    }
                    let qd = s.deriv_u(uu, vv);
                    for (e, f) in [(du.x, qd.x), (du.y, qd.y), (du.z, qd.z)] {
                        assert!(e.lo() - pad <= f && f <= e.hi() + pad, "{name} ∂u");
                    }
                    let qvv = s.deriv_vv(uu, vv);
                    for (e, f) in [(dvv.x, qvv.x), (dvv.y, qvv.y), (dvv.z, qvv.z)] {
                        assert!(e.lo() - pad <= f && f <= e.hi() + pad, "{name} ∂vv");
                    }
                }
            }
        }
    }
}

/// PR 3 dry run at the certified tier: residual certification of a
/// correct cache ENCLOSES 0; a wrong cache's residual enclosure
/// EXCLUDES 0 (so the trilean gate escalates/fails, never guesses).
#[test]
fn tier3_interval_certification() {
    let mk = |cx: f64| Curve3::Circle {
        center: Point3::new(
            Interval::from_f64(cx),
            Interval::from_f64(1.0),
            Interval::from_f64(3.0),
        ),
        axis: Vec3::new(Interval::zero(), Interval::zero(), Interval::from_f64(1.0)),
        radius: Interval::from_f64(2.0),
        u_ref: Vec3::new(Interval::from_f64(1.0), Interval::zero(), Interval::zero()),
    };
    let cyl_center = Point3::new(
        Interval::from_f64(1.0),
        Interval::from_f64(1.0),
        Interval::from_f64(3.0),
    );
    let good = mk(1.0);
    let bad = mk(1.0 + 1e-6);
    for t in [0.0, 0.9, 2.5, 4.0, 6.0] {
        let p = good.eval(Interval::from_f64(t));
        let res = (p - cyl_center).norm_squared() - Interval::from_f64(4.0);
        assert!(contains(res, 0.0), "good cache must certify at t={t}");
        assert!(res.hi() - res.lo() < 1e-13, "and tightly");
    }
    let mut excluded = false;
    for t in [0.0, 0.9, 2.5, 4.0, 6.0] {
        let p = bad.eval(Interval::from_f64(t));
        let res = (p - cyl_center).norm_squared() - Interval::from_f64(4.0);
        if !contains(res, 0.0) {
            excluded = true;
        }
    }
    assert!(excluded, "wrong cache's residual must exclude 0 somewhere");
}

/// Decoration chains: a Def-decorated floor/reduce_periodic result
/// still CLASSIFIES downstream (Decide's poison threshold is < Def) —
/// the whole chain θ → reduce → sin_cos → residual stays classifiable;
/// an empty input stays poison through the same chain.
#[test]
fn decoration_chain_through_reduce_and_eval() {
    let ci = match lift_surface(&surfaces()[1].1) {
        s @ Surface::Cylinder { .. } => s,
        _ => unreachable!(),
    };
    // Seam-straddling reduce: Def decoration, then evaluate — the
    // result must still be a usable (non-poison) enclosure.
    let reduced = iv(6.2, 6.4).reduce_periodic(Interval::tau());
    let p = ci.eval(reduced, Interval::from_f64(0.3));
    assert!(
        !p.x.lo().is_nan(),
        "Def-decorated parameter must evaluate, not poison"
    );
    // Residual still certifies (contains 0) over the widened enclosure.
    let (origin, axis, r) = match ci {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (origin, axis, radius),
        _ => unreachable!(),
    };
    let d = p - origin;
    let rho2 = d.norm_squared() - d.dot(axis) * d.dot(axis);
    let res = rho2 - r * r;
    assert!(contains(res, 0.0));
    // Poison stays poison through the same chain.
    let empty = iv(-4.0, -1.0).sqrt(); // empty enclosure
    let p = ci.eval(
        empty.reduce_periodic(Interval::tau()),
        Interval::from_f64(0.3),
    );
    assert!(p.x.lo().is_nan(), "empty parameter must stay poison");
}

/// Dual<Interval> through a REAL consumer chain: derivative of the
/// cylinder's seam-reduced azimuth evaluation — value channel
/// bit-identical to the plain interval run, tangent honest across the
/// seam (unbounded where the reduction jumps).
#[test]
fn dual_interval_consumer_chain() {
    let ci = match lift_surface(&surfaces()[1].1) {
        s @ Surface::Cylinder { .. } => s,
        _ => unreachable!(),
    };
    let cd: Surface<DualInterval> = match &ci {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => Surface::Cylinder {
            origin: Point3::new(
                Dual::constant(origin.x),
                Dual::constant(origin.y),
                Dual::constant(origin.z),
            ),
            axis: Vec3::new(
                Dual::constant(axis.x),
                Dual::constant(axis.y),
                Dual::constant(axis.z),
            ),
            radius: Dual::constant(*radius),
            u_ref: Vec3::new(
                Dual::constant(u_ref.x),
                Dual::constant(u_ref.y),
                Dual::constant(u_ref.z),
            ),
        },
        _ => unreachable!(),
    };
    // Away from the seam: reduce then evaluate; tangent finite and the
    // value channel identical to the plain run.
    let theta = Dual::new(Interval::from_f64(7.0), Interval::from_f64(1.0));
    let reduced = Real::reduce_periodic(theta, Dual::constant(Interval::tau()));
    let p = cd.eval(reduced, Dual::constant(Interval::from_f64(0.5)));
    let plain = ci.eval(
        Interval::from_f64(7.0).reduce_periodic(Interval::tau()),
        Interval::from_f64(0.5),
    );
    assert_eq!(p.x.value.lo().to_bits(), plain.x.lo().to_bits());
    assert_eq!(p.x.value.hi().to_bits(), plain.x.hi().to_bits());
    assert!(p.x.deriv.lo().is_finite() && p.x.deriv.hi().is_finite());
}
