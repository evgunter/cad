//! **The tour's one seed finder: "the circle at radius `r`, station
//! `y`"** — the consumer-side twin of `sweep::test_support::arcs_at`,
//! shared by every suite in this cargo root through `#[path]`.
//!
//! The tour is a detached cargo root that reaches the kernel through
//! the `pncad` façade. `sweep`'s `test_support` module is compiled only
//! under the crate's own `test-support` feature, and the only edge that
//! turns it on for this root would also turn it on for the `sweep`
//! artifact the `demo-tour` binary links in the same invocation — every
//! `--all-targets` build, which is what CI runs here. So the scan is
//! spelled on this side of the façade, once: a suite driving the kernel
//! from an outside consumer's seat writes what a consumer can write.
//!
//! `1e-9` on both halves is the kernel home's window and its reason: a
//! fixture states its rims analytically, so this is a
//! fixture-selection tolerance and not a kernel predicate. The door it
//! feeds (`query::rim_of`) carries no tolerance at all.
//!
//! Naming a rim by its geometry at all is the consumer-door gap
//! `no-public-rim-arc-selector` owns: `rim_of` takes a seed EDGE, so
//! the library answers "which rim is this arc's" and leaves "which arc
//! do I mean" to the caller.

#![allow(dead_code)]

use pncad::geom::Curve3;
use pncad::prelude::query;
use pncad::topo::{Body, EdgeKey};

/// Which arcs a scan will accept as a seed — the one axis on which the
/// tour's suites differ, because their fixtures do.
#[derive(Clone, Copy, Debug)]
pub enum Seeds {
    /// CLOSED circle edges only. A revolved body whose profile stays
    /// off the axis mints one closed edge per latitude rim, and a
    /// closed edge is never a chart seam.
    Closed,
    /// Circle edges whose two supports are DIFFERENT surfaces. A
    /// pole-touching profile seam-splits every rim into arcs, so
    /// closedness cannot be the filter; excluding a co-surface pair is
    /// what keeps a seam meridian from being the seed.
    TwoSided,
}

/// Every circle edge at radius `r` and station `y` that `seeds` admits,
/// in key order — the raw scan, and deliberately NOT a rim.
#[must_use]
pub fn arcs_at(body: &Body<f64>, r: f64, y: f64, seeds: Seeds) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            let Some(e) = body.get_edge(k) else {
                return false;
            };
            let admitted = match seeds {
                Seeds::Closed => {
                    body.get_half_edge(e.he_plus)
                        .map(|h| Some(h.start) == body.half_edge_end(e.he_plus))
                        == Some(true)
                }
                Seeds::TwoSided => {
                    let surface_of = |he| {
                        let l = body.get_half_edge(he)?.parent_loop;
                        Some(body.get_face(body.get_loop(l)?.face)?.surface)
                    };
                    match (surface_of(e.he_plus), surface_of(e.he_minus)) {
                        (Some(a), Some(b)) => a != b,
                        _ => false,
                    }
                }
            };
            if !admitted {
                return false;
            }
            let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
                return false;
            };
            matches!(*c.carrier(), Curve3::Circle { center, radius, .. }
                if (center.y - y).abs() < 1e-9 && (radius - r).abs() < 1e-9)
        })
        .collect()
}

/// **The rim at radius `r` and station `y`, whole.** The scan names one
/// of its arcs by the numbers the fixture states, and `query::rim_of`
/// hands back the rim that arc belongs to. An empty answer means no arc
/// sits there, which is what a suite asserting a rim's absence means.
///
/// # Panics
///
/// If the door refuses the arc the scan chose: that is a statement
/// about the FIXTURE, and it is louder here than as an empty answer.
#[must_use]
pub fn rim_at(body: &Body<f64>, r: f64, y: f64, seeds: Seeds) -> Vec<EdgeKey> {
    match arcs_at(body, r, y, seeds).first() {
        None => Vec::new(),
        Some(seed) => query::rim_of(body, *seed)
            .unwrap_or_else(|e| panic!("the rim at radius {r}, station {y} is one rim: {e}")),
    }
}
