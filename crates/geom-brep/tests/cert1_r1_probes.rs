//! **CERT-1 R1 adversarial probes** — reviewer-authored near-polar
//! bodies against the span-derived sphere extent and the
//! `(sin v, cos v)` rim lever (PR 1220, frozen head bc815c2c).
//!
//! Every row asserts a closed form or a typed refusal; each doc
//! comment says what the probe attacks. Generic over the scalar where
//! the interval lane matters.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::Tol;
use geom_core::{Band, Point3, Real, Vec3};

const RS: f64 = 0.010;
const PI: f64 = core::f64::consts::PI;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn v3<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}
fn p3<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}
fn edge<T: Real>(carrier: Curve3<T>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<T> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge {
        carrier_id: None,
        carrier,
        t0: T::from_f64(t0),
        t1: T::from_f64(t1),
        forward,
        start,
        end,
    }
}

fn sphere<T: Real>() -> Surface<T> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: T::from_f64(RS),
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// Rim at latitude `v`, azimuth `u0 → u1`.
fn rim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, RS * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64(RS * v.cos()),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// Meridian great circle at azimuth `u`; parameter = latitude on the
/// `u` side, `t = π/2` the north pole, `(π/2, 3π/2)` the far side.
fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: T::from_f64(RS),
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

fn assert_area(kind: &str, edges: &[LoopEdge<f64>], exact: f64) {
    match curved_face(&sphere(), edges, 1.0, band()) {
        Ok(fc) => {
            let rel = (fc.area - exact).abs() / exact;
            assert!(
                rel < 1e-12,
                "{kind}: area {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
                fc.area
            );
        }
        Err(e) => panic!("{kind}: refused: {e:?}"),
    }
}

// ------------------------------------------------------------------
// Extent — pole-crossing arcs at varied anchors and windings
// ------------------------------------------------------------------

/// Half-cap with the split vertex at several latitudes, including
/// past the pole (t = 2.0 > π/2) and barely off the base (t = 0.6):
/// the folded extent must not depend on where the ordinary vertex
/// sits.
#[test]
fn probe_half_cap_split_anchor_sweep() {
    let b = 0.5;
    for split in [0.6, 1.2, core::f64::consts::FRAC_PI_2, 2.0, PI - 0.51] {
        let edges = vec![
            rim(b, 0.0, PI, 0, 1),
            great(0.0, PI - b, split, 1, 2),
            great(0.0, split, b, 2, 0),
        ];
        assert_area(
            &format!("half-cap split at t={split}"),
            &edges,
            RS * RS * PI * (1.0 - b.sin()),
        );
    }
}

/// Split vertex EXACTLY at the north pole: both meridian arcs are
/// anchored at a chart pole (one endpoint latitude is exactly 1), the
/// `props_meridian_pole` decision sits on its Zero. The half-cap area
/// must be unchanged.
#[test]
fn probe_half_cap_split_exactly_at_the_pole() {
    let b = 0.5;
    let edges = vec![
        rim(b, 0.0, PI, 0, 1),
        great(0.0, PI - b, core::f64::consts::FRAC_PI_2, 1, 2),
        great(0.0, core::f64::consts::FRAC_PI_2, b, 2, 0),
    ];
    assert_area(
        "half-cap split exactly at the pole",
        &edges,
        RS * RS * PI * (1.0 - b.sin()),
    );
}

/// Rimless hemisphere with an ASYMMETRIC split: one arc spans 3π/2
/// (contains BOTH poles), the other π/2 (contains neither). The
/// one-arc-two-poles case exercises both pushes of the span fold on a
/// single edge.
#[test]
fn probe_rimless_one_arc_carries_both_poles() {
    let a = 0.3;
    let edges = vec![
        great(0.0, a, a + 1.5 * PI, 0, 1),
        great(0.0, a + 1.5 * PI, a + 2.0 * PI, 1, 0),
    ];
    assert_area(
        "rimless hemisphere, both poles in one arc",
        &edges,
        2.0 * PI * RS * RS,
    );
}

