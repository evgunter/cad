//! Review probes for PR 4765 (a boolean pinch records the vertex
//! identity that made it). Printing probes: `--nocapture`.
#![allow(
    clippy::panic,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::print_stdout
)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, BooleanBody, BooleanResult, DecisionSite};

fn tol() -> Tol {
    Tol::witness()
}

type Op =
    fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, topo::BooleanError>;

fn ops() -> [(&'static str, Op); 3] {
    [
        ("intersect", topo::intersect::<f64>),
        ("union", topo::union::<f64>),
        ("subtract", topo::subtract::<f64>),
    ]
}

/// (groups of ≥2 vertices on one exact point that share ONE point key,
/// groups on one point over distinct keys, contact rows, VertexFusion
/// rows, tier 3′ verdict).
fn shape(r: &BooleanBody<f64>) -> (usize, usize, usize, usize, bool) {
    let mut by_point: std::collections::BTreeMap<[u64; 3], Vec<_>> = Default::default();
    for (_, v) in r.body.vertices() {
        let p = r
            .body
            .vertex_points()
            .find(|_| true)
            .map(|_| ())
            .map(|()| ());
        let _ = p;
        let _ = v;
    }
    for (k, p) in r.body.vertex_points() {
        let pk = r.body.get_vertex(k).unwrap().point;
        by_point
            .entry([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .or_default()
            .push(pk);
    }
    let mut same = 0;
    let mut distinct = 0;
    for keys in by_point.values().filter(|k| k.len() >= 2) {
        let mut k = keys.clone();
        k.sort();
        k.dedup();
        if k.len() < keys.len() {
            same += 1;
        } else {
            distinct += 1;
        }
    }
    let vf = r
        .coincidences
        .iter()
        .filter(|c| c.site == DecisionSite::VertexFusion)
        .count();
    let t3 = topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).is_ok();
    (same, distinct, r.contacts.rows().len(), vf, t3)
}

fn probe(
    name: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
) -> Vec<(String, Option<BooleanBody<f64>>)> {
    let mut out = Vec::new();
    for (order, p, q) in [("xy", x, y), ("yx", y, x)] {
        for (op, run) in ops() {
            let what = format!("{name} {order} {op}");
            match run(p, q, tol()) {
                Ok(BooleanResult::Body(r)) => {
                    let s = shape(&r);
                    let at_p: Vec<_> = r
                        .body
                        .vertex_points()
                        .filter(|(_, p)| (p.x, p.y, p.z) == (1.0, 1.0, 1.0))
                        .map(|(k, _)| (k, r.body.get_vertex(k).unwrap().point))
                        .collect();
                    println!(
                        "PROBE {what}: same-key groups {} distinct-key groups {} contacts {} vf {} t3 {} kind {:?} at P {:?}",
                        s.0, s.1, s.2, s.3, s.4, r.kind, at_p
                    );
                    for c in &r.coincidences {
                        println!("    row {:?} {:?} {:?}", c.site, c.relation, c.cells);
                    }
                    out.push((what, Some(r)));
                }
                other => {
                    println!("PROBE {what}: {other:?}");
                    out.push((what, None));
                }
            }
        }
    }
    out
}

const PINCH_AT: [f64; 3] = [1.0, 1.0, 1.0];

fn unit(v: [f64; 3]) -> [f64; 3] {
    let n = v.iter().map(|c| c * c).sum::<f64>().sqrt();
    v.map(|c| c / n)
}

/// A side-`s` cube on `m`'s side of the plane through `at` with normal `m`.
fn cube_beyond_at(at: [f64; 3], m: [f64; 3], s: f64) -> AtRestBody<f64> {
    let m = unit(m);
    let u = if m[0].abs() + m[1].abs() > 1e-9 {
        unit([m[1], -m[0], 0.0])
    } else {
        [1.0, 0.0, 0.0]
    };
    let w = [
        m[1] * u[2] - m[2] * u[1],
        m[2] * u[0] - m[0] * u[2],
        m[0] * u[1] - m[1] * u[0],
    ];
    let v = at;
    let body = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (s * (x - 0.5), s * (y - 0.5), s * z);
            geom_core::Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    );
    finished("cube", body, tol())
}

fn ell() -> (AtRestBody<f64>, topo::VertexKey) {
    let prism = common::prism::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        1.0,
        tol(),
    );
    let corner = prism.top[3];
    (finished("ell", prism.body, tol()), corner)
}

