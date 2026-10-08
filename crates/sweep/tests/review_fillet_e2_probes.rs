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
//! - A polygonal ring is metered edge by edge: a pocket turned off the
//!   screen's sample lattice is cleared, crossed past the screen, and
//!   brought into the sliver band, each answered by the exact meter.
//! - `FILLET3_GEOMETRY_RECOURSE` IS handed to a caller: a tilted bore
//!   leaves an ELLIPTICAL ring on the top face, which no ring meter
//!   covers, and every outer-edge fillet refuses. The sentence names
//!   the ring and the order that answers it; the followability suite
//!   executes the order.
//! - `CORNER_SUPPORT_NOT_PLANAR` stays unreachable for the reason the
//!   chain gate states: an open chain's supports must be plane–plane at
//!   every link, so no corner with a curved support is ever admitted.
//!   And a plane–CYLINDER closed rim carves, which the assembly
//!   recourse's "circular plane–sphere rims" does not say.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::blend::build::fillet_edges;
use sweep::blend::{
    BlendError, FILLET3_ASSEMBLY_RECOURSE, FILLET3_GEOMETRY_RECOURSE, FILLET3_RING_RECOURSE,
};
use sweep::test_support::{
    arcs_at, cube, dome_profile, prism, realized, revolved_about_y, rim_arcs_at,
};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, query, validate_geometric};
use topo::{RimBreak, RimError};

fn tol() -> Tol {
    Tol::witness()
}

fn v(x: f64, y: f64, bulge: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), bulge)
}

fn subtract(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    realized(BooleanOp::Subtract, a, b, tol())
}

