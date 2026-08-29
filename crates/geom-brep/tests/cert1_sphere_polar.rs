//! **The sphere's polar acceptance defects (issue 723 and issue 893),
//! pinned by closed forms.**
//!
//! Two mechanisms, both accepting-direction, both sphere-only:
//!
//! * **Extent from endpoints** (issue 723). The sphere arms take
//!   `(lo, hi)` from `min_max` over meridian ENDPOINT latitudes, and a
//!   great-circle arc contains both poles: an arc whose parameter span
//!   crosses a pole reaches latitude ±1 in its INTERIOR, where an
//!   endpoint fold never looks. The face is accepted and measures
//!   short — executed at −47% through the STEP door (rim-bearing arm)
//!   and −29.3% on the rimless two-band arm. The extent must come from
//!   the arc's **stored span** — the torus's own derivation — not from
//!   its endpoints.
//! * **The rim lever collapses toward the poles** (issue 893). Sphere
//!   rims minted `RimLevel::Unit(sin v, 0)`, so the level chord is the
//!   AXIAL separation `R·|Δ sin v|`, which vanishes as `cos v̄ → 0`:
//!   two genuinely distinct near-polar rims decide `Zero` and a
//!   non-rectangular domain PASSES the iso-rectangle predicate.
//!
//! Every row asserts the closed form (or the typed refusal), never a
//! regression capture. Offsets in the near-polar rows come from the
//! run's OWN `Band`, never from an ε literal — this file is on CI's
//! `eps ∈ {default, 1e-6, 1e-12}` matrix, and a literal states a
//! claim about one of the three.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge {
        carrier,
        t0,
        t1,
        forward,
        start,
        end,
    }
}

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim at latitude `v` (an axis-parallel circle), `u0 → u1`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, RS * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RS * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The meridian great circle whose plane contains the axis at azimuth
/// `u`; its parameter IS the latitude on the `u` side, so `t = π/2` is
/// the north pole and `t ∈ (π/2, 3π/2)` descends the `u + π` side.
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: RS,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

fn accepts_exactly(kind: &str, edges: &[LoopEdge<f64>], exact_area: f64) {
    let band = Band::linear(Tol::witness()).unwrap();
    match curved_face(&sphere(), edges, 1.0, band) {
        Ok(fc) => {
            let rel = (fc.area - exact_area).abs() / exact_area;
            assert!(
                rel < 1e-12,
                "{kind}: area {:.15e} != exact {exact_area:.15e} (rel {rel:.3e})",
                fc.area
            );
        }
        Err(e) => panic!("{kind}: refused: {e:?}"),
    }
}

// ---------------------------------------------------------------------
// Issue 723 — the extent, on both sphere arms
// ---------------------------------------------------------------------

/// **The rim-bearing arm: a pole-crossing meridian arc measures the
/// half-cap exactly.** The domain is `[0, π] × [b, π/2]` — issue 723's
/// executed face: one rim half-circle at latitude `b` and one meridian
/// great-circle arc from `(0, b)` over the north pole down to
/// `(π, b)`, split by one ordinary vertex at `t = 1.0` (the split is
/// what made the wrong number reachable: without it the endpoint fold
/// sees `lo == hi` and refuses degenerate; with it the face was
/// ACCEPTED at `R²·π·(sin 1 − sin b)` — −30.45% of the closed form).
/// The exact area is the half-cap's: `R²·π·(1 − sin b)`.
#[test]
fn a_pole_crossing_meridian_arc_measures_the_half_cap_exactly() {
    let b = 0.5;
    let split = 1.0;
    let half_cap = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ];
    accepts_exactly(
        "half-cap (pole-crossing meridian, split)",
        &half_cap,
        RS * RS * core::f64::consts::PI * (1.0 - b.sin()),
    );
}