const POSES: [[f64; 3]; 8] = [
    [1.0, 1.3, 0.7],
    [1.0, 1.3, 0.4],
    [1.2, 1.0, 0.3],
    [2.0, 0.4, 1.0],
    [1.0, 0.2, 0.5],
    [-1.0, -0.9, -1.3],
    [-1.0, -0.6, -1.2],
    [-0.8, -0.7, -1.0],
];

/// Claim 1/2 sweep over the PR's poses: every same-key result carries
/// ≥1 VertexFusion row; print the rest.
#[test]
fn probe_poses() {
    let (ell, _) = ell();
    let mut silent = Vec::new();
    for m in POSES {
        let cube = cube_beyond_at(PINCH_AT, m, 4.0);
        for (what, r) in probe(&format!("pose {m:?}"), &ell, &cube) {
            if let Some(r) = r {
                let s = shape(&r);
                if s.0 > 0 && s.3 == 0 {
                    silent.push(what);
                }
            }
        }
    }
    println!("SILENT {silent:?}");
}

/// Edge × edge point touch: a cube rolled 45° about x (top ridge along
/// x through (0, 0, 1)) under a cube rolled 45° about y (bottom ridge
/// along y through (0, 0, 1)).
#[test]
fn probe_ridge_cross() {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    // unit cube centred at origin, rotated 45° about x: corner offsets
    // (y, z) -> ((y - z) h, (y + z) h); top ridge at z = h.
    let low = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (x, y, z) = (2.0 * x - 1.0, y - 0.5, z - 0.5);
            geom_core::Point3::new(x, (y - z) * h, (y + z) * h + 1.0 - h)
        },
        tol(),
    );
    let high = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (x, y, z) = (x - 0.5, 2.0 * y - 1.0, z - 0.5);
            geom_core::Point3::new((x - z) * h, y, (x + z) * h + 1.0 + h)
        },
        tol(),
    );
    let low = finished("low", low, tol());
    let high = finished("high", high, tol());
    probe("ridge cross", &low, &high);
}

/// Corner kiss: two cubes sharing only the point (1, 1, 1).
#[test]
fn probe_corner_kiss() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b = finished("b", brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol()), tol());
    probe("corner kiss", &a, &b);
    // Edge kiss crossing: an x-bar on top of a y-bar.
    let c = finished("c", brick((0.0, 3.0), (1.0, 2.0), (0.0, 1.0), tol()), tol());
    let d = finished("d", brick((1.0, 2.0), (0.0, 3.0), (1.0, 2.0), tol()), tol());
    probe("bar on bar (area)", &c, &d);
}

/// A tilted cube's corner on the middle of a brick's top face.
#[test]
fn probe_vertex_on_face() {
    let a = finished("a", brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0), tol()), tol());
    // cube_beyond_at with m vertical puts a FACE on the plane; use a
    // diagonal so only the corner (the cube's (0,0,0) mapped) touches.
    let d = unit([1.0, 1.0, 1.0]);
    let up = [0.0, 0.0, 1.0];
    // rotate so d -> up: Rodrigues.
    let k = unit([
        d[1] * up[2] - d[2] * up[1],
        d[2] * up[0] - d[0] * up[2],
        d[0] * up[1] - d[1] * up[0],
    ]);
    let cos = d[2];
    let sin = (1.0 - cos * cos).sqrt();
    let rot = move |p: [f64; 3]| {
        let kxp = [
            k[1] * p[2] - k[2] * p[1],
            k[2] * p[0] - k[0] * p[2],
            k[0] * p[1] - k[1] * p[0],
        ];
        let kdp = k[0] * p[0] + k[1] * p[1] + k[2] * p[2];
        [0, 1, 2].map(|i| p[i] * cos + kxp[i] * sin + k[i] * kdp * (1.0 - cos))
    };
    let at = [1.0, 1.0, 1.0];
    let b = common::mapped_cube::<f64>(
        move |x, y, z| {
            let q = rot([x, y, z]);
            geom_core::Point3::new(at[0] + q[0], at[1] + q[1], at[2] + q[2])
        },
        tol(),
    );
    let b = finished("tilted", b, tol());
    probe("vertex on face", &a, &b);
}

