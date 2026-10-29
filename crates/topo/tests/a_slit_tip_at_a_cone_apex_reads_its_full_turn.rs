//! PROBE (measurement on main).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{Body, FaceKey, HalfEdgeKey, VertexKey};

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

pub struct Notched {
    pub body: Body<f64>,
    pub lateral: FaceKey,
}

/// The cone of half-angle 45° about +z, apex at the origin, rim at
/// z = 1, with a tetrahedral cavity A P Q1 Q2 touching its wall along
/// the ruling A P only.
pub fn notched_cone(flip: bool) -> Notched {
    let t = tol();
    let line = |a: Point3<f64>, b: Point3<f64>| EdgeCurveSpec::line_between(a, b);
    let a = Point3::new(0.0, 0.0, 0.0);
    let p = Point3::new(0.0, 1.0, 1.0);
    let (q1, q2) = if flip {
        (Point3::new(-0.1, 0.3, 0.6), Point3::new(0.1, 0.3, 0.6))
    } else {
        (Point3::new(0.1, 0.3, 0.6), Point3::new(-0.1, 0.3, 0.6))
    };
        let mut b = Body::<f64>::new();
    let seed = b.mvfs(a, true).unwrap();
    let f0 = seed.face;
    let e1 = b.mev(MevSite::Lone { r#loop: seed.r#loop }, p, line(a, p), t).unwrap();
    let lens = b
        .mef(MefSite::Chords { he1: e1.he_minus, he2: e1.he_plus }, line(p, a), FaceSurface::Inherit, t)
        .unwrap();
    let f1 = lens.face;
    let (va, vp) = (vertex_at(&b, a), vertex_at(&b, p));
    // The cavity in f1.
    let h = out_of(&b, va, f1);
    b.mev(MevSite::Fan { he1: h, he2: h }, q1, line(a, q1), t).unwrap();
    let vq1 = vertex_at(&b, q1);
    let wall1 = b
        .mef(MefSite::Chords { he1: out_of(&b, vq1, f1), he2: out_of(&b, vp, f1) }, line(q1, p), FaceSurface::Inherit, t)
        .unwrap();
    // f1 is wall a (it keeps e1's minus half); wall1 holds e2's.
    let wall_a = f1;
    let t2 = wall1.face;
    let h = out_of(&b, va, t2);
    b.mev(MevSite::Fan { he1: h, he2: h }, q2, line(a, q2), t).unwrap();
    let vq2 = vertex_at(&b, q2);
    let wall_b = b
        .mef(MefSite::Chords { he1: out_of(&b, vq2, t2), he2: out_of(&b, vp, t2) }, line(q2, p), FaceSurface::Inherit, t)
        .unwrap()
        .face;
    let cap_a = b
        .mef(MefSite::Chords { he1: out_of(&b, vq1, t2), he2: out_of(&b, vq2, t2) }, line(q1, q2), FaceSurface::Inherit, t)
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
    let plane = |c: &[Point3<f64>]| FaceSurface::New { surface: topo::test_support::plane(c, t), sense: true };
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
        (base, vec![Point3::new(0.0, -1.0, 1.0), Point3::new(1.0, 0.0, 1.0), Point3::new(0.0, 1.0, 1.0)]),
        (wall_a, vec![q1, p, a]),
        (wall_b, vec![q2, a, p]),
        (cap_a, vec![q1, a, q2]),
        (cap_p, vec![q1, q2, p]),
    ] {
        if let Err(e) = b.set_face_surface(f, plane(&c)) {
            eprintln!("PROBE set {f:?}: {e:?}");
        }
    }
    // At rest: the rim in the base's chart, every line edge as the
    // section of its two faces' surfaces.
    let rim_edge = b
        .edges()
        .find(|(_, e)| b.get_half_edge(e.he_plus).unwrap().start == b.half_edge_end(e.he_plus).unwrap())
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
    let edges: Vec<_> = b.edges().map(|(k, e)| (k, e.he_plus)).filter(|&(k, _)| k != rim_edge).collect();
    for (edge, he) in edges {
        let (s1, s2) = topo::readback::edge_sides(&b, edge).unwrap().surfaces();
        let p0 = *b.get_point(b.get_vertex(b.get_half_edge(he).unwrap().start).unwrap().point).unwrap();
        let p1 = *b.get_point(b.get_vertex(b.half_edge_end(he).unwrap()).unwrap().point).unwrap();
        let witness = p0.lerp(p1, 0.5);
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(s1, s2),
            selector: geom_brep::CurveSelector::Witness(witness),
        };
        b.set_edge_curve(edge, spec, t).unwrap();
    }
    topo::pcurves::mint_pcurves(&mut b, t).unwrap();
    Notched { body: b, lateral: f0 }
}

/// A box `[-5, 5]² × [-10, 0]` in the frame whose third axis is `n`,
/// its top face the plane through `o` along `n`.
fn slab(o: Point3<f64>, n: Vec3<f64>) -> topo::AtRestBody<f64> {
    use topo::test_support::{FaceGeometry, describe_as_intersections, finished, prism_ops};
    let n = n / n.norm();
    let u0 = if n.x.abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u = (u0 - n * u0.dot(n)) / (u0 - n * u0.dot(n)).norm();
    let v = n.cross(u);
    let mut b = Body::<f64>::new();
    prism_ops(
        &mut b,
        &[(-5.0, -5.0), (5.0, -5.0), (5.0, 5.0), (-5.0, 5.0)],
        (-10.0, 0.0),
        |x, y, z| o + u * x + v * y + n * z,
        FaceGeometry::Certified,
        tol(),
    );
    describe_as_intersections(&mut b, tol());
    finished("the slab", b, tol())
}

fn probe(label: &str, body: &topo::AtRestBody<f64>, o: Point3<f64>, n: Vec3<f64>) {
    let plane = topo::test_support::split_plane(o, n / n.norm(), tol());
    match topo::split(body, &plane, tol()) {
        Ok(_) => eprintln!("PROBE {label} split: ok"),
        Err(e) => eprintln!("PROBE {label} split: {e:?}"),
    }
    let s = slab(o, n);
    match topo::intersect(body, &s, tol()) {
        Ok(_) => eprintln!("PROBE {label} intersect slab: ok"),
        Err(e) => eprintln!("PROBE {label} intersect slab: {e:?}"),
    }
}

#[test]
fn probe_measure() {
    let body = topo::test_support::finished("the notched cone", notched_cone(false).body, tol());
    eprintln!("PROBE t3 {:?}", topo::validate_geometric(&body, tol()));
    eprintln!("PROBE t3' {:?}", topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()));
    eprintln!(
        "PROBE vol {:?} want {}",
        topo::mass_properties(&body, tol(), topo::test_support::pinned_resolution(tol())).map(|m| m.enclosure.volume),
        core::f64::consts::PI / 3.0 - 0.01
    );
    let o = Point3::new(0.0, 0.0, 0.0);
    probe("y=0", &body, o, Vec3::new(0.0, 1.0, 0.0));
    probe("tilted", &body, o, Vec3::new(0.3, 1.0, -0.2));
    probe("tilted2", &body, o, Vec3::new(1.0, 0.5, 0.0));
}

#[test]
fn probe_build() {
    for flip in [false, true] {
        let mut n = notched_cone(flip);
        match <f64 as topo::AtRestPolicy>::gate_at_rest_kept(n.body, tol()) {
            Ok(_) => eprintln!("PROBE flip {flip}: at rest"),
            Err(e) => {
                for x in e {
                    eprintln!("PROBE flip {flip}: {x:?}");
                }
            }
        }
    }
}
