//! **S81: "these two rim levels are the same level" is one rule.**
//!
//! `props/curved.rs` decided it twice — once to GROUP a rim's arcs in
//! `du_of_rims`, once to place every rim at an extreme in
//! `require_rims_at_extremes` — and the two disagreed on three things:
//! the metric (componentwise `Δsin`/`Δcos` against the Euclidean chord
//! `√(Δs² + Δc²)`), the lever arm on the torus (`major` against
//! `minor`, on consecutive lines of `torus()`), and the direction a
//! structurally impossible input fails in. Both now go through
//! `level_coincides`, at the arm the level's own dimension names.
//!
//! **Which arm won: `minor`, the exact one.** A `RimLevel::Unit` pair
//! on the torus is a pair of MINOR-circle directions; the point
//! deviation an angular difference between them induces is that
//! difference at the minor radius, because that is the radius the
//! direction turns about. `major` is the azimuthal lever — right for a
//! Δu angle and for a ±1 traversal-direction difference, which is why
//! `du_of_rims` still meters those at `major`, and wrong for a level.
//! It overstated by `major / minor`; on the gasket below that is 1000,
//! and the row is what the overstatement costs.
//!
//! The rows here are the two directions of the change: a face that was
//! refused and should not have been, and the refusal floor that keeps
//! the merge from being a rule that groups everything.
//!
//! **Every offset comes from the run's own `Band`, never from a
//! literal.** This suite is on CI's `eps ∈ {default, 1e-6, 1e-12}`
//! matrix, and an ε-literal states a claim about one of the three.
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

/// A gasket: a 1 mm tube on a 1 m ring, so `major / minor = 1000` and
/// the two candidate levers are three orders apart. Ordinary geometry
/// — an O-ring groove's wall.
const MAJOR: f64 = 1.0;
const MINOR: f64 = 0.001;

/// The band face `[0, 1.1] × [va, vb]` in `(u, v)`, with the BOTTOM rim
/// arriving as two arcs split at `u = 0.7` and the second arc's minor
/// angle displaced by `wobble` radians — the split vertex a boolean or
/// a re-merge leaves a hair off level.
fn gasket_band(va: f64, vb: f64, wobble: f64) -> (Surface<f64>, Vec<LoopEdge<f64>>) {
    let s = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: MAJOR,
        minor_radius: MINOR,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = |v: f64, u0: f64, u1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, MINOR * v.sin()),
                axis: v3(0.0, 0.0, 1.0),
                radius: MAJOR + MINOR * v.cos(),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            a,
            b,
        )
    };
    let mer = |u: f64, v0: f64, v1: f64, a: u32, b: u32| {
        edge(
            Curve3::Circle {
                center: p3(MAJOR * u.cos(), MAJOR * u.sin(), 0.0),
                axis: v3(u.sin(), -u.cos(), 0.0),
                radius: MINOR,
                u_ref: v3(u.cos(), u.sin(), 0.0),
            },
            v0,
            v1,
            a,
            b,
        )
    };
    let edges = vec![
        rim(va, 0.0, 0.7, 0, 1),
        rim(va + wobble, 0.7, 1.1, 1, 2),
        mer(1.1, va, vb, 2, 3),
        rim(vb, 1.1, 0.0, 3, 4),
        mer(0.0, vb, va, 4, 0),
    ];
    (s, edges)
}

fn exact_area(va: f64, vb: f64) -> f64 {
    MINOR * 1.1 * (MAJOR * (vb - va) + MINOR * (vb.sin() - va.sin()))
}

/// **A rim split half an ε off level is one rim.**
///
/// The wobble is taken from the run's OWN band, never from a literal —
/// this file is on the `eps ∈ {default, 1e-6, 1e-12}` matrix, and a
/// literal states a claim about one of the three. `MINOR · wobble =
/// 0.5 · band.zero()`, so the split arc is displaced half a
/// coincidence threshold: the two arcs are the same level by the run's
/// own tolerance, and the face is a genuine iso-rectangle.
///
/// Metered at `major` instead, the SAME angle reads
/// `0.5 · (MAJOR/MINOR) · cos v · zero ≈ 490 · zero` — past `escalate`
/// at any K — so the two arcs became two groups whose span sums (0.7
/// and 0.4) then disagreed: **measured on this branch's parent,
/// `NotIsoRectangle { what: "props_du_consistent" }`.**
///
/// Goes red by putting `major` back on the level margin — the group
/// splits and the refusal returns — and red the other way if the area
/// stops being exact.
#[test]
fn a_rim_arc_split_within_epsilon_of_its_level_stays_one_group() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (va, vb) = (0.2, 0.7);
    let (s, edges) = gasket_band(va, vb, 0.5 * band.zero() / MINOR);
    let got =
        curved_face(&s, &edges, 1.0, band).expect("a rim wobbled half an epsilon is still one rim");
    let exact = exact_area(va, vb);
    let rel = (got.area - exact).abs() / exact;
    assert!(
        rel < 1e-12,
        "area {:.15e} != exact {exact:.15e} (rel {rel:.3e})",
        got.area
    );
}

/// **The floor that keeps the merge honest.** The exact lever must
/// still REFUSE a genuinely distinct level, or "one rule at the minor
/// radius" would just be a rule that groups everything. Ten times the
/// run's own ESCALATE threshold is decisively outside the band at any ε
/// and any K, and the arc is then not at either extreme: the
/// iso-rectangle predicate — the same rule, the same arm — refuses it.
/// That refusal is the one the parent gave too; what changed is only
/// where the boundary between the two answers sits, and it now sits at
/// the level's own lever.
#[test]
fn a_rim_arc_well_outside_the_band_is_still_refused() {
    let band = Band::linear(Tol::witness()).unwrap();
    let (s, edges) = gasket_band(0.2, 0.7, 10.0 * band.escalate() / MINOR);
    assert!(
        matches!(
            curved_face(&s, &edges, 1.0, band),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_level"
            })
        ),
        "a rim decisively off its level is not at an extreme"
    );
}

/// The control: no wobble at all, so nothing about the split can be
/// what carries the row above.
#[test]
fn the_unwobbled_split_rim_measures_exactly() {
    let (va, vb) = (0.2, 0.7);
    let (s, edges) = gasket_band(va, vb, 0.0);
    let got =
        curved_face(&s, &edges, 1.0, Band::linear(Tol::witness()).unwrap()).expect("computes");
    let exact = exact_area(va, vb);
    assert!((got.area - exact).abs() / exact < 1e-12);
}
