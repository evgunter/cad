//! M5 PR 9 (C12.2): the second-order sector lane, end to end on the
//! tangent-plane graze — the configuration whose first-order sector
//! data ties everywhere along the contact ruling.
//!
//! The fixture is PR 5's extruded disc (two half-walls sharing one
//! cylinder surface, meridian ruling edges at (±0.5, 0)) split by the
//! VERTICAL TANGENT plane x = 0.5: the plane contains the meridian
//! edge, both rim arcs depart exactly in-plane (the first-order
//! departure trilean honestly returns On), and the wall faces' local
//! normals are plane-parallel at the ON vertices (the old
//! `TangencyUnsupported` door). Before PR 9 this refused at first
//! order; now the second-order lane classifies it — the arcs curve
//! definitely toward Below, the walls definitely bend off the plane —
//! and whatever the pipeline ultimately refuses on is a DOWNSTREAM
//! honest verdict (the one-sided graze has no two-sided split), never
//! the first-order tie.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;
use topo::splitting::{SplitPlane, SplitReduceError, split};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The PR 5 disc: two half-circle arcs (bulge 1), radius 0.5 —
/// extrudes to a cylinder whose two wall faces share ONE cylinder
/// surface, with meridian ruling edges at (±0.5, 0).
fn cylinder_body() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.5, 0.0), 1.0),
        ProfileVertex::new(p2(0.5, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn the_tangent_graze_resolves_past_first_order() {
    // The old doors must NOT fire: no TangencyUnsupported (the wall
    // definitely bends off the plane — tangent_sector_osculation),
    // no ConsecutiveOnSectors (the arcs' in-plane departures resolve
    // at second order — tangent_sector_order2), no SliverSector on
    // those predicates. What remains is the documented one-sided
    // graze residue, refused DOWNSTREAM (the degenerate section /
    // finish net) — a tangent plane cannot two-side a convex body.
    use geom_core::k_stats::{start_verdict_log, take_verdict_log};
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.5, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
    };
    start_verdict_log();
    let out = split(&body, &plane, Tol::witness());
    let v = take_verdict_log();
    // The second-order lane ran, by name (telemetry from birth).
    for name in [
        "tangent_sector_order2",
        "tangent_sector_order2_arm",
        "tangent_sector_osculation",
    ] {
        assert!(
            v.iter().any(|x| x.predicate == name),
            "{name} never reached the funnel (recorded: {:?})",
            v.iter()
                .map(|x| x.predicate)
                .collect::<std::collections::BTreeSet<_>>()
        );
    }
    // The pipeline runs ALL the way to the documented one-sided
    // tangency net: the TangentIntersection section chords were
    // minted and CERTIFIED (the jet gate ran inside a real body),
    // the section polygon then honestly bounds zero area — the
    // degenerate side has no material. The graze refuses THERE, with
    // the recourse, never at the first-order tie.
    let err = out.expect_err("a tangent plane cannot two-side a convex body");
    let msg = format!("{err}");
    assert!(
        msg.contains("zero area") && msg.contains("one-sided tangency"),
        "the graze must reach the degenerate-section net: {msg}"
    );
    assert!(msg.contains("declare the coincidence"), "{msg}");
}

#[test]
fn an_off_ruling_tangent_plane_still_grazes_honestly() {
    // The same tangent plane rotated to touch mid-wall (azimuth π/2,
    // the ruling (0, 0.5, z) — no vertex there): the graze root lands
    // mid-arc, gets inserted as a double-root vertex, and the
    // neighborhood at THAT vertex ties at first order the same way.
    // Pin: never the first-order refusals.
    let body = cylinder_body();
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    };
    if let Err(topo::splitting::SplitError::Reduce(
        e @ (SplitReduceError::TangencyUnsupported { .. }
        | SplitReduceError::ConsecutiveOnSectors { .. }),
    )) = split(&body, &plane, Tol::witness())
    {
        panic!("second order owns the graze: {e}")
    }
}

