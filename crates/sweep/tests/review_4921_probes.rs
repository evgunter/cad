//! Review probes for PR 4921 (the reduction's face–face row). Each case
//! runs every op in both operand orders against an independent oracle
//! (closed-form volume, solid count, point membership from signed
//! distances) and prints one line per op; `#[ignore]`d, run by hand
//! with `--nocapture` at each `CAD_TOLERANCE_EPS`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Point3, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::{AtRestBody, Body, BooleanResult, SolidContainment};

fn tol() -> Tol {
    Tol::witness()
}

#[derive(Clone)]
enum Shape {
    Box([f64; 6]),
    /// centre x, y, radius, z0, height, vertex phase
    Cyl([f64; 6]),
    /// a box with a rectangular through-slot in z: box, slot (x0,x1,y0,y1)
    Slotted([f64; 6], [f64; 4]),
    /// disjoint parts, built by unions
    Parts(Vec<Shape>),
}

impl Shape {
    fn scaled(&self, s: f64) -> Shape {
        match self {
            Shape::Box(b) => Shape::Box(b.map(|v| v * s)),
            Shape::Cyl(c) => Shape::Cyl([c[0] * s, c[1] * s, c[2] * s, c[3] * s, c[4] * s, c[5]]),
            Shape::Slotted(b, h) => Shape::Slotted(b.map(|v| v * s), h.map(|v| v * s)),
            Shape::Parts(p) => Shape::Parts(p.iter().map(|x| x.scaled(s)).collect()),
        }
    }
    /// Signed distance (approximate outside corners, exact sign).
    fn sd(&self, p: Point3<f64>) -> f64 {
        let boxd = |b: &[f64; 6]| {
            let dx = (b[0] - p.x).max(p.x - b[1]);
            let dy = (b[2] - p.y).max(p.y - b[3]);
            let dz = (b[4] - p.z).max(p.z - b[5]);
            dx.max(dy).max(dz)
        };
        match self {
            Shape::Box(b) => boxd(b),
            Shape::Cyl(c) => {
                let d = ((p.x - c[0]).powi(2) + (p.y - c[1]).powi(2)).sqrt() - c[2];
                let dz = (c[3] - p.z).max(p.z - (c[3] + c[4]));
                d.max(dz)
            }
            Shape::Slotted(b, h) => {
                let slot = (h[0] - p.x)
                    .max(p.x - h[1])
                    .max((h[2] - p.y).max(p.y - h[3]));
                boxd(b).max(-slot)
            }
            Shape::Parts(ps) => ps.iter().map(|x| x.sd(p)).fold(f64::INFINITY, f64::min),
        }
    }
    fn bbox(&self) -> [f64; 6] {
        match self {
            Shape::Box(b) | Shape::Slotted(b, _) => *b,
            Shape::Cyl(c) => [
                c[0] - c[2],
                c[0] + c[2],
                c[1] - c[2],
                c[1] + c[2],
                c[3],
                c[3] + c[4],
            ],
            Shape::Parts(ps) => ps.iter().map(Shape::bbox).fold(
                [
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                    f64::INFINITY,
                    f64::NEG_INFINITY,
                ],
                |a, b| {
                    [
                        a[0].min(b[0]),
                        a[1].max(b[1]),
                        a[2].min(b[2]),
                        a[3].max(b[3]),
                        a[4].min(b[4]),
                        a[5].max(b[5]),
                    ]
                },
            ),
        }
    }
    fn build(&self) -> AtRestBody<f64> {
        match self {
            Shape::Box(b) => finished(
                "box",
                brick::<f64>((b[0], b[1]), (b[2], b[3]), (b[4], b[5]), tol()),
                tol(),
            ),
            Shape::Cyl(c) => ext(vec![circle(c[0], c[1], c[2], c[5])], c[3], c[4]),
            Shape::Slotted(b, h) => ext(
                vec![rect(b[0], b[2], b[1], b[3]), rect(h[0], h[2], h[1], h[3])],
                b[4],
                b[5] - b[4],
            ),
            Shape::Parts(ps) => {
                let mut acc = ps[0].build();
                for p in &ps[1..] {
                    let q = p.build();
                    acc = match topo::union(&acc, &q, tol()) {
                        Ok(BooleanResult::Body(u)) => finished("parts", u.body.into_body(), tol()),
                        other => panic!("parts union: {:?}", other.map(|_| ())),
                    };
                }
                acc
            }
        }
    }
}

fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    bulge_loop(vec![
        (at(phase), 1.0),
        (at(phase + core::f64::consts::PI), 1.0),
    ])
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x0, y0), 0.0),
        (Point2::new(x1, y0), 0.0),
        (Point2::new(x1, y1), 0.0),
        (Point2::new(x0, y1), 0.0),
    ])
}

fn ext(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("ext", body, tol())
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    U,
    AmB,
    BmA,
    I,
}

/// One case: runs each op in both orders, checks the oracle, prints.
/// `v_i` is the closed-form volume of A ∩ B; `want_ff` the face–face
/// record count expected per op (U, A∖B, B∖A, ∩).
fn case(label: &str, a: &Shape, b: &Shape, v_i: f64, want_ff: [usize; 4]) -> usize {
    let (ab, bb) = (a.build(), b.build());
    let vol = |x: &Body<f64>| topo::mass_properties(x, tol()).unwrap().volume;
    let (va, vb) = (vol(&ab), vol(&bb));
    let band = Band::linear(tol()).unwrap();
    let scale = {
        let bx = a.bbox();
        (bx[1] - bx[0]).abs().max((bx[3] - bx[2]).abs())
    };
    let mut bad = 0;
    for swapped in [false, true] {
        for (k, op) in [Op::U, Op::AmB, Op::BmA, Op::I].into_iter().enumerate() {
            // A∖B with swapped operands is computed as subtract(B, A) under the label B∖A etc.
            let (x, y, real) = match (op, swapped) {
                (Op::U, false) => (&ab, &bb, "A∪B"),
                (Op::U, true) => (&bb, &ab, "B∪A"),
                (Op::I, false) => (&ab, &bb, "A∩B"),
                (Op::I, true) => (&bb, &ab, "B∩A"),
                (Op::AmB, _) => (&ab, &bb, "A∖B"),
                (Op::BmA, _) => (&bb, &ab, "B∖A"),
            };
            if swapped && matches!(op, Op::AmB | Op::BmA) {
                continue;
            }
            let got = match op {
                Op::U => topo::union(x, y, tol()),
                Op::I => topo::intersect(x, y, tol()),
                _ => topo::subtract(x, y, tol()),
            };
            let want_v = match op {
                Op::U => va + vb - v_i,
                Op::AmB => va - v_i,
                Op::BmA => vb - v_i,
                Op::I => v_i,
            };
            let member = |p: Point3<f64>| -> Option<bool> {
                let (sa, sb) = (a.sd(p), b.sd(p));
                if sa.abs() < 1e-3 * scale || sb.abs() < 1e-3 * scale {
                    return None;
                }
                let (ia, ib) = (sa < 0.0, sb < 0.0);
                Some(match op {
                    Op::U => ia || ib,
                    Op::AmB => ia && !ib,
                    Op::BmA => ib && !ia,
                    Op::I => ia && ib,
                })
            };
            let line = match got {
                Err(e) => match e.kind() {
                    topo::BooleanErrorKind::CarryRefused | topo::BooleanErrorKind::JoinDesync => {
                        format!("REFUSED {e:?}").chars().take(220).collect()
                    }
                    k => format!("REFUSED {k:?}"),
                },
                Ok(BooleanResult::Empty) => {
                    let ok = want_v.abs() < 1e-9 * scale.powi(3).max(1.0);
                    if !ok {
                        bad += 1;
                    }
                    format!("Empty{}", if ok { "" } else { " BAD-VOL" })
                }
                Ok(BooleanResult::Body(r)) => {
                    let v = vol(&r.body);
                    let mut notes = String::new();
                    if (v - want_v).abs() > 1e-7 * scale.powi(3) {
                        notes += &format!(" BAD-VOL(got {v:.9} want {want_v:.9})");
                        bad += 1;
                    }
                    // membership on a grid over the union of boxes
                    let (bx, by) = (a.bbox(), b.bbox());
                    let lo = [bx[0].min(by[0]), bx[2].min(by[2]), bx[4].min(by[4])];
                    let hi = [bx[1].max(by[1]), bx[3].max(by[3]), bx[5].max(by[5])];
                    let n = 7;
                    let mut miss = 0;
                    for i in 0..n {
                        for j in 0..n {
                            for kk in 0..n {
                                let f = |q: usize, l: f64, h: f64| {
                                    l + (h - l) * (q as f64 + 0.37) / n as f64
                                };
                                let p = Point3::new(
                                    f(i, lo[0], hi[0]),
                                    f(j, lo[1], hi[1]),
                                    f(kk, lo[2], hi[2]),
                                );
                                let Some(want) = member(p) else { continue };
                                let got = topo::point_in_solid(&r.body, p, band, tol())
                                    .map(|c| c == SolidContainment::In);
                                if got != Ok(want) {
                                    miss += 1;
                                }
                            }
                        }
                    }
                    if miss > 0 {
                        notes += &format!(" BAD-MEMBER({miss})");
                        bad += 1;
                    }
                    let ff = r.contacts.face_face().count();
                    if ff != want_ff[k] {
                        notes += &format!(" BAD-FF(got {ff} want {})", want_ff[k]);
                        bad += 1;
                    }
                    let tier3p = topo::validate_pseudomanifold(&r.body, &r.contacts, tol())
                        .map(|()| "3'ok".to_string())
                        .unwrap_or_else(|e| {
                            format!(
                                "3'err:{}",
                                format!("{e:?}").chars().take(90).collect::<String>()
                            )
                        });
                    format!(
                        "solids={} v={v:.9} kinds={:?} {tier3p}{notes}",
                        r.body.solids().count(),
                        r.contacts
                            .kinds()
                            .collect::<Vec<_>>()
                            .iter()
                            .map(|k| format!("{k:?}").chars().take(2).collect::<String>())
                            .collect::<Vec<_>>()
                    )
                }
            };
            println!("R4921 {label} | {real}: {line}");
        }
    }
    bad
}

