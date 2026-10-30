#![allow(unused_imports, dead_code)]
//! Review probes for PR 6097 (not for merge).






















#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::{Construct, MevSite};
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, ValidationError, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

/// The unit hemisphere `y ≥ 0` on its base disk: two quarter-sphere lunes
/// meeting along two meridians at the pole.
fn hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The cone of radius and height 1 on its base disk, apex at `NORTH`.
fn cone() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// The vertex at `at`.
fn vertex_at(body: &Body<f64>, at: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|&(v, _)| (point(body, v) - at).norm() < 1e-12)
        .map(|(v, _)| v)
        .expect("a vertex there")
}

/// The half-edges leaving `v`, in half-edge-slot order.
fn leaving(body: &Body<f64>, v: VertexKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .filter(|(_, h)| h.start == v)
        .map(|(he, _)| he)
        .collect()
}

/// Whether the joint the mint stored at `he` (into `he` from its loop's
/// predecessor) is a reset.
fn resets(body: &Body<f64>, he: HalfEdgeKey) -> bool {
    body.joint(he).is_some_and(topo::JointElement::is_reset)
}

/// **The two meridians at the pole made one wrap edge**: the second is
/// killed, merging the two lunes into the face it leaves, and the first is
/// restated as that face's wrap edge. Where `join`, the join then takes
/// the rim's two arcs back into one closed edge at the vertex the kill
/// left with nothing else at it; where not, that vertex is left for a
/// spur to stand on. Returns the face, the wrap edge and its tip.
fn one_wrap_edge(mut body: Body<f64>, join: bool) -> (Body<f64>, FaceKey, EdgeKey, VertexKey) {
    let tip = vertex_at(&body, NORTH);
    let out = leaving(&body, tip);
    assert_eq!(out.len(), 2, "two meridians leave the pole");
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let face = body.face_of_half_edge(out[0]).unwrap();
    let surface = body.get_face(face).unwrap().surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the survivor restates as the face's wrap edge");
    if join {
        body.join_edges(Band::linear(tol()).unwrap(), tol())
            .expect("the rim joins");
    }
    let face = body.face_of_half_edge(out[0]).unwrap();
    (body, face, wraps, tip)
}

/// **A meridian spur of the unit sphere** from the equator vertex at
/// `(x, 0, 0)` (`x = ±1`) up to the latitude `top`, spliced into `face`'s
/// loop there and described by `description`. Returns its edge and tip.
fn spur(
    body: &mut Body<f64>,
    face: FaceKey,
    x: f64,
    top: f64,
    description: EdgeDescriptionSpec<f64>,
) -> (EdgeKey, VertexKey) {
    let from = vertex_at(body, Point3::new(x, 0.0, 0.0));
    let he = leaving(body, from)
        .into_iter()
        .find(|&he| body.face_of_half_edge(he) == Some(face))
        .expect("the face's loop leaves that vertex");
    let carrier = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, x),
        radius: 1.0,
        u_ref: Vec3::new(x, 0.0, 0.0),
    };
    let tip = carrier.eval(top);
    let mut spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, top).unwrap();
    spec.description = description;
    let made = body
        .mev(MevSite::Fan { he1: he, he2: he }, tip, spec, tol())
        .expect("the spur splices into the face's loop");
    (made.edge, made.vertex)
}

/// The hemisphere with its one wrap edge killed along with the pole: its
/// curved face is bounded by the equator alone, and holds the pole.
fn equator_only() -> (Body<f64>, FaceKey) {
    let (mut body, face, wraps, tip) = one_wrap_edge(hemisphere(), true);
    let e = body.get_edge(wraps).unwrap();
    let toward = if body.get_half_edge(e.he_plus).unwrap().start == tip {
        e.he_minus
    } else {
        e.he_plus
    };
    body.kev_describing(toward, &[], tol())
        .expect("the wrap edge and its tip die");
    (body, face)
}

/// Every face's reading summed: a third of the flux is the volume.
fn measured(body: &Body<f64>, what: &str) -> (f64, f64) {
    body.faces()
        .map(|(face, _)| {
            let r = topo::test_support::face_reading(body, face, tol(), 1e-6)
                .unwrap_or_else(|e| panic!("{what}: face {face:?} reads: {e}"));
            (r.flux, r.area)
        })
        .fold((0.0, 0.0), |(f, a), (g, b)| (f + g, a + b))
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: {got}, closed form {want}"
    );
}