/// A unit square with ONE filleted corner: line → tangent arc → line —
/// the fillet-grade tangency (two tangent joins at the arc's ends).
fn filleted_block() -> Body<f64> {
    // Corner at (1, 1) filleted with radius 0.25: the arc runs from
    // (1, 0.75) to (0.75, 1), bulge tan(π/8) (a CCW quarter arc).
    let b = (std::f64::consts::PI / 8.0).tan();
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.75), b),
        ProfileVertex::new(p2(0.75, 1.0), 0.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    // The tangency is authored, so it is DECLARED (the #101
    // discipline): joints 3 (arc→line) and 2 (line→arc).
    lp = lp.with_tangent_joints(vec![2, 3]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn a_fillet_grade_tangency_must_carry_and_does() {
    // OQ7 both levels, the MUST side: the filleted corner's two
    // tangent struts come out of construction described
    // TangentIntersection (the upgrade pass descended), the body is
    // tier-3 valid, and the MARK records the tangency.
    let body = filleted_block();
    let marks =
        topo::contact_marks(&body, Tol::witness()).expect("the filleted block is tier-3 valid");
    let tangent_edges: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            matches!(
                body.get_curve_geom(e.curve)
                    .and_then(|g| g.certified())
                    .map(geom_brep::EdgeCurve::description),
                Some(geom_brep::EdgeGeometry::TangentIntersection { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        tangent_edges.len(),
        2,
        "the fillet's two tangent struts carry the intrinsic description"
    );
    for e in tangent_edges {
        assert_eq!(marks.get(e), Some(&topo::ContactMark::Tangent));
    }
}

#[test]
fn the_must_carry_fires_when_the_description_is_conventional() {
    // The enforcement direction: strip one tangent strut back to its
    // conventional MappedCurve description (certifiable — the
    // residual schedule cannot see the dihedral) and tier 3 must
    // refuse TangentNotIntrinsic, naming the edge.
    let mut body = filleted_block();
    let (edge, curve) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).and_then(|g| g.certified())?;
            matches!(
                c.description(),
                geom_brep::EdgeGeometry::TangentIntersection { .. }
            )
            .then(|| (k, c.clone()))
        })
        .expect("a tangent strut exists");
    let (t0, t1) = curve.params();
    let geom::Curve3::Line { origin, dir } = *curve.carrier() else {
        panic!("a strut is a line");
    };
    // The conventional description extrude would have kept: the
    // extruded profile point under the identity placement.
    let spec = geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeGeometry::MappedCurve(geom_brep::MappedCurve::ExtrudedPoint {
            point: p2(origin.x, origin.y),
            place: geom_core::Affine3::identity(),
            vec: dir * (t1 - t0),
        }),
        carrier: geom::Curve3::Line { origin, dir },
        param_start: t0,
        param_end: t1,
    };
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the conventional description certifies (residuals only)");
    let errs = topo::validate_geometric(&body, Tol::witness()).expect_err("the must-carry fires");
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::TangentNotIntrinsic { edge: k } if *k == edge
        )),
        "expected TangentNotIntrinsic for {edge:?}: {errs:?}"
    );
    let msg = errs
        .iter()
        .find(|e| matches!(e, topo::ValidationError::TangentNotIntrinsic { .. }))
        .map(|e| format!("{e}"))
        .unwrap();
    assert!(msg.contains("jet-determinate"), "{msg}");
    assert!(msg.contains("G2"), "{msg}");
}

#[test]
fn a_g2_underdetermined_join_must_not_carry() {
    // The exempt-BY-PREDICATE direction: the plain disc cylinder's
    // two meridian struts join two wall faces on ONE shared surface —
    // κ_rel is exactly zero (same surface both sides: the locus is
    // under-determined), the mark says so, the description stays
    // conventional, and the body stays valid. No exemption list
    // anywhere — the zero-side margin IS the exemption.
    let body = cylinder_body();
    let marks =
        topo::contact_marks(&body, Tol::witness()).expect("the disc cylinder is tier-3 valid");
    let mut saw_underdetermined = 0;
    for (k, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if matches!(c.description(), geom_brep::EdgeGeometry::MappedCurve(_))
            && matches!(c.carrier(), geom::Curve3::Line { .. })
        {
            assert_eq!(
                marks.get(k),
                Some(&topo::ContactMark::SmoothUnderdetermined),
                "a same-surface smooth strut is under-determined by the predicate"
            );
            saw_underdetermined += 1;
        }
    }
    assert_eq!(saw_underdetermined, 2, "the disc has two meridian struts");
}
