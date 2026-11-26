//! PROBE (heldrecords/probe-minted-margin): the pierce row's set-aside
//! (v, s) meetings read by the census's point-inside-edge door, across
//! poses of the second prism nudged perpendicular to F and the lying
//! cylinder's top moved along E. Run with `PROBE_MARGIN=1` and
//! `--nocapture`; set `CAD_TOLERANCE_EPS` for ε. Measures; asserts nothing.


use geom_core::{Point2, Point3, Tol};
use sweep::test_support::{corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanResult};

const RC: f64 = 2.0;
const C: (f64, f64) = (1.5, 1.0);
const LYING_CAPPED: (f64, f64) = (1.47, 3.0);
const LYING_WHOLE: (f64, f64) = (0.5, 3.0);

fn tol() -> Tol {
    Tol::witness()
}

fn lying(span: (f64, f64), top: f64) -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(1.0, top - RC), RC, tol()).unwrap();
    let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_yz(Point3::new(
        span.0, 0.0, 0.0,
    )));
    let p = profile::Profile::new(plane, vec![lp.into()]).validate(tol()).unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance { depth: span.1 - span.0, side: ExtrudeSide::Along },
        tol(),
    )
    .unwrap()
    .body;
    finished("the cylinder", body, tol())
}

fn bearing(a: f64, r: f64) -> (f64, f64) {
    let (s, c) = a.to_radians().sin_cos();
    (C.0 + r * c, C.1 + r * s)
}

fn prism(foot: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
    finished("a prism", prism_at(corners(foot), z.0, z.1 - z.0, tol()), tol())
}

/// The sector and the triangle, the triangle moved `d` along F's normal
/// (toward the sector for `d > 0`).
fn prisms(d: f64) -> [AtRestBody<f64>; 2] {
    let (s, c) = 80f64.to_radians().sin_cos();
    let n = (-s, c);
    let tri: Vec<_> = [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)]
        .iter()
        .map(|&(x, y)| (x + d * n.0, y + d * n.1))
        .collect();
    // E's actual offset from F's plane, in f64.
    let (a, b) = (tri[0], tri[1]);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt();
    let gap = ((C.0 - a.0) * dy - (C.1 - a.1) * dx) / len;
    eprintln!("PM_POSEGAP d={d:e} E-to-F signed gap={gap:e}");
    [
        prism(&[C, bearing(120.0, 0.4), bearing(170.0, 0.4)], (0.47, 1.7)),
        prism(&tri, (0.44, 1.81)),
    ]
}

const OPS: [(&str, topo::BooleanOp); 3] = [
    ("union", topo::BooleanOp::Union),
    ("intersection", topo::BooleanOp::Intersect),
    ("subtraction", topo::BooleanOp::Subtract),
];

fn short<E: core::fmt::Debug>(e: &E) -> String {
    let s = format!("{e:?}");
    s.chars().take(400).collect()
}

#[test]
fn probe_minted_margin() {
    let eps = tol().eps();
    let tops: Vec<f64> = std::env::var("PM_TOPS")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![1.0, 0.5, 1.65]);
    let fracs: Vec<f64> = std::env::var("PM_FRACS")
        .map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or(vec![0.0, 0.25, -0.25, 0.5, -0.5, 0.9, -0.9]);
    eprintln!("PM_EPS eps={eps:e} k={}", tol().k());
    for &top in &tops {
        for &frac in &fracs {
            let d = frac * eps;
            let bodies = prisms(d);
            for (span_name, span) in [("capped", LYING_CAPPED), ("whole", LYING_WHOLE)] {
                let cyl = lying(span, top);
                for (order, a, b) in [("sector∪triangle", &bodies[0], &bodies[1]), ("triangle∪sector", &bodies[1], &bodies[0])] {
                    let pose = format!("top={top} frac={frac} span={span_name} order={order}");
                    eprintln!("PM_POSE {pose} JOIN");
                    let (pair, records) = match topo::union(a, b, tol()) {
                        Ok(BooleanResult::Body(r)) => (r.body.clone(), r.contacts.carried()),
                        other => {
                            eprintln!("PM_RESULT {pose} join FAILS {}", short(&other));
                            continue;
                        }
                    };
                    for (name, op) in OPS {
                        for held_first in [true, false] {
                            eprintln!("PM_POSE {pose} op={name} held_first={held_first}");
                            let (x, y, ra, rb) = if held_first {
                                (&pair, &cyl, records.clone(), topo::OperandRecords::default())
                            } else {
                                (&cyl, &pair, topo::OperandRecords::default(), records.clone())
                            };
                            let inputs = topo::BooleanInputs { records_a: ra, records_b: rb };
                            let r = topo::boolean_op_with(op, x, y, &inputs, topo::SweepStrategy::Realized, tol());
                            let verdict = match &r {
                                Ok(BooleanResult::Body(r)) => {
                                    let t3 = topo::validate_geometric(&r.body, tol()).is_ok();
                                    format!("BUILDS tier3={t3}")
                                }
                                Ok(other) => format!("OK-nonbody {}", short(other)),
                                Err(e) => format!("REFUSES {}", short(e)),
                            };
                            eprintln!("PM_RESULT {pose} op={name} held_first={held_first} {verdict}");
                        }
                    }
                }
            }
        }
    }
}
