//! REVIEW PROBES for PR 4680 (`flux/rim-span-decide`), frozen head
//! fcebff019a. Built on the PR's own `stored_span_winding_bound.rs`
//! fixtures (copied above the probes).
//!
//! RED ON THE HEAD BY DESIGN (findings): the three `probe_cone_*` rows —
//! the cone's flux lane on its ellipse branch, at f64 and `Interval`,
//! and the cone's material sign, read a past-bound rim span raw.
//! GREEN ON THE HEAD, and the only rows that kill two lever mutants the
//! PR's suite lets survive: the three `probe_torus_*` rows (the torus
//! rim decide levered at `minor` instead of `r_c`; the torus meridian
//! decide levered at `major` instead of `minor`).
//! **A rim span, and a torus single-edge meridian span, past the
//! winding bound refuse typed at every consumer.** Certification
//! bounds every edge's stored span `0 < Δt ≤ τ`
//! (`interval_span_forward`, `interval_span_winding`, at the linear
//! band with the carrier radius as the lever), and every closed form
//! that reads a span rests on that bound. A loop built without a body
//! (`LoopEdge::hand_built`) can state any span, so the parse decides
//! the bound itself:
//!
//! - every **rim**, of every kind, where the rim is classified, as
//!   `props_rim_span_forward` / `props_rim_span_winding` — the cylinder
//!   and torus charts sum rim spans into `Δu`, the cone's sum is
//!   required consistent and closed, and a rim's span is the face's
//!   `Δu` candidate on every kind;
//! - a torus meridian carried by **one** edge, as
//!   `props_meridian_span_forward` / `props_meridian_span_winding` —
//!   the sphere doors' names for the same per-edge question, at the
//!   meridian's own radius (the minor radius).
//!
//! The pair under every row is one iso rectangle per kind: a rim at
//! each level spanning `[0, Δu]` and a meridian at each end of it. On
//! main the `3π` rim rectangle measured `3π/τ` of its band and the
//! `3π` torus meridian folded through `sin`/`cos` — both silently.
//!
//! Every offset is derived from the run's own `Band`: this file runs
//! on CI's ε matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{
    FaceContribution, LoopEdge, PropsError, boundary_material_sign, curved_face,
    require_iso_rectangle,
};
use geom_core::{Band, Point3, Real, Tol, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

/// The cylinder's and the sphere's radius, the torus's major radius.
const R: f64 = 0.010;
/// The torus's minor radius.
const MINOR: f64 = 0.003;
/// The cylinder's height.
const H: f64 = 0.004;
/// The cone's half angle and its two slant levels.
const HALF: f64 = 0.5;
const S0: f64 = 0.004;
const S1: f64 = 0.009;
/// The sphere's and the torus's lower level, and the default extent.
const V0: f64 = -0.2;
const DV: f64 = 0.7;

const RIM_WINDING: &str = "props_rim_span_winding";
const MERIDIAN_WINDING: &str = "props_meridian_span_winding";
const MERIDIAN_FORWARD: &str = "props_meridian_span_forward";

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn f<T: Real>(x: f64) -> T {
    T::from_f64(x)
}
fn p<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(f(x), f(y), f(z))
}
fn v<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(f(x), f(y), f(z))
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Cylinder,
    Cone,
    Sphere,
    Torus,
}
const KINDS: [Kind; 4] = [Kind::Cylinder, Kind::Cone, Kind::Sphere, Kind::Torus];

fn surface<T: Real>(kind: Kind) -> Surface<T> {
    match kind {
        Kind::Cylinder => Surface::Cylinder {
            origin: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            radius: f(R),
            u_ref: v(1.0, 0.0, 0.0),
        },
        Kind::Cone => Surface::Cone {
            apex: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            half_angle: f(HALF),
            u_ref: v(1.0, 0.0, 0.0),
        },
        Kind::Sphere => Surface::Sphere {
            center: p(0.0, 0.0, 0.0),
            radius: f(R),
            axis: v(0.0, 0.0, 1.0),
            u_ref: v(1.0, 0.0, 0.0),
        },
        Kind::Torus => Surface::Torus {
            center: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            major_radius: f(R),
            minor_radius: f(MINOR),
            u_ref: v(1.0, 0.0, 0.0),
        },
    }
}

