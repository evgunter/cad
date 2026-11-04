//! **Probes for the knife-edge blend** (BAND,
//! `work/band/a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined.md`):
//! which bodies a user can author through the public doors carry a cusp
//! or slit edge, and what each blend verb answers on it today. Each row
//! pins the outcome it measured, so a change to any of them shows here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{Open, Profile, ProfileLoop, SketchPlane, Start};
use sweep::blend::{BlendError, chamfer_edges, fillet_edges};
use sweep::test_support::finished;
use sweep::{ExtrudeSide, Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{AtRestBody, Body, BooleanResult, ContactMark, EdgeKey};

fn tol() -> Tol {
    Tol::witness()
}

fn validated(loops: Vec<ProfileLoop<f64>>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("the probe's profile validates")
}

fn slab(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy_at(Point3::new(0.0, 0.0, z0)), loops)
        .validate(tol())
        .expect("the probe's profile validates");
    extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the probe extrudes")
    .body
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

fn circle(cx: f64, cy: f64, r: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(cx + r, cy), 1.0),
        (Point2::new(cx - r, cy), 1.0),
    ])
}

/// The edges check 4 marks `Tangent` with both ends satisfying `at`.
fn tangent_edges_at(body: &Body<f64>, at: impl Fn(&Point3<f64>) -> bool) -> Vec<EdgeKey> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    topo::contact_marks(body, tol())
        .expect("the body is tier-3 valid")
        .iter()
        .filter(|(_, m)| **m == ContactMark::Tangent)
        .map(|(e, _)| e)
        .filter(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            at(&point(body.get_half_edge(he).unwrap().start))
                && at(&point(body.half_edge_end(he).unwrap()))
        })
        .collect()
}

/// Every edge with both ends satisfying `at`, with its check-4 mark.
fn edges_at(
    body: &Body<f64>,
    at: impl Fn(&Point3<f64>) -> bool,
) -> Vec<(EdgeKey, Option<ContactMark>)> {
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let marks = topo::contact_marks(body, tol()).ok();
    body.edges()
        .map(|(e, _)| e)
        .filter(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            at(&point(body.get_half_edge(he).unwrap().start))
                && at(&point(body.half_edge_end(he).unwrap()))
        })
        .map(|e| (e, marks.as_ref().and_then(|m| m.get(e).copied())))
        .collect()
}

/// What each verb answers on `edge` at `size`: `"KnifeEdge"`, the
/// refusal's variant name, or `"built"` / `"left as it is"`.
fn answers(what: &str, body: &AtRestBody<f64>, edge: EdgeKey, size: f64) -> [String; 2] {
    let name = |e: &BlendError| {
        format!("{e:?}")
            .split([' ', '{', '('])
            .next()
            .unwrap()
            .to_owned()
    };
    let read = |verb: &str, got: Result<sweep::blend::Blended<f64>, sweep::blend::BlendRefusal>| {
        let out = match got {
            Ok(b) if b.blend_faces.is_empty() && b.band_faces.is_empty() => {
                "left as it is".to_owned()
            }
            Ok(_) => "built".to_owned(),
            Err(r) => name(&r.error),
        };
        eprintln!("PROBE {what} {verb} at {size}: {out}");
        out
    };
    [
        read("fillet", fillet_edges(body, &[edge], size, tol())),
        read("chamfer", chamfer_edges(body, &[edge], size, tol())),
    ]
}

/// Two unit tubes on a plate, the valley between them a slit: the
/// profile's top runs over the right tube's arc into the kiss at the
/// origin and out over the left tube's, the plate filling the lower
/// cusp. Authored raw; the profile door decides the kiss a cusp joint.
fn valley() -> ProfileLoop<f64> {
    let sweep_angle = std::f64::consts::PI - 0.6_f64.atan2(0.8);
    let b = (sweep_angle / 4.0).tan();
    bulge_loop(vec![
        (Point2::new(-1.8, -1.5), 0.0),
        (Point2::new(1.8, -1.5), 0.0),
        (Point2::new(1.8, 0.6), b),
        (Point2::new(0.0, 0.0), b),
        (Point2::new(-1.8, 0.6), 0.0),
    ])
}