fn lens(r: f64, depth: f64) -> f64 {
    if depth <= 0.0 {
        return 0.0;
    }
    r * r * ((r - depth) / r).acos() - (r - depth) * (2.0 * r * depth - depth * depth).sqrt()
}

/// Two circles of radii r1, r2 at centre distance d: their overlap area.
fn circle_overlap(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        let r = r1.min(r2);
        return core::f64::consts::PI * r * r;
    }
    let a1 = ((d * d + r1 * r1 - r2 * r2) / (2.0 * d * r1)).acos();
    let a2 = ((d * d + r2 * r2 - r1 * r1) / (2.0 * d * r2)).acos();
    r1 * r1 * (a1 - a1.sin() * a1.cos()) + r2 * r2 * (a2 - a2.sin() * a2.cos())
}

#[test]
#[ignore = "review probe, run by hand"]
fn review_4921_tangent_battery() {
    let eps = tol().eps();
    println!("R4921 eps={eps:e}");
    let pi = core::f64::consts::PI;
    let mut bad = 0;
    for s in [1.0, 1e-2, 1e2] {
        // plane × cylinder, outside: rod beside the plate's y = 2 face
        for (pose, d) in [
            ("tangent", 0.0),
            ("in-band+", eps * 0.1),
            ("in-band-", -eps * 0.1),
            ("gap", 1e3 * eps),
            ("overlap", -1e3 * eps),
            ("gap-big", 1e-4),
            ("overlap-big", -1e-4),
        ] {
            for (span, z0, h, ffu) in [
                ("line-ends-inside", 0.25, 0.5, 1),
                ("rod-past-face", -0.5, 2.0, 1),
            ] {
                let plate = Shape::Box([-2.0, 2.0, -2.0, 2.0, 0.0, 1.0]);
                let rod = Shape::Cyl([0.0, 2.5 + d, 0.5, z0, h, 0.0]);
                let h_in = (z0 + h).min(1.0) - z0.max(0.0);
                let v_i = lens(0.5, -d) * h_in;
                let ff = if pose.starts_with("tangent") || pose.starts_with("in-band") {
                    ffu
                } else {
                    0
                };
                bad += case(
                    &format!("s={s} p×c {span} {pose}"),
                    &plate.scaled(s),
                    &rod.scaled(s),
                    v_i * s.powi(3),
                    [ff, 0, 0, 0],
                );
            }
        }
        // cylinder × cylinder, outside, parallel axes
        for (pose, d) in [
            ("tangent", 0.0),
            ("in-band", eps * 0.1),
            ("gap-big", 1e-4),
            ("overlap-big", -1e-4),
        ] {
            let a = Shape::Cyl([0.0, 0.0, 1.0, 0.0, 1.0, pi / 2.0]);
            let b = Shape::Cyl([1.5 + d, 0.0, 0.5, 0.25, 0.5, pi / 2.0]);
            let v_i = circle_overlap(1.0, 0.5, 1.5 + d) * 0.5;
            let ff = usize::from(pose == "tangent" || pose == "in-band");
            bad += case(
                &format!("s={s} c×c out {pose}"),
                &a.scaled(s),
                &b.scaled(s),
                v_i * s.powi(3),
                [ff, 0, 0, 0],
            );
        }
        // cylinder × cylinder, inside tangency, B's caps inside A
        for (pose, d) in [
            ("tangent", 0.0),
            ("in-band", eps * 0.1),
            ("inside-big", -1e-4),
        ] {
            let a = Shape::Cyl([0.0, 0.0, 2.0, 0.0, 1.0, pi / 2.0]);
            let b = Shape::Cyl([1.0 + d, 0.0, 1.0, 0.25, 0.5, pi / 2.0]);
            let v_i = pi * 0.5;
            let ff = usize::from(pose != "inside-big");
            bad += case(
                &format!("s={s} c×c in {pose}"),
                &a.scaled(s),
                &b.scaled(s),
                v_i * s.powi(3),
                [0, ff, 0, 0],
            );
        }
        // plane × cylinder inside: a bore tangent to the plate's y = 2 face from inside
        {
            let plate = Shape::Box([-2.0, 2.0, -2.0, 2.0, 0.0, 1.0]);
            let rod = Shape::Cyl([0.0, 1.5, 0.5, 0.25, 0.5, 0.0]);
            bad += case(
                &format!("s={s} p×c in"),
                &plate.scaled(s),
                &rod.scaled(s),
                pi * 0.125 * s.powi(3),
                [0, 1, 0, 0],
            );
        }
    }
    println!("R4921 bad={bad}");
}