/// The rim at level `lv` (height, slant or angle by kind): a coaxial
/// circle whose parameter is the azimuth `u`.
fn rim_carrier<T: Real>(kind: Kind, lv: f64) -> Curve3<T> {
    let (z, r) = match kind {
        Kind::Cylinder => (lv, R),
        Kind::Cone => (lv * HALF.cos(), lv * HALF.sin()),
        Kind::Sphere => (R * lv.sin(), R * lv.cos()),
        Kind::Torus => (MINOR * lv.sin(), R + MINOR * lv.cos()),
    };
    Curve3::Circle {
        center: p(0.0, 0.0, z),
        axis: v(0.0, 0.0, 1.0),
        radius: f(r),
        u_ref: v(1.0, 0.0, 0.0),
    }
}

/// The meridian at azimuth `u`, whose parameter is the level.
fn meridian_carrier<T: Real>(kind: Kind, u: f64) -> Curve3<T> {
    let (c, s) = (u.cos(), u.sin());
    match kind {
        Kind::Cylinder => Curve3::Line {
            origin: p(R * c, R * s, 0.0),
            dir: v(0.0, 0.0, 1.0),
        },
        Kind::Cone => Curve3::Line {
            origin: p(0.0, 0.0, 0.0),
            dir: v(HALF.sin() * c, HALF.sin() * s, HALF.cos()),
        },
        // `axis × u_ref = ẑ`, so the parameter is the latitude (sphere)
        // or the minor angle (torus).
        Kind::Sphere => Curve3::Circle {
            center: p(0.0, 0.0, 0.0),
            axis: v(s, -c, 0.0),
            radius: f(R),
            u_ref: v(c, s, 0.0),
        },
        Kind::Torus => Curve3::Circle {
            center: p(R * c, R * s, 0.0),
            axis: v(s, -c, 0.0),
            radius: f(MINOR),
            u_ref: v(c, s, 0.0),
        },
    }
}

/// The kind's two levels: `(lo, lo + extent)`, the angular kinds'
/// extent being `dv`.
fn levels(kind: Kind, dv: f64) -> (f64, f64) {
    match kind {
        Kind::Cylinder => (0.0, H),
        Kind::Cone => (S0, S1),
        Kind::Sphere | Kind::Torus => (V0, V0 + dv),
    }
}

/// One iso rectangle: the low rim over the STORED interval `rim`
/// (`[0, Δu]` on a sound loop), the meridian up at its end, the high
/// rim back, the meridian down at `u = 0`. Each meridian is stored over
/// `meridian` (the levels, on a sound loop). Intervals are stored
/// verbatim, so a reversed one is stated as such.
fn rect<T: Real>(kind: Kind, rim: (f64, f64), meridian: (f64, f64)) -> Vec<LoopEdge<T>> {
    let (lo, hi) = meridian;
    let e = |c: Curve3<T>, (t0, t1): (f64, f64), forward: bool, a: u32, b: u32| {
        LoopEdge::hand_built(c, f(t0), f(t1), forward, a, b)
    };
    vec![
        e(rim_carrier(kind, lo), rim, true, 0, 1),
        e(meridian_carrier(kind, rim.1), meridian, true, 1, 2),
        e(rim_carrier(kind, hi), rim, false, 2, 3),
        e(meridian_carrier(kind, rim.0), meridian, false, 3, 0),
    ]
}

/// The kind's rectangle over `[0, du]` at its default levels.
fn rim_rect<T: Real>(kind: Kind, du: f64) -> Vec<LoopEdge<T>> {
    rect(kind, (0.0, du), levels(kind, DV))
}

