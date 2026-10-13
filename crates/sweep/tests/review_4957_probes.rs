//! Review probes for PR 4957 (bore ruling step 4: the planar band and
//! the chamfer's exact triangle).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::interval::iv;
use geom_core::{Decide, Interval, Point2, Point3, Real, Vec3};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::BlendError;
use sweep::blend::build::{Blended, chamfer_edges, fillet_edges};
use sweep::test_support::{
    at_rest, extruded, finished, prism, prism_on, realized, sketch_from_axes,
};
use topo::boolean::BooleanOp;
use topo::{Body, ContactRecords, EdgeKey, validate_geometric, validate_pseudomanifold};

use crate::band_planar_cut_off::{D, Verb, edge, tol, volume_enclosure};

fn poly<T: Real>(pts: &[(f64, f64)]) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(
        pts.iter()
            .map(|&(x, y)| (Point2::new(f(x), f(y)), f(0.0)))
            .collect(),
    )
}

fn round<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

/// The parallelogram `V = (0, 0)`, `(1, 0)`, `(1, 0) + e`, `e`, with
/// `e = (cos θ, sin θ)`, holed by `hole`, extruded `1` along `z`: the
/// crease at `V` is a plane–plane edge of interior angle `θ`.
fn dihedral<T: Decide + topo::AtRestPolicy>(
    theta: f64,
    hole: ProfileLoop<T>,
) -> topo::AtRestBody<T> {
    let (c, s) = (theta.cos(), theta.sin());
    let outline = poly::<T>(&[(0.0, 0.0), (1.0, 0.0), (1.0 + c, s), (c, s)]);
    let body = extruded(
        SketchPlane::xy(),
        vec![outline, hole],
        T::from_f64(1.0),
        tol(),
    );
    finished("dihedral", body, tol())
}

fn crease_at_origin<T: Real + geom_core::Bounds>(body: &Body<T>) -> EdgeKey {
    let mid = |x: T| 0.5 * (x.lo() + x.hi());
    topo::query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let on = |q: Point3<T>| mid(q.x).abs() < 1e-12 && mid(q.y).abs() < 1e-12;
            on(p(body.get_half_edge(he).unwrap().start)) && on(p(body.half_edge_end(he).unwrap()))
        })
        .expect("the crease at V")
}

/// The point on the bisector of `θ` at distance `dist` from the chord
/// of the chamfer of setback `D`, towards `V`.
fn inside_chord(theta: f64, dist: f64) -> (f64, f64) {
    let t = D * (theta / 2.0).cos() - dist;
    (t * (theta / 2.0).cos(), t * (theta / 2.0).sin())
}

fn chamfer(body: &Body<f64>, e: EdgeKey) -> Result<Blended<f64>, BlendError> {
    chamfer_edges(&at_rest(body, tol()), &[e], D, tol()).map_err(|r| r.error)
}

fn assert_built(out: &Blended<f64>, src: &Body<f64>, removed: f64, what: &str) {
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3 {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3' {e:?}"));
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "{what}: no ring kept"
    );
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!((c.s, c.genus()), (1, Ok(0)), "{what}: genus 0");
    let ((v0, p0), (v1, p1)) = (volume_enclosure(src), volume_enclosure(&out.body));
    assert!(
        ((v0 - v1) - removed).abs() < 1e-10 + p0 + p1,
        "{what}: ΔV {} vs closed form {removed}",
        v0 - v1
    );
}

/// **P1: the triangle is exact at a non-right dihedral.** At `θ` of
/// 45°, 60°, 120° and 135° the chamfer of setback `D` cuts off the
/// triangle `V, D·(1, 0), D·e`, area `½D²·sin θ`. A bore `1e-4` inside
/// the chord goes at `ΔV = (½D² sin θ − πa²)·1`; one `1e-4` across it,
/// or centred on it, refuses `RingClearance` measured; one `1e-4` off a
/// support near `V` goes too.
#[test]
fn p1_the_chamfer_triangle_is_exact_at_every_dihedral() {
    let (a, hair) = (0.004, 1e-4);
    for deg in [45.0f64, 60.0, 120.0, 135.0] {
        let th = deg.to_radians();
        let removed = 0.5 * D * D * th.sin() - PI * a * a;
        let (x, y) = inside_chord(th, a + hair);
        let body = dihedral::<f64>(th, round(x, y, a));
        let out = chamfer(&body, crease_at_origin(&body))
            .unwrap_or_else(|e| panic!("{deg}°: a hair inside the chord goes, got {e}"));
        assert_built(&out, &body, removed, &format!("{deg}° hair inside chord"));
        for (what, dist) in [("hair across", a - hair), ("on", 0.0)] {
            let (x, y) = inside_chord(th, dist);
            let body = dihedral::<f64>(th, round(x, y, a));
            let got = chamfer(&body, crease_at_origin(&body));
            assert!(
                matches!(got, Err(BlendError::RingClearance { bounded: false, .. })),
                "{deg}° {what} the chord: measured RingClearance, got {:?}",
                got.map(|_| ())
            );
        }
        // Near the support y = 0: centre a + hair off it, a third of the
        // way from V to the foot.
        let (x, y) = (D / 3.0 + a, a + hair);
        let body = dihedral::<f64>(th, round(x, y, a));
        match chamfer(&body, crease_at_origin(&body)) {
            Ok(out) => assert_built(&out, &body, removed, &format!("{deg}° near the rim")),
            Err(e) => panic!("{deg}°: a bore a hair off the support inside the triangle, got {e}"),
        }
    }
}

