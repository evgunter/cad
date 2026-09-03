//! VERBS-OFF-A r1 reviewer probes — adversarial consumer rows for
//! `geom_brep::offset_surface`, independent of the shipped
//! `offset_mint.rs` fixtures.
//!
//! What these rows push on, beyond the shipped suite:
//!
//! - **The cone apex band** (the dropped-refusal residue): for
//!   `|d|·cot α` larger than a sample's `v`, the extended-field
//!   pushforward lands on the minted cone's OTHER nappe — the
//!   meridian signed distance to the minted cone is `−v′·sin 2α`, not
//!   0. The door is complete-locus and owes nothing here, but the row
//!   pins the closed form of exactly what a windowed consumer would
//!   see, so it goes red if the door's semantics ever shift silently.
//! - **More realized-float regimes**: the sphere and torus floors at
//!   the 1e16 collapse scale (the shipped red plants only the
//!   cylinder); the `d = −(r + 0.5)` sign combination whose exact-real
//!   margin is NEGATIVE while the realized float is exactly 0.0; the
//!   torus RING margin metered on the realized minor at scale (refuse
//!   when the realized minor lands exactly on `major`, mint when it
//!   lands two ulps under).
//! - **Half-angle near its validity edges** (`α = 1e-4`,
//!   `α = π/2 − 1e-6`): the mint stays a cone and the
//!   defining-equation rows still certify, with scale-aware bounds
//!   (the slide is `~1e4·d` at the small edge).
//! - **Tiny and huge radii** defining-equation rows (`r = 1e-9`
//!   cylinder, `r = 1e12` sphere) at proportionate `d`.
//! - **Band edges that track the RUN tolerance**: rows derive their
//!   margins from `Tol::witness()` (ε and K), so the escalation and
//!   both definite sides are exercised honestly at
//!   `CAD_TOLERANCE_EPS ∈ {default, 1e-6, 1e-9, 1e-12}` rather than
//!   only at the default band.
//! - **Interval lane**: a WIDE half-angle enclosure's slid apex must
//!   contain both endpoint slides (containment that would go red if
//!   the enclosure degrades); the ring predicate escalating on a
//!   straddling `d` and refusing typed on a definite one; the
//!   realized-radius floor at scale at `T = Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6};

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::{OffsetError, offset_surface};
use geom_core::{Point3, Tol, Vec3};

fn zcyl(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    }
}

fn zcone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 2.0),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// Meridian signed distance to a cone (extended generator line — the
/// same spelling the shipped suite uses, restated here independently).
fn cone_dist(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64, q: Point3<f64>) -> f64 {
    let rel = q - apex;
    let h = rel.dot(axis);
    let rho = (rel - axis * h).norm();
    rho * alpha.cos() - h * alpha.sin()
}

/// The dropped cone refusal's residue, pinned as a closed form: when
/// the parameterization shift `v ↦ v + d·cot α` carries a sample
/// across `v = 0`, the extended-field pushforward is NOT at meridian
/// distance 0 from the minted cone — it sits at `−v′·sin 2α` (it
/// crossed the axis; the matched parameter is on the other nappe).
/// The complete-locus door owes nothing here; the row exists so the
/// exact size and shape of the windowed-consumer question (OFF-N /
/// shell face replacement) is pinned by a test, not only by prose.
#[test]
fn cone_apex_band_pushforward_crosses_nappes() {
    let alpha = FRAC_PI_6;
    let d = -0.9; // |d|·cot α ≈ 1.559
    let v = 0.8; // v′ = v + d·cot α ≈ −0.759 < 0: crossed the apex
    let base = zcone(alpha);
    let minted = offset_surface(&base, d, band()).unwrap();
    let Surface::Cone { apex, axis, .. } = minted else {
        panic!("cone offset must stay a cone");
    };
    let u = 0.9;
    let p = base.eval(u, v);
    let n = base.normal(u, v); // v > 0: the chart normal IS the extended field here
    let q = p + n * d;
    let v_shift = v + d * (alpha.cos() / alpha.sin());
    assert!(v_shift < 0.0, "the fixture must cross the apex band");
    let got = cone_dist(apex, axis, alpha, q);
    let predicted = -v_shift * (2.0 * alpha).sin();
    assert!(
        (got - predicted).abs() < 1e-12,
        "meridian distance {got} vs closed form {predicted}"
    );
    // And it is FAR from on-surface: the wrong-nappe error is O(|v′|),
    // not a rounding residue — the row a windowed consumer must refuse.
    assert!(got > 0.5, "the apex-band defect must be first-order: {got}");
}