fn report(what: &str, body: &Body<f64>) {
    eprintln!("PROBE {what}: tier2 = {:?}", topo::validate_closed(body));
    eprintln!("PROBE {what}: tier3 = {:?}", topo::validate_geometric(body, tol()));
}

fn face_with_surface(body: &Body<f64>, v: VertexKey, kind: &str) -> (HalfEdgeKey, FaceKey) {
    for he in leaving(body, v) {
        let f = body.face_of_half_edge(he).unwrap();
        let s = body.get_face(f).unwrap().surface;
        let k = format!("{:?}", body.get_surface(s).unwrap());
        if k.contains(kind) {
            return (he, f);
        }
    }
    panic!("no {kind} face at vertex")
}

/// A non-wrap chain link then a wrap link to the pole: a two-edge spur.
#[test]
fn probe_chain_spur_last_link_wrap() {
    let (mut body, face) = equator_only();
    let surface = body.get_face(face).unwrap().surface;
    let from = vertex_at(&body, Point3::new(1.0, 0.0, 0.0));
    let he = leaving(&body, from)
        .into_iter()
        .find(|&he| body.face_of_half_edge(he) == Some(face))
        .unwrap();
    let axis = Vec3::new(0.0, 0.6, 0.8);
    let a = geom::Curve3::Circle { center: Point3::origin(), axis, radius: 1.0, u_ref: Vec3::new(1.0, 0.0, 0.0) };
    let p = a.eval(1.0);
    let mut spec = EdgeCurveSpec::arc_of_circle(a, 0.0, 1.0).unwrap();
    spec.description = EdgeDescriptionSpec::chart(surface);
    let m = body.mev(MevSite::Fan { he1: he, he2: he }, p, spec, tol()).expect("link A");
    let back = leaving(&body, m.vertex)[0];
    let pv = p - Point3::origin();
    let ax = pv.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
    let b = geom::Curve3::Circle { center: Point3::origin(), axis: ax, radius: 1.0, u_ref: pv };
    let top = pv.y.acos();
    let mut spec = EdgeCurveSpec::arc_of_circle(b.clone(), 0.0, top).unwrap();
    spec.description = EdgeDescriptionSpec::wrap(surface);
    eprintln!("PROBE chain tip at {:?}", b.eval(top));
    body.mev(MevSite::Fan { he1: back, he2: back }, b.eval(top), spec, tol()).expect("link B");
    report("chain", &body);
}

/// The one-face ball's lone wrap meridian split at the equator.
#[test]
fn probe_split_lone_strut() {
    let mut ball = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    let north = vertex_at(&ball, NORTH);
    let out = leaving(&ball, north);
    let wraps = ball.get_half_edge(out[0]).unwrap().edge;
    let face = ball.face_of_half_edge(out[0]).unwrap();
    let surface = ball.get_face(face).unwrap().surface;
    ball.kef_describing(out[1], &[(wraps, EdgeDescriptionSpec::wrap(surface))], tol()).unwrap();
    report("lone", &ball);
    let curve = ball.get_edge(wraps).unwrap().curve;
    eprintln!("PROBE lone edge curve {}", format!("{:?}", ball.get_curve_geom(curve)).chars().take(600).collect::<String>());
    let mut r = Err(None);
    for t in [0.0, PI / 2.0, -PI / 2.0, PI, 1.5 * PI, 0.5, 1.0] {
        let mut b = ball.clone();
        match b.split_edge(wraps, t, tol()) {
            Ok(_) => { eprintln!("PROBE split at {t}"); ball = b; r = Ok(()); break; }
            Err(e) => { eprintln!("PROBE split {t}: {e:?}"); r = Err(Some(())); }
        }
    }
    if r.is_ok() {
        report("split", &ball);
        let (flux, area) = measured(&ball, "split");
        eprintln!("PROBE split ball volume {} (4pi/3 = {}), area {} (4pi = {})", flux / 3.0, 4.0 * PI / 3.0, area, 4.0 * PI);
        let tips: Vec<_> = ball.vertices().map(|(v, _)| (point(&ball, v), leaving(&ball, v).len())).collect();
        eprintln!("PROBE split ball vertices {tips:?}");
    }
}

