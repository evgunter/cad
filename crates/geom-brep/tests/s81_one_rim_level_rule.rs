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
//! **The torus has since left the rule's flux arm**: its reader is the
//! chart Green form over the loop's lift, which reads each rim at its
//! own level and groups nothing; its rows below say what that reads.
//!
//! The rows here are the two directions of the change: a face that was
//! refused and should not have been, and the refusal floor that keeps
//! the merge from being a rule that groups everything.
//!
//! **Every offset comes from the run's own `Band`, never from a
//! literal.** This suite is on CI's `eps ∈ {default, 1e-6, 1e-12}`
//! matrix, and an ε-literal states a claim about one of the three.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face};

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

/// **A rim split half an ε off level measures, at its own level.**
///
/// The wobble is taken from the run's OWN band, never from a literal —
/// this file is on the `eps ∈ {default, 1e-6, 1e-12}` matrix, and a
/// literal states a claim about one of the three. `MINOR · wobble =
/// 0.5 · band.zero()`, so the split arc is displaced half a
/// coincidence threshold, and the loop closes within the band.
///
/// The torus reader is the chart Green form `−∮ G(v) du`, which reads
/// each rim at the level its carrier states rather than grouping rims
/// by a level rule: the wobbled arc contributes at its own level, so
/// the area differs from the unwobbled band's by exactly that arc's
/// strip — `Δu · r(R + r cos v)·wobble` to first order — and by no more.
/// Metered at `major`, the old grouping rule split the arcs into two
/// groups and refused `props_du_consistent`; nothing here groups.
#[test]
fn a_rim_arc_split_within_epsilon_of_its_level_measures_at_its_own_level() {
    let band = band();
    let (va, vb) = (0.2, 0.7);
    let wobble = 0.5 * band.zero() / MINOR;
    let (s, edges) = gasket_band(va, vb, wobble);
    let got = curved_face(&s, &edges, true, band)
        .expect("a rim wobbled half an epsilon still bounds the band");
    let exact = exact_area(va, vb);
    let strip = 0.4 * MINOR * (MAJOR + MINOR * va.cos()) * wobble;
    let off = (got.area - exact).abs();
    assert!(
        (off - strip).abs() <= 1e-3 * strip + 1e-15,
        "area {:.15e} vs exact {exact:.15e}: off by {off:.3e}, the wobbled strip is {strip:.3e}",
        got.area
    );
}

/// **The floor.** Ten times the run's own ESCALATE threshold off its
/// level, the split arc's ends no longer meet its neighbours' at any ε
/// and any K: the loop does not close, and the reader refuses it
/// (`props_loop_closed`) rather than integrate a boundary that bounds
/// nothing.
#[test]
fn a_rim_arc_well_outside_the_band_is_still_refused() {
    let band = band();
    let (s, edges) = gasket_band(0.2, 0.7, 10.0 * band.escalate() / MINOR);
    assert!(
        matches!(
            curved_face(&s, &edges, true, band),
            Err(PropsError::NotIsoRectangle {
                what: "props_loop_closed"
            })
        ),
        "a rim decisively off its level leaves the loop open"
    );
}

/// The control: no wobble at all, so nothing about the split can be
/// what carries the row above.
#[test]
fn the_unwobbled_split_rim_measures_exactly() {
    let (va, vb) = (0.2, 0.7);
    let (s, edges) = gasket_band(va, vb, 0.0);
    let got = curved_face(&s, &edges, true, band()).expect("computes");
    let exact = exact_area(va, vb);
    assert!((got.area - exact).abs() / exact < 1e-12);
}
