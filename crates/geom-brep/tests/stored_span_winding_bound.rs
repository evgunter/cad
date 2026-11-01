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

/// The refusal a rim span outside the bound draws from `consumer` on
/// `kind`, by the decide's half (`"forward"` or `"winding"`). The
/// sphere's flux lane and material sign decide EVERY arc's span on
/// their own loop walk before the shared parse runs (Gauss–Bonnet
/// integrates each one), so a rim there refuses under the arc decide's
/// name, as `SphereLoop`; everywhere else it is the rim decide's.
fn rim_refusal<V: core::fmt::Debug>(
    kind: Kind,
    consumer: &str,
    half: &str,
    r: &Result<V, PropsError>,
) -> bool {
    match (kind, consumer) {
        (Kind::Sphere, "flux lane" | "material sign") => matches!(
            r,
            Err(PropsError::SphereLoop { what })
                if *what == format!("props_sphere_arc_span_{half}")
        ),
        _ => refuses_as(r, &format!("props_rim_span_{half}")),
    }
}

/// The consumers that read a curved face's rim spans, on one loop: the
/// flux lane, the material sign, the shape door.
fn consumers(kind: Kind, edges: &[LoopEdge<f64>]) -> [(&'static str, Result<(), PropsError>); 3] {
    let s = surface::<f64>(kind);
    let bd = band();
    [
        (
            "flux lane",
            curved_face(&s, edges, true, bd).map(|_: FaceContribution<f64>| ()),
        ),
        (
            "material sign",
            boundary_material_sign(&s, edges, bd).map(|_| ()),
        ),
        ("shape door", require_iso_rectangle(&s, edges, bd)),
    ]
}

// ---------------------------------------------------------------------
// The rim decide, every kind, every consumer
// ---------------------------------------------------------------------

/// **A `3π` rim refuses under the rim decide's name**, at the flux
/// lane, the material sign and the shape door of every kind. The
/// rectangle closes (the rim ends at `u = π`, where the up-meridian
/// stands), so nothing but the span bound can refuse it.
#[test]
fn a_rim_past_the_winding_bound_refuses_at_every_consumer_of_every_kind() {
    let mut misses = Vec::new();
    for kind in KINDS {
        for (consumer, r) in consumers(kind, &rim_rect::<f64>(kind, 3.0 * PI)) {
            if !rim_refusal(kind, consumer, "winding", &r) {
                misses.push(format!("{kind:?} {consumer}: {r:?}"));
            }
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// **A rim stored reversed refuses under the forward half's name**:
/// the headroom `τ − Δt` of a reversed span is definitely positive, so
/// the winding half alone would admit it.
#[test]
fn a_reversed_rim_refuses_at_every_consumer_of_every_kind() {
    let mut misses = Vec::new();
    for kind in KINDS {
        let edges = rect::<f64>(kind, (PI / 2.0, 0.0), levels(kind, DV));
        for (consumer, r) in consumers(kind, &edges) {
            if !rim_refusal(kind, consumer, "forward", &r) {
                misses.push(format!("{kind:?} {consumer}: {r:?}"));
            }
        }
    }
    assert!(misses.is_empty(), "{}", misses.join("\n"));
}

/// **A full-period rim is admitted by the rim decide — at the period,
/// not short of it.** Its headroom is `Zero`, which certification
/// admits; so is a span inside the coincidence band above τ. The
/// full-period rectangle (both meridians on the seam) measures the
/// whole band at every kind, and the other consumers answer.
#[test]
fn a_full_period_rim_is_admitted_at_every_consumer_of_every_kind() {
    let bd = band();
    for kind in KINDS {
        let (lo, hi) = levels(kind, DV);
        // The wider rim's radius: the overrun is inside the coincidence
        // band at each rim's own lever.
        let lever = [lo, hi]
            .map(|lv| match rim_carrier::<f64>(kind, lv) {
                Curve3::Circle { radius, .. } => radius,
                _ => unreachable!("a rim is a circle"),
            })
            .into_iter()
            .fold(0.0, f64::max);
        for du in [TAU, TAU + 0.5 * bd.zero() / lever] {
            let edges = rim_rect::<f64>(kind, du);
            for (consumer, r) in consumers(kind, &edges) {
                assert_eq!(r, Ok(()), "{kind:?} Δu = τ + {:e}: {consumer}", du - TAU);
            }
            let fc = curved_face(&surface::<f64>(kind), &edges, true, bd).unwrap();
            let exact = exact_area(kind, du, (lo, hi));
            let rel = (fc.area.abs() - exact).abs() / exact;
            assert!(rel < 1e-12, "{kind:?}: area {:e} != {exact:e}", fc.area);
        }
    }
}

/// **The decide is certification's, so the band is.** A rim span past
/// τ by more than the escalation band refuses; one in the ambiguity
/// band escalates under the rim decide's name; one inside the
/// coincidence band is answered (the previous row).
#[test]
fn the_rim_decide_escalates_in_certifications_ambiguity_band() {
    let bd = band();
    // The cylinder: every rim is at the cylinder radius, so the lever
    // is `R` on both rims.
    let mid = TAU + 0.5 * (bd.zero() + bd.escalate()) / R;
    for (consumer, r) in consumers(Kind::Cylinder, &rim_rect::<f64>(Kind::Cylinder, mid)) {
        assert!(
            matches!(&r, Err(PropsError::Escalated { cause, .. })
                if cause.predicate == Some(RIM_WINDING)),
            "{consumer}: {r:?}"
        );
    }
    let past = TAU + 2.0 * bd.escalate() / R;
    for (consumer, r) in consumers(Kind::Cylinder, &rim_rect::<f64>(Kind::Cylinder, past)) {
        assert!(refuses_as(&r, RIM_WINDING), "{consumer}: {r:?}");
    }
}

/// **The interval lane decides the same way**: the `3π` rim refuses
/// by name on the cylinder's flux lane at the interval scalar, and the
/// full-period rim is answered.
#[test]
fn the_rim_decide_holds_at_the_interval_scalar() {
    use geom_core::Interval;
    let bd = band();
    let s = surface::<Interval>(Kind::Cylinder);
    let r = curved_face(
        &s,
        &rim_rect::<Interval>(Kind::Cylinder, 3.0 * PI),
        true,
        bd,
    );
    assert!(refuses_as(&r, RIM_WINDING), "{r:?}");
    let r = curved_face(&s, &rim_rect::<Interval>(Kind::Cylinder, TAU), true, bd);
    assert!(r.is_ok(), "{r:?}");
}

// ---------------------------------------------------------------------
// The torus single-edge meridian
// ---------------------------------------------------------------------

/// **A torus meridian of one edge spanning `3π` refuses under the
/// meridian decide's name** at every consumer. The rectangle closes —
/// the high rim sits at `v₀ + 3π ≡ v₀ + π` — so it is the span alone
/// that no edge certifies. The rims are a quarter turn, inside the
/// bound, so the rim decide admits them.
#[test]
fn a_torus_single_edge_meridian_past_the_winding_bound_refuses_at_every_consumer() {
    let edges = rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (V0, V0 + 3.0 * PI));
    for (consumer, r) in consumers(Kind::Torus, &edges) {
        assert!(refuses_as(&r, MERIDIAN_WINDING), "{consumer}: {r:?}");
    }
}

/// **A torus meridian of one edge stored reversed refuses under the
/// forward half's name** at every consumer.
#[test]
fn a_torus_single_edge_reversed_meridian_refuses_at_every_consumer() {
    let edges = rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (V0 + DV, V0));
    for (consumer, r) in consumers(Kind::Torus, &edges) {
        assert!(refuses_as(&r, MERIDIAN_FORWARD), "{consumer}: {r:?}");
    }
}

/// **The control: a sound single-edge torus meridian is answered**,
/// and the flux lane measures the rectangle's closed form.
#[test]
fn a_torus_single_edge_meridian_within_the_bound_is_answered() {
    let bd = band();
    let (du, lv) = (PI / 2.0, (V0, V0 + DV));
    let edges = rect::<f64>(Kind::Torus, (0.0, du), lv);
    for (consumer, r) in consumers(Kind::Torus, &edges) {
        assert_eq!(r, Ok(()), "{consumer}");
    }
    let fc = curved_face(&surface::<f64>(Kind::Torus), &edges, true, bd).unwrap();
    let exact = exact_area(Kind::Torus, du, lv);
    let rel = (fc.area.abs() - exact).abs() / exact;
    assert!(rel < 1e-12, "area {:e} != {exact:e}", fc.area);
}

/// **The interval lane**: the `3π` single-edge meridian refuses by
/// name at the interval scalar too.
#[test]
fn the_torus_meridian_decide_holds_at_the_interval_scalar() {
    use geom_core::Interval;
    let edges = rect::<Interval>(Kind::Torus, (0.0, PI / 2.0), (V0, V0 + 3.0 * PI));
    let r = curved_face(&surface::<Interval>(Kind::Torus), &edges, true, band());
    assert!(refuses_as(&r, MERIDIAN_WINDING), "{r:?}");
}

// ---------------------------------------------------------------------
// The cone reads circle spans the parse never mints as rims
// ---------------------------------------------------------------------

/// A short arc of an ellipse on the cone — its section by a plane
/// through the low rim's start, tilted off the axis — leaving that
/// point: a valid edge whose own decides all admit, so whatever else
/// the cone's reader refuses in a loop holding it is the loop's.
fn section_arc<T: Real>() -> LoopEdge<T> {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let (s, c) = HALF.sin_cos();
    let tilt: f64 = 0.3;
    let n = [tilt.sin(), 0.0, tilt.cos()];
    let p0 = [S0 * s, 0.0, S0 * c];
    let far = dot(n, p0) / dot(n, [-s, 0.0, c]);
    let center = [(p0[0] - far * s) / 2.0, 0.0, (p0[2] + far * c) / 2.0];
    let half = [p0[0] - center[0], 0.0, p0[2] - center[2]];
    let major = dot(half, half).sqrt();
    let across = center[2] * HALF.tan();
    let carrier = Curve3::Ellipse {
        center: p(center[0], center[1], center[2]),
        axis: v(n[0], n[1], n[2]),
        semi_u: f(major),
        semi_v: f(((across - center[0]) * (across + center[0])).sqrt()),
        u_ref: v(half[0] / major, 0.0, half[2] / major),
    };
    LoopEdge::hand_built(carrier, f(0.0), f(0.2), true, 0, 0)
}

/// **The cone's vector-area reader decides the circle spans it
/// integrates, whatever else the loop holds.** Before the decide the
/// `3π` rectangle plus an ellipse edge answered the `3π` area
/// (`1.4685e-4` m²) — the ellipse steered `cone_face_closed_form` onto
/// a branch that never reached the rim parse, and `loop_vector_area`
/// reads `R²·(Δt − sin Δt)` — at `f64` and at the interval scalar alike. Every cone
/// face is now read by the one reader, which decides every span before
/// anything integrates it: with a valid ellipse arc first or last in
/// the loop, the rim past the bound is what refuses.
#[test]
fn the_cones_ellipse_branch_refuses_a_rim_past_the_winding_bound() {
    use geom_core::Interval;
    let bd = band();
    for first in [true, false] {
        let mut edges = rim_rect::<f64>(Kind::Cone, 3.0 * PI);
        let mut iv = rim_rect::<Interval>(Kind::Cone, 3.0 * PI);
        if first {
            edges.insert(0, section_arc());
            iv.insert(0, section_arc());
        } else {
            edges.push(section_arc());
            iv.push(section_arc());
        }
        let r = curved_face(&surface::<f64>(Kind::Cone), &edges, true, bd);
        assert!(refuses_as(&r, RIM_WINDING), "f64, arc first {first}: {r:?}");
        let r = curved_face(&surface::<Interval>(Kind::Cone), &iv, true, bd);
        assert!(
            refuses_as(&r, RIM_WINDING),
            "interval, arc first {first}: {r:?}"
        );
    }
    // The control: a full-period rectangle answers the closed form.
    let edges = rim_rect::<f64>(Kind::Cone, TAU);
    let fc = curved_face(&surface::<f64>(Kind::Cone), &edges, true, bd).unwrap();
    let exact = exact_area(Kind::Cone, TAU, levels(Kind::Cone, DV));
    let rel = (fc.area.abs() - exact).abs() / exact;
    assert!(rel < 1e-12, "area {:e} != {exact:e}", fc.area);
}

/// **The cone's material sign and its flux lane agree on a `3π` rim**:
/// both refuse under the rim decide's name. On main the material sign
/// answered `Encoded(Positive)` where the flux lane (through the rim
/// parse) refused.
#[test]
fn the_cones_material_sign_refuses_what_its_flux_lane_refuses() {
    let s = surface::<f64>(Kind::Cone);
    let edges = rim_rect::<f64>(Kind::Cone, 3.0 * PI);
    let flux = curved_face(&s, &edges, true, band());
    let side = boundary_material_sign(&s, &edges, band());
    assert!(refuses_as(&flux, RIM_WINDING), "flux lane: {flux:?}");
    assert!(refuses_as(&side, RIM_WINDING), "material sign: {side:?}");
}

// ---------------------------------------------------------------------
// The torus levers: the rim's own radius, and the minor radius
// ---------------------------------------------------------------------

/// **The torus rim decide is levered at the rim circle's radius
/// `R + r·cos v`**: a rim overrun of `2·escalate/r_c` refuses, where at
/// the minor radius (`r_c/r ≈ 3.3`) it would be under `escalate` and
/// only escalate. A meridian overrun of `2·escalate/r` refuses too; a
/// LARGER lever would refuse it as well, so the meridian's lever is
/// pinned by the band row below, which a lever other than `r` moves.
#[test]
fn the_torus_levers_are_the_rim_radius_and_the_minor_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, hi) = levels(Kind::Torus, DV);
    let rc_min = [lo, hi]
        .map(|lv| R + MINOR * lv.cos())
        .into_iter()
        .fold(f64::MAX, f64::min);
    let du = TAU + 2.0 * bd.escalate() / rc_min;
    let r = curved_face(&s, &rect::<f64>(Kind::Torus, (0.0, du), (lo, hi)), true, bd);
    assert!(refuses_as(&r, RIM_WINDING), "rim: {r:?}");
    let dv = TAU + 2.0 * bd.escalate() / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + dv)),
        true,
        bd,
    );
    assert!(refuses_as(&r, MERIDIAN_WINDING), "meridian: {r:?}");
}