fn on_the_z_axis(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

#[test]
fn two_tubes_valley_authored_as_a_profile_refuses_knife_edge() {
    let profile = validated(vec![valley()]);
    assert_eq!(
        profile.loops()[0].cusp_joints(),
        &[3],
        "the kiss is a cusp joint"
    );
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let [edge] = tangent_edges_at(&body, on_the_z_axis)[..] else {
        panic!("one tangent edge on the kiss");
    };
    let operand = finished("the valley", body, tol());
    for r in [0.05, 0.3] {
        assert_eq!(
            answers("valley", &operand, edge, r),
            ["KnifeEdge", "KnifeEdge"]
        );
    }
}

/// The crescent between the unit circle and its tangent line at 45°,
/// `.cusp()` at the tangency.
fn crescent() -> ProfileLoop<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    Open.at(Point2::new(1.0, 0.0))
        .angle(std::f64::consts::FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(Point2::new(h, h), tol())
        .unwrap()
        .cusp()
        .line(1.0, tol())
        .unwrap()
        .line_to(Start, tol())
        .unwrap()
        .into()
}

#[test]
fn a_cylinder_plane_cusp_and_a_sphere_cone_cusp_rim_refuse_knife_edge() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let profile = validated(vec![crescent()]);
    let prism = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    let [edge] = tangent_edges_at(&prism, |p| (p.x - h).abs() < 1e-9 && (p.y - h).abs() < 1e-9)[..]
    else {
        panic!("one tangent edge on the crescent's kiss");
    };
    let operand = finished("the crescent prism", prism, tol());
    assert_eq!(
        answers("cylinder–plane cusp", &operand, edge, 0.02),
        ["KnifeEdge", "KnifeEdge"]
    );

    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ring = revolve(&profile, axis, Revolution::Full, tol())
        .unwrap()
        .body;
    let [rim] = tangent_edges_at(&ring, |p| {
        (p.y - h).abs() < 1e-9 && (p.x.hypot(p.z) - h).abs() < 1e-9
    })[..] else {
        panic!("one tangent rim");
    };
    let operand = finished("the crescent ring", ring, tol());
    assert_eq!(
        answers("sphere–cone cusp rim", &operand, rim, 0.02),
        ["KnifeEdge", "KnifeEdge"]
    );
}

/// **Two tangent tubes through the Boolean.** The union of two unit
/// discs kissing along `x = 1, y = 0` is an assembly of two solids;
/// each keeps its own edge on the kiss, a seam between its two arcs.
#[test]
fn two_tangent_tubes_unioned_are_two_solids_whose_kiss_edges_are_seams() {
    let a = finished("left", slab(vec![circle(0.0, 0.0, 1.0)], 0.0, 1.0), tol());
    let b = finished("right", slab(vec![circle(2.0, 0.0, 1.0)], 0.0, 1.0), tol());
    let Ok(BooleanResult::Body(u)) = topo::union(&a, &b, tol()) else {
        panic!("the kiss unions");
    };
    let on_kiss = |p: &Point3<f64>| (p.x - 1.0).abs() < 1e-9 && p.y.abs() < 1e-9;
    let kiss = edges_at(&u.body, on_kiss);
    eprintln!("PROBE union kind {:?}, kiss edges {kiss:?}", u.kind);
    assert_eq!(kiss.len(), 2);
    for (e, _) in kiss {
        assert_eq!(
            answers("tubes ∪", &u.body, e, 0.1),
            ["left as it is", "left as it is"]
        );
    }
}

/// **The valley through the Boolean**: a plate whose top face passes
/// through both tube axes and the kiss, unioned with the two tubes.
#[test]
fn the_valley_through_the_boolean() {
    let plate = finished(
        "plate",
        slab(vec![rect(-1.8, -1.5, 1.8, 0.0)], 0.0, 1.0),
        tol(),
    );
    let a = finished("left", slab(vec![circle(-1.0, 0.0, 1.0)], 0.0, 1.0), tol());
    let b = finished("right", slab(vec![circle(1.0, 0.0, 1.0)], 0.0, 1.0), tol());
    let show = |what: &str, got: &Result<BooleanResult<f64>, topo::BooleanError>| match got {
        Ok(BooleanResult::Body(r)) => eprintln!(
            "PROBE {what}: body {:?}, {} solids",
            r.kind,
            r.body.solids().count()
        ),
        Ok(BooleanResult::Empty) => eprintln!("PROBE {what}: empty"),
        Err(e) => eprintln!(
            "PROBE {what}: {:?} {}",
            e.kind(),
            e.render(geom_brep::recourse::Reading::Build)
        ),
    };
    let pa = topo::union(&plate, &a, tol());
    show("plate ∪ left", &pa);
    let ab = topo::union(&a, &b, tol());
    show("left ∪ right", &ab);
    if let Ok(BooleanResult::Body(pa)) = &pa {
        let pab = topo::union(&pa.body, &b, tol());
        show("(plate ∪ left) ∪ right", &pab);
        if let Ok(BooleanResult::Body(r)) = &pab {
            let kiss = tangent_edges_at(&r.body, on_the_z_axis);
            eprintln!("PROBE valley ∪: {} kiss edges", kiss.len());
            for e in kiss {
                answers("valley ∪", &r.body, e, 0.1);
            }
        }
    }
    if let Ok(BooleanResult::Body(ab)) = &ab {
        show(
            "(left ∪ right) ∪ plate",
            &topo::union(&ab.body, &plate, tol()),
        );
    }
}
