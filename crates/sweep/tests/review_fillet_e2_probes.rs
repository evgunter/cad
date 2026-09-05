//! **FILLET-E2 review probes** — front-door witnesses against two of
//! the "unreachable" verdicts in `blend_recourse_followability.rs`,
//! and a characterization of the chain gate that stands behind a
//! third.
//!
//! - `FILLET3_RING_RECOURSE` IS handed to a caller: the clearance
//!   screen samples each boundary edge at `CHAIN_SAMPLES = 9` places (a
//!   45° lattice on a circle), so a ring whose closest approach to a
//!   requested edge sits off that lattice reads a sampled gap larger
//!   than the true one, the screen passes, and the surgery's exact ring
//!   check refuses `RingClearance`. The sentence is then followable:
//!   the reduced size builds.
//! - `FILLET3_GEOMETRY_RECOURSE` IS handed to a caller: a square
//!   pocket leaves a ring of LINE carriers on the top face, and
//!   `ring_circle` refuses every outer-edge fillet at every radius. As
//!   filed this was also a dead recourse of issue 1278's class — the
//!   sentence endorsed planar supports on line/circle carriers, which
//!   the request already had. The sentence has since been rewritten to
//!   name the ring and the order that answers it; this row keeps the
//!   witness, and the followability suite executes the order.
//! - `CORNER_SUPPORT_NOT_PLANAR` stays unreachable for the reason the
//!   chain gate states: an open chain's supports must be plane–plane at
//!   every link, so no corner with a curved support is ever admitted.
//!   And a plane–CYLINDER closed rim carves, which the assembly
//!   recourse's "circular plane–sphere rims" does not say.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::blend::{
    BlendError, FILLET3_ASSEMBLY_RECOURSE, FILLET3_GEOMETRY_RECOURSE, FILLET3_RING_RECOURSE,
};
use sweep::test_support::{arcs_at, cube, dome_profile, prism, revolved_about_y, rim_arcs_at};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::RimError;
use topo::boolean::{BooleanDeclarations, BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, EdgeKey, query, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn v(x: f64, y: f64, bulge: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(p2(x, y), bulge)
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    boolean_op_with(
        BooleanOp::Subtract,
        a,
        b,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        tol(),
    )
    .expect("the subtraction runs")
    .body()
    .expect("the subtraction leaves a body")
    .body
    .clone()
}

/// A sphere of radius 0.3 centred at `c`.
fn ball_at(c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![v(0.0, -0.3, 1.0), v(0.0, 0.3, 0.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let b = revolve(&vp, axis, Revolution::Full, tol()).unwrap().body;
    topo::transform_rigid(&b, &Affine3::translation(c), tol()).unwrap()
}

/// The edges of `body` on line carriers.
fn line_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|k| {
            body.get_edge(*k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(*c.carrier(), geom::Curve3::Line { .. }))
        })
        .collect()
}

/// The refusal a request meets, or a panic naming what built instead.
fn refusal(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) -> BlendError {
    match fillet_edges(body, edges, r, tol()) {
        Err(e) => e.error,
        Ok(_) => panic!("{what}: expected a refusal, the request built"),
    }
}

/// The request builds and passes tier-3 validation.
fn builds(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) {
    let out = fillet_edges(body, edges, r, tol())
        .unwrap_or_else(|e| panic!("{what}: the request must build, got {e:?}"));
    validate_geometric(&out.body, tol())
        .unwrap_or_else(|e| panic!("{what}: and the result must be tier-3 valid, got {e:?}"));
}

/// **`FILLET3_RING_RECOURSE` reaches the front door, and is followable.**
///
/// A 2×2×2 square prism turned 30° about its axis, dimpled at its top
/// centre by a radius-0.3 sphere: the ring (radius √0.08 = 0.2828) sits
/// 0.7172 from each top edge, and its closest points lie 15° off the
/// screen's 45° sample lattice, so the screen reads the gap as
/// `1 − 0.2828·cos 15° = 0.7268`. Setbacks between the two are passed
/// by the screen and refused by the exact ring check.
///
/// Red when the screen stops overestimating a sampled gap, or the
/// surgery stops checking rings exactly. Its lattice-ALIGNED twin, on
/// which the screen does answer first, is
/// `blend_recourse_followability::the_ring_recourse_is_screened_first_on_a_lattice_aligned_dimple`;
/// the pair is what separates the fixture's property from the door's.
#[test]
fn the_ring_recourse_reaches_the_front_door_off_the_sample_lattice_and_is_followable() {
    let hd = core::f64::consts::SQRT_2;
    let turned = prism(
        (0..4)
            .map(|k| {
                let th = (75.0 + 90.0 * f64::from(k)).to_radians();
                v(1.0 + hd * th.cos(), 1.0 + hd * th.sin(), 0.0)
            })
            .collect(),
        2.0,
        tol(),
    );
    let dimpled = subtract(&turned, &ball_at(Vec3::new(1.0, 1.0, 2.1)));
    validate_geometric(&dimpled, tol()).expect("the dimpled prism is valid");
    let edges = line_edges(&dimpled);
    assert_eq!(edges.len(), 12, "the dimple leaves the twelve box edges");

    let err = refusal(&dimpled, &edges, 0.72, "a setback inside the sampled gap");
    let BlendError::RingClearance { margin, .. } = err else {
        panic!("the exact ring check is what refuses past the screen, got {err:?}")
    };
    assert_eq!(margin.predicate, "fillet3_ring_clearance");
    assert!(
        margin.value().is_some_and(|m| m < 0.0 && m > -0.01),
        "the ring sits 0.7172 from the edge and the setback is 0.72: {margin}"
    );
    assert!(
        err.to_string().contains(FILLET3_RING_RECOURSE),
        "the caller is handed the ring recourse: {err}"
    );
    // Followed: "reduce the blend size" builds.
    builds(&dimpled, &edges, 0.715, "the reduced blend size");
    // And past the sampled gap the screen answers, as the PR's row pins
    // on its axis-aligned fixture.
    let screened = refusal(&dimpled, &edges, 0.73, "a setback past the sampled gap");
    assert!(
        matches!(screened, BlendError::FaceClearanceUncertified { .. }),
        "the screen answers once the sampled gap is exceeded too, got {screened:?}"
    );
}

/// **`FILLET3_GEOMETRY_RECOURSE` reaches the front door at a line
/// ring.**
///
/// A square pocket protruding through the cube's top face leaves a
/// ring of four LINE carriers. `ring_circle` reads circle rings only,
/// so every outer-edge fillet refuses `UnsupportedGeometry` — at 0.05
/// as at 0.3 — and no radius builds.
///
/// As filed this row was named `…_and_cannot_be_followed`, and it was
/// right: the sentence endorsed planar supports on line and circle
/// carriers, which is exactly what the twelve requested edges already
/// were, so following it changed nothing. The sentence has since been
/// rewritten to name the RING and the order that answers it, and
/// `blend_recourse_followability::the_geometry_recourse_names_a_ring_and_an_order_that_builds`
/// executes that order. This row keeps the witness — the refusal, at
/// every radius, carrying the geometry recourse — which is what makes
/// the other row's premise true.
#[test]
fn the_geometry_recourse_reaches_the_front_door_at_a_line_ring() {
    let pocket = topo::transform_rigid(
        &cube(0.3, tol()),
        &Affine3::translation(Vec3::new(0.35, 0.35, 0.8)),
        tol(),
    )
    .unwrap();
    let body = subtract(&cube(1.0, tol()), &pocket);
    validate_geometric(&body, tol()).expect("the pocketed cube is valid");
    let mid = |k: EdgeKey| -> Point3<f64> {
        let e = body.get_edge(k).unwrap();
        let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = g.params();
        g.carrier().eval((t0 + t1) / 2.0)
    };
    let on = |c: f64| c.abs() < 1e-9 || (c - 1.0).abs() < 1e-9;
    let outer: Vec<EdgeKey> = line_edges(&body)
        .into_iter()
        .filter(|k| {
            let m = mid(*k);
            on(m.x) || on(m.y)
        })
        .collect();
    assert_eq!(outer.len(), 12, "the outer box's twelve edges");

    for r in [0.05, 0.1, 0.3] {
        let err = refusal(&body, &outer, r, "the outer edges of a pocketed box");
        assert!(
            matches!(err, BlendError::UnsupportedGeometry { .. }),
            "r = {r}: the ring's line carriers are what refuse, got {err:?}"
        );
        let shown = err.to_string();
        assert!(
            shown.contains("a ring edge's carrier is not a circle")
                && shown.contains(FILLET3_GEOMETRY_RECOURSE),
            "r = {r}: the caller reads the geometry recourse about a ring: {shown}"
        );
    }
}

/// An edge's certified carrier-parameter interval — the fixture-side
/// read the open-arc row measures its refusal's `gap` against.
fn carrier_params(body: &Body<f64>, k: EdgeKey) -> (f64, f64) {
    body.get_curve_geom(body.get_edge(k).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .params()
}

/// **The chain gate behind `CORNER_SUPPORT_NOT_PLANAR`, and the rim the
/// assembly recourse does not name.**
///
/// A half-revolved dome's equator is an open plane–sphere arc ending at
/// corners whose third face is the sphere — the request that would
/// reach the "corner support is not a plane" geometry site. It never
/// does: the chain gate requires plane–plane supports on every open
/// link, and answers with the assembly recourse. That recourse names
/// "circular plane–sphere rims" as the closed chains that carve; a
/// cylinder's plane–cylinder top rim carves too.
#[test]
fn open_plane_sphere_arcs_meet_the_chain_gate_and_a_plane_cylinder_rim_carves() {
    let half = revolved_about_y(
        dome_profile(1.0),
        Revolution::Partial(core::f64::consts::PI),
        tol(),
    );
    let arcs = arcs_at(&half, 1.0, 0.0);
    assert!(!arcs.is_empty(), "the half dome keeps its equator arcs");
    for a in &arcs {
        // The arc is NOT a rim, and the rim door is what says so: a half
        // revolve's arcs do not close, so `rim_of` names the matched set
        // and the parameter it stops at rather than handing back a
        // partial rim a fillet request would then stall on.
        match topo::query::rim_of(&half, *a) {
            Err(RimError::NotOneRim { arcs: matched, gap }) => {
                // The door matches on this arc's OWN circle and its OWN
                // support pair, so it names FEWER arcs than the radius
                // scan found — the scan's other hits at this radius sit
                // between different surfaces. A door that matched across
                // support pairs would fail here.
                assert!(
                    matched.len() < arcs.len(),
                    "the door is narrower than the radius scan: it named \
                     {matched:?} of the scan's {arcs:?}"
                );
                // And the walk stops at one of THIS arc's own ends: an
                // open arc dangles at the end it does not close onto.
                let ends = carrier_params(&half, *a);
                let wrapped = |t: f64| {
                    let x = t.rem_euclid(core::f64::consts::TAU);
                    if x > PI {
                        x - core::f64::consts::TAU
                    } else {
                        x
                    }
                };
                assert!(
                    (gap - wrapped(ends.0)).abs() < 1e-9 || (gap - wrapped(ends.1)).abs() < 1e-9,
                    "the gap is at one of the arc's own endpoints {ends:?}, got {gap}"
                );
            }
            other => panic!("an open arc is not one rim, got {other:?}"),
        }
        let err = refusal(&half, &[*a], 0.05, "an open plane–sphere arc");
        assert!(
            matches!(err, BlendError::UnsupportedChain { .. }),
            "the chain gate answers before any corner is admitted, got {err:?}"
        );
        let shown = err.to_string();
        assert!(
            shown.contains("an open chain's supports are not plane–plane")
                && shown.contains(FILLET3_ASSEMBLY_RECOURSE),
            "{shown}"
        );
    }

    let cyl = revolved_about_y(
        vec![
            v(0.0, 0.0, 0.0),
            v(1.0, 0.0, 0.0),
            v(1.0, 1.0, 0.0),
            v(0.0, 1.0, 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let top = rim_arcs_at(&cyl, 1.0, 1.0);
    assert!(!top.is_empty(), "the cylinder's top rim");
    builds(&cyl, &top, 0.1, "a closed plane–cylinder rim");
    assert!(
        !FILLET3_ASSEMBLY_RECOURSE.contains("cylinder"),
        "the assembly recourse does not name the plane–cylinder rim that carves: \
         {FILLET3_ASSEMBLY_RECOURSE}"
    );
}