/// Rimless hemisphere split exactly AT its poles — the loop the arm
/// was written for, with each arc ANCHORED at a pole (t0 = π/2 and
/// t0 = 3π/2): the pole containment decision is exactly Zero at both
/// ends of both arcs and the fold must still produce [−1, 1].
#[test]
fn probe_rimless_pole_split_anchored_at_poles() {
    let h = core::f64::consts::FRAC_PI_2;
    let edges = vec![
        great(0.0, h, 3.0 * h, 0, 1),
        great(0.0, 3.0 * h, 5.0 * h, 1, 0),
    ];
    assert_area(
        "rimless hemisphere anchored at its poles",
        &edges,
        2.0 * PI * RS * RS,
    );
}

/// The same pole-anchored loop through the INTERVAL scalar: the
/// stored spans put every `props_meridian_pole` decision exactly on
/// its Zero with the sign enclosure straddling — the shipped chord
/// margin must stay tight (±chord hull) and certify without
/// escalating. The first draft's `atan2`/`floor` margin is expected
/// to blow up here (branch cut at the south-pole anchor, integer
/// step at the north's).
#[cfg(feature = "interval")]
#[test]
fn probe_interval_pole_anchored_hemisphere_certifies() {
    use geom_core::{Bounds, Interval};
    let h = core::f64::consts::FRAC_PI_2;
    let edges: Vec<LoopEdge<Interval>> = vec![
        great(0.0, h, 3.0 * h, 0, 1),
        great(0.0, 3.0 * h, 5.0 * h, 1, 0),
    ];
    let fc = curved_face(
        &sphere::<Interval>(),
        &edges,
        Interval::from_f64(1.0),
        band(),
    )
    .expect("pole-anchored hemisphere must certify at interval");
    let exact = 2.0 * PI * RS * RS;
    let (lo, hi) = (fc.area.lo(), fc.area.hi());
    assert!(
        lo <= exact && exact <= hi && (hi - lo) < 1e-9,
        "interval area [{lo:e}, {hi:e}] must tightly enclose {exact:e}"
    );
}

/// The half-cap through the interval scalar, split at an ordinary
/// vertex: the accepting path of `props_meridian_pole` (sign
/// decisively positive) must also hold at interval.
#[cfg(feature = "interval")]
#[test]
fn probe_interval_half_cap_certifies() {
    use geom_core::{Bounds, Interval};
    let b = 0.5;
    let edges: Vec<LoopEdge<Interval>> = vec![
        rim(b, 0.0, PI, 0, 1),
        great(0.0, PI - b, 1.0, 1, 2),
        great(0.0, 1.0, b, 2, 0),
    ];
    let fc = curved_face(
        &sphere::<Interval>(),
        &edges,
        Interval::from_f64(1.0),
        band(),
    )
    .expect("split half-cap must certify at interval");
    let exact = RS * RS * PI * (1.0 - 0.5_f64.sin());
    let (lo, hi) = (fc.area.lo(), fc.area.hi());
    assert!(
        lo <= exact && exact <= hi && (hi - lo) < 1e-9,
        "interval area [{lo:e}, {hi:e}] must tightly enclose {exact:e}"
    );
}

// ------------------------------------------------------------------
// The rim lever — separations spanning the band
// ------------------------------------------------------------------

/// The 893 staircase at rim separations spanning the band: chords of
/// `0.25·zero` and `0.5·zero` accept (one level by this ε);
/// `10·escalate` and `1000·escalate` refuse by the named predicate;
/// the seam between `zero` and `escalate` may go either way but must
/// REFUSE typed (Escalated), never accept-and-measure the staircase.
#[test]
fn probe_near_polar_separation_ladder() {
    let b = band();
    let d0 = 0.002;
    let v2 = core::f64::consts::FRAC_PI_2 - d0;
    let ladder: [(f64, &str); 5] = [
        (0.25 * b.zero(), "accept"),
        (0.5 * b.zero(), "accept"),
        (3.0 * b.zero().max(0.3 * b.escalate()), "refuse-any"),
        (10.0 * b.escalate(), "refuse-iso"),
        (1000.0 * b.escalate(), "refuse-iso"),
    ];
    for (sep, want) in ladder {
        let dv = sep / RS;
        let edges = vec![
            rim(0.2, -1.0, 1.0, 0, 1),
            great(1.0, 0.2, v2, 1, 2),
            rim(v2, 1.0, 0.0, 2, 3),
            great(0.0, v2, v2 - dv, 3, 4),
            rim(v2 - dv, 0.0, -1.0, 4, 5),
            great(-1.0, v2 - dv, 0.2, 5, 0),
        ];
        let got = curved_face(&sphere(), &edges, 1.0, b);
        match want {
            "accept" => {
                got.unwrap_or_else(|e| panic!("sep {sep:e}: sub-band step refused: {e:?}"));
            }
            "refuse-iso" => assert!(
                matches!(
                    got,
                    Err(PropsError::NotIsoRectangle {
                        what: "props_rim_level"
                    })
                ),
                "sep {sep:e}: staircase must refuse by props_rim_level: {got:?}"
            ),
            "refuse-any" => assert!(
                got.is_err(),
                "sep {sep:e} (ambiguity seam): must refuse typed, got {got:?}"
            ),
            _ => unreachable!(),
        }
    }
}

