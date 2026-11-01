//! Review probes for `record_spans` and the carry of an edge-edge record
//! onto the reduction's split pieces (`ops::carried`): poses that stress
//! a span's bounds, printed one line per run so base and head diff. Run
//! with `--ignored --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanInputs, BooleanOp, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn zprism(pts: &[(f64, f64, f64)], z: (f64, f64)) -> topo::Body<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z.0));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    extrude(
        &p,
        Extrusion::Distance {
            depth: z.1 - z.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

fn minus(a: topo::Body<f64>, b: topo::Body<f64>) -> topo::Body<f64> {
    let a = finished("a", a, tol());
    let b = finished("b", b, tol());
    match topo::subtract(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => topo::Body::clone(&r.body),
        o => panic!("{o:?}"),
    }
}

fn counts(c: &topo::ContactRecords) -> [usize; 6] {
    [
        c.vertex_vertex().count(),
        c.vertex_face().count(),
        c.vertex_edge().count(),
        c.edge_edge().count(),
        c.edge_face().count(),
        c.face_face().count(),
    ]
}

fn line(
    r: Result<BooleanResult<f64>, topo::BooleanError>,
) -> (String, Option<topo::BooleanBody<f64>>) {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = topo::validate_pseudomanifold(&b.body, &b.contacts, tol());
            let v = match v {
                Ok(()) => "t3p=ok".to_string(),
                Err(es) => format!("t3p=ERR {es:?}"),
            };
            (format!("records={:?} {v}", counts(&b.contacts)), Some(b))
        }
        Ok(BooleanResult::Empty) => ("EMPTY".into(), None),
        Err(e) => (format!("REFUSED {e:?}"), None),
    }
}

fn poses() -> Vec<(&'static str, topo::Body<f64>)> {
    let wide = || brick((-1.0, 1.0), (0.5, 1.5), (-0.5, 2.5), tol());
    vec![
        (
            "end_in_face",
            brick((-1.0, 1.0), (0.5, 1.5), (-0.5, 1.0), tol()),
        ),
        ("whole_in_face", wide()),
        (
            "caps_cross",
            brick((-1.0, 1.0), (0.5, 1.5), (0.5, 1.5), tol()),
        ),
        (
            "hole",
            minus(wide(), brick((-0.3, 0.2), (0.0, 2.0), (0.7, 1.1), tol())),
        ),
        (
            "two_holes",
            minus(
                minus(wide(), brick((-0.3, 0.2), (0.0, 2.0), (0.2, 0.6), tol())),
                brick((-0.2, 0.3), (0.0, 2.0), (1.0, 1.7), tol()),
            ),
        ),
        (
            "notch_on_apex",
            minus(wide(), brick((-2.0, 0.0), (0.0, 2.0), (0.9, 1.1), tol())),
        ),
        (
            "corner_inside",
            brick((0.0, 1.0), (0.5, 1.5), (0.5, 1.5), tol()),
        ),
        (
            "corner_overhang",
            brick((0.0, 1.0), (0.5, 1.5), (1.0, 3.0), tol()),
        ),
        (
            "corner_whole",
            brick((0.0, 1.0), (0.5, 1.5), (-1.0, 3.0), tol()),
        ),
        (
            "corner_end_at_end",
            brick((0.0, 1.0), (0.5, 1.5), (0.0, 1.0), tol()),
        ),
    ]
}

fn rot(angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(0.3, -0.2, 0.1),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        angle,
    )
}

fn turned(b: &topo::Body<f64>, angle: f64) -> AtRestBody<f64> {
    let b = if angle == 0.0 {
        b.clone()
    } else {
        topo::transform_rigid(
            b,
            &geom_core::test_support::rigid(&rot(angle), tol()),
            tol(),
        )
        .unwrap()
    };
    finished("op", b, tol())
}

#[test]
#[ignore = "review probe battery"]
fn review_4737_span_battery() {
    let tri = [(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)];
    let apex = zprism(&tri, (0.0, 2.0));
    for angle in [0.0, 0.3, 1.1] {
        let a = turned(&apex, angle);
        for (name, b) in poses() {
            let b = turned(&b, angle);
            for (op, r) in [
                ("U", std::panic::catch_unwind(|| topo::union(&a, &b, tol()))),
                (
                    "S",
                    std::panic::catch_unwind(|| topo::subtract(&a, &b, tol())),
                ),
                (
                    "R",
                    std::panic::catch_unwind(|| topo::subtract(&b, &a, tol())),
                ),
                (
                    "I",
                    std::panic::catch_unwind(|| topo::intersect(&a, &b, tol())),
                ),
            ] {
                let s = match r {
                    Ok(r) => line(r).0,
                    Err(_) => "PANIC".into(),
                };
                println!("SPAN ang={angle} {name} {op} => {s}");
            }
            // A carried record cut by a plate across the contact.
            let Ok(Ok(BooleanResult::Body(x))) =
                std::panic::catch_unwind(|| topo::union(&a, &b, tol()))
            else {
                continue;
            };
            let plate = turned(&brick((-2.0, 2.0), (-2.0, 3.0), (0.8, 1.123), tol()), angle);
            for (op, bop) in [
                ("I", BooleanOp::Intersect),
                ("S", BooleanOp::Subtract),
                ("U", BooleanOp::Union),
            ] {
                let inputs = BooleanInputs {
                    records_b: x.contacts.carried(),
                    ..BooleanInputs::default()
                };
                let r = std::panic::catch_unwind(|| {
                    topo::boolean_op_with(
                        bop,
                        &plate,
                        &x.body,
                        &inputs,
                        topo::SweepStrategy::Realized,
                        tol(),
                    )
                });
                let s = match r {
                    Ok(r) => line(r).0,
                    Err(_) => "PANIC".into(),
                };
                println!("CARRY ang={angle} {name} plate {op} x => {s}");
            }
        }
    }
}