/// A wrap spur on a cylinder's lateral face, and on a plane cap.
#[test]
fn probe_wrap_spur_cylinder_and_plane() {
    for kind in ["Cylinder", "Plane"] {
        let mut body = revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(1.0, 0.0), 0.0),
                (Point2::new(1.0, 1.0), 0.0),
                (Point2::new(0.0, 1.0), 0.0),
            ],
            Revolution::Full,
            tol(),
        );
        let from = body
            .vertices()
            .map(|(v, _)| v)
            .find(|&v| { let p = point(&body, v); p.y.abs() < 1e-12 && (p.x * p.x + p.z * p.z - 1.0).abs() < 1e-9 })
            .unwrap();
        let fp = point(&body, from);
        let (he, face) = face_with_surface(&body, from, kind);
        let surface = body.get_face(face).unwrap().surface;
        let to = if kind == "Cylinder" { fp + Vec3::new(0.0, 0.5, 0.0) } else { Point3::new(fp.x * 0.5, 0.0, fp.z * 0.5) };
        let mut spec = EdgeCurveSpec::line_between(fp, to);
        spec.description = EdgeDescriptionSpec::wrap(surface);
        let r = body.mev(MevSite::Fan { he1: he, he2: he }, to, spec, tol());
        eprintln!("PROBE {kind} mev {:?}", r.as_ref().map(|_| ()));
        if r.is_ok() {
            report(kind, &body);
        }
    }
}

/// A wrap spur on a sphere face described as a wrap edge of ANOTHER
/// surface (the base plane).
#[test]
fn probe_wrap_flag_on_foreign_surface() {
    let (mut body, face, _, _) = one_wrap_edge(hemisphere(), false);
    let from = vertex_at(&body, Point3::new(-1.0, 0.0, 0.0));
    let (_, plane) = face_with_surface(&body, from, "Plane");
    let psurf = body.get_face(plane).unwrap().surface;
    let he = leaving(&body, from).into_iter().find(|&he| body.face_of_half_edge(he) == Some(face)).unwrap();
    let carrier = geom::Curve3::Circle { center: Point3::origin(), axis: Vec3::new(0.0, 0.0, -1.0), radius: 1.0, u_ref: Vec3::new(-1.0, 0.0, 0.0) };
    let tip = carrier.eval(0.7);
    let mut spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, 0.7).unwrap();
    spec.description = EdgeDescriptionSpec::wrap(psurf);
    let r = body.mev(MevSite::Fan { he1: he, he2: he }, tip, spec, tol());
    eprintln!("PROBE foreign mev {:?}", r.as_ref().map(|_| ()));
    if r.is_ok() {
        report("foreign", &body);
    }
}

/// A finished wrap-tip hemisphere as a boolean operand.
#[test]
fn probe_tip_operand_booleans() {
    let tol = tol();
    let (body, _, _, _) = one_wrap_edge(hemisphere(), true);
    let tip = topo::test_support::finished("tip hemisphere", body, tol);
    let bricks = [
        ("pole inside brick", ((-0.3, 0.3), (0.8, 2.0), (-0.3, 0.3))),
        ("side cut, pole clear", ((0.5, 2.0), (-0.5, 2.0), (-2.0, 2.0))),
        ("brick across the wrap edge", ((-2.0, 2.0), (0.5, 2.0), (-2.0, 0.0))),
    ];
    for (what, (x, y, z)) in bricks {
        let block = topo::test_support::finished("brick", sweep::test_support::brick(x, y, z, tol), tol);
        for (op, f) in [
            ("union", topo::boolean::union::<f64> as fn(&_, &_, Tol) -> _),
            ("subtract", topo::boolean::subtract::<f64>),
            ("intersect", topo::boolean::intersect::<f64>),
        ] {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&tip, &block, tol)));
            let s = match r {
                Err(_) => "PANIC".to_string(),
                Ok(Ok(out)) => match out.body() {
                    Some(b) => format!("Ok: tier3 {:?}", topo::validate_geometric(&b.body, tol).map_err(|e| e.len())),
                    None => "Ok: empty".to_string(),
                },
                Ok(Err(e)) => format!("Err {}", format!("{e:?}").chars().take(220).collect::<String>()),
            };
            eprintln!("PROBE boolean {what} {op}: {s}");
        }
    }
}

