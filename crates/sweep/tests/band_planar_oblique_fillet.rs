//! **The plane–plane fillet cut off at an oblique end face**, beside the
//! shape rows in `band_planar_cut_off_shapes`: the concave side, where
//! the band adds its section and the slanted end walls lose the sliver
//! under an elliptic arc; the near-perpendicular sliver band, where the
//! kind-picker escalates or decides the circle; what the downstream
//! doors make of the ellipse edges (the tessellator, and the boolean
//! beside, through and apart from the band); and the `Interval` replay.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::interval::iv;
use core::f64::consts::PI;
use geom::Curve3;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Sign, Vec3};
use profile::SketchPlane;
use sweep::blend::BlendError;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    assert_naming_totality, block, brick, finished, prism, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, EdgeKey, query, validate_geometric};

use crate::band_one_cycle_members::{assert_taken, cup_faces};
use crate::band_planar_cut_off::{
    D, Verb, carve, edge, midpoint_tol, pad_ceiling, tol, volume, volume_enclosure,
};

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
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.5));
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
/// edge's normal plane. A lean definite at the edge's lever is the
/// exact ellipse, though its axes are `r·s²/2` apart, far inside the
/// band: it builds at the closed form, its arcs ellipses. A lean inside
/// the zero band decides the circle and builds. Both leans are read off
/// the run's band, so the row holds at every eps row.
#[test]
fn a_near_perpendicular_end_builds_its_ellipse_or_decides_the_circle() {
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
    let carrier = |out: &sweep::blend::build::Blended<f64>, arc: EdgeKey| {
        let c = out.body.get_edge(arc).unwrap().curve;
        out.body
            .get_curve_geom(c)
            .and_then(|g| g.certified())
            .map(|c| c.carrier().clone())
    };
    let s = 20.0 * band.escalate();
    assert!(
        D * s * s / 2.0 < band.zero(),
        "the axes are within the zero band at s = {s}"
    );
    let body = trapezoid(s);
    let removed = Verb::Fillet.section() * (2.0 - 2.0 * s * Verb::Fillet.centroid());
    let out = carve(
        &body,
        &[top(&body)],
        Verb::Fillet,
        removed,
        &format!("a definite lean of {s}"),
    );
    for (arc, _, _) in &out.naming.as_ref().expect("births").arcs {
        assert!(
            matches!(carrier(&out, *arc), Some(Curve3::Ellipse { .. })),
            "a definite lean of {s} is the ellipse"
        );
    }
    // Decided Zero, the circle stands for the section to within the
    // lean: the band's trim reads it as the spine's normal section and
    // the end face as its own, so the volume agrees with the closed
    // form to the order of the lean times the section.
    let s = band.zero() / 10.0;
    let body = trapezoid(s);
    let edges = [top(&body)];
    let out = Verb::Fillet
        .run(&body, &edges)
        .unwrap_or_else(|e| panic!("a lean of {s} builds the circle, got {e}"));
    validate_geometric(&out.body, tol()).expect("tier 3");
    assert_naming_totality(&body, &out, &edges, "a lean inside the zero band");
    let removed = Verb::Fillet.section() * (2.0 - 2.0 * s * Verb::Fillet.centroid());
    let dv = volume(&body) - volume(&out.body);
    assert!(
        (dv - removed).abs() < 10.0 * s * Verb::Fillet.section(),
        "a lean of {s}: ΔV {dv} vs the closed form {removed}"
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
/// (the closed-form rows above), and the tessellator meshes it
/// watertight. The boolean takes it with a brick beside the band and
/// with one through it, in every op, tier-3 valid at the closed forms:
/// the notch's overlap is its box, the slot's its box less the band's
/// section over the slot's width. With a brick wholly apart, the
/// containment door's rays from the far brick meet nothing and read the
/// side at infinity. `∖` keeps the filleted body, `∪` adds the brick,
/// `∩` is empty.
#[test]
fn the_ellipse_edges_pass_the_tessellator_and_the_boolean() {
    let body = parallelogram::<f64>();
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &[e], D, tol())
        .expect("the oblique fillet builds");
    let mesh = mesh::tessellate(&out.body, 5e-3, tol()).expect("the filleted body tessellates");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let (v, pad) = volume_enclosure(&out.body);
    for (what, cutter, overlap) in [
        (
            "a notch beside the band",
            brick((1.0, 1.4), (0.8, 1.2), (0.5, 1.5), tol()),
            0.4 * 0.2 * 0.5,
        ),
        (
            "a slot through the band",
            brick((0.9, 1.1), (-0.5, 0.3), (0.6, 1.5), tol()),
            0.2 * 0.3 * 0.4 - Verb::Fillet.section() * 0.2,
        ),
    ] {
        let (brick_v, _) = volume_enclosure(&cutter);
        for (op, want) in [
            (BooleanOp::Subtract, v - overlap),
            (BooleanOp::Union, v + brick_v - overlap),
            (BooleanOp::Intersect, overlap),
        ] {
            let result = realized(op, &out.body, &cutter, tol());
            validate_geometric(&result, tol())
                .unwrap_or_else(|e| panic!("{what} ({op:?}): tier 3, got {e:?}"));
            let (got, pad_r) = volume_enclosure(&result);
            assert!(
                pad + pad_r < pad_ceiling() && (got - want).abs() < 1e-12 + pad + pad_r,
                "{what} ({op:?}): V {got} ± {} vs the closed form {want}",
                pad + pad_r
            );
            assert!(
                (got - want).abs() < midpoint_tol(),
                "{what} ({op:?}): V's midpoint {got} is off the closed form {want}"
            );
        }
    }
    let operand = finished("the filleted parallelogram", out.body, tol());
    let apart = finished(
        "a brick apart",
        brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol()),
        tol(),
    );
    for (op, result, want) in [
        ("subtract", topo::subtract(&operand, &apart, tol()), v),
        ("union", topo::union(&operand, &apart, tol()), v + 1.0),
    ] {
        let result = result.unwrap_or_else(|e| panic!("a brick apart ({op}): builds, got {e:?}"));
        let result = &result.body().expect("not empty").body;
        let (got, pad_r) = volume_enclosure(result);
        assert!(
            (got - want).abs() < 1e-12 + pad + pad_r,
            "a brick apart ({op}): V {got} vs the closed form {want}"
        );
    }
    assert!(
        matches!(
            topo::intersect(&operand, &apart, tol()),
            Ok(topo::BooleanResult::Empty)
        ),
        "a brick apart (intersect): empty"
    );
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
    let body = topo::test_support::finished("body", parallelogram::<Interval>(), tol());
    let e = edge_at(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &[e],
        iv(D),
        tol(),
    )
    .expect("carves at Interval");
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
        topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol()))
            .expect("interval props"),
        topo::mass_properties(
            &topo::test_support::finished("the measured body", topo::Body::clone(&out.body), tol()),
            tol(),
            topo::test_support::pinned_resolution(tol()),
        )
        .expect("interval props"),
    );
    let removed = p0.enclosure.volume - p1.enclosure.volume;
    let pad = p0.enclosure.volume_pad + p1.enclosure.volume_pad;
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