/// A sphere of radius 0.3 centred at `c`.
fn ball_at(c: Vec3<f64>) -> Body<f64> {
    let lp = bulge_loop(vec![v(0.0, -0.3, 1.0), v(0.0, 0.3, 0.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
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
    match fillet_edges(&sweep::test_support::at_rest(body, tol()), edges, r, tol()) {
        Err(e) => e.error,
        Ok(_) => panic!("{what}: expected a refusal, the request built"),
    }
}

/// The request builds and passes tier-3 validation.
fn builds(body: &Body<f64>, edges: &[EdgeKey], r: f64, what: &str) {
    let out = fillet_edges(&sweep::test_support::at_rest(body, tol()), edges, r, tol())
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
    let text = err.to_string();
    let BlendError::RingClearance { chain, margin, .. } = err else {
        panic!("the exact ring check is what refuses past the screen, got {err:?}")
    };
    assert_eq!(
        chain,
        sweep::blend::Convexity::Convex,
        "the prism's edges are convex"
    );
    assert!(
        text.contains(
            "lies in the part of a face the blend cuts away with the material it removes"
        ) && !text.contains("adds"),
        "a convex band cuts the ring away: {text}"
    );
    assert_eq!(margin.predicate, "fillet3_ring_clearance");
    assert!(
        margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .is_some_and(|m| m < 0.0 && m > -0.01),
        "the ring sits 0.7172 from the edge and the setback is 0.72: {margin}"
    );
    assert!(
        text.contains(FILLET3_RING_RECOURSE),
        "the caller is handed the ring recourse: {text}"
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

/// **The same refusal on the CONCAVE side, and its sentence says so.**
/// The ring check meters every support ring against every trimline
/// whatever the chain's convexity, and a concave band ADDS material: a
/// ring within its setback is buried under the band, not cut away. So
/// `RingClearance` carries the chain's convexity and renders the fate
/// that is true of it.
///
/// The fixture is the vented cavity (`common::cavity`), its round vent
/// moved to `(2.125, 1.7)`: its mouth — a ring of radius 0.5 in the
/// cavity's ceiling — sits `g = 0.2` from the ceiling's `y = 1` edge,
/// and its nearest point to that edge lies at `x = 2.125`, halfway
/// between two of the edge's `CHAIN_SAMPLES` stations (`2.0`, `2.25`).
/// The screen therefore reads `√(0.2² + 0.125²) ≈ 0.2358`, and a
/// concave setback (the radius, at a right-angled edge) between the two
/// is passed by the screen and refused by the exact ring check at
/// `g − r`. Measured: `r = 0.199` carves, `0.201` refuses here,
/// `0.24` is answered by the screen.
#[test]
fn the_ring_refusal_beside_a_concave_band_says_the_band_buries_the_ring() {
    use crate::common::cavity::{brick, cavity_edges, cut, rod};
    use sweep::blend::Convexity;
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.125, 1.7), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut("cavity", &cut("vent", &block, &vent), &cavity);
    validate_geometric(&body, tol()).expect("the off-centre vented cavity is valid");
    let edges = cavity_edges(&body);
    assert_eq!(edges.len(), 12, "the cavity's twelve concave edges");

    let err = refusal(&body, &edges, 0.22, "a setback inside the sampled gap");
    let text = err.to_string();
    let BlendError::RingClearance { chain, margin, .. } = err else {
        panic!("the exact ring check is what refuses past the screen, got {err:?}")
    };
    assert_eq!(chain, Convexity::Concave, "the cavity's chain is concave");
    assert!(
        margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .is_some_and(|m| (m - (0.2 - 0.22)).abs() < 1e-12),
        "the ring sits 0.2 from the edge and the setback is 0.22: {margin}"
    );
    assert!(
        text.contains("lies in the part of a face the blend buries under the material it adds"),
        "a concave band buries the ring: {text}"
    );
    assert!(
        !text.contains("removes"),
        "and nothing is said to be removed: {text}"
    );
    assert!(
        text.contains(FILLET3_RING_RECOURSE),
        "the caller is handed the ring recourse: {text}"
    );
    // Followed: "reduce the blend size" builds.
    builds(&body, &edges, 0.19, "the reduced blend size");
    // Past the sampled gap the screen answers first.
    let screened = refusal(&body, &edges, 0.24, "a setback past the sampled gap");
    assert!(
        matches!(screened, BlendError::FaceClearanceUncertified { .. }),
        "the screen answers once the sampled gap is exceeded too, got {screened:?}"
    );
}

/// The twelve box edges of a unit cube that has been cut into: the
/// line edges whose midpoints lie on a side face.
fn outer_box_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let mid = |k: EdgeKey| -> Point3<f64> {
        let e = body.get_edge(k).unwrap();
        let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = g.params();
        g.carrier().eval((t0 + t1) / 2.0)
    };
    let on = |c: f64| c.abs() < 1e-9 || (c - 1.0).abs() < 1e-9;
    line_edges(body)
        .into_iter()
        .filter(|k| {
            let m = mid(*k);
            on(m.x) || on(m.y)
        })
        .collect()
}

/// **A polygonal ring is metered edge by edge: clear, crossed, in
/// band.**
///
/// A diamond pocket (a square turned 45°, half-diagonal 0.15, centred
/// at `(0.45, 0.45)`) cut through a unit cube's top face leaves a ring
/// of four LINE carriers. Its vertex `(0.45, 0.30)` comes 0.30 from the
/// `y = 0` top edge (and `(0.30, 0.45)` as near the `x = 0` one), at an
/// abscissa between the screen's 1/8 sample lattice, so the screen reads
/// that gap as `√(0.30² + 0.05²) = 0.3041`. Setbacks between the two
/// pass the screen and are decided by the exact ring meter.
///
/// - r = 0.1 clears the ring by 0.2: the twelve edges build, tier-3
///   valid.
/// - r = 0.302 crosses the vertex by 0.002: `RingClearance`, the
///   margin the closed form `0.30 − r` gives.
/// - r = 0.30 less five ε leaves the vertex inside the sliver band:
///   `Escalated`, from `fillet3_ring_clearance`.
#[test]
fn a_polygonal_ring_is_metered_edge_by_edge_clear_crossed_and_in_band() {
    let diamond = prism(
        vec![
            v(0.45, 0.30, 0.0),
            v(0.60, 0.45, 0.0),
            v(0.45, 0.60, 0.0),
            v(0.30, 0.45, 0.0),
        ],
        0.3,
        tol(),
    );
    let diamond = topo::transform_rigid(
        &diamond,
        &Affine3::translation(Vec3::new(0.0, 0.0, 0.8)),
        tol(),
    )
    .unwrap();
    let body = subtract(&cube(1.0, tol()), &diamond);
    validate_geometric(&body, tol()).expect("the pocketed cube is valid");
    let outer = outer_box_edges(&body);
    assert_eq!(outer.len(), 12, "the outer box's twelve edges");

    builds(&body, &outer, 0.1, "a setback clear of the diamond ring");

    let err = refusal(&body, &outer, 0.302, "a setback past the diamond's vertex");
    let BlendError::RingClearance { margin, .. } = &err else {
        panic!("the exact ring meter refuses past the screen, got {err:?}")
    };
    assert!(
        margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .is_some_and(|m| (m - (0.30 - 0.302)).abs() < 1e-12),
        "the vertex sits 0.30 from the edge and the setback is 0.302: {margin}"
    );
    assert!(
        err.to_string().contains(FILLET3_RING_RECOURSE),
        "the caller is handed the ring recourse: {err}"
    );

    let eps = tol().get().eps;
    let err = refusal(
        &body,
        &outer,
        0.30 - 5.0 * eps,
        "a setback in the sliver band",
    );
    let BlendError::Escalated { source, .. } = &err else {
        panic!("a margin of five eps escalates, got {err:?}")
    };
    assert_eq!(source.predicate, Some("fillet3_ring_clearance"));
}

/// **A polygonal ring beside a hole's rim is metered against the rim's
/// trim circle, edge by edge.**
///
/// A unit cube bored through by a radius-0.15 hole at `(0.3, 0.5)`, and
/// pocketed on its top face by a square whose nearest vertex lies 0.25
/// from the hole's axis, 11.25° off the `x` axis — between the hole
/// arcs' screen samples. The rim's trim circle on the top face has
/// radius `0.15 + r`, so the ring clears it by `0.1 − r`, and the
/// screen reads that gap as 0.107.
///
/// - r = 0.09 clears the ring: the rim builds, tier-3 valid.
/// - r = 0.105 crosses the vertex by 0.005, past the screen:
///   `RingClearance`, decided by the exact per-edge meter.
#[test]
fn a_polygonal_ring_beside_a_hole_rim_is_metered_against_its_trim_circle() {
    let hole = prism(vec![v(0.15, 0.5, 1.0), v(0.45, 0.5, 1.0)], 1.4, tol());
    let hole = topo::transform_rigid(
        &hole,
        &Affine3::translation(Vec3::new(0.0, 0.0, -0.2)),
        tol(),
    )
    .unwrap();
    let (a, h) = (11.25f64.to_radians(), 0.1);
    let (ux, uy) = (a.cos(), a.sin());
    let (px, py) = (0.3 + 0.25 * ux, 0.5 + 0.25 * uy);
    let pocket = prism(
        vec![
            v(px, py, 0.0),
            v(px + h * (ux + uy), py + h * (uy - ux), 0.0),
            v(px + 2.0 * h * ux, py + 2.0 * h * uy, 0.0),
            v(px + h * (ux - uy), py + h * (uy + ux), 0.0),
        ],
        0.3,
        tol(),
    );
    let pocket = topo::transform_rigid(
        &pocket,
        &Affine3::translation(Vec3::new(0.0, 0.0, 0.8)),
        tol(),
    )
    .unwrap();
    let body = subtract(&subtract(&cube(1.0, tol()), &hole), &pocket);
    validate_geometric(&body, tol()).expect("the bored, pocketed cube is valid");
    let rim: Vec<EdgeKey> = query::all_edges(&body)
        .into_iter()
        .filter(|k| {
            let e = body.get_edge(*k).unwrap();
            let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            matches!(*g.carrier(), geom::Curve3::Circle { center, .. }
                if (center.z - 1.0).abs() < 1e-9)
        })
        .collect();
    assert!(!rim.is_empty(), "the hole's top rim");

    builds(&body, &rim, 0.09, "a rim whose trim clears the square ring");
    let err = refusal(
        &body,
        &rim,
        0.105,
        "a rim whose trim crosses the square ring",
    );
    let BlendError::RingClearance { margin, .. } = &err else {
        panic!("the exact ring meter refuses past the screen, got {err:?}")
    };
    assert!(
        margin
            .reading
            .diagnostic_f64_for_error_text()
            .value()
            .is_some_and(|m| (m - (0.25 - (0.15 + 0.105))).abs() < 1e-12),
        "the vertex sits 0.25 from the axis and the trim at 0.255: {margin}"
    );
}

/// **A ring of arcs of TWO circles is metered arc by arc, not read as
/// its first arc's circle.**
///
/// Two radius-0.1 bores into a unit cube's top face, at `(0.5, 0.5)`
/// and `(0.45, 0.35)`, overlap: their mouth is one ring of two arcs of
/// different circles. The second bore comes 0.25 from the `y = 0` top
/// edge, at an abscissa off the screen's lattice (sampled gap 0.255).
/// Reading the whole ring as one circle adopted from whichever arc its
/// cycle walk starts on read either bore's clearance, and in one of the
/// two subtraction orders a setback of 0.252 carved through the second
/// bore's arc into a body that failed tier-3 validation
/// (`RingMeetsOuter`). In both orders now:
///
/// - r = 0.249 clears both arcs: the twelve edges build, tier-3 valid.
/// - r = 0.252 crosses the second arc by 0.002: `RingClearance`.
#[test]
fn a_ring_of_arcs_of_two_circles_is_metered_arc_by_arc() {
    let bore = |x: f64, y: f64, z: f64| {
        let b = prism(vec![v(x - 0.1, y, 1.0), v(x + 0.1, y, 1.0)], 0.5, tol());
        topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 0.0, z)), tol()).unwrap()
    };
    for (first, second) in [((0.5, 0.5), (0.45, 0.35)), ((0.45, 0.35), (0.5, 0.5))] {
        // Different floors, so the two bores share no face.
        let body = subtract(
            &subtract(&cube(1.0, tol()), &bore(first.0, first.1, 0.8)),
            &bore(second.0, second.1, 0.7),
        );
        validate_geometric(&body, tol()).expect("the twice-bored cube is valid");
        let outer = outer_box_edges(&body);
        assert_eq!(outer.len(), 12, "the outer box's twelve edges");

        builds(&body, &outer, 0.249, "a setback clear of both arcs");
        let err = refusal(&body, &outer, 0.252, "a setback past the second bore's arc");
        let BlendError::RingClearance { margin, .. } = &err else {
            panic!("bored {first:?} first: the exact ring meter refuses, got {err:?}")
        };
        assert!(
            margin
                .reading
                .diagnostic_f64_for_error_text()
                .value()
                .is_some_and(|m| (m - (0.25 - 0.252)).abs() < 1e-12),
            "bored {first:?} first: the arc sits 0.25 from the edge: {margin}"
        );
    }
}