/// **The torus meridian's ambiguity band is metered at the minor
/// radius**: an overrun mid-band at `r` escalates under the meridian
/// decide's name, and one inside the coincidence band at `r` is not
/// refused by it.
#[test]
fn the_torus_meridian_band_is_metered_at_the_minor_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, _) = levels(Kind::Torus, DV);
    let escalates = |r: &Result<FaceContribution<f64>, PropsError>| {
        matches!(r, Err(PropsError::Escalated { cause, .. })
            if cause.predicate == Some(MERIDIAN_WINDING))
    };
    let mid = TAU + 0.5 * (bd.zero() + bd.escalate()) / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + mid)),
        true,
        bd,
    );
    assert!(escalates(&r), "mid-band meridian: {r:?}");
    let inside = TAU + 0.5 * bd.zero() / MINOR;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, PI / 2.0), (lo, lo + inside)),
        true,
        bd,
    );
    assert!(
        !refuses_as(&r, MERIDIAN_WINDING) && !escalates(&r),
        "coincident meridian: {r:?}"
    );
}

/// **The torus rim's ambiguity band is metered at the rim's own
/// radius**: an overrun mid-band at both rims' levers (the geometric
/// mean of the band's edges over the geometric mean of the two radii)
/// escalates under the rim decide's name.
#[test]
fn the_torus_rim_band_is_metered_at_the_rim_radius() {
    let bd = band();
    let s = surface::<f64>(Kind::Torus);
    let (lo, hi) = levels(Kind::Torus, DV);
    let rc = ((R + MINOR * lo.cos()) * (R + MINOR * hi.cos())).sqrt();
    let mid = TAU + (bd.zero() * bd.escalate()).sqrt() / rc;
    let r = curved_face(
        &s,
        &rect::<f64>(Kind::Torus, (0.0, mid), (lo, hi)),
        true,
        bd,
    );
    assert!(
        matches!(&r, Err(PropsError::Escalated { cause, .. })
            if cause.predicate == Some(RIM_WINDING)),
        "mid-band rim: {r:?}"
    );
}