/// A parallelogram prism of height one whose top front edge, `(0, 0, 1)`
/// to `(2, 0, 1)`, ends at two parallel walls leaning `s` along `x` per
/// unit of `y`.
fn leaning(s: f64) -> Body<f64> {
    prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    )
}

/// **Steep walls, to 89°**: the parallelogram's top front and top back
/// edges at leans out to `s = ±60`, each removing its section over the
/// unit-spaced walls' length of two — the major semi-axis `r·√(1 + s²)`
/// runs to six metres beside a sliver a tenth tall, and the arcs lie on
/// the band and the walls to `1e-12` ([`carve`]).
#[test]
fn steep_walls_cut_off_at_the_closed_form() {
    for s in [2.0, 8.0, 20.0, 60.0, -20.0] {
        let body = leaning(s);
        for (what, a, b) in [
            ("front", [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
            ("back", [s, 1.0, 1.0], [2.0 + s, 1.0, 1.0]),
        ] {
            let e = edge(&body, a, b);
            carve(
                &body,
                &[e],
                Verb::Fillet,
                2.0 * Verb::Fillet.section(),
                &format!("the {what} edge at s = {s}"),
            );
        }
    }
}

/// **A draft-angle end wall, in the user's terms**: a one-millimetre
/// fillet at a wall drafted 0.25° and 0.5° off square. The tilt is
/// definite, so the end is the exact ellipse, though at 0.25° its
/// semi-axes differ by `r·(sec θ − 1) ≈ 9.5e-9 m`, inside the default
/// band: both drafts build, tier-3 valid, ending in ellipses, and remove
/// the fillet's section times the length between the walls at its
/// centroid (the walls `x = s·y` and `x = 2 − s·y` stand `2 − 2·s·ȳ`
/// apart there).
#[test]
fn a_drafted_wall_builds_its_ellipse() {
    let r = 1e-3;
    for deg in [0.25f64, 0.5] {
        let s = deg.to_radians().tan();
        let body = prism(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(2.0, 0.0), 0.0),
                (Point2::new(2.0 - s, 1.0), 0.0),
                (Point2::new(s, 1.0), 0.0),
            ],
            1.0,
            tol(),
        );
        let top = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
        let out = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &[top],
            r,
            tol(),
        )
        .unwrap_or_else(|e| panic!("a {deg}° draft builds, got {}", e.error));
        validate_geometric(&out.body, tol()).expect("tier 3");
        let section = r * r * (1.0 - PI / 4.0);
        let centroid = r * (10.0 - 3.0 * PI) / (12.0 - 3.0 * PI);
        let removed = section * (2.0 - 2.0 * s * centroid);
        let ((v0, pad0), (v1, pad1)) = (volume_enclosure(&body), volume_enclosure(&out.body));
        let (dv, pad) = (v0 - v1, pad0 + pad1);
        assert!(
            pad < pad_ceiling() && (dv - removed).abs() < 1e-12 + pad,
            "{deg}°: ΔV {dv:e} ± {pad:e} vs the closed form {removed:e}"
        );
        let arcs = &out.naming.as_ref().expect("births").arcs;
        assert!(!arcs.is_empty(), "{deg}°: the band ends in arcs");
        for (arc, _, _) in arcs {
            let c = out.body.get_edge(*arc).unwrap().curve;
            assert!(
                matches!(
                    out.body
                        .get_curve_geom(c)
                        .and_then(|g| g.certified())
                        .map(|c| c.carrier()),
                    Some(Curve3::Ellipse { .. })
                ),
                "{deg}°: a definite draft ends in the ellipse"
            );
        }
    }
}