/// The SAME separation at the equator and near the pole must decide
/// the same way now that the chord is direction-true: 10·escalate of
/// true separation refuses in both regimes (the old axial lever
/// refused at the equator and accepted near the pole).
#[test]
fn probe_lever_is_latitude_uniform() {
    let b = band();
    let dv = 10.0 * b.escalate() / RS;
    for base in [0.1, 1.2, core::f64::consts::FRAC_PI_2 - 0.002] {
        let (v1, v2) = (base - dv, base);
        let edges = vec![
            rim(0.05, -1.0, 1.0, 0, 1),
            great(1.0, 0.05, v2, 1, 2),
            rim(v2, 1.0, 0.0, 2, 3),
            great(0.0, v2, v1, 3, 4),
            rim(v1, 0.0, -1.0, 4, 5),
            great(-1.0, v1, 0.05, 5, 0),
        ];
        assert!(
            matches!(
                curved_face(&sphere(), &edges, 1.0, b),
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_level"
                })
            ),
            "staircase at base latitude {base} must refuse"
        );
    }
}

// ------------------------------------------------------------------
// The boundary of the served lane
// ------------------------------------------------------------------

/// A polar CAP — one full rim at latitude `b`, no meridians: the pole
/// is in the face INTERIOR and no edge's span can fold it. Recording
/// the disposition: the extent fix cannot see this face (its only
/// level list is `{sin b}`), so it must refuse `DegenerateFace` — the
/// same lo == hi artifact the no-split twin just retired, one arm
/// over. Executed to pin what is true at this head.
#[test]
fn probe_full_polar_cap_disposition() {
    let b = 0.5;
    let edges = vec![rim(b, 0.0, 2.0 * PI, 0, 0)];
    let got = curved_face(&sphere(), &edges, 1.0, band());
    assert!(
        matches!(got, Err(PropsError::DegenerateFace)),
        "recording the cap disposition changed: {got:?}"
    );
}

/// A GENUINE near-polar rectangular band (both rims at the extremes,
/// domain honestly [0,1] × [v1, v2]) whose rims are 10·escalate of
/// TRUE separation apart: the chord lever now measures them distinct,
/// but `require_extent` and `props_rim_side` still meter the AXIAL
/// sine difference `(hi − lo) × R ≈ 10·escalate·cos v̄` — in-band at
/// d0 = 0.002 — so the face refuses (DegenerateFace/Escalated) even
/// though its true width is decisively nonzero. Refusing direction —
/// recording the disposition, not asserting it correct.
#[test]
fn probe_near_polar_true_rectangle_disposition() {
    let b = band();
    let d0 = 0.002;
    let v2 = core::f64::consts::FRAC_PI_2 - d0;
    let dv = 10.0 * b.escalate() / RS;
    let v1 = v2 - dv;
    let edges = vec![
        rim(v1, 0.0, 1.0, 0, 1),
        great(1.0, v1, v2, 1, 2),
        rim(v2, 1.0, 0.0, 2, 3),
        great(0.0, v2, v1, 3, 0),
    ];
    let got = curved_face(&sphere(), &edges, 1.0, b);
    assert!(
        got.is_err(),
        "recording: the near-polar true rectangle now returns {got:?}"
    );
    eprintln!("near-polar true rectangle: {got:?}");
}