/// **`FILLET3_GEOMETRY_RECOURSE` reaches the front door at an
/// elliptical ring.**
///
/// A bore tilted 0.4 rad off the vertical meets the cube's top face in
/// an ellipse, which neither the one-circle reading nor the per-edge
/// line and circle meters cover, so every outer-edge fillet refuses
/// `UnsupportedGeometry` — at 0.05 as at 0.1 — carrying the geometry
/// recourse.
/// `blend_recourse_followability::the_geometry_recourse_names_a_ring_and_an_order_that_builds`
/// executes the order that recourse names.
#[test]
fn the_geometry_recourse_reaches_the_front_door_at_an_elliptical_ring() {
    let body = subtract(&cube(1.0, tol()), &crate::common::tilted_bore());
    validate_geometric(&body, tol()).expect("the bored cube is valid");
    let outer = outer_box_edges(&body);
    assert_eq!(outer.len(), 12, "the outer box's twelve edges");

    for r in [0.05, 0.1] {
        let err = refusal(&body, &outer, r, "the outer edges of a bored box");
        assert!(
            matches!(err, BlendError::UnsupportedGeometry { .. }),
            "r = {r}: the ring's ellipse carriers are what refuse, got {err:?}"
        );
        let shown = err.to_string();
        assert!(
            shown.contains("neither a line nor a circle")
                && shown.contains(FILLET3_GEOMETRY_RECOURSE),
            "r = {r}: the caller reads the geometry recourse about a ring: {shown}"
        );
    }
}