/// A round pin of radius `rad` and length `h` whose axis runs along
/// `a × b` from `origin`, to drill a hole in a wall.
fn pin(origin: Point3<f64>, a: Vec3<f64>, b: Vec3<f64>, rad: f64, h: f64) -> Body<f64> {
    prism_on(
        sketch_from_axes(origin, a, b, tol()),
        vec![(Point2::new(-rad, 0.0), 1.0), (Point2::new(rad, 0.0), 1.0)],
        h,
        tol(),
    )
}

/// The leaning prism drilled through its `x = s·y` wall by a pin of
/// radius `rad`, at `t` along the wall's top edge from the old vertex
/// `(0, 0, 1)` and `h` down from the top.
fn drilled(s: f64, t: f64, h: f64, rad: f64) -> Body<f64> {
    let len = (1.0 + s * s).sqrt();
    let along = Vec3::new(s, 1.0, 0.0) / len;
    let out = Vec3::new(-1.0, s, 0.0) / len;
    let at = Point3::new(0.0, 0.0, 1.0 - h) + along * t + out * 0.1;
    let z = Vec3::new(0.0, 0.0, 1.0);
    realized(
        BooleanOp::Subtract,
        &leaning(s),
        &pin(at, along, z, rad, 0.1 + 0.4 * t),
        tol(),
    )
}

/// **What the cut-off takes from a drilled end wall**: the sliver
/// between the elliptic arc and the old vertex, and nothing of the wall
/// the band keeps. On the 45° wall (`s = 1`, major semi-axis `r·√2`
/// along the wall, minor `r` down it, about `(r·√2, r)` in wall
/// coordinates), a hole wholly in the sliver — mid-sliver, or in the
/// cusp at the major vertex — refuses as a bound at its wall: its rings
/// read inside the sliver, through the elliptic arc, but its wall meets
/// the window's end plane, so the reach cannot certify it inside, and a
/// bound outranks an inside reading. The hole in the cusp at the top
/// foot reaches past the arc (its top `(0.1, 0.0045)` is inside the
/// ellipse, whose arc is at `0.0995` there) and refuses `RingClearance`
/// as a measurement. A hole inside the ellipse but outside the disc of
/// its minor semi-axis, or near its major vertex, builds, as it does on
/// the square wall and at a 0.3 lean.
#[test]
fn a_hole_in_a_slanted_wall_refuses_in_the_sliver_and_builds_in_the_kept_wall() {
    let front = |body: &Body<f64>| edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    for (what, t, h, rad) in [
        ("mid-sliver", 0.04, 0.015, 0.004),
        ("the cusp at the major vertex", 0.004, 0.05, 0.0018),
    ] {
        let body = drilled(1.0, t, h, rad);
        match Verb::Fillet.run(&body, &[front(&body)]) {
            Err(BlendError::FaceClearance {
                at: topo::EntityId::Face(_),
                margin,
                bounded: true,
                ..
            }) => assert_eq!(margin.sign, Sign::Negative, "{what}: definite, {margin}"),
            other => panic!(
                "a hole at {what} is wholly in the sliver: a bound at its wall, got {:?}",
                other.map(|_| ())
            ),
        }
    }
    let body = drilled(1.0, 0.10, 0.003, 0.0015);
    let top = Verb::Fillet.run(&body, &[front(&body)]).map(|_| ());
    assert!(
        matches!(top, Err(BlendError::RingClearance { bounded: false, .. })),
        "a hole across the arc at the top foot: a measured RingClearance, got {top:?}"
    );
    for (what, t, h, rad) in [
        (
            "inside the ellipse, outside its minor disc",
            0.03,
            0.07,
            0.005,
        ),
        (
            "inside the ellipse near its major vertex",
            0.012,
            0.10,
            0.004,
        ),
    ] {
        for s in [0.0, 0.3, 1.0] {
            let body = drilled(s, t, h, rad);
            let out = Verb::Fillet
                .run(&body, &[front(&body)])
                .unwrap_or_else(|e| panic!("a hole {what} at s = {s} is kept: builds, got {e}"));
            validate_geometric(&out.body, tol())
                .unwrap_or_else(|e| panic!("a hole {what} at s = {s}: tier 3, got {e:?}"));
        }
    }
}