#[test]
fn probe_tip_operand_volumes() {
    let tol = tol();
    let (body, _, _, _) = one_wrap_edge(hemisphere(), true);
    let tip = topo::test_support::finished("tip hemisphere", body, tol);
    for (what, (x, y, z)) in [
        ("pole inside brick", ((-0.3, 0.3), (0.8, 2.0), (-0.3, 0.3))),
        ("side cut", ((0.5, 2.0), (-0.5, 2.0), (-2.0, 2.0))),
    ] {
        let block = topo::test_support::finished("brick", sweep::test_support::brick(x, y, z, tol), tol);
        let out = topo::boolean::subtract(&tip, &block, tol).unwrap();
        let b = Body::clone(&out.body().unwrap().body);
        let (flux, _) = measured(&b, what);
        eprintln!("PROBE subtract {what}: volume {}", flux / 3.0);
    }
}

#[test]
fn probe_plain_hemisphere_across() {
    let tol = tol();
    let plain = topo::test_support::finished("plain", hemisphere(), tol);
    let (x, y, z) = ((-2.0, 2.0), (0.5, 2.0), (-2.0, 0.0));
    let block = topo::test_support::finished("brick", sweep::test_support::brick(x, y, z, tol), tol);
    let r = topo::boolean::subtract(&plain, &block, tol);
    eprintln!("PROBE plain across subtract: {}", format!("{:?}", r.as_ref().map(|_| ())).chars().take(200).collect::<String>());
    let (body, _, _, _) = one_wrap_edge(hemisphere(), true);
    let e = body.edges().count();
    eprintln!("PROBE tip hemisphere edges {e}");
}

fn one_face_ball() -> (Body<f64>, EdgeKey) {
    let mut ball = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    let north = vertex_at(&ball, NORTH);
    let out = leaving(&ball, north);
    let wraps = ball.get_half_edge(out[0]).unwrap().edge;
    let face = ball.face_of_half_edge(out[0]).unwrap();
    let surface = ball.get_face(face).unwrap().surface;
    ball.kef_describing(out[1], &[(wraps, EdgeDescriptionSpec::wrap(surface))], tol()).unwrap();
    (ball, wraps)
}

/// Kill the edge of `e` that runs to `tip`, with `tip`.
fn kill_to(body: &mut Body<f64>, e: EdgeKey, tip: VertexKey) {
    let d = body.get_edge(e).unwrap();
    let toward = if body.get_half_edge(d.he_plus).unwrap().start == tip { d.he_minus } else { d.he_plus };
    body.kev_describing(toward, &[], tol()).expect("kev");
}

/// C7: the one-face ball, and a short lone wrap strut cut from it.
#[test]
fn probe_c7_one_face_ball_and_short_lone_strut() {
    let (ball, _) = one_face_ball();
    report("c7 one-face ball", &ball);
    let (flux, area) = measured(&ball, "c7 ball");
    eprintln!("PROBE c7 one-face ball volume {} area {}", flux / 3.0, area);

    // The meridian runs north (t=0) to south (t=pi) about axis -z.
    let (mut b, wraps) = one_face_ball();
    let north = vertex_at(&b, NORTH);
    let south = vertex_at(&b, Point3::new(0.0, -1.0, 0.0));
    let s1 = b.split_edge(wraps, 0.3, tol()).expect("split near north");
    eprintln!("PROBE c7 split1 {s1:?}");
    // find the edges at each pole and kill them with the pole
    let en = b.get_half_edge(leaving(&b, north)[0]).unwrap().edge;
    kill_to(&mut b, en, north);
    let es = b.get_half_edge(leaving(&b, south)[0]).unwrap().edge;
    let s2 = b.split_edge(es, PI - 0.3, tol());
    eprintln!("PROBE c7 split2 {:?}", s2.as_ref().map(|_| ()));
    let es = b.get_half_edge(leaving(&b, south)[0]).unwrap().edge;
    kill_to(&mut b, es, south);
    let ends: Vec<_> = b.vertices().map(|(v, _)| (point(&b, v), leaving(&b, v).len())).collect();
    eprintln!("PROBE c7 short strut vertices {ends:?}, edges {}", b.edges().count());
    report("c7 short lone strut", &b);
}
