//! **BLEND-3 review probes (R1).**
//!
//! Independent rows for the concave plane–plane chamfer, written
//! against the claims BLEND-3 makes rather than against its
//! implementation. Four questions, each answered by execution:
//!
//! 1. **Is the "third door" really the ring gate?** The unit says the
//!    fixture's vent is round because a square chimney refused on the
//!    ring-clearance pass, which is not a convexity door at all. The
//!    square vent is built here and the refusal is read.
//! 2. **Is the shape argument tight?** BLEND-3 argues that the
//!    requested set must be a whole component of the edge graph, so a
//!    pocket cannot supply one, an unvented cavity is two shells, and
//!    the twelve-edge cavity is therefore the shape. The first two
//!    steps are executed here; the last one is FALSIFIED — a
//!    triangular-prism cavity is the same argument's shape with nine
//!    edges and six corners, and it carves.
//! 3. **Does the ring gate still meter on the concave side?** A
//!    concave carve moves the ceiling's trimlines the other way round
//!    the material. A vent wide enough to collide with them must
//!    refuse rather than mint, or the gate is passing by accident on
//!    the fixture's generous clearance.
//! 4. **Is the added volume the mirrored removed volume, measured
//!    rather than asserted?** The unit checks the cavity against a
//!    closed form. This row derives the same number from an
//!    INDEPENDENT execution — the volume the identical chamfer removes
//!    from a cube — so a closed form and the carve cannot be wrong
//!    together in the same direction.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::BlendError;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::{Body, EdgeKey, subtract, validate, validate_closed};

/// The setback every row here uses, meters — BLEND-3's own.
const D: f64 = 0.25;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn sketch_at(z: f64) -> SketchPlane<f64> {
    SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z) - Point3::origin(),
    ))
}

/// A prism over any CCW polygon — the rectangle helper, generalized,
/// so a three-sided cavity is as authorable as a four-sided one.
fn prism(poly: &[Point2<f64>], z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon(poly.to_vec());
    let profile = Profile::new(sketch_at(z0), vec![lp])
        .validate(Tol::witness())
        .expect("a convex polygon is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a prism extrudes")
        .body
}

fn brick(lo: Point3<f64>, hi: Point3<f64>) -> Body<f64> {
    prism(
        &[
            Point2::new(lo.x, lo.y),
            Point2::new(hi.x, lo.y),
            Point2::new(hi.x, hi.y),
            Point2::new(lo.x, hi.y),
        ],
        lo.z,
        hi.z,
    )
}

/// A circular rod, as BLEND-3's fixture authors one: two half arcs.
fn rod(center: Point2<f64>, r: f64, z0: f64, z1: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(center.x - r, center.y), 1.0),
        ProfileVertex::new(Point2::new(center.x + r, center.y), 1.0),
    ]);
    let profile = Profile::new(sketch_at(z0), vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(z1 - z0), Tol::witness())
        .expect("a rod extrudes")
        .body
}

