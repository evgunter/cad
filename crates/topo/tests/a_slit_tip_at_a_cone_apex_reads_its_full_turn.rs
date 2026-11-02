//! **A slit's tip at a cone's apex reads the face's whole turn.** The
//! cone of half-angle 45° about +z, apex A at the origin, rim at
//! `z = 1`, holding the tetrahedral cavity A R Q1 Q2, `R = (0, 1, 1)` on
//! the rim and `Q1, Q2 = (±0.1, 0.3, 0.6)` inside: the cavity touches
//! the wall along the ruling A R only, so the material either side of it
//! is two wedges, wall | cavity face A R Q1 and cavity face A R Q2 | wall
//! (`crates/topo/README.md`, tier 3), and the wall carries the ruling as
//! a slit, two coincident edges from the rim to their tip at the apex.
//! The wall's one loop runs round the rim and up the slit to the apex
//! and back: a cap less a slit, which sweeps the whole turn round the
//! apex. No door builds it (a Boolean refuses an edge at a cone's apex,
//! `CrossingAtConeApex`), so it is built through the Euler doors.
//!
//! Split through the apex, by planes that hold both slit edges on one
//! side, the walk reads the wall's corner at the apex between the slit's
//! two edges as the window of the whole turn, and each side holds its
//! closed form, the cone over its part of the rim's disc less the
//! cavity, and passes tiers 3 and 3′.
//!
//! A Boolean against the half-space beyond each of those planes reads the
//! apex as the wall's rulings too (`boolean::sectors::build_sectors`): each
//! op builds its closed form and places a lattice of points as the region
//! does, but for the half-space less the cone where the cavity lies in
//! the half-space, whose result holds the cavity touching the cone's hole
//! along the slit's ruling, and refuses typed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{
    Body, BooleanError, ContactRecords, FaceKey, HalfEdgeKey, SplitPart, VertexKey, split,
    validate_geometric, validate_pseudomanifold,
};

fn tol() -> Tol {
    Tol::witness()
}

fn out_of(body: &Body<f64>, v: VertexKey, f: FaceKey) -> HalfEdgeKey {
    let hs: Vec<_> = body
        .half_edges()
        .filter(|(h, he)| he.start == v && body.face_of_half_edge(*h) == Some(f))
        .map(|(h, _)| h)
        .collect();
    assert_eq!(hs.len(), 1, "one half-edge out of {v:?} in {f:?}: {hs:?}");
    hs[0]
}

fn vertex_at(body: &Body<f64>, p: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|(_, v)| body.get_point(v.point).unwrap().distance(p) < 1e-12)
        .unwrap()
        .0
}