/// Both of a face–face row's faces divided by the join: plate (with a
/// disjoint block crossing the rod) ∪ rod (with a disjoint block crossing
/// the plate's y = 2 face); and the inside form, a slotted plate minus a
/// rod tangent to its y = 2 face from inside plus a block across it.
#[test]
#[ignore = "review probe, run by hand"]
fn review_4921_both_faces_divided() {
    let plate = Shape::Box([-2.0, 2.0, -2.0, 2.0, 0.0, 1.0]);
    let rod = Shape::Cyl([0.0, 2.5, 0.5, 0.25, 0.5, 0.0]);
    // crosses the rod's wall at its far side, clear of the plate
    let block_a = Shape::Box([-0.1, 0.1, 2.8, 3.2, 0.0, 1.0]);
    // crosses the plate's y = 2 face over its whole height, clear of the rod
    let block_b = Shape::Box([1.0, 1.5, 1.5, 2.5, -0.5, 1.5]);
    let a = Shape::Parts(vec![plate.clone(), block_a.clone()]);
    let b = Shape::Parts(vec![rod.clone(), block_b.clone()]);
    // A ∩ B: block_a ∩ rod (box ∩ circle segment over z 0.25..0.75) + plate ∩ block_b.
    let seg = {
        // circle centre (0,2.5) r .5, region y in [2.8, 3.0], |x| <= .1
        let n = 4000;
        let mut area = 0.0;
        for i in 0..n {
            let x = -0.1 + 0.2 * (i as f64 + 0.5) / n as f64;
            let ytop = 2.5 + (0.25 - x * x).sqrt();
            area += (ytop - 2.8).max(0.0) * 0.2 / n as f64;
        }
        area
    };
    let v_i = seg * 0.5 + 0.5 * 0.5 * 1.0;
    let mut bad = case("divided out", &a, &b, v_i, [1, 0, 0, 0]);
    // inside: slotted plate, the slot crossing the rod's wall near y = 1
    let slotted = Shape::Slotted([-2.0, 2.0, -2.0, 2.0, 0.0, 1.0], [-0.1, 0.1, 0.8, 1.2]);
    let rod_in = Shape::Cyl([0.0, 1.5, 0.5, 0.25, 0.5, 0.0]);
    let b_in = Shape::Parts(vec![rod_in, block_b]);
    let seg_in = {
        let n = 4000;
        let mut area = 0.0;
        for i in 0..n {
            let x = -0.1 + 0.2 * (i as f64 + 0.5) / n as f64;
            let ybot = 1.5 - (0.25 - x * x).sqrt();
            area += (1.2 - ybot).max(0.0) * 0.2 / n as f64;
        }
        area
    };
    let v_i_in = (core::f64::consts::PI * 0.25 - seg_in) * 0.5 + 0.5 * 0.5 * 1.0;
    bad += case("divided in", &slotted, &b_in, v_i_in, [0, 1, 0, 0]);
    println!("R4921 bad={bad}");
}