/// The realized-radius floor at the 1e16 collapse scale for the two
/// kinds the shipped red does not plant (sphere, torus minor), plus
/// the OTHER sign combination on the cylinder: `d = −(r + 0.5)` has a
/// NEGATIVE exact-real margin and a realized float of exactly 0.0 —
/// the door must refuse in both readings, and does, because the
/// realized sum is the metered quantity.
#[test]
fn realized_floor_at_scale_sphere_torus_and_negative_exact() {
    let d = -(1.0e16_f64 - 0.5); // rounds to −1e16; realized 0.0
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0e16,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    assert!(matches!(
        offset_surface(&sphere, d, band()),
        Err(OffsetError::RadiusFloor { .. })
    ));
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius: 3.0e16,
        minor_radius: 1.0e16,
        u_ref: Vec3::unit_x(),
    };
    assert!(matches!(
        offset_surface(&torus, d, band()),
        Err(OffsetError::RadiusFloor { .. })
    ));
    // Exact-real margin −0.5 m, realized 0.0: still a refusal (the
    // coincident-with-zero arm), never a mint.
    let d_neg = -(1.0e16_f64 + 0.5);
    assert_eq!(d_neg, -1.0e16, "fixture rounding premise");
    assert!(matches!(
        offset_surface(&zcyl(1.0e16), d_neg, band()),
        Err(OffsetError::RadiusFloor { .. })
    ));
}

/// The RING margin is also metered on the realized minor: at
/// `major = 1e16` (f64 spacing 2.0), `d = 1e16 − 2` realizes
/// `minor + d = major` exactly — refused at the crossing — while
/// `d = 1e16 − 4` realizes two ulps under and mints, storing exactly
/// the metered float.
#[test]
fn ring_meters_the_realized_minor_at_scale() {
    let torus = || Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius: 1.0e16,
        minor_radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let d_cross = 1.0e16 - 2.0;
    assert!(matches!(
        offset_surface(&torus(), d_cross, band()),
        Err(OffsetError::TorusRing { .. })
    ));
    let d_under = 1.0e16 - 4.0;
    let minted = offset_surface(&torus(), d_under, band()).unwrap();
    let Surface::Torus { minor_radius, .. } = minted else {
        panic!("torus offset must stay a torus");
    };
    assert_eq!(minor_radius.to_bits(), (2.0 + d_under).to_bits());
    assert_eq!(minor_radius, 1.0e16 - 2.0);
}

/// Half-angle near both validity edges: the mint stays a cone with
/// `α` carried bit-verbatim, and the defining equation (mint sample at
/// signed distance `d` from the base, meridian spelling) holds with a
/// bound scaled to the slide's magnitude (`~1e4·|d|` at `α = 1e-4`).
#[test]
fn cone_alpha_near_edges_defining_equation() {
    for alpha in [1.0e-4, FRAC_PI_2 - 1.0e-6] {
        let base = zcone(alpha);
        for d in [0.25, -0.25] {
            let minted = offset_surface(&base, d, band()).unwrap();
            let Surface::Cone {
                apex: base_apex, ..
            } = base
            else {
                unreachable!()
            };
            let Surface::Cone {
                apex, half_angle, ..
            } = minted
            else {
                panic!("cone offset must stay a cone");
            };
            assert_eq!(half_angle.to_bits(), alpha.to_bits());
            let slide = d / alpha.sin();
            let tol = 64.0 * f64::EPSILON * slide.abs().max(1.0);
            assert!(
                (apex.z - (base_apex.z - slide)).abs() <= tol,
                "alpha={alpha} d={d}: apex {} vs {}",
                apex.z,
                base_apex.z - slide
            );
            // Defining equation on a sample staying on the v>0 nappe
            // after the shift (v chosen beyond |d·cot α| at both
            // edges: cot(1e-4) ≈ 1e4 ⇒ v = 2·|slide| clears it).
            let v = 2.0 * slide.abs().max(1.0);
            let q = minted.eval(0.7, v);
            let got = cone_dist(base_apex, Vec3::unit_z(), alpha, q);
            let dtol = 1e-11 * v.max(1.0);
            assert!(
                (got - d).abs() < dtol,
                "alpha={alpha} d={d}: mint sample at {got} from base (tol {dtol})"
            );
        }
    }
}

