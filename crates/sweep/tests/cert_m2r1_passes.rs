//! CERT-M2 R1 probe (both-tree compatible: compiles at the merge base
//! f3c035579 and at the head 8f5384515). Dumps the verdicts of the two
//! passes the PR body claims byte-identical (`validate_pseudomanifold`,
//! `contact_marks`) plus `validate_geometric` at the certifying scalars
//! and `mass_properties` over a corpus of valid and corrupt bodies at
//! f64 / Dual64 / Interval. Run with `--nocapture`, grep `M2R1|`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::{Band, Point2, Point3, Real, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, ContactRecords, PropsQuadLane, SplitPart, SplitPlane, split};

fn v<T: Real>(x: f64, y: f64, b: f64) -> ProfileVertex<T> {
    ProfileVertex::new(Point2::new(T::from_f64(x), T::from_f64(y)), T::from_f64(b))
}

fn profile<T: geom_core::Decide>(lp: ProfileLoop<T>) -> profile::ValidatedProfile<T> {
    Profile::new(SketchPlane::<T>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

/// The generic corpus: name, body. Valid bodies first, then their
/// reverted (NegativeVolume) twins.
pub fn corpus<T: PropsQuadLane>() -> Vec<(String, Body<T>)> {
    let tol = Tol::witness();
    let mut out: Vec<(String, Body<T>)> = Vec::new();
    // L-prism (planar, closed form).
    let l = ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
        v(1.0, 2.0, 0.0),
        v(0.0, 2.0, 0.0),
    ]);
    let l_prism = extrude(&profile(l), Extrusion::Distance(T::from_f64(1.0)), tol)
        .unwrap()
        .body;
    out.push(("l_prism".into(), l_prism));
    // Cylinder (two semicircular arcs), closed form.
    let c = ProfileLoop::new(vec![v(-1.0, 0.0, 1.0), v(1.0, 0.0, 1.0)]);
    let cyl = extrude(&profile(c), Extrusion::Distance(T::from_f64(2.0)), tol)
        .unwrap()
        .body;
    out.push(("cylinder".into(), cyl.clone()));
    // Cylinder cut by an oblique plane: the ellipse-trimmed face needs the
    // quadrature lane (DL3's `cut_cylinder` class).
    let phi = 0.4;
    let plane = SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(0.0), T::from_f64(1.0)),
        normal: Vec3::new(T::from_f64(phi.sin()), T::from_f64(0.0), T::from_f64(phi.cos())),
    };
    let res = split(&cyl, &plane, tol).unwrap();
    if let SplitPart::Body(above) = &res.above {
        out.push(("cut_cylinder_above".into(), above.clone()));
    }
    if let SplitPart::Body(below) = &res.below {
        out.push(("cut_cylinder_below".into(), below.clone()));
    }
    // Washer: rectangle revolved a full turn (torus-free, cylinder walls).
    let w = ProfileLoop::new(vec![
        v(1.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
    ]);
    let axis = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    let washer = revolve(&profile(w), axis, Revolution::Full, tol).unwrap().body;
    out.push(("washer".into(), washer));
    // Quarter washer (partial revolve — wedge caps).
    let w2 = ProfileLoop::new(vec![
        v(1.0, 0.0, 0.0),
        v(2.0, 0.0, 0.0),
        v(2.0, 1.0, 0.0),
        v(1.0, 1.0, 0.0),
    ]);
    let axis2 = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    let quarter = revolve(
        &profile(w2),
        axis2,
        Revolution::Partial(T::from_f64(FRAC_PI_2)),
        tol,
    )
    .unwrap()
    .body;
    out.push(("quarter_washer".into(), quarter));
    // Grooved washer: a semicircular arc in the outer wall (torus face,
    // quadrature at certifying scalars).
    let g = ProfileLoop::new(vec![
        v(1.0, 0.0, 0.0),
        v(3.0, 0.0, 0.0),
        v(3.0, 0.5, -1.0),
        v(3.0, 1.5, 0.0),
        v(3.0, 2.0, 0.0),
        v(1.0, 2.0, 0.0),
    ]);
    let axis3 = RevolveAxis {
        origin: Point2::new(T::from_f64(0.0), T::from_f64(0.0)),
        dir: Vec2::new(T::from_f64(0.0), T::from_f64(1.0)),
    };
    if let Ok(t) = revolve(&profile(g), axis3, Revolution::Full, tol) {
        out.push(("grooved_washer".into(), t.body));
    }
    // Reverted twins.
    let reverted: Vec<(String, Body<T>)> = out
        .iter()
        .filter_map(|(n, b)| b.revert().ok().map(|r| (format!("{n}~reverted"), r)))
        .collect();
    out.extend(reverted);
    let _ = PI;
    out
}

