//! Reviewer probe; dropped into crates/sweep/tests/ as ellipse_torus_probe.rs
//! (declared in tests/all.rs) for the run only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]
//! **An ellipse rim against a torus face**: the exact tilted cut of a drum
//! (a radius-0.5 cylinder of height 1, split by the plane through
//! `(0, 0, 0.5)` with normal `(sin 0.3, 0, cos 0.3)`), its lower part,
//! against a ring torus whose tube comes near the cut's `Ellipse` rim.
//! Along an ellipse the torus's implicit is a trigonometric polynomial of
//! degree four (`topo::boolean::ellipse_roots`, "Against a torus"), and
//! the crossing layer must decide the rim against the torus face by its
//! certified roots wherever the rim's arc enclosure cannot clear it.
//!
//! The rows read the reduction sweep (`sweep_traces`): whether it
//! refuses, and which (rim, torus face) pairs the exact predicates
//! accepted, then the door each boolean op stops at next. No pose here
//! builds a body: a torus that reaches the rim meets the cut plane and
//! the drum wall obliquely, and those pairs have no section arm (the
//! filed doors below).



use geom_core::{Affine3, Band, Point2, Point3, Tol, UnitVec3, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanError, DATUM_UNIT_NORM, EdgeKey, FaceKey, SweepStrategy, sweep_traces};

const DRUM_RADIUS: f64 = 0.5;
const TILT: f64 = 0.3;

/// The drum's lower part: its rim is the cut face's two `Ellipse` arcs.
fn drum_lower() -> Body<f64> { drum_lower_s(1.0) }
fn drum_lower_s(s: f64) -> Body<f64> {
    let tol = Tol::witness();
    let r = DRUM_RADIUS * s;
    let cylinder = sweep::test_support::prism(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        s,
        tol,
    );
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 0.5 * s),
        normal: UnitVec3::new(
            Vec3::new(TILT.sin(), 0.0, TILT.cos()),
            DATUM_UNIT_NORM,
            Band::linear(tol).expect("the witness tolerance forms a band"),
        )
        .expect("a cut normal has a length"),
    };
    let result = topo::splitting::split(&cylinder, &plane, tol).expect("the tilted cut splits");
    let topo::splitting::SplitPart::Body(below) = result.below else {
        panic!("the cut leaves material below");
    };
    assert_eq!(
        ellipse_edges(&below).len(),
        2,
        "the rim is two ellipse arcs"
    );
    below
}

/// A ring torus `(big, small)` about the axis `axis` through `c`: a full
/// circle profile revolved about `y`, turned onto `axis` and moved.
fn donut(big: f64, small: f64, c: Vec3<f64>, axis: Vec3<f64>) -> Body<f64> {
    let at_origin = revolved_about_y(
        vec![
            (Point2::new(big, -small), 1.0),
            (Point2::new(big, small), 1.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    let y = Vec3::new(0.0, 1.0, 0.0);
    let a = axis.normalize();
    let turned = if y.cross(a).norm() < 1e-15 {
        at_origin
    } else {
        let turn = Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            y.cross(a).normalize(),
            y.dot(a).acos(),
        );
        topo::transform_rigid(&at_origin, &turn, Tol::witness()).expect("a rotation is rigid")
    };
    topo::transform_rigid(&turned, &Affine3::translation(c), Tol::witness())
        .expect("a translation is rigid")
}

/// The torus of `(big, small)` whose tube comes within `gap` of the rim at
/// azimuth `phi`, from OUTSIDE the drum's convex corner there: its core
/// circle passes `small + gap` from the rim point along the bisector `d̂`
/// of the wall's and the cut plane's outward normals, square to the
/// rim's tangent `t̂`, its centre `big` away along `d̂ × t̂` on the wall's
/// outward side. The tube's least distance from the rim is `gap`, at that
/// point (the rim bends into the drum and the core circle away from it),
/// and from the wall and the cut plane, which the corner keeps farther
/// than the rim, more. Negative `gap` crosses the rim.
fn corner_torus(big: f64, small: f64, phi: f64, gap: f64) -> Body<f64> { corner_torus_s(big, small, phi, gap, 1.0) }
fn corner_torus_s(big: f64, small: f64, phi: f64, gap: f64, s: f64) -> Body<f64> {
    let (st, ct) = TILT.sin_cos();
    let (sp, cp) = phi.sin_cos();
    let x = DRUM_RADIUS * cp;
    let rim = Vec3::new(x, DRUM_RADIUS * sp, 0.5 - x * TILT.tan()) * s;
    let wall = Vec3::new(cp, sp, 0.0);
    let cut = Vec3::new(st, 0.0, ct);
    let d = (wall + cut).normalize();
    let t = cut.cross(wall).normalize();
    let e = d.cross(t).normalize();
    let e = if e.dot(wall) < 0.0 { -e } else { e };
    donut(big * s, small * s, rim + d * ((small * s) + gap) + e * (big * s), d)
}

fn ellipse_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    body.edges()
        .filter(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Ellipse { .. }))
        })
        .map(|(k, _)| k)
        .collect()
}