/// Tiny and huge radii, proportionate `d`: the defining equation holds
/// with relative-scale bounds, both signs.
#[test]
fn tiny_and_huge_radii_defining_equation() {
    // Tiny cylinder, sized to the RUN band: the smallest mintable
    // realized radius is K·ε (below `escalate` the floor escalates,
    // below `zero` it refuses — measured: at default ε = 1e-9 a
    // realized 1.25e-9 m ESCALATES). Probe three decades above it.
    let t = Tol::witness().get();
    let r0 = 1.0e3 * t.k * t.eps;
    for d in [0.25 * r0, -0.25 * r0] {
        let base = zcyl(r0);
        let minted = offset_surface(&base, d, band()).unwrap();
        let q = minted.eval(1.3, 0.5);
        let rho = (q - Point3::new(0.0, 0.0, q.z)).norm();
        assert!(
            (rho - (r0 + d)).abs() < 1e-12 * r0,
            "tiny cylinder d={d}: rho {rho}"
        );
    }
    // r = 1e12 sphere, d = ±1.0.
    for d in [1.0, -1.0] {
        let base = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0e12,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let minted = offset_surface(&base, d, band()).unwrap();
        let q = minted.eval(0.9, 0.4);
        let r = (q - Point3::new(0.0, 0.0, 0.0)).norm();
        // |q| − r_base should be d, to ~ulp(1e12) per operation.
        assert!(
            ((r - 1.0e12) - d).abs() < 1e-3,
            "huge sphere d={d}: signed distance {}",
            r - 1.0e12
        );
    }
}

/// Band edges derived from the RUN tolerance, so the row is honest at
/// every `CAD_TOLERANCE_EPS`: a realized margin at `ε·√K` escalates
/// (inside the band at any K > 1), `2·K·ε` mints, `−2·K·ε` refuses.
/// Monotone in the right direction: this row goes red if the band's
/// sides ever swap or the floor stops metering the realized value.
#[test]
fn band_edges_track_run_tolerance() {
    let t = Tol::witness().get();
    let (eps, k) = (t.eps, t.k);
    assert!(k > 1.0, "band premise");
    let in_band = eps * k.sqrt();
    match offset_surface(&zcyl(1.0), -(1.0 - in_band), band()) {
        Err(OffsetError::Escalated { source }) => {
            assert_eq!(source.predicate, Some("offset_radius_floor"));
        }
        other => panic!("eps={eps}: in-band realized {in_band} must escalate, got {other:?}"),
    }
    let definite = 2.0 * k * eps;
    let minted = offset_surface(&zcyl(1.0), -(1.0 - definite), band()).unwrap();
    let Surface::Cylinder { radius, .. } = minted else {
        panic!("cylinder offset must stay a cylinder");
    };
    assert_eq!(radius.to_bits(), (1.0f64 - (1.0 - definite)).to_bits());
    assert!(matches!(
        offset_surface(&zcyl(1.0), -(1.0 + definite), band()),
        Err(OffsetError::RadiusFloor { .. })
    ));
}