/// The body, through the Euler doors (module docs).
fn notched_cone() -> Body<f64> {
    let t = tol();
    let line = |a: Point3<f64>, b: Point3<f64>| EdgeCurveSpec::line_between(a, b);
    let a = Point3::new(0.0, 0.0, 0.0);
    let p = Point3::new(0.0, 1.0, 1.0);
    let (q1, q2) = (Point3::new(0.1, 0.3, 0.6), Point3::new(-0.1, 0.3, 0.6));
    let mut b = Body::<f64>::new();
    let seed = b.mvfs(a, true).unwrap();
    let f0 = seed.face;
    let e1 = b
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p,
            line(a, p),
            t,
        )
        .unwrap();
    let lens = b
        .mef(
            MefSite::Chords {
                he1: e1.he_minus,
                he2: e1.he_plus,
            },
            line(p, a),
            FaceSurface::Inherit,
            t,
        )
        .unwrap();
    let f1 = lens.face;
    let (va, vp) = (vertex_at(&b, a), vertex_at(&b, p));
    // The cavity in f1.
    let h = out_of(&b, va, f1);
    b.mev(MevSite::Fan { he1: h, he2: h }, q1, line(a, q1), t)
        .unwrap();
    let vq1 = vertex_at(&b, q1);
    let wall1 = b
        .mef(
            MefSite::Chords {
                he1: out_of(&b, vq1, f1),
                he2: out_of(&b, vp, f1),
            },
            line(q1, p),
            FaceSurface::Inherit,
            t,
        )
        .unwrap();
    // f1 is wall a (it keeps e1's minus half); wall1 holds e2's.
    let wall_a = f1;
    let t2 = wall1.face;
    let h = out_of(&b, va, t2);
    b.mev(MevSite::Fan { he1: h, he2: h }, q2, line(a, q2), t)
        .unwrap();
    let vq2 = vertex_at(&b, q2);
    let wall_b = b
        .mef(
            MefSite::Chords {
                he1: out_of(&b, vq2, t2),
                he2: out_of(&b, vp, t2),
            },
            line(q2, p),
            FaceSurface::Inherit,
            t,
        )
        .unwrap()
        .face;
    let cap_a = b
        .mef(
            MefSite::Chords {
                he1: out_of(&b, vq1, t2),
                he2: out_of(&b, vq2, t2),
            },
            line(q1, q2),
            FaceSurface::Inherit,
            t,
        )
        .unwrap()
        .face;
    let cap_p = t2;
    // The rim, a circle at P.
    let h = out_of(&b, vp, f0);
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::new(0.0, 0.0, -1.0),
        radius: 1.0,
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let base = b
        .mef(
            MefSite::Chords { he1: h, he2: h },
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::TAU).unwrap(),
            FaceSurface::Inherit,
            t,
        )
        .unwrap()
        .face;
    let plane = |c: &[Point3<f64>]| FaceSurface::New {
        surface: topo::test_support::plane(c, t),
        sense: true,
    };
    b.set_face_surface(
        f0,
        FaceSurface::New {
            surface: Surface::Cone {
                apex: a,
                axis: Vec3::new(0.0, 0.0, 1.0),
                half_angle: core::f64::consts::FRAC_PI_4,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    for (f, c) in [
        (
            base,
            vec![
                Point3::new(0.0, -1.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
            ],
        ),
        (wall_a, vec![q1, p, a]),
        (wall_b, vec![q2, a, p]),
        (cap_a, vec![q1, a, q2]),
        (cap_p, vec![q1, q2, p]),
    ] {
        b.set_face_surface(f, plane(&c)).unwrap();
    }
    // At rest: the rim in the base's chart, every line edge as the
    // section of its two faces' surfaces.
    let rim_edge = b
        .edges()
        .find(|(_, e)| {
            b.get_half_edge(e.he_plus).unwrap().start == b.half_edge_end(e.he_plus).unwrap()
        })
        .unwrap()
        .0;
    {
        let (s1, s2) = topo::readback::edge_sides(&b, rim_edge).unwrap().surfaces();
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(0.0, 0.0, -1.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        };
        let mut spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU).unwrap();
        spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(Point3::new(0.0, -1.0, 1.0)),
        };
        b.set_edge_curve(rim_edge, spec, t).unwrap();
    }
    let edges: Vec<_> = b
        .edges()
        .map(|(k, e)| (k, e.he_plus))
        .filter(|&(k, _)| k != rim_edge)
        .collect();
    for (edge, he) in edges {
        let (s1, s2) = topo::readback::edge_sides(&b, edge).unwrap().surfaces();
        let p0 = *b
            .get_point(
                b.get_vertex(b.get_half_edge(he).unwrap().start)
                    .unwrap()
                    .point,
            )
            .unwrap();
        let p1 = *b
            .get_point(b.get_vertex(b.half_edge_end(he).unwrap()).unwrap().point)
            .unwrap();
        let witness = p0.lerp(p1, 0.5);
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(witness),
        };
        b.set_edge_curve(edge, spec, t).unwrap();
    }
    topo::pcurves::mint_pcurves(&mut b, t).unwrap();
    b
}

/// The cavity's volume, `|det(R − A, Q1 − A, Q2 − A)| / 6`.
const CAVITY: f64 = 0.01;

/// A plane through the apex along `n` (`n_z` small enough that it
/// meets the rim) holds below it the cone over the part of the rim's
/// unit disc where `n·(x, y, 1) < 0`: a third of that area, the disc's
/// segment off the line at its distance `d` from the centre.
fn cone_below(n: Vec3<f64>) -> f64 {
    let d = n.z / n.x.hypot(n.y);
    let segment = |d: f64| d.acos() - d * (1.0 - d * d).sqrt();
    // The centre reads `n_z`: below where it is negative.
    let area = if d < 0.0 {
        core::f64::consts::PI - segment(-d)
    } else {
        segment(d)
    };
    area / 3.0
}

/// **The body is the slit form, at rest**: tiers 3 and 3′ pass and it
/// holds the cone less the cavity.
#[test]
fn the_cone_carries_the_ruling_as_a_slit_to_its_apex() {
    let body = topo::test_support::finished("the notched cone", notched_cone(), tol());
    let want = core::f64::consts::PI / 3.0 - CAVITY;
    let v = topo::test_support::pinned_volume("the notched cone", &body, tol()).volume;
    assert!(
        (v - want).abs() < 1e-12,
        "the notched cone holds {v}, not {want}"
    );
    assert_eq!(validate_geometric(&body, tol()), Ok(()), "tier 3");
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), tol()),
        Ok(()),
        "tier 3′"
    );
}

