//! **The plane–plane fillet cut off at an oblique end face**, beside the
//! shape rows in `band_planar_cut_off_shapes`: the concave side, where
//! the band adds its section and the slanted end walls lose the sliver
//! under an elliptic arc; the near-perpendicular sliver band, where the
//! kind-picker escalates or decides the circle; what the downstream
//! doors make of the ellipse edges (the tessellator, and the boolean's
//! containment door, which refuses them); and the `Interval` replay.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::interval::iv;
use geom::Curve3;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Vec3};
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, block, brick, finished, prism, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, BooleanError, EdgeKey, mass_properties, query, validate_geometric};

use crate::band_planar_cut_off::{D, Verb, carve, edge, tol};

/// The parallelogram prism whose top front edge ends at two parallel
/// side walls slanted 26.6° off its normal plane.
fn parallelogram<T: geom_core::Decide + topo::AtRestPolicy>() -> Body<T> {
    let p = |x: f64, y: f64| (Point2::new(T::from_f64(x), T::from_f64(y)), T::zero());
    prism(
        vec![p(0.0, 0.0), p(2.0, 0.0), p(2.5, 1.0), p(0.5, 1.0)],
        T::one(),
        tol(),
    )
}

/// **The concave side at slanted end walls**: a block less a
/// parallelogram pocket whose floor edges along `x` end at the pocket's
/// two parallel slanted walls, a unit apart along the edge. Both verbs
/// add their section over that unit, and each wall loses the sliver
/// under the end curve — an elliptic arc under the fillet.
#[test]
fn a_pocket_floor_edge_at_slanted_walls_adds_its_section_on_both_verbs() {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.5),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        tol(),
    );
    let pocket = prism_on(
        plane,
        vec![
            (Point2::new(0.5, 0.25), 0.0),
            (Point2::new(1.5, 0.25), 0.0),
            (Point2::new(1.8, 1.25), 0.0),
            (Point2::new(0.8, 1.25), 0.0),
        ],
        1.0,
        tol(),
    );
    let body = realized(
        BooleanOp::Subtract,
        &block(2.5, 1.5, 1.0, tol()),
        &pocket,
        tol(),
    );
    validate_geometric(&body, tol()).expect("the pocketed block is tier-3 valid");
    let e = edge(&body, [0.5, 0.25, 0.5], [1.5, 0.25, 0.5]);
    for verb in [Verb::Chamfer, Verb::Fillet] {
        let out = carve(
            &body,
            &[e],
            verb,
            -verb.section(),
            "a slanted pocket's floor edge",
        );
        if let Verb::Fillet = verb {
            let rec = out.naming.as_ref().expect("births");
            let ellipses = rec
                .arcs
                .iter()
                .filter(|(a, _, _)| {
                    let c = out.body.get_edge(*a).unwrap().curve;
                    matches!(
                        out.body
                            .get_curve_geom(c)
                            .and_then(|g| g.certified())
                            .map(|c| c.carrier()),
                        Some(Curve3::Ellipse { .. })
                    )
                })
                .count();
            assert_eq!(ellipses, 2, "both concave ends are elliptic arcs");
        }
    }
}

/// **The near-perpendicular sliver band, through the door**: a
/// trapezoid prism whose end walls lean by `s` off the top front
/// edge's normal plane. A lean definite at the edge's lever whose
/// ellipse's axes, `r·s²/2` apart, are one circle escalates under
/// `fillet3_cap_transverse` through `ellipse_axes_distinct`; a lean
/// inside the zero band decides the circle and builds. Both leans are
/// read off the run's band, so the row holds at every eps row.
#[test]
fn a_near_perpendicular_end_escalates_or_decides_the_circle() {
    let band = Band::linear(tol()).expect("the door's band");
    let trapezoid = |s: f64| {
        prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        )
    };
    let top = |body: &Body<f64>| edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let s = 20.0 * band.escalate();
    assert!(
        D * s * s / 2.0 < band.zero(),
        "the axes are one circle at s = {s}"
    );
    let body = trapezoid(s);
    match Verb::Fillet.run(&body, &[top(&body)]) {
        Err(BlendError::Escalated {
            decision, source, ..
        }) => {
            assert_eq!(decision, sweep::blend::BlendDecision::CapTransverse);
            assert_eq!(source.predicate, Some("ellipse_axes_distinct"));
        }
        other => panic!("s = {s}: the sliver band escalates, got {other:?}"),
    }
    let s = band.zero() / 10.0;
    let body = trapezoid(s);
    let out = carve(
        &body,
        &[top(&body)],
        Verb::Fillet,
        Verb::Fillet.section() * (2.0 - 2.0 * s * Verb::Fillet.centroid()),
        &format!("a lean of {s}"),
    );
    for (arc, _, _) in &out.naming.as_ref().expect("births").arcs {
        let c = out.body.get_edge(*arc).unwrap().curve;
        assert!(
            matches!(
                out.body
                    .get_curve_geom(c)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(Curve3::Circle { .. })
            ),
            "a lean inside the zero band is the circle"
        );
    }
}