/// The bit-exact round-trip claim's boundary, made visible: at a
/// NON-dyadic `d` the claim is not made, and the row measures the
/// actual slack (≤ 1 ulp of the radius) rather than asserting bit
/// identity — red if the mint ever grows extra arithmetic.
#[test]
fn round_trip_non_dyadic_slack_is_one_ulp() {
    let base = zcyl(2.5);
    for d in [0.1, -0.1, 1.0e-3] {
        let there = offset_surface(&base, d, band()).unwrap();
        let back = offset_surface(&there, -d, band()).unwrap();
        let Surface::Cylinder { radius, .. } = back else {
            panic!("cylinder offset must stay a cylinder");
        };
        let ulp = f64::EPSILON * 2.5;
        assert!(
            (radius - 2.5).abs() <= ulp,
            "d={d}: round-trip radius {radius} off by more than one ulp"
        );
    }
}

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::{Bounds, Interval, Real};

    fn icyl(radius: Interval) -> Surface<Interval> {
        Surface::Cylinder {
            origin: Point3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
            axis: Vec3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(1.0),
            ),
            radius,
            u_ref: Vec3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
        }
    }

    /// A WIDE half-angle enclosure: the minted apex height must
    /// enclose BOTH endpoint slides — the containment claim that
    /// degrades (and goes red) if interval division ever stops
    /// rounding outward.
    #[test]
    fn wide_half_angle_apex_encloses_endpoint_slides() {
        let (lo, hi) = (
            core::f64::consts::FRAC_PI_6 - 1.0e-3,
            core::f64::consts::FRAC_PI_6 + 1.0e-3,
        );
        let cone = Surface::Cone {
            apex: Point3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(2.0),
            ),
            axis: Vec3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(1.0),
            ),
            half_angle: Interval::from_bounds(lo, hi),
            u_ref: Vec3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
        };
        let d = 0.5;
        let minted = offset_surface(&cone, Interval::from_f64(d), band()).unwrap();
        let Surface::Cone { apex, .. } = minted else {
            panic!("cone offset must stay a cone");
        };
        for a in [lo, hi] {
            let z = 2.0 - d / a.sin();
            assert!(
                apex.z.lo() <= z && z <= apex.z.hi(),
                "apex enclosure [{}, {}] must contain endpoint slide {z}",
                apex.z.lo(),
                apex.z.hi()
            );
        }
    }

    /// The ring predicate at `T = Interval`: a `d` enclosure straddling
    /// the crossing escalates named; a definite crossing refuses typed
    /// (floor decided first and passing).
    #[test]
    fn ring_straddle_escalates_definite_refuses() {
        let torus = Surface::Torus {
            center: Point3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
            axis: Vec3::new(
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
                Interval::from_f64(1.0),
            ),
            major_radius: Interval::from_f64(2.0),
            minor_radius: Interval::from_f64(1.0),
            u_ref: Vec3::new(
                Interval::from_f64(1.0),
                Interval::from_f64(0.0),
                Interval::from_f64(0.0),
            ),
        };
        match offset_surface(&torus, Interval::from_bounds(0.99, 1.01), band()) {
            Err(OffsetError::Escalated { source }) => {
                assert_eq!(source.predicate, Some("offset_torus_ring"));
            }
            other => panic!("straddling ring must escalate, got {other:?}"),
        }
        assert!(matches!(
            offset_surface(&torus, Interval::from_f64(1.5), band()),
            Err(OffsetError::TorusRing { .. })
        ));
    }

    /// The realized-radius floor at the 1e16 collapse scale, at
    /// `T = Interval`: the thin realized enclosure is coincident with
    /// zero and refuses typed (not an escalation).
    #[test]
    fn interval_floor_at_scale_refuses_typed() {
        let d = Interval::from_f64(-(1.0e16 - 0.5)); // −1e16 after f64 rounding
        assert!(matches!(
            offset_surface(&icyl(Interval::from_f64(1.0e16)), d, band()),
            Err(OffsetError::RadiusFloor { .. })
        ));
    }
}