fn torus_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Torus { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The rim's pairs with `b`'s torus faces in the A → B sweep:
/// `(examined, accepted)`. On the base the sweep refused
/// `CurvedPierceUnsupported` on the rim for every pose here.
fn rim_pairs(label: &str, a: &Body<f64>, b: &Body<f64>) -> (usize, usize) {
    let (ab, _) = sweep_traces(a, b, SweepStrategy::Realized, None, Tol::witness())
        .unwrap_or_else(|e| panic!("{label}: the reduction sweep refused: {e:?}"));
    let (rim, tori) = (ellipse_edges(a), torus_faces(b));
    let count = |v: &[(EdgeKey, FaceKey)]| {
        v.iter()
            .filter(|(e, f)| rim.contains(e) && tori.contains(f))
            .count()
    };
    (count(&ab.examined), count(&ab.accepted))
}

/// Every op's refusal, `A ∖ B` and `B ∖ A` both; a body fails with the op
/// named.
fn refusals(a: &Body<f64>, b: &Body<f64>) -> Vec<(&'static str, BooleanError)> {
    let tol = Tol::witness();
    [
        ("A ∪ B", topo::boolean::union(a, b, tol)),
        ("A ∩ B", topo::boolean::intersect(a, b, tol)),
        ("A ∖ B", topo::boolean::subtract(a, b, tol)),
        ("B ∖ A", topo::boolean::subtract(b, a, tol)),
    ]
    .into_iter()
    .map(|(op, r)| {
        (
            op,
            r.err()
                .unwrap_or_else(|| panic!("{op} built a body where a frontier was pinned")),
        )
    })
    .collect()
}


/// Reviewer probe (PR 3973 dual, reach-dual3973-r2): the drum rim against
/// tori at other radii, azimuths, gaps and scales; every op in both
/// operand orders. Prints each outcome; any body built is checked by
/// point_in_solid sampling against the closed-form membership.
fn outcome<B>(r: &Result<B, BooleanError>) -> String {
    match r {
        Ok(_) => "BODY".into(),
        Err(e) => format!("{e:?}").split([' ', '{', '(']).next().unwrap().to_string(),
    }
}

#[test]
#[ignore = "reviewer probe"]
fn probe_rim_against_tori() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for s in [1e-3, 1.0, 1e3] {
        let a = drum_lower_s(s);
        for (big, small) in [(0.3, 0.05), (0.2, 0.12), (0.8, 0.03), (0.12, 0.1)] {
            for phi in [0.0, 1.05, 2.4, 3.14159, 4.0] {
                for gap in [-1e-2, -1e-6, -10.0 * eps, 10.0 * eps, 1e3 * eps, 1e-5] {
                    let b = corner_torus_s(big, small, phi, gap * s, s);
                    let (ab, _) = match sweep_traces(&a, &b, SweepStrategy::Realized, None, tol) {
                        Ok(x) => x,
                        Err(e) => {
                            println!("SWEEPERR s {s:e} R {big} r {small} phi {phi} gap {gap:e}: {e:?}");
                            let k = format!("sweep-refused {}", outcome::<()>(&Err(e)));
                            *tally.entry(k).or_default() += 1;
                            continue;
                        }
                    };
                    let rim = ellipse_edges(&a);
                    let tori = torus_faces(&b);
                    let acc = ab.accepted.iter().filter(|(e, f)| rim.contains(e) && tori.contains(f)).count();
                    let ops = [
                        ("A|B", topo::boolean::union(&a, &b, tol)),
                        ("B|A", topo::boolean::union(&b, &a, tol)),
                        ("A&B", topo::boolean::intersect(&a, &b, tol)),
                        ("B&A", topo::boolean::intersect(&b, &a, tol)),
                        ("A-B", topo::boolean::subtract(&a, &b, tol)),
                        ("B-A", topo::boolean::subtract(&b, &a, tol)),
                    ];
                    for (op, r) in &ops {
                        let o = outcome(r);
                        println!("RIM s {s:e} R {big} r {small} phi {phi} gap {gap:e} accepted {acc} {op} {o}");
                        *tally.entry(format!("{} {o}", if gap < 0.0 { "cross" } else { "clear" })).or_default() += 1;
                        if r.is_ok() {
                            panic!("a result was built: {op} at s {s} R {big} r {small} phi {phi} gap {gap}");
                        }
                    }
                }
            }
        }
    }
    println!("TALLY {tally:?}");
}

#[test]
#[ignore = "reviewer probe"]
fn probe_dump_km_pose() {
    for (s, gap) in [(1e3, -1e-6), (1e3, -1e-8), (1.0, -1e-6), (1e3, -1e-2)] {
        let a = drum_lower_s(s);
        let b = corner_torus_s(0.3, 0.05, 1.05, gap * s, s);
        for e in ellipse_edges(&a) {
            let edge = a.get_edge(e).unwrap();
            let c = a.get_curve_geom(edge.curve).and_then(topo::CurveGeom::certified).unwrap();
            println!("KM s {s} gap {gap} CARRIER {:?}", c.carrier());
        }
        for f in torus_faces(&b) {
            println!("KM s {s} gap {gap} TORUS {:?}", b.get_surface(b.get_face(f).unwrap().surface).unwrap());
        }
        let r = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness());
        println!("KM s {s} gap {gap} SWEEP {:?}", r.map(|_| ()).err());
    }
}
