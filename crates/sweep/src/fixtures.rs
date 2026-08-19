//! **Fixtures for this crate's in-crate pins.** The sites they cover
//! are private to their modules, so those pins cannot live in
//! `tests/`; hosting the bodies here rather than inside one of the
//! modules that uses them keeps neither test module the owner of the
//! other's fixture.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::{Band, Point2, Tolerance};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, EdgeKey};

use crate::fillet::battery::{FilletRequest, Link, run_battery};
use crate::{Extrusion, extrude};

/// The cube's side, meters.
pub(crate) const L: f64 = 1.0;
/// The blend radius, meters.
pub(crate) const R: f64 = 0.1;

/// An axis-aligned cube: eight trivalent corners, every one of them
/// geometrically CONVEX.
pub(crate) fn cube() -> Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(L)).unwrap().body
}

/// Every edge of `body` resolved by the fillet battery, in edge order
/// — the same list `whole_body_links` hands the plan.
pub(crate) fn all_links(body: &Body<f64>) -> Vec<Link<f64>> {
    let tol = Tolerance::get();
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let verdict = run_battery(
        &FilletRequest {
            body,
            edges,
            radius: R,
        },
        Band::new(tol.eps, tol.k * tol.eps).unwrap(),
    )
    .expect("the battery resolves every edge of a cube");
    let mut links: Vec<Link<f64>> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links.iter().cloned())
        .collect();
    links.sort_by_key(|l| l.edge);
    links
}