/// **A split through the apex builds**, each side at its closed form,
/// the cavity above: along the axis square to the slit's plane, along
/// it tilted, and leaning off it.
#[test]
fn a_split_through_the_apex_at_the_slits_tip_builds() {
    let body = topo::test_support::finished("the notched cone", notched_cone(), tol());
    for n in [
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.5, 0.0),
        Vec3::new(0.3, 1.0, -0.2),
    ] {
        let what = format!("the split along {n:?}");
        let plane =
            topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.0), n / n.norm(), tol());
        let r = split(&body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let below = cone_below(n);
        let above = core::f64::consts::PI / 3.0 - below - CAVITY;
        for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
            let what = format!("{what}, {side}");
            let SplitPart::Body(b) = part else {
                panic!("{what}: empty");
            };
            let v = topo::test_support::pinned_volume(&what, b, tol()).volume;
            assert!((v - want).abs() < 1e-12, "{what}: holds {v}, not {want}");
            assert_eq!(validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
            assert_eq!(
                validate_pseudomanifold(b, &ContactRecords::default(), tol()),
                Ok(()),
                "{what}: tier 3′"
            );
        }
    }
}

/// **The split through the apex with each normal reversed**, the
/// slit's edges and the cavity below the plane, and through planes
/// holding or nearly holding the slit's ruling, the cavity above.
#[test]
fn a_split_through_the_apex_reversed_or_near_the_slit_builds() {
    let body = topo::test_support::finished("the notched cone", notched_cone(), tol());
    for n in [
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(-1.0, -0.5, 0.0),
        Vec3::new(-0.3, -1.0, 0.2),
        // Through a cavity face's plane, tilted 1e-3 and not: the slit's
        // ruling a hair above the plane, or in it.
        Vec3::new(-0.3, -0.1, 0.101),
        Vec3::new(0.3, -0.1, 0.101),
        Vec3::new(-0.3, -0.1, 0.1),
        Vec3::new(0.3, -0.1, 0.1),
    ] {
        let what = format!("the split along {n:?}");
        let plane =
            topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.0), n / n.norm(), tol());
        let r = split(&body, &plane, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        let cavity_above = n.dot(Vec3::new(0.0, 0.3, 0.6)) > 0.0;
        let below = cone_below(n) - if cavity_above { 0.0 } else { CAVITY };
        let above = core::f64::consts::PI / 3.0 - CAVITY - below;
        for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
            let what = format!("{what}, {side}");
            let SplitPart::Body(b) = part else {
                panic!("{what}: empty");
            };
            let v = topo::test_support::pinned_volume(&what, b, tol()).volume;
            assert!((v - want).abs() < 1e-12, "{what}: holds {v}, not {want}");
            assert_eq!(validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
            assert_eq!(
                validate_pseudomanifold(b, &ContactRecords::default(), tol()),
                Ok(()),
                "{what}: tier 3′"
            );
        }
    }
}

/// The half-space `n·p > 0` (`n` unit), as the cube of side 8 with a face
/// on its plane, the apex at that face's centre.
fn half_space(n: Vec3<f64>) -> topo::AtRestBody<f64> {
    use geom_core::Affine3;
    let b = topo::test_support::brick((0.0, 8.0), (-4.0, 4.0), (-4.0, 4.0), tol());
    let x = Vec3::new(1.0, 0.0, 0.0);
    let c = x.cross(n);
    let turn = if c.norm() < 1e-15 {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            core::f64::consts::PI * (1.0 - n.x) / 2.0,
        )
    } else {
        Affine3::rotation_about_axis(
            Point3::origin(),
            c / c.norm(),
            x.dot(n).clamp(-1.0, 1.0).acos(),
        )
    };
    let moved =
        topo::transform_rigid(&b, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    topo::test_support::finished("the half-space", moved, tol())
}

/// The notched cone's region as a signed margin, positive inside: the
/// cone less the cavity, each the least of its faces' margins.
fn the_notched_cone(p: Point3<f64>) -> f64 {
    let cone = (p.z - p.x.hypot(p.y)).min(1.0 - p.z);
    let a = Point3::new(0.0, 0.0, 0.0);
    let r = Point3::new(0.0, 1.0, 1.0);
    let (q1, q2) = (Point3::new(0.1, 0.3, 0.6), Point3::new(-0.1, 0.3, 0.6));
    let corners = [a, r, q1, q2];
    let mut cavity = f64::INFINITY;
    for k in 0..4 {
        let [u, v, w] = [0, 1, 2].map(|j| corners[(k + 1 + j) % 4]);
        let n = (v - u).cross(w - u);
        let n = n / n.norm();
        // Inward: toward the corner off this face.
        let n = if n.dot(corners[k] - u) > 0.0 { n } else { -n };
        cavity = cavity.min(n.dot(p - u));
    }
    cone.min(-cavity)
}