/// Claim 2 / skip rule: operand already pinched (the TWO_RUNS
/// intersection), then cut again through the pinch point.
#[test]
fn probe_inherited_and_mixed() {
    let (ell, _) = ell();
    let cube = cube_beyond_at(PINCH_AT, POSES[0], 4.0);
    let pinched = match topo::intersect(&ell, &cube, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("{other:?}"),
    };
    println!(
        "BASE {:?} at P {:?}",
        shape(&pinched),
        pinched
            .body
            .vertex_points()
            .filter(|(_, p)| (p.x, p.y, p.z) == (1.0, 1.0, 1.0))
            .map(|(k, _)| (k, pinched.body.get_vertex(k).unwrap().point))
            .collect::<Vec<_>>()
    );
    let base = pinched.body;
    // a far disjoint brick
    let far = finished(
        "far",
        brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol()),
        tol(),
    );
    probe("pinched vs far", &base, &far);
    // a big brick containing it
    let big = finished(
        "big",
        brick((-5.0, 6.0), (-5.0, 6.0), (-5.0, 6.0), tol()),
        tol(),
    );
    probe("pinched vs big", &base, &big);
    // a half-space box through the pinch point at various normals
    for m in [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, -1.0, 0.0],
        [0.3, -1.0, 0.5],
        [-0.3, 1.0, -0.5],
        [1.0, -0.2, -0.4],
        [-1.0, 0.2, 0.4],
        [1.0, 1.3, 0.7],
        [-1.0, -1.3, -0.7],
    ] {
        let h = cube_beyond_at(PINCH_AT, m, 8.0);
        probe(&format!("pinched vs halfspace {m:?}"), &base, &h);
    }
}