fn dump<T: PropsQuadLane + core::fmt::Debug>(scalar: &str, name: &str, body: &Body<T>) {
    let tol = Tol::witness();
    let _ = Band::linear(tol).unwrap();
    println!(
        "M2R1|{scalar}|{name}|pseudomanifold|{:?}",
        topo::validate_pseudomanifold(body, &ContactRecords::default(), tol)
    );
    let marks = topo::contact_marks(body, tol).map(|m| {
        let mut v: Vec<String> = m.iter().map(|(k, m)| format!("{k:?}={m:?}")).collect();
        v.sort();
        v
    });
    println!("M2R1|{scalar}|{name}|contact_marks|{marks:?}");
    println!(
        "M2R1|{scalar}|{name}|mass_properties|{:?}",
        topo::mass_properties(body, tol)
    );
}

fn dump_composed<T: PropsQuadLane + geom_core::CertifiedBounds + core::fmt::Debug>(
    scalar: &str,
    name: &str,
    body: &Body<T>,
) {
    println!(
        "M2R1|{scalar}|{name}|validate_geometric|{:?}",
        topo::validate_geometric(body, Tol::witness())
    );
}

#[test]
fn m2r1_passes_f64() {
    for (n, b) in corpus::<f64>() {
        dump("f64", &n, &b);
        dump_composed("f64", &n, &b);
    }
    for (n, b) in f64_only_corpus() {
        dump("f64", &n, &b);
        dump_composed("f64", &n, &b);
    }
}

#[test]
fn m2r1_passes_dual64() {
    for (n, b) in corpus::<geom_core::Dual64>() {
        dump("dual64", &n, &b);
    }
}

#[cfg(feature = "interval")]
#[test]
fn m2r1_passes_interval() {
    for (n, b) in corpus::<geom_core::Interval>() {
        dump("interval", &n, &b);
        dump_composed("interval", &n, &b);
    }
}

// ---- f64-only corrupt constructions (check 8 / check 9 failures). ----

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn plane_chart_at_y(body: &Body<f64>, y: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

fn revolved(lp: ProfileLoop<f64>) -> Body<f64> {
    revolve(
        &profile(lp),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A ring standing on its own outer loop (check 9), built the way
/// `verbs_shell.rs::a_ring_standing_on_its_outer_loop_refuses_at_tier_3`
/// builds it; plus its reverted twin so check 7 WOULD also fire.
pub fn f64_only_corpus() -> Vec<(String, Body<f64>)> {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut out = Vec::new();
    let vessel = revolved(ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(0.5, 0.0, 0.0),
        v(0.5, 0.4, 0.0),
        v(0.0, 0.4, 0.0),
    ]));
    let tube = revolved(ProfileLoop::new(vec![
        v(0.30, 0.0, 0.0),
        v(0.50, 0.0, 0.0),
        v(0.50, 0.40, 0.0),
        v(0.30, 0.40, 0.0),
    ]));
    let t = 0.05;
    for (what, body, y) in [("ring_on_outer_vessel", vessel, 0.4), ("ring_on_outer_tube", tube, 0.40)] {
        let mut sealed = topo::shell(&body, t, 1e-6, tol).expect("sealed shell");
        let mouth = plane_chart_at_y(&sealed, y);
        let counterpart = plane_chart_at_y(&sealed, y - t);
        let plane_of = |b: &Body<f64>, f: topo::FaceKey| match b.get_surface(b.get_face(f).unwrap().surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => (*origin, *normal),
            other => panic!("non-planar cap {other:?}"),
        };
        let (o_from, n_from) = plane_of(&sealed, counterpart[0]);
        let (o_onto, _) = plane_of(&sealed, mouth[0]);
        let back = (o_onto - o_from).dot(n_from);
        topo::replace_faces_offset(&mut sealed, &counterpart, back, 1e-6, band, tol).unwrap();
        for (&rim, &source) in mouth.iter().zip(&counterpart) {
            sealed.kfmrh(rim, source).unwrap();
        }
        if let Ok(r) = sealed.revert() {
            out.push((format!("{what}~reverted"), r));
        }
        out.push((what.to_string(), sealed));
    }
    // Diagonal chord split of a quarter washer wall (check 2 + check 8).
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let tq = revolve(
        &profile(lp),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(FRAC_PI_2),
        tol,
    )
    .unwrap();
    let mut body = tq.body;
    let wall = tq.walls[0][1].expect("outer wall");
    let outer = body.get_face(wall).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("cycle");
    };
    let cycle = body.loop_cycle(first).unwrap();
    let (he1, he2) = (cycle[0], cycle[2]);
    let point_of = |body: &Body<f64>, he| {
        let vv = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(vv).unwrap().point).unwrap()
    };
    let (a, b) = (point_of(&body, he1), point_of(&body, he2));
    body.mef(
        topo::MefSite::Chords { he1, he2 },
        topo::EdgeCurveSpec::line_between(a, b),
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    if let Ok(r) = body.revert() {
        out.push(("chord_split~reverted".into(), r));
    }
    out.push(("chord_split".into(), body));
    out
}