/// **The rimless arm: the hemisphere split at ±π/4 still measures
/// `2πR²`.** The same set of points as the pole-split loop the arm was
/// written for — one great circle, both halves on the SAME carrier, so
/// `props_band_coplanar`'s margin is bitwise 0 — with the two vertices
/// moved off the poles to latitudes ±π/4. Both poles then fall in the
/// arcs' INTERIORS; the endpoint fold gave `(∓√2/2)` and the face
/// measured −29.2893% (issue 723, comment 4). Pure topology must not
/// move the answer: the area is the hemisphere's, `2πR²`.
#[test]
fn the_rimless_hemisphere_split_off_its_poles_still_measures() {
    let q = core::f64::consts::FRAC_PI_4;
    let band = vec![
        great(0.0, q, 5.0 * q, 0, 1),
        great(0.0, 5.0 * q, 9.0 * q, 1, 0),
    ];
    accepts_exactly(
        "rimless hemisphere split at ±π/4",
        &band,
        2.0 * core::f64::consts::PI * RS * RS,
    );
}

// ---------------------------------------------------------------------
// Issue 893 — the rim lever, near the pole
// ---------------------------------------------------------------------

/// The near-polar staircase: `u ∈ [−1, 1]` over `[v0, v1]` plus
/// `u ∈ [0, 1]` over `[v1, v2]`, with `v1 < v2` both near the north
/// pole — `w(v)` changes at `v1`, so the domain is NOT an
/// iso-rectangle. `dv` (the rim separation, radians) is the knob the
/// two rows below turn against the run's own band.
fn near_polar_staircase(v0: f64, v1: f64, v2: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(v0, -1.0, 1.0, 0, 1),
        great(1.0, v0, v2, 1, 2),
        rim(v2, 1.0, 0.0, 2, 3),
        great(0.0, v2, v1, 3, 4),
        rim(v1, 0.0, -1.0, 4, 5),
        great(-1.0, v1, v0, 5, 0),
    ]
}

/// **Two genuinely distinct near-polar rims are not one level** (issue
/// 893's ask 1 — the row no suite had). At `δ0 = 0.002` off the pole
/// with rim separation `dv = 10·escalate/R`, the rims' true point
/// separation is `~R·dv = 10·escalate` — decisively distinct at any ε
/// and any K — while their AXIAL separation is
/// `R·Δsin v ≈ R·dv·(δ0 + dv/2) < 0.007·(10·escalate) < zero`: the
/// collapsed `(sin v, 0)` lever decided the pair `Zero`, the interior
/// rim passed as coincident with the extreme, and the staircase was
/// ACCEPTED. The honest answer is the iso-rectangle refusal, by the
/// one named predicate.
#[test]
fn two_distinct_near_polar_rims_are_not_one_level() {
    let band = Band::linear(Tol::witness()).unwrap();
    let d0 = 0.002;
    let dv = 10.0 * band.escalate() / RS;
    let v2 = core::f64::consts::FRAC_PI_2 - d0;
    let edges = near_polar_staircase(0.2, v2 - dv, v2);
    assert!(
        matches!(
            curved_face(&sphere(), &edges, 1.0, band),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_level"
            })
        ),
        "two near-polar rims 10·escalate apart must not pass as one level"
    );
}

/// **The floor's other side: a step WITHIN the band is one level.**
/// The same staircase with the rim separation shrunk until the true
/// point separation (the direction chord at R) is `0.5·zero` — the
/// two rims are the same level by the run's own tolerance, so the
/// face is a genuine iso-rectangle at this ε and must stay ACCEPTED.
/// A lever fix that refused everything near the pole could not keep
/// this row green. The asserted area is the rectangle closed form
/// `2R²·(sin v2 − sin v0)`; the sub-tolerance step displaces it by
/// `R²·Δsin ≈ R·(0.5·zero)·δ0 ≲ 1e-8` relative, so the bound is 1e-6.
#[test]
fn a_near_polar_step_within_the_band_is_still_one_level() {
    let band = Band::linear(Tol::witness()).unwrap();
    let d0 = 0.002;
    let dv = 0.5 * band.zero() / RS;
    let v0 = 0.2;
    let v2 = core::f64::consts::FRAC_PI_2 - d0;
    let edges = near_polar_staircase(v0, v2 - dv, v2);
    let got = curved_face(&sphere(), &edges, 1.0, band)
        .expect("a rim step half an epsilon of point separation is one level");
    let closed = 2.0 * RS * RS * (v2.sin() - v0.sin());
    let rel = (got.area - closed).abs() / closed;
    assert!(
        rel < 1e-6,
        "area {:.15e} != rectangle closed form {closed:.15e} (rel {rel:.3e})",
        got.area
    );
}