/// The leaning prism of `band_planar_consume`, bored along `x` from
/// `(0, p)` to `(2, q)` in `(x, (y, z))`.
fn bored(s: f64, p: (f64, f64), q: (f64, f64), a: f64, u: Vec3<f64>) -> Body<f64> {
    let leaning = prism(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 + s, 1.0), 0.0),
            (Point2::new(s, 1.0), 0.0),
        ],
        1.0,
        tol(),
    );
    let t = Vec3::new(2.0, q.0 - p.0, q.1 - p.1);
    let th = t.normalize();
    let over = 1.0 + s.abs();
    let o = Point3::new(0.0, p.0, p.1) - th * (over / th.x);
    let u = (u - th * u.dot(th)).normalize();
    let pin = prism_on(
        sketch_from_axes(o, u, th.cross(u), tol()),
        vec![(Point2::new(-a, 0.0), 1.0), (Point2::new(a, 0.0), 1.0)],
        t.norm() + 2.0 * over / th.x,
        tol(),
    );
    realized(BooleanOp::Subtract, &leaning, &pin, tol())
}

fn front(body: &Body<f64>) -> EdgeKey {
    edge(body, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0])
}

/// In `(y, z)`: the point `dist` from the chamfer's chord towards the
/// old vertex `(0, 1)`, on the normal through the chord's midpoint.
fn past_chord_yz(dist: f64) -> (f64, f64) {
    let m = (D / 2.0, 1.0 - D / 2.0);
    (m.0 - dist / 2f64.sqrt(), m.1 + dist / 2f64.sqrt())
}

/// **P2: the triangle at oblique caps.** At leans `±1` and `0.3` a bore
/// along the edge `1e-4` inside the chord goes at `ΔV = (D²/2 − πa²)·2`;
/// one `1e-4` across it refuses `RingClearance` measured.
#[test]
fn p2_the_chamfer_triangle_holds_at_oblique_caps() {
    let (a, hair) = (0.004, 1e-4);
    for s in [1.0, -1.0, 0.3] {
        let p = past_chord_yz(a + hair);
        let body = bored(s, p, p, a, Vec3::new(0.0, 1.0, 0.0));
        let out = chamfer(&body, front(&body))
            .unwrap_or_else(|e| panic!("lean {s}: a hair inside the chord goes, got {e}"));
        assert_built(
            &out,
            &body,
            (D * D / 2.0 - PI * a * a) * 2.0,
            &format!("lean {s}"),
        );
        let p = past_chord_yz(a - hair);
        let body = bored(s, p, p, a, Vec3::new(0.0, 1.0, 0.0));
        let got = chamfer(&body, front(&body));
        assert!(
            matches!(got, Err(BlendError::RingClearance { bounded: false, .. })),
            "lean {s}: a hair across the chord, measured RingClearance, got {:?}",
            got.map(|_| ())
        );
    }
}

/// **P3: a tilted bore inside the triangle at one cap and outside it at
/// the other crosses the chamfer plane mid-length, and refuses**; so
/// does one whose far ring crosses the chord, and one whose far ring
/// lies in the kept material beside the rim.
#[test]
fn p3_a_bore_leaving_the_chamfer_prism_refuses() {
    let a = 0.003;
    let inside = past_chord_yz(0.015);
    for (what, far) in [
        (
            "far ring wholly in the kept material past the chord",
            past_chord_yz(-0.02),
        ),
        ("far ring across the chord", past_chord_yz(0.0)),
        (
            "far ring in the kept material below the triangle",
            (0.01, 1.0 - D - 0.02),
        ),
    ] {
        let body = bored(0.0, inside, far, a, Vec3::new(0.0, 1.0, 0.0));
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3 {e:?}"));
        let got = Verb::Chamfer.run(&body, &[front(&body)]);
        eprintln!("P3 {what}: {:?}", got.as_ref().map(|_| ()));
        assert!(
            got.is_err(),
            "{what}: a bore crossing the chamfer plane must refuse"
        );
    }
    // Under the fillet the first pose's bore lies in the kept ball at
    // both caps (≈ 0.086 and 0.051 from the ball's centre, radius 0.1):
    // clear of the band, it is kept and the band takes its section.
    let body = bored(
        0.0,
        inside,
        past_chord_yz(-0.02),
        a,
        Vec3::new(0.0, 1.0, 0.0),
    );
    let out = Verb::Fillet
        .run(&body, &[front(&body)])
        .expect("the kept bore builds");
    let ((v0, p0), (v1, p1)) = (volume_enclosure(&body), volume_enclosure(&out.body));
    let want = (1.0 - PI / 4.0) * D * D * 2.0;
    assert!(
        ((v0 - v1) - want).abs() < 1e-10 + p0 + p1,
        "fillet ΔV {} vs {want}",
        v0 - v1
    );
    let c = topo::readback::euler_counts(&out.body);
    assert_eq!(c.genus(), Ok(1), "the fillet keeps the bore");
    {}
}