/// **Downstream of the ellipse**: the filleted parallelogram, whose
/// band is trimmed by two elliptic arcs that also bound the slanted
/// walls. Mass properties measure it through the certified quadrature
/// (the closed-form rows above); the tessellator meshes it watertight;
/// and the boolean refuses it as an operand at the containment door —
/// beside the band, through it and wholly apart from it alike — because
/// the point-in-solid probe's at-infinity side is measured in closed
/// form only, and a cylinder face trimmed by an ellipse has none. That
/// is the boolean's open row
/// (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`),
/// which an obliquely cut rod reaches without any fillet; the cut is
/// pinned here so it retires with it.
#[test]
fn the_ellipse_edges_pass_the_tessellator_and_stop_at_the_boolean_containment_door() {
    let body = parallelogram::<f64>();
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = fillet_edges(&body, &[e], D, tol()).expect("the oblique fillet builds");
    let mesh = mesh::tessellate(&out.body, 5e-3, tol()).expect("the filleted body tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let operand = finished("the filleted parallelogram", out.body, tol());
    for (what, cutter) in [
        (
            "a notch beside the band",
            brick((1.0, 1.4), (0.8, 1.2), (0.5, 1.5), tol()),
        ),
        (
            "a slot through the band",
            brick((0.9, 1.1), (-0.5, 0.3), (0.6, 1.5), tol()),
        ),
        (
            "a block apart",
            brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol()),
        ),
    ] {
        let cutter = finished(what, cutter, tol());
        for (op, result) in [
            ("subtract", topo::subtract(&operand, &cutter, tol())),
            ("union", topo::union(&operand, &cutter, tol())),
        ] {
            match result {
                Err(BooleanError::Containment(topo::PointInSolidError::VolumeUncertified)) => {}
                other => panic!(
                    "{what} ({op}): the containment door's closed-form probe refuses, got {:?} \
                     — retire this pin with the boolean's row",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// The edge between two points of an `Interval` body, either way round.
fn edge_at(body: &Body<Interval>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let near = |p: &Point3<Interval>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("an edge between the two points")
}

/// **The `Interval` replay**: the parallelogram's oblique fillet carves
/// at the certified scalar, tier-3 valid with naming total, its ends
/// ellipses, and its volume enclosure brackets the closed form
/// `(1 − π/4)·r²·2`.
#[test]
fn the_oblique_fillet_carves_at_the_certified_scalar() {
    let body = parallelogram::<Interval>();
    let e = edge_at(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = fillet_edges(&body, &[e], iv(D), tol()).expect("carves at Interval");
    validate_geometric(&out.body, tol()).expect("tier 3 at Interval");
    assert_naming_totality(&body, &out, &[e], "the oblique fillet at Interval");
    for (arc, _, _) in &out.naming.as_ref().expect("births").arcs {
        let c = out.body.get_edge(*arc).unwrap().curve;
        assert!(
            matches!(
                out.body
                    .get_curve_geom(c)
                    .and_then(|g| g.certified())
                    .map(|c| c.carrier()),
                Some(Curve3::Ellipse { .. })
            ),
            "an oblique end is an ellipse at Interval"
        );
    }
    let (p0, p1) = (
        mass_properties(&body, tol()).expect("interval props"),
        mass_properties(&out.body, tol()).expect("interval props"),
    );
    let removed = p0.volume - p1.volume;
    let pad = p0.volume_pad + p1.volume_pad;
    let truth = Verb::Fillet.section() * 2.0;
    assert!(
        removed.lo() - pad <= truth && truth <= removed.hi() + pad,
        "ΔV {removed:?} ± {pad} brackets {truth}"
    );
    assert!(
        removed.hi() - removed.lo() + 2.0 * pad < 1e-6,
        "the enclosure is a claim: {removed:?} ± {pad}"
    );
}