fn cut(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    subtract(a, b, Tol::witness())
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// Every edge both of whose endpoints are corners of the given cavity
/// polygon swept between `z0` and `z1` — found by position, never by
/// index, so the row cannot silently address a different edge.
fn cavity_edges_of(body: &Body<f64>, poly: &[Point2<f64>], z0: f64, z1: f64) -> Vec<EdgeKey> {
    let is_corner = |p: Point3<f64>| {
        let z_ok = (p.z - z0).abs() < 1e-9 || (p.z - z1).abs() < 1e-9;
        z_ok && poly
            .iter()
            .any(|q| (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |v| {
                body.get_vertex(v)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => is_corner(a) && is_corner(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

// ---------------------------------------------------------------
// 1. The third door.
// ---------------------------------------------------------------

/// **The door that shaped the fixture is the RING gate, and it is not
/// a convexity door.** BLEND-3 reports that its first draft vented the
/// cavity through a square chimney and was refused because a ring
/// edge's carrier was not a circle. Same cavity, same twelve edges,
/// square vent: the refusal is read here rather than taken on the
/// record's word, which is what makes the three-door ordering claim
/// checkable by someone who was not there.
#[test]
fn r1_a_square_vent_refuses_on_the_ring_gate_not_on_convexity() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = brick(Point3::new(1.7, 1.7, 2.5), Point3::new(2.3, 2.3, 5.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut(&cut(&block, &vent), &cavity);
    assert_eq!(validate(&body), Ok(()), "tier 1");
    assert_eq!(
        body.shells().count(),
        1,
        "a square vent also leaves one shell"
    );

    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let edges = cavity_edges_of(&body, &poly, 1.0, 3.0);
    assert_eq!(edges.len(), 12, "the cavity's twelve edges");

    let err = chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("a square vent's ring is not a circle");
    let text = err.error.to_string();
    assert!(
        !text.contains("mixed-convexity") && !text.contains("concave chain"),
        "the third door must not be a convexity door — got {text}"
    );
    assert!(
        text.contains("circle") || text.contains("ring"),
        "expected the ring gate's own refusal, got {text}"
    );
}

// ---------------------------------------------------------------
// 2. The shape argument — two steps confirmed, one falsified.
// ---------------------------------------------------------------

/// **Step one of the argument: a pocket ends at mixed corners.** A
/// cavity opened to the top face is the shape a modeller reaches for
/// first, and BLEND-3 argues it cannot serve: its concave edges run
/// into the rim, where they meet convex ones. Requesting the pocket's
/// eight concave edges is refused, and the refusal is a corner or
/// run-out one — never a carve.
#[test]
fn r1_a_pocket_cannot_supply_an_all_concave_component() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let pocket = brick(Point3::new(1.0, 1.0, 2.0), Point3::new(3.0, 3.0, 5.0));
    let body = cut(&block, &pocket);
    assert_eq!(validate_closed(&body), Ok(()), "tier 2");

    // The pocket's four floor edges and its four vertical ones: every
    // edge of the pocket that is concave, and nothing else. The rim
    // edges at z = 4 are convex, and leaving them out is the point —
    // an all-concave request that cannot close.
    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let all = cavity_edges_of(&body, &poly, 2.0, 4.0);
    assert_eq!(all.len(), 12, "floor, walls and rim");
    let z_of = |k: &EdgeKey| -> Vec<f64> {
        let e = body.get_edge(*k).expect("an edge");
        let h = body.get_half_edge(e.he_plus).expect("a half-edge");
        let end = body.half_edge_end(e.he_plus).expect("an end");
        [h.start, end]
            .iter()
            .map(|v| {
                body.get_vertex(*v)
                    .and_then(|x| body.get_point(x.point))
                    .expect("a point")
                    .z
            })
            .collect()
    };
    let edges: Vec<EdgeKey> = all
        .iter()
        .filter(|k| z_of(k).iter().any(|z| (z - 2.0).abs() < 1e-9))
        .copied()
        .collect();
    assert_eq!(
        edges.len(),
        8,
        "four floor edges and four verticals — the pocket's concave set"
    );

    let err = chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("a pocket's concave edges are not a closed component");
    assert!(
        matches!(
            err.error,
            BlendError::UnsupportedCorner { .. } | BlendError::UnsupportedRunOut { .. }
        ),
        "expected a corner/run-out refusal at the pocket's rim, got {:?}",
        err.error
    );
}

/// **Step two: an UNVENTED cavity is two shells and the body door
/// says so.** This is the step that forces a vent to exist at all.
#[test]
fn r1_an_unvented_cavity_refuses_at_the_body_door() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut(&block, &cavity);
    assert_eq!(
        body.shells().count(),
        2,
        "a sealed cavity is a second shell"
    );

    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let edges = cavity_edges_of(&body, &poly, 1.0, 3.0);
    assert_eq!(edges.len(), 12, "the sealed cavity still has twelve edges");
    chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect_err("the surgery's body door admits one shell");
}

/// **The minimality claim is FALSE as written.** BLEND-3's fixture doc
/// says a cavity's twelve edges "are the only small shape that does"
/// supply an isolated all-concave component. The argument it rests on
/// — the requested set must be a whole component of the edge graph, so
/// it must be a cavity, so the body must be vented — is sound, and the
/// two rows above execute its steps. What does not follow is the
/// TWELVE: any convex prism cut as a cavity has the same property, and
/// a triangular one has nine edges and six corners rather than twelve
/// and eight. It is built and carved here, through the same public
/// path and the same two subtractions, so the fixture is a choice
/// within the class rather than the only member of it.
#[test]
fn r1_a_nine_edge_triangular_cavity_is_a_simpler_shape_of_the_same_class() {
    // A CCW triangle well inside the block, and a vent on its incentre
    // (2, 1.618) whose radius clears the inset triangle's inradius
    // (0.618 − D = 0.368).
    let tri = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(2.0, 3.0),
    ];
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 1.618), 0.2, 2.5, 5.0);
    let cavity = prism(&tri, 1.0, 3.0);
    let body = cut(&cut(&block, &vent), &cavity);

    assert_eq!(validate(&body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&body), Ok(()), "tier 2");
    assert_eq!(body.shells().count(), 1, "the triangular cavity vents too");

    let edges = cavity_edges_of(&body, &tri, 1.0, 3.0);
    assert_eq!(
        edges.len(),
        9,
        "three verticals and two triangles — nine edges, against the fixture's twelve"
    );

    let out = chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect("a triangular cavity's nine concave edges chamfer");
    assert_eq!(out.blend_faces.len(), 9, "one strip per concave edge");
    assert_eq!(
        out.corner_faces.len(),
        6,
        "one flat patch per all-concave trihedron — six, against the fixture's eight"
    );
    assert_eq!(validate(&out.body), Ok(()), "tier 1 after the carve");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2 after the carve");
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "tier 3 after the carve"
    );
}

// ---------------------------------------------------------------
// 3. The ring gate, metered on the concave side.
// ---------------------------------------------------------------

/// **The ring gate must go red as the clearance DEGRADES, not only
/// where it is violated outright.** The fixture's vent (r = 0.5) sits
/// 0.25 m clear of the chamfered ceiling's trimlines, so every row in
/// the unit passes the gate comfortably and none of them would notice
/// if the gate stopped metering on the concave side. Widening the vent
/// until its ring reaches past the trimline must refuse — a body that
/// minted here would be a strip cut through its own ring.
#[test]
fn r1_the_ring_gate_still_meters_on_the_concave_side() {
    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let carve = |r: f64| {
        let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
        let vent = rod(Point2::new(2.0, 2.0), r, 2.5, 5.0);
        let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
        let body = cut(&cut(&block, &vent), &cavity);
        let edges = cavity_edges_of(&body, &poly, 1.0, 3.0);
        assert_eq!(edges.len(), 12, "twelve cavity edges at r = {r}");
        chamfer_edges(&body, &edges, D, band(), Tol::witness()).map(|o| o.body)
    };

    // The fixture's own clearance: the ring at 2.5 against a trimline
    // at 2.75.
    assert!(carve(0.5).is_ok(), "the fixture's vent carves");

    // The ring now reaches 2.9, past the ceiling trimline at 2.75.
    let err = carve(0.9).expect_err("a ring past the trimline must refuse, not mint");
    assert!(
        !matches!(
            err.error,
            BlendError::Op { .. } | BlendError::Certify { .. }
        ),
        "the degradation must be caught at a door, not by a downstream Euler/certify \
         failure — got {:?}",
        err.error
    );
}

// ---------------------------------------------------------------
// 4. The added volume, from an independent execution.
// ---------------------------------------------------------------

/// **The mirror claim as a differential, not as a closed form.**
/// BLEND-3 checks its cavity against `64 − a³ − vent + (6ad² −
/// 16⁄3 d³)`. A closed form written by the same author as the carve can
/// be wrong with it. This row never writes the added term down: it
/// MEASURES what the identical chamfer removes from a cube of the
/// cavity's own side, and requires the cavity to have gained exactly
/// that. Then, and only as a second opinion, it checks that measured
/// number against the algebra.
#[test]
fn r1_the_cavity_gains_what_the_mirrored_cube_loses() {
    let a = 2.0_f64;

    // (i) What the same chamfer REMOVES from a cube of side a.
    let cube_body = cube(a, Tol::witness());
    let cube_edges: Vec<EdgeKey> = cube_body.edges().map(|(k, _)| k).collect();
    let chamfered_cube = chamfer_edges(&cube_body, &cube_edges, D, band(), Tol::witness())
        .expect("a cube chamfers")
        .body;
    let cube_vol = topo::mass_properties(&chamfered_cube, Tol::witness())
        .expect("closed-form props")
        .volume;
    let removed = a.powi(3) - cube_vol;

    // (ii) What the cavity GAINS, measured the same way.
    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut(&cut(&block, &vent), &cavity);
    let plain = topo::mass_properties(&body, Tol::witness())
        .expect("closed-form props")
        .volume;
    let edges = cavity_edges_of(&body, &poly, 1.0, 3.0);
    let carved = chamfer_edges(&body, &edges, D, band(), Tol::witness())
        .expect("the cavity chamfers")
        .body;
    let gained = topo::mass_properties(&carved, Tol::witness())
        .expect("closed-form props")
        .volume
        - plain;

    assert!(
        (gained - removed).abs() <= 1e-12 * removed,
        "the concave carve must ADD exactly what the mirrored convex carve REMOVES: \
         gained {gained}, removed {removed}"
    );

    // Second opinion: the algebra BLEND-3 asserts, re-derived here as
    // twelve prisms of (d²/2)(a − 2d) plus eight corner boxes in which
    // the three strip half-spaces all sit inside the corner patch's,
    // so each box contributes 5d³/6.
    let by_hand = 12.0 * (D * D / 2.0) * (a - 2.0 * D) + 8.0 * (5.0 / 6.0) * D.powi(3);
    assert!(
        (removed - by_hand).abs() <= 1e-12 * by_hand,
        "measured {removed} vs the prism/corner-box derivation {by_hand}"
    );
    let compact = 6.0 * a * D * D - (16.0 / 3.0) * D.powi(3);
    assert!(
        (by_hand - compact).abs() <= 1e-12 * compact,
        "the two spellings of the same volume must agree: {by_hand} vs {compact}"
    );
}

// ---------------------------------------------------------------
// 5. Both signs in ONE request — the capability the widening
//    creates and the unit's fixture never exercises.
// ---------------------------------------------------------------

/// Every edge both of whose endpoints are corners of the block
/// `[0,4]³` — the outer, all-CONVEX component of the same body.
fn block_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let corner = |p: Point3<f64>| {
        [p.x, p.y, p.z]
            .iter()
            .all(|c| c.abs() < 1e-9 || (c - 4.0).abs() < 1e-9)
    };
    let mut found: Vec<EdgeKey> = body
        .edges()
        .filter(|(k, _)| {
            let Some(e) = body.get_edge(*k) else {
                return false;
            };
            let Some(h) = body.get_half_edge(e.he_plus) else {
                return false;
            };
            let Some(end) = body.half_edge_end(e.he_plus) else {
                return false;
            };
            let pt = |v| {
                body.get_vertex(v)
                    .and_then(|x| body.get_point(x.point))
                    .copied()
            };
            match (pt(h.start), pt(end)) {
                (Some(a), Some(b)) => corner(a) && corner(b),
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    found.sort_unstable();
    found
}

/// **One call, both material sides.** Until this PR every chamfer
/// request in the tree carried ONE convexity, because a concave one
/// was refused at door 1. Widening the doors per verb makes a request
/// spanning both signs reachable for the first time — the vented
/// cavity's twelve concave edges AND the block's twelve convex ones,
/// two whole components of one edge graph, in a single `chamfer_edges`
/// call.
///
/// This is where a convexity read ONCE for the request rather than
/// once per link would show: the sense bit, the trimline direction and
/// the corner patch's outward fold are all per-link quantities, and a
/// single-sign fixture cannot tell that apart from a per-request one.
/// The volume is the arithmetic that settles it — the concave half must
/// ADD its mirrored chamfer while the convex half REMOVES its own, in
/// the same body, in the same carve.
#[test]
fn r1_one_request_carries_both_convexity_signs() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut(&cut(&block, &vent), &cavity);

    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let concave = cavity_edges_of(&body, &poly, 1.0, 3.0);
    let convex = block_edges(&body);
    assert_eq!(concave.len(), 12, "the cavity's concave component");
    assert_eq!(convex.len(), 12, "the block's convex component");
    assert!(
        concave.iter().all(|k| !convex.contains(k)),
        "the two components are disjoint"
    );

    let before = topo::mass_properties(&body, Tol::witness())
        .expect("closed-form props")
        .volume;

    let mut both = concave;
    both.extend(convex);
    let out = chamfer_edges(&body, &both, D, band(), Tol::witness())
        .expect("one request may span both material sides");

    assert_eq!(out.blend_faces.len(), 24, "one strip per requested edge");
    assert_eq!(
        out.corner_faces.len(),
        16,
        "eight concave corners and eight convex"
    );
    assert_eq!(validate(&out.body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    // The concave half adds a cube-of-side-2's worth of chamfer; the
    // convex half removes a cube-of-side-4's worth. Both terms are the
    // same closed form at the two sides' own scales.
    let chamfer_of = |a: f64| 6.0 * a * D * D - (16.0 / 3.0) * D.powi(3);
    let after = topo::mass_properties(&out.body, Tol::witness())
        .expect("closed-form props")
        .volume;
    let want = before + chamfer_of(2.0) - chamfer_of(4.0);
    assert!(
        (after - want).abs() <= 1e-12 * want,
        "a two-sided carve must add on one side and remove on the other: \
         got {after}, want {want}"
    );

    let mesh = mesh::tessellate(&out.body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}

/// **The FILLET carves that same two-sided request** — the widening
/// this probe's first form pinned as missing (its assertion was the
/// rolling ball's refusal, expected "until #644"; issue 644 then
/// closed it). Per-request folds would still pass a one-sided
/// fixture, so the volume is again the arithmetic that settles it:
/// the concave component must ADD its rounded-void complement while
/// the convex one REMOVES its own rounded-cube complement, in the
/// same body, in the same carve — the ball's Steiner terms at each
/// side's own scale.
#[test]
fn r1_one_fillet_request_carries_both_convexity_signs() {
    let block = brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
    let vent = rod(Point2::new(2.0, 2.0), 0.5, 2.5, 5.0);
    let cavity = brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    let body = cut(&cut(&block, &vent), &cavity);
    let poly = [
        Point2::new(1.0, 1.0),
        Point2::new(3.0, 1.0),
        Point2::new(3.0, 3.0),
        Point2::new(1.0, 3.0),
    ];
    let mut both = cavity_edges_of(&body, &poly, 1.0, 3.0);
    both.extend(block_edges(&body));
    assert_eq!(both.len(), 24, "both components");

    let before = topo::mass_properties(&body, Tol::witness())
        .expect("closed-form props")
        .volume;

    let out = sweep::blend::build::fillet_edges(&body, &both, D, band(), Tol::witness())
        .expect("one fillet request may span both material sides");
    assert_eq!(out.blend_faces.len(), 24, "one band per requested edge");
    assert_eq!(
        out.corner_faces.len(),
        16,
        "eight concave octants and eight convex"
    );
    assert_eq!(validate(&out.body), Ok(()), "tier 1");
    assert_eq!(validate_closed(&out.body), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&out.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );

    // What a full twelve-edge fillet at radius r removes from a cube
    // of side a is the complement of the Minkowski (Steiner) closed
    // form; the concave side ADDS the same complement at its own
    // scale. A declared copy class (siblings: the concave-fillet
    // suite's fixture oracle, the blend4 R1 probes' prism form, the
    // die suites); evgunter/cad issue 1364 owns the shared home.
    let rounded = |a: f64| {
        let l = a - 2.0 * D;
        l.powi(3)
            + 6.0 * l * l * D
            + 3.0 * core::f64::consts::PI * l * D * D
            + (4.0 / 3.0) * core::f64::consts::PI * D.powi(3)
    };
    let fillet_of = |a: f64| a.powi(3) - rounded(a);
    let after = topo::mass_properties(&out.body, Tol::witness())
        .expect("closed-form props")
        .volume;
    let want = before + fillet_of(2.0) - fillet_of(4.0);
    assert!(
        (after - want).abs() <= 1e-12 * want,
        "a two-sided fillet must add on one side and remove on the other: \
         got {after}, want {want}"
    );

    let mesh = mesh::tessellate(&out.body, 5e-3, Tol::witness()).expect("tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
}