/// **The square wall's mid-sliver hole goes with the band under both
/// verbs**: the fillet's round end reads its mouth inside the sliver,
/// and the chamfer's chord end inside its triangle; the hole is blind,
/// along the spine, `0.016` deep, and its floor is read inside the
/// band's core, so it is a cup the carve takes. The band runs `2` along
/// `x`: `ΔV = −(2A − π·a²·0.016)`.
#[test]
fn a_hole_in_a_square_walls_sliver_goes_with_the_band_under_both_verbs() {
    let (rad, deep) = (0.004, 0.016);
    let body = drilled(0.0, 0.04, 0.015, rad);
    let front = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let cups = cup_faces(&body, (0.04, 0.985), rad, (0.0, deep));
    assert_eq!(cups.len(), 3, "the hole's two half-walls and floor");
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let section = match verb {
            Verb::Fillet => (1.0 - core::f64::consts::FRAC_PI_4) * D * D,
            Verb::Chamfer => D * D / 2.0,
        };
        let want = -(2.0 * section - core::f64::consts::PI * rad * rad * deep);
        let what = "the square wall's mid-sliver hole";
        assert_taken(verb, &body, &[front], D, (&cups, 1, 0), (want, 1.0), what);
    }
}

/// **A steep wall's own edges stay clear of its sliver**: at `s = 20`
/// (87°) the ellipse's major semi-axis is two metres, the sliver a
/// tenth tall, and the wall's bottom edge `0.9` below the ellipse's
/// centre. A hole far along the wall puts the wall through the ring
/// meter, which reads every one of its edges against the sliver's box
/// in the ellipse's frame, so the bottom edge, clear of it by `0.9`,
/// builds.
#[test]
fn a_steep_drilled_wall_keeps_its_bottom_edge_clear() {
    let body = drilled(20.0, 10.0, 0.5, 0.05);
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = Verb::Fillet
        .run(&body, &[e])
        .unwrap_or_else(|e| panic!("a hole far along a steep wall builds, got {e}"));
    validate_geometric(&out.body, tol()).expect("tier 3");
}

/// **A brick through a steep elliptic end**: on the `s = 3` filleted
/// parallelogram a brick crossing the band's end arc builds in every op
/// at every ε, consistent with itself (`S + I = V`, `U − S = brick`).
#[test]
fn a_brick_through_a_steep_elliptic_end_builds_in_every_op() {
    let body = leaning(3.0);
    let e = edge(&body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let out = fillet_edges(&sweep::test_support::at_rest(&body, tol()), &[e], D, tol())
        .expect("the s = 3 fillet builds");
    let cutter = brick((-0.5, 0.15), (-0.5, 0.05), (0.92, 1.5), tol());
    let (v, _) = volume_enclosure(&out.body);
    let (b, _) = volume_enclosure(&cutter);
    let mut got = [0.0; 3];
    for (slot, op) in
        got.iter_mut()
            .zip([BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect])
    {
        let result = realized(op, &out.body, &cutter, tol());
        validate_geometric(&result, tol()).unwrap_or_else(|e| panic!("{op:?}: tier 3, got {e:?}"));
        *slot = volume_enclosure(&result).0;
    }
    let [sub, uni, int] = got;
    assert!(
        (sub + int - v).abs() < midpoint_tol() && (uni - sub - b).abs() < midpoint_tol(),
        "S {sub} + I {int} = V {v}, U {uni} − S = brick {b}"
    );
}