/// Claim 3, the census at rest: two blocks kissing at one corner,
/// grafted as two solids of one body, each its own group.
#[test]
fn probe_census_corner_kiss() {
    let block = |(x0, x1): (f64, f64), (y0, y1): (f64, f64), (z0, z1): (f64, f64)| {
        common::prism_z::<f64>(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)], z0, z1, tol()).body
    };
    for (name, b) in [
        ("corner", block((1.0, 2.0), (1.0, 2.0), (1.0, 2.0))),
        ("edge cross", block((0.25, 0.75), (1.0, 2.0), (1.0, 2.0))),
    ] {
        let mut body = block((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
        topo::graft_disjoint_all_keyed(&mut body, &b).unwrap();
        let census = |records: &topo::ContactRecords| {
            topo::validate_pseudomanifold(&body, records, tol())
                .err()
                .unwrap_or_default()
        };
        let bare = census(&topo::ContactRecords::default());
        let groups: std::collections::BTreeMap<_, _> =
            body.solids().map(|(s, _)| s).zip(0u32..).collect();
        let rec = topo::record_at_rest(
            &body,
            &topo::ContactRecords::default(),
            census,
            &|s| groups.get(&s).copied(),
            0,
        );
        println!(
            "CENSUS {name}: bare findings {} rows {:?} records {} findings {:?} confirm {:?}",
            bare.len(),
            rec.rows
                .iter()
                .map(|r| (r.site, r.cells))
                .collect::<Vec<_>>(),
            rec.records.rows().len(),
            rec.findings,
            topo::validate_pseudomanifold(&body, &rec.records, tol())
        );
        let one = topo::record_at_rest(
            &body,
            &topo::ContactRecords::default(),
            census,
            &|_| Some(0),
            0,
        );
        println!(
            "CENSUS {name} one group: rows {} findings {}",
            one.rows.len(),
            one.findings.len()
        );
    }
}

/// Claim 1, skip rule: two operands each ALREADY pinched at P (the
/// TWO_RUNS intersection and a rotated copy about an axis through P).
/// Whatever the op makes at P, every vertex there descends from an
/// operand vertex that already shared a key, so `pinch_rows` emits
/// nothing — even where the op decided A's copies onto B's.
#[test]
fn probe_two_pinched_operands() {
    let (ell, _) = ell();
    let cube = cube_beyond_at(PINCH_AT, POSES[0], 4.0);
    let base = match topo::intersect(&ell, &cube, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{other:?}"),
    };
    let p = geom_core::Point3::new(1.0, 1.0, 1.0);
    for (axis, angle) in [
        ([0.0, 0.0, 1.0], std::f64::consts::PI),
        ([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2),
        ([0.0, 0.0, 1.0], -std::f64::consts::FRAC_PI_2),
        ([1.0, 0.0, 0.0], std::f64::consts::PI),
        ([0.0, 1.0, 0.0], std::f64::consts::PI),
        ([1.0, 1.0, 0.0], std::f64::consts::PI),
        ([1.0, -1.0, 0.0], std::f64::consts::PI),
        ([1.0, 1.0, 1.0], 2.0 * std::f64::consts::FRAC_PI_3),
    ] {
        let a = unit(axis);
        let map = geom_core::Affine3::rotation_about_axis(
            p,
            geom_core::Vec3::new(a[0], a[1], a[2]),
            angle,
        );
        let moved = match topo::transform_rigid(&base, &map, tol()) {
            Ok(b) => finished("rotated", b, tol()),
            Err(e) => {
                println!("ROT {axis:?} {angle}: transform {e:?}");
                continue;
            }
        };
        let at_p = moved
            .vertex_points()
            .filter(|(_, q)| (q.x, q.y, q.z) == (1.0, 1.0, 1.0))
            .count();
        println!("ROT {axis:?} {angle}: rotated copy has {at_p} vertices at P");
        probe(&format!("ROT {axis:?} {angle:.3}"), &base, &moved);
    }
}

/// The skip rule with two operands each already pinched at P EXACTLY:
/// the TWO_RUNS intersection, and the same built from the L turned 180°
/// about the vertical through P (its reflex corner still (1, 1, 1)).
#[test]
fn probe_two_exactly_pinched_operands() {
    let (ell, _) = ell();
    let base = match topo::intersect(&ell, &cube_beyond_at(PINCH_AT, POSES[0], 4.0), tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{other:?}"),
    };
    let turned = common::prism::<f64>(
        &[
            (2.0, 2.0),
            (0.0, 2.0),
            (0.0, 1.0),
            (1.0, 1.0),
            (1.0, 0.0),
            (2.0, 0.0),
        ],
        1.0,
        tol(),
    );
    let turned = finished("turned ell", turned.body, tol());
    for m in POSES
        .iter()
        .map(|m| [-m[0], -m[1], m[2]])
        .chain(POSES.iter().copied())
    {
        let other = match topo::intersect(&turned, &cube_beyond_at(PINCH_AT, m, 4.0), tol()) {
            Ok(BooleanResult::Body(r)) => r,
            other => {
                println!("EXACT {m:?}: other operand {other:?}");
                continue;
            }
        };
        let s = shape(&other);
        println!(
            "EXACT {m:?}: other operand same-key groups {} vf {}",
            s.0, s.3
        );
        probe(&format!("EXACT {m:?}"), &base, &other.body);
    }
}