/// **The chain gate behind `CORNER_SUPPORT_NOT_PLANAR`, and the rim the
/// assembly recourse has to name.**
///
/// A half-revolved dome's equator is an open plane–sphere arc ending at
/// corners whose third face is the sphere — the request that would
/// reach the "corner support is not a plane" geometry site. It never
/// does: the chain gate admits plane–plane and ruled arms on an open
/// link and nothing else, so a coaxial torus arm on an open arc answers
/// with the assembly recourse.
///
/// **And a cylinder's plane–cylinder top rim carves**, which is a
/// SECOND claim about that same sentence: the rim is a solid of
/// revolution's latitude rim between two coaxial surfaces of
/// revolution, so the sentence's CLOSED clause has to name that family
/// or it refuses by implication a request the surgery answers. The
/// assert below was a negative — it pinned that the sentence did not
/// name it — and it is positive now that the under-described-door
/// class was swept
/// (`work/blend/blend-recourses-under-describe-their-doors.md`).
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
        // revolve's arcs do not close, so `rim_of` names the vertex the
        // chain dangles at rather than handing back a partial rim a
        // fillet request would then stall on.
        match topo::query::rim_of(&half, *a) {
            Err(RimError::NotOneRim { walked, at, how }) => {
                assert_eq!(how, RimBreak::Dangles, "an open arc dangles");
                // The walk follows this arc's OWN support pair, so it
                // stays within the scan's arcs and stops at an end of
                // the last one it walked.
                assert!(
                    walked.len() < arcs.len(),
                    "the door is narrower than the radius scan: it walked \
                     {walked:?} of the scan's {arcs:?}"
                );
                assert!(
                    walked.iter().all(|k| arcs.contains(k)),
                    "the walk stays on the radius scan's arcs: {walked:?} of {arcs:?}"
                );
                let last = *walked.last().unwrap();
                let e = half.get_edge(last).unwrap();
                let ends = [
                    half.get_half_edge(e.he_plus).unwrap().start,
                    half.half_edge_end(e.he_plus).unwrap(),
                ];
                assert!(ends.contains(&at), "the walk stops at an end of {last:?}");
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
            shown.contains(
                "an open chain's supports are neither a plane–plane nor a ruled cylinder pair"
            ) && shown.contains(FILLET3_ASSEMBLY_RECOURSE),
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
    // The closed clause — everything after the verb condition — must
    // name the family this rim belongs to. It is read as its own
    // segment so the OPEN clause's ruled cylinder link, which is a
    // different door, cannot satisfy it by accident.
    let closed_clause = FILLET3_ASSEMBLY_RECOURSE
        .split_once("For a fillet, ")
        .expect("the assembly sentence's closed clause opens with its verb condition")
        .1;
    for want in ["coaxial surfaces of revolution", "latitude rim"] {
        assert!(
            closed_clause.contains(want),
            "the assembly recourse's closed clause must name the plane–cylinder rim \
             that carves ({want:?} missing): {closed_clause}"
        );
    }
}