/// **P4: within one escalate width of the chord never builds.** A bore
/// whose ring lies `±escalate/2` from the chord refuses (escalated or
/// typed), never consumed.
#[test]
fn p4_a_ring_within_the_escalate_width_of_the_chord_never_goes() {
    let a = 0.004;
    let esc = geom_core::Band::linear(tol()).unwrap().escalate();
    for off in [0.5 * esc, -0.5 * esc, 0.0] {
        let (x, y) = inside_chord(PI / 2.0, a + off);
        let body = dihedral::<f64>(PI / 2.0, round(x, y, a));
        let got = chamfer(&body, crease_at_origin(&body));
        eprintln!("P4 off {off:e}: {:?}", got.as_ref().map(|_| ()));
        assert!(
            got.is_err(),
            "off {off:e}: within the band of the chord must not build"
        );
    }
}

/// **P5: at the certified scalar**, a ring a hair across the chord
/// refuses and one a hair inside goes, under the chamfer.
#[test]
fn p5_the_triangle_at_interval() {
    let (a, hair) = (0.004, 1e-4);
    let th = PI / 2.0;
    let (x, y) = inside_chord(th, a - hair);
    let body = dihedral::<Interval>(th, round(x, y, a));
    let got = chamfer_edges(
        &at_rest(&body, tol()),
        &[crease_at_origin(&body)],
        iv(D),
        tol(),
    );
    assert!(
        matches!(
            got.as_ref().map_err(|r| &r.error),
            Err(BlendError::RingClearance { bounded: false, .. })
        ),
        "Interval: a hair across the chord, got {:?}",
        got.as_ref().map(|_| ()).map_err(|r| &r.error)
    );
    let (x, y) = inside_chord(th, a + hair);
    let body = dihedral::<Interval>(th, round(x, y, a));
    let got = chamfer_edges(
        &at_rest(&body, tol()),
        &[crease_at_origin(&body)],
        iv(D),
        tol(),
    );
    assert!(
        got.is_ok(),
        "Interval: a hair inside goes, got {:?}",
        got.err().map(|r| r.error)
    );
}

/// **P6: the fillet at a non-right dihedral**: a bore a hair past the
/// arc goes, one a hair short refuses measured, at 60° and 120°.
#[test]
fn p6_the_planar_fillet_at_non_right_dihedrals() {
    let (a, hair) = (0.004, 1e-4);
    for deg in [60.0f64, 120.0] {
        let th = deg.to_radians();
        // Ball centre on the bisector at D / sin(θ/2) from V; the arc's
        // nearest point to V is D before it.
        let c = D / (th / 2.0).sin();
        let at = |dist_from_arc: f64| {
            let t = c - D - dist_from_arc;
            (t * (th / 2.0).cos(), t * (th / 2.0).sin())
        };
        let (x, y) = at(a + hair);
        let body = dihedral::<f64>(th, round(x, y, a));
        let e = crease_at_origin(&body);
        let got = fillet_edges(&at_rest(&body, tol()), &[e], D, tol()).map_err(|r| r.error);
        eprintln!("P6 {deg}° inside: {:?}", got.as_ref().map(|_| ()));
        let out = got.unwrap_or_else(|e| panic!("{deg}°: a hair past the arc goes, got {e}"));
        let sector = (PI - th) / 2.0 * D * D;
        let kite = D * D / (th / 2.0).tan();
        assert_built(
            &out,
            &body,
            kite - sector - PI * a * a,
            &format!("{deg}° fillet"),
        );
        let (x, y) = at(a - hair);
        let body = dihedral::<f64>(th, round(x, y, a));
        let e = crease_at_origin(&body);
        let got = fillet_edges(&at_rest(&body, tol()), &[e], D, tol()).map_err(|r| r.error);
        assert!(
            matches!(got, Err(BlendError::RingClearance { bounded: false, .. })),
            "{deg}°: a hair across the arc, got {:?}",
            got.map(|_| ())
        );
    }
}