/// The closed-form area of the kind's rectangle `Δu × [lo, hi]`.
fn exact_area(kind: Kind, du: f64, (lo, hi): (f64, f64)) -> f64 {
    match kind {
        Kind::Cylinder => du * R * (hi - lo),
        Kind::Cone => du * HALF.sin() * (hi.powi(2) - lo.powi(2)) / 2.0,
        Kind::Sphere => du * R.powi(2) * (hi.sin() - lo.sin()),
        Kind::Torus => du * MINOR * (R * (hi - lo) + MINOR * (hi.sin() - lo.sin())),
    }
}

fn refuses_as<V: core::fmt::Debug>(r: &Result<V, PropsError>, name: &str) -> bool {
    matches!(r, Err(PropsError::NotIsoRectangle { what }) if *what == name)
}

/// A zero-span ellipse edge on the cone (its contribution to the loop's
/// vector area is exactly zero), appended only to steer
/// `cone_face_closed_form` onto its ellipse branch.
fn null_ellipse<T: Real>() -> LoopEdge<T> {
    let c = Curve3::Ellipse {
        center: p(0.0, 0.0, S0 * HALF.cos()),
        axis: v(0.0, 0.0, 1.0),
        major: f(S0 * HALF.sin()),
        minor: f(S0 * HALF.sin() * 0.5),
        u_ref: v(1.0, 0.0, 0.0),
    };
    LoopEdge::hand_built(c, f(0.0), f(0.0), true, 0, 0)
}

/// REVIEW PROBE (PR 4680, claim 1): the cone's flux lane reads a
/// past-bound rim RAW whenever its outer loop holds an ellipse edge —
/// that branch never calls `cone_boundary`, so the rim is never minted
/// through `Rim::decided`, and `loop_vector_area` integrates `R²·Δt`.
/// The same 3π rectangle without the ellipse refuses (the PR's row).
#[test]
fn probe_cone_flux_lane_reads_a_3pi_rim_raw_on_its_ellipse_branch() {
    let s = surface::<f64>(Kind::Cone);
    let bd = band();
    let plain = rim_rect::<f64>(Kind::Cone, 3.0 * PI);
    let r = curved_face(&s, &plain, true, bd);
    assert!(refuses_as(&r, RIM_WINDING), "control: {r:?}");
    let mut steered = plain.clone();
    steered.push(null_ellipse());
    let r = curved_face(&s, &steered, true, bd);
    assert!(
        refuses_as(&r, RIM_WINDING),
        "ellipse branch answered a 3π rim: {r:?}; exact τ-rect area {:e}, 3π-rect {:e}",
        exact_area(Kind::Cone, TAU, levels(Kind::Cone, DV)),
        exact_area(Kind::Cone, 3.0 * PI, levels(Kind::Cone, DV)),
    );
}

/// REVIEW PROBE: the same at the interval scalar.
#[test]
fn probe_cone_flux_lane_reads_a_3pi_rim_raw_at_the_interval_scalar() {
    use geom_core::Interval;
    let s = surface::<Interval>(Kind::Cone);
    let mut steered = rim_rect::<Interval>(Kind::Cone, 3.0 * PI);
    steered.push(null_ellipse());
    let r = curved_face(&s, &steered, true, band());
    assert!(refuses_as(&r, RIM_WINDING), "{r:?}");
}

/// REVIEW PROBE: the cone's material sign answers a 3π rim the flux
/// lane refuses (disclosed in the PR's test-file doc as linear-and-exact).
#[test]
fn probe_cone_material_sign_answers_a_3pi_rim_the_flux_lane_refuses() {
    let s = surface::<f64>(Kind::Cone);
    let edges = rim_rect::<f64>(Kind::Cone, 3.0 * PI);
    let flux = curved_face(&s, &edges, true, band());
    let side = boundary_material_sign(&s, &edges, band());
    assert!(
        refuses_as(&flux, RIM_WINDING) == refuses_as(&side, RIM_WINDING),
        "flux {flux:?} vs material sign {side:?}"
    );
}

