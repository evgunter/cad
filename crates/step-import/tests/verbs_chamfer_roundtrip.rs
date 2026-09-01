//! **VERBS-CHAMFER's interchange row**: the chamfered cube survives a
//! STEP write and read.
//!
//! The chamfer's whole claim is that its output is exact analytic
//! geometry — planes, straight edges, a planar patch per corner —
//! rather than a fitted band. A STEP round trip is the outside
//! evidence for that: the AP214 analytic subset can carry a plane and
//! a line, so a body that survives the trip with its census, its
//! tiers and its certified volume intact is a body that was made of
//! the shapes the writer has words for.
//!
//! It lives here rather than in `sweep` because this is the crate that
//! dev-depends on BOTH halves of the trip.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::census;
use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::chamfer::chamfer_edges;
use sweep::{Extrusion, extrude};

/// The cube side and the chamfer setback, meters.
const L: f64 = 1.0;
const D: f64 = 0.1;

/// A unit cube with all twelve edges chamfered, built through the
/// public doors a consumer would use.
fn chamfered_cube() -> topo::Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let cube = extrude(&profile, Extrusion::Distance(L), Tol::witness())
        .expect("the cube extrudes")
        .body;
    let edges: Vec<topo::EdgeKey> = cube.edges().map(|(k, _)| k).collect();
    chamfer_edges(&cube, &edges, D, Tol::witness())
        .expect("a cube's twelve edges chamfer")
        .body
}

/// **Write, read, compare.** Census, all three validity tiers and the
/// certified volume come back unchanged, and the file carries the
/// analytic entities rather than a fitted surface.
#[test]
fn the_chamfered_cube_round_trips_through_step() {
    let native = chamfered_cube();
    let text = step_export::step_string(
        &native,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the chamfered cube exports");
    assert!(
        text.contains("PLANE(") && text.contains("LINE("),
        "the file must carry the analytic entities the chamfer mints"
    );

    let back = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the chamfered cube must re-import as a solid, got {other:?}"),
    };

    assert_eq!(census(&back), census(&native), "census across the trip");
    assert_eq!(topo::validate(&back), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&back), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&back, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    let want = topo::mass_properties(&native, Tol::witness()).expect("native props");
    let got = topo::mass_properties(&back, Tol::witness()).expect("imported props");
    assert!(
        (got.volume - want.volume).abs() <= 1e-12 * want.volume,
        "volume across the trip: {} vs {}",
        got.volume,
        want.volume
    );
}