/// Holds `b` to `want` and to `region` at a lattice about the cone, and to
/// tiers 3 and 3′ over the contacts it records.
fn holds(what: &str, b: &topo::BooleanBody<f64>, want: f64, region: &dyn Fn(Point3<f64>) -> f64) {
    let v = topo::test_support::pinned_volume(what, &b.body, tol()).volume;
    assert!((v - want).abs() < 1e-9, "{what}: holds {v}, not {want}");
    assert_eq!(validate_geometric(&b.body, tol()), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(&b.body, &b.contacts, tol()),
        Ok(()),
        "{what}: tier 3′"
    );
    let band = geom_core::Band::linear(tol()).unwrap();
    let n = 8;
    let at = |t: usize, o: f64, lo: f64, w: f64| lo + w * (t as f64 + o) / (n as f64 - 1.0 + 0.37);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let p = Point3::new(
                    at(i, 0.11, -1.1, 2.2),
                    at(j, 0.23, -1.1, 2.2),
                    at(k, 0.07, -0.1, 1.2),
                );
                let s = region(p);
                if s.abs() < 3e-3 {
                    continue;
                }
                let want = if s > 0.0 {
                    topo::SolidContainment::In
                } else {
                    topo::SolidContainment::Out
                };
                assert_eq!(
                    topo::point_in_solid(&b.body, p, band, tol()),
                    Ok(want),
                    "{what}: places {p:?}"
                );
            }
        }
    }
}

/// **A Boolean through the apex at the slit's tip builds**, against the
/// half-space beyond each of the split rows' planes: each op at its
/// closed form, the cone over its part of the rim's disc less the cavity
/// where the cavity lies on its side, and placing a lattice of points as
/// the region does; but for the half-space less the cone where the
/// cavity lies in the half-space, which refuses typed.
#[test]
fn a_boolean_through_the_apex_at_the_slits_tip_builds() {
    let body = topo::test_support::finished("the notched cone", notched_cone(), tol());
    let cone = core::f64::consts::PI / 3.0 - CAVITY;
    for n in [
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 0.5, 0.0),
        Vec3::new(0.3, 1.0, -0.2),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(-1.0, -0.5, 0.0),
        Vec3::new(-0.3, -1.0, 0.2),
        Vec3::new(-0.3, -0.1, 0.101),
        Vec3::new(0.3, -0.1, 0.101),
        Vec3::new(-0.3, -0.1, 0.1),
        Vec3::new(0.3, -0.1, 0.1),
    ] {
        // A plane holding a cavity face, `(±0.3, −0.1, 0.1)`.
        let flush = n.y == -0.1 && n.z == 0.1;
        let n = n / n.norm();
        let h = half_space(n);
        // The cavity lies on the half-space's side where its centroid does.
        let cavity_beyond = n.dot(Vec3::new(0.0, 0.3, 0.6)) > 0.0;
        let beyond =
            core::f64::consts::PI / 3.0 - cone_below(n) - if cavity_beyond { CAVITY } else { 0.0 };
        let half = move |p: Point3<f64>| n.dot(p - Point3::origin());
        type Region = Box<dyn Fn(Point3<f64>) -> f64>;
        for (op, r, want, region) in [
            (
                "c ∩ h",
                topo::intersect(&body, &h, tol()),
                beyond,
                Box::new(move |p| the_notched_cone(p).min(half(p))) as Region,
            ),
            (
                "h ∩ c",
                topo::intersect(&h, &body, tol()),
                beyond,
                Box::new(move |p| the_notched_cone(p).min(half(p))),
            ),
            (
                "c ∖ h",
                topo::subtract(&body, &h, tol()),
                cone - beyond,
                Box::new(move |p| the_notched_cone(p).min(-half(p))),
            ),
            (
                "c ∪ h",
                topo::union(&body, &h, tol()),
                512.0 + cone - beyond,
                Box::new(move |p| the_notched_cone(p).max(half(p))),
            ),
            (
                "h ∖ c",
                topo::subtract(&h, &body, tol()),
                512.0 - beyond,
                Box::new(move |p| half(p).min(-the_notched_cone(p))),
            ),
        ] {
            let what = format!("n {n:?}, {op}");
            match r {
                Ok(topo::BooleanResult::Body(b)) => holds(&what, &b, want, &region),
                // The box less the cone holds the cavity as a solid of its
                // own, touching the cone's hole along the slit's ruling:
                // the result's two coincident edges there refuse, or where
                // the plane holds a cavity face, the edge it leaves in it.
                Err(BooleanError::LineWedges {
                    fault: topo::LineWedgeFault::Overlap { .. },
                    ..
                }) if op == "h ∖ c" && cavity_beyond && !flush => {}
                Err(BooleanError::EdgeInFace {
                    gap: topo::EdgeInFaceGap::Pocket,
                    ..
                }) if op == "h ∖ c" && cavity_beyond && flush => {}
                other => panic!("{what}: {other:?}"),
            }
        }
    }
}