/// REVIEW PROBE (claim 2, the lever): the torus rim decide is levered
/// at the rim circle's radius `R + r cos v`, and the meridian decide at
/// the minor radius. An overrun of 0.5·zero/r_c on the rim is admitted;
/// 0.5·zero/minor on the meridian is admitted; 2·escalate/lever refuses.
#[test]
fn probe_torus_levers_are_the_rim_radius_and_the_minor_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, hi) = levels(Kind::Torus, DV);
    let rc_min = [lo, hi]
        .map(|lv| R + MINOR * lv.cos())
        .into_iter()
        .fold(f64::MAX, f64::min);
    // 2·escalate at the rim lever is under escalate at the minor lever
    // (r_c/minor ≈ 3.3), so a rim decide levered at the minor radius
    // escalates instead of refusing.
    let du = TAU + 2.0 * bd.escalate() / rc_min;
    let r = curved_face(&s, &rect::<f64>(Kind::Torus, (0.0, du), (lo, hi)), true, bd);
    assert!(
        refuses_as(&r, RIM_WINDING),
        "rim overrun {:e} rad (2·escalate at r_c={rc_min}, {:.2}·escalate at minor) : {r:?}",
        du - TAU,
        (du - TAU) * MINOR / bd.escalate()
    );
    // The meridian at the minor radius: 2·escalate/minor refuses.
    let dv = TAU + 2.0 * bd.escalate() / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + dv)),
        true,
        bd,
    );
    assert!(refuses_as(&r, MERIDIAN_WINDING), "meridian: {r:?}");
}

/// REVIEW PROBE (claim 2, the lever, the admit/escalate side): a single
/// torus meridian overrunning τ inside the ambiguity band AT THE MINOR
/// RADIUS escalates under the meridian decide's name, and one inside
/// the coincidence band at the minor radius is not refused by it.
#[test]
fn probe_torus_meridian_band_is_metered_at_the_minor_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, _) = levels(Kind::Torus, DV);
    let mid = TAU + 0.5 * (bd.zero() + bd.escalate()) / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + mid)),
        true,
        bd,
    );
    assert!(
        matches!(&r, Err(PropsError::Escalated { cause, .. }) if cause.predicate == Some(MERIDIAN_WINDING)),
        "mid-band meridian: {r:?}"
    );
    let inside = TAU + 0.5 * bd.zero() / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + inside)),
        true,
        bd,
    );
    assert!(
        !refuses_as(&r, MERIDIAN_WINDING)
            && !matches!(&r, Err(PropsError::Escalated { cause, .. }) if cause.predicate == Some(MERIDIAN_WINDING)),
        "coincident meridian: {r:?}"
    );
}

/// REVIEW PROBE: the torus RIM band at the rim radius — mid-band
/// escalates under the rim decide's name.
#[test]
fn probe_torus_rim_band_is_metered_at_the_rim_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, hi) = levels(Kind::Torus, DV);
    let rcs = [lo, hi].map(|lv| R + MINOR * lv.cos());
    // Mid-band at BOTH rims' levers: pick the overrun mid-band at the
    // geometric mean so both are inside (zero, escalate) when the band
    // ratio exceeds rc_max/rc_min.
    let rc = (rcs[0] * rcs[1]).sqrt();
    let mid = TAU + ((bd.zero() * bd.escalate()).sqrt()) / rc;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, mid), (lo, hi)),
        true,
        bd,
    );
    assert!(
        matches!(&r, Err(PropsError::Escalated { cause, .. }) if cause.predicate == Some(RIM_WINDING)),
        "mid-band rim: {r:?} (zero {:e}, escalate {:e})",
        bd.zero(),
        bd.escalate()
    );
}
