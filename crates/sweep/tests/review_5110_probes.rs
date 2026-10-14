//! Review probes for PR 5110 (`band/cap-sliver-unpinned-terms`).
//!
//! With the end curve's terms gone, `reach` and the floors of
//! `CapSliver::of` come from the two rim pieces alone, so on a rim ARC
//! whose window holds its circle's critical point they come from
//! `CircleFrame`'s whole-circle arms (`far` and `low`). This file is the
//! front door the review used to look for such a rim: a prism whose
//! profile corner joins a line to an arc, filleted along the corner.
//! With `CapSliver::of` instrumented to log each rim piece's
//! whole-circle-versus-ends gap, no configuration below (nor any of
//! 1164 sweep rows) gave a circle rim a gap above 1e-15, and an
//! `of`-scoped mutant forcing every arm to its ends turned all 578 cap
//! rows green.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Point2};
use profile::SketchPlane;
use profile::{ProfileLoop, test_support::bulge_loop};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{extruded, finished};
use topo::{Body, EdgeKey, query};

use crate::band_planar_cut_off::tol;

fn mid<T: Bounds>(x: T) -> f64 {
    0.5 * (x.lo() + x.hi())
}

/// A prism along `z` over `outline` (with `holes`), height 1.
fn prism_over(outline: ProfileLoop<f64>, holes: Vec<ProfileLoop<f64>>) -> topo::AtRestBody<f64> {
    let mut loops = vec![outline];
    loops.extend(holes);
    finished(
        "probe",
        extruded(SketchPlane::xy(), loops, 1.0, tol()),
        tol(),
    )
}

/// The vertical edge at `(x, y)`.
fn vertical_edge(body: &Body<f64>, x: f64, y: f64) -> EdgeKey {
    let at = |p: geom_core::Point3<f64>| (mid(p.x) - x).abs() < 1e-9 && (mid(p.y) - y).abs() < 1e-9;
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            at(p(body.get_half_edge(he).unwrap().start)) && at(p(body.half_edge_end(he).unwrap()))
        })
        .expect("the vertical edge")
}

pub(crate) fn loop_of(pts: &[(f64, f64, f64)]) -> ProfileLoop<f64> {
    bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    )
}

/// Fillet the vertical edge at `v` of the prism over `outline` (+holes)
/// at radius `r`; `Ok(faces)` or the refusal.
pub(crate) fn fillet_at(
    outline: ProfileLoop<f64>,
    holes: Vec<ProfileLoop<f64>>,
    v: (f64, f64),
    r: f64,
) -> Result<usize, BlendError> {
    let body = prism_over(outline, holes);
    let e = vertical_edge(&body, v.0, v.1);
    let at_rest = sweep::test_support::at_rest(&body, tol());
    fillet_edges(&at_rest, &[e], r, tol())
        .map(|o| o.body.faces().count())
        .map_err(|e| e.error)
}

/// The corner `(1, 1)` between the profile line `y = 1` and an arc
/// `(1, 0) → (1, 1)` of bulge `b` (outward for `b > 0`), filleted at
/// `r`: every one builds. Evidence for the review, not a gate.
#[test]
#[ignore = "exploration: cargo nextest run -p sweep --run-ignored all -E 'test(review_5110_probes::explore)'"]
fn explore_arc_rim_corners() {
    for b in [0.1, 0.3, 0.6, 0.9, -0.1, -0.3] {
        for r in [0.1, 0.25, 0.4] {
            let outline = loop_of(&[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, b),
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
            ]);
            let got = fillet_at(outline, vec![], (1.0, 1.0), r);
            eprintln!("bulge {b} r {r}: {got:?}");
        }
    }
}