/// The rod on the plate, then a union with a far brick carrying every
/// record: does an op that touches neither cell refuse on the cap
/// circles' edge–face records?
#[test]
#[ignore = "review probe, run by hand"]
fn review_4921_far_op_carrying_every_record() {
    let plate = Shape::Box([-2.0, 2.0, -2.0, 2.0, 0.0, 1.0]).build();
    let rod = Shape::Cyl([0.0, 2.5, 0.5, 0.25, 0.5, 0.0]).build();
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("{:?}", other.map(|_| ())),
    };
    println!(
        "R4921 far: first kinds {:?}",
        u.contacts.kinds().collect::<Vec<_>>()
    );
    println!(
        "R4921 far: first coincidences {:?}",
        u.coincidences
            .iter()
            .map(|c| (c.relation, c.site))
            .collect::<Vec<_>>()
    );
    println!("R4921 far: first records {:?}", u.contacts.rows());
    let far = Shape::Box([10.0, 11.0, 10.0, 11.0, 0.0, 1.0]).build();
    let hole = Shape::Box([-1.0, -0.5, -1.0, -0.5, -1.0, 2.0]).build();
    for (what, other, op) in [
        ("∪ far", &far, topo::BooleanOp::Union),
        ("∖ hole in plate", &hole, topo::BooleanOp::Subtract),
    ] {
        for (rec, records_a) in [
            (
                "ff only",
                topo::OperandRecords {
                    ff: u.contacts.carried().ff,
                    ..Default::default()
                },
            ),
            ("all", u.contacts.carried()),
        ] {
            let got = topo::boolean_op_with(
                op,
                &u.body,
                other,
                &topo::BooleanInputs {
                    records_a,
                    ..topo::BooleanInputs::default()
                },
                topo::SweepStrategy::Realized,
                tol(),
            );
            let line = match got {
                Ok(BooleanResult::Body(r)) => format!(
                    "solids={} kinds={:?} 3'={:?}",
                    r.body.solids().count(),
                    r.contacts.kinds().collect::<Vec<_>>(),
                    topo::validate_pseudomanifold(&r.body, &r.contacts, tol())
                        .map_err(|e| format!("{e:?}").chars().take(120).collect::<String>())
                ),
                Ok(BooleanResult::Empty) => "Empty".into(),
                Err(e) => format!("REFUSED {e:?}").chars().take(200).collect(),
            };
            println!("R4921 far: {what} [{rec}]: {line}");
        }
    }
}
