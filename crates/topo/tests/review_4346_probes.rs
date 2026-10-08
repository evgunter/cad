//! Review probes for PR 4346 (scratch branch, not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common;

use common::finished;
use common::meeting::{MEET, PLATE, at, corners_disjoint, orders, shape};
use geom_core::{Point3, Tol};
use std::collections::BTreeMap;
use topo::{
    AtRestBody, BooleanBody, BooleanDeclarations, BooleanError, BooleanResult, FacePairDeclaration,
    union, union_with, validate_geometric, validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}


/// An upright prism over the triangle from `apex` to the points at the
/// bearings `a0` and `a1` (degrees) 0.4 from [`MEET`], over `z`.
#[derive(Clone, Copy)]
struct Prism {
    apex: (f64, f64),
    a0: f64,
    a1: f64,
    z: (f64, f64),
}

impl Prism {
    fn on_line(a0: f64, a1: f64, z: (f64, f64)) -> Self {
        Self {
            apex: (MEET[0], MEET[1]),
            a0,
            a1,
            z,
        }
    }

    fn footprint(&self) -> [(f64, f64); 3] {
        let at = |a: f64| {
            let (s, c) = f64::to_radians(a).sin_cos();
            (0.4f64.mul_add(c, MEET[0]), 0.4f64.mul_add(s, MEET[1]))
        };
        [self.apex, at(self.a0), at(self.a1)]
    }

    fn body(&self) -> AtRestBody<f64> {
        let body = common::prism_z(&self.footprint(), self.z.0, self.z.1, t()).body;
        finished("a prism on the line", body, t())
    }

    /// **The analytic oracle**: whether `q` is inside the prism, read
    /// off its closed form rather than any body.
    fn holds(&self, q: [f64; 3]) -> bool {
        let p = self.footprint();
        let left = |(x0, y0): (f64, f64), (x1, y1): (f64, f64)| {
            (x1 - x0) * (q[1] - y0) - (y1 - y0) * (q[0] - x0) > 0.0
        };
        self.z.0 < q[2] && q[2] < self.z.1 && (0..3).all(|i| left(p[i], p[(i + 1) % 3]))
    }
}

/// The part of the convex polygon `p` (counterclockwise) left of the
/// directed line `a → b`.
fn clip(p: &[(f64, f64)], a: (f64, f64), b: (f64, f64)) -> Vec<(f64, f64)> {
    let side = |q: (f64, f64)| (b.0 - a.0) * (q.1 - a.1) - (b.1 - a.1) * (q.0 - a.0);
    let mut out = Vec::new();
    for i in 0..p.len() {
        let (q0, q1) = (p[i], p[(i + 1) % p.len()]);
        let (s0, s1) = (side(q0), side(q1));
        if s0 >= 0.0 {
            out.push(q0);
        }
        if (s0 >= 0.0) != (s1 >= 0.0) {
            let u = s0 / (s0 - s1);
            out.push((u.mul_add(q1.0 - q0.0, q0.0), u.mul_add(q1.1 - q0.1, q0.1)));
        }
    }
    out
}

fn area(p: &[(f64, f64)]) -> f64 {
    (0..p.len())
        .map(|i| {
            let ((x0, y0), (x1, y1)) = (p[i], p[(i + 1) % p.len()]);
            x0 * y1 - x1 * y0
        })
        .sum::<f64>()
        / 2.0
}

/// **The union's volume in closed form**: by inclusion and exclusion
/// over the prisms, each subset's common part a prism over its
/// footprints' intersection (convex, clipped exactly) and its z-ranges';
/// with the plate, only the parts above its top count beside it.
fn union_volume(prisms: &[Prism], plate: bool) -> f64 {
    let floor = if plate { PLATE[2].1 } else { f64::NEG_INFINITY };
    let mut v = if plate { 6.0 } else { 0.0 };
    for set in 1..1usize << prisms.len() {
        let ps: Vec<&Prism> = (0..prisms.len())
            .filter(|i| set >> i & 1 == 1)
            .map(|i| &prisms[i])
            .collect();
        let mut common = ps[0].footprint().to_vec();
        for p in &ps[1..] {
            let f = p.footprint();
            for i in 0..3 {
                common = clip(&common, f[i], f[(i + 1) % 3]);
            }
        }
        let lo = ps.iter().map(|p| p.z.0).fold(floor, f64::max);
        let hi = ps.iter().map(|p| p.z.1).fold(f64::INFINITY, f64::min);
        let sign = if ps.len() % 2 == 1 { 1.0 } else { -1.0 };
        v += sign * area(&common) * (hi - lo).max(0.0);
    }
    v
}

fn in_plate(q: [f64; 3]) -> bool {
    (0..3).all(|i| PLATE[i].0 < q[i] && q[i] < PLATE[i].1)
}

/// Three prisms over 50° sectors, each 70° from the next.
fn three() -> Vec<Prism> {
    vec![
        Prism::on_line(0.0, 50.0, (0.5, 2.0)),
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
        Prism::on_line(240.0, 290.0, (0.44, 1.81)),
    ]
}

/// Four prisms over 50° sectors, each 40° from the next.
fn four() -> Vec<Prism> {
    vec![
        Prism::on_line(0.0, 50.0, (0.5, 2.0)),
        Prism::on_line(90.0, 140.0, (0.47, 1.7)),
        Prism::on_line(180.0, 230.0, (0.44, 1.81)),
        Prism::on_line(270.0, 320.0, (0.41, 1.63)),
    ]
}

/// [`three`] with the second prism's corner moved `d` along +x off the
/// line.
fn three_off(d: f64) -> Vec<Prism> {
    let mut p = three();
    p[1].apex.0 += d;
    p
}

/// The left fold of `members` by union in `order`, or the step that
/// refused and why.
fn fold(
    members: &[AtRestBody<f64>],
    order: &[usize],
) -> Result<BooleanBody<f64>, (usize, BooleanError)> {
    let mut body = members[order[0]].clone();
    let mut last = None;
    for (k, &i) in order.iter().enumerate().skip(1) {
        match union(&body, &members[i], t()) {
            Ok(BooleanResult::Body(r)) => {
                body = r.body.clone();
                last = Some(r);
            }
            Ok(BooleanResult::Empty) => panic!("step {k} of {order:?} is empty"),
            Err(e) => return Err((k, e)),
        }
    }
    Ok(last.expect("a fold of two or more members"))
}

fn inside_of(body: &AtRestBody<f64>, q: [f64; 3]) -> Option<bool> {
    let band = geom_core::Band::linear(t()).unwrap();
    match topo::point_in_solid(body, Point3::new(q[0], q[1], q[2]), band, t()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        Ok(topo::SolidContainment::OnBoundary)
        | Err(
            topo::PointInSolidError::Escalated { .. }
            | topo::PointInSolidError::Loop(topo::PointInLoopError::Escalated { .. }),
        ) => None,
        Err(e) => panic!("point in solid at {q:?}: {e:?}"),
    }
}

/// Probes about the line: a 7 × 7 × 13 grid within 0.39 of it across,
/// from z = 0.06 to 1.86, offset off the fixtures' planes.
fn probes() -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    for i in 0..7 {
        for j in 0..7 {
            for k in 0..13 {
                let across = |n: i32, d: f64| 0.13f64.mul_add(f64::from(n) - 3.0, d);
                out.push([
                    MEET[0] + across(i, 0.0137),
                    MEET[1] + across(j, 0.0071),
                    0.15f64.mul_add(f64::from(k), 0.0593),
                ]);
            }
        }
    }
    out
}


fn plate_body() -> AtRestBody<f64> {
    finished("the plate", common::brick(PLATE[0], PLATE[1], PLATE[2], t()), t())
}

fn err_kind(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    let head: String = s.chars().take_while(|c| *c != '{' && *c != '(').collect();
    match e {
        BooleanError::ClassificationInvariant { what } | BooleanError::JoinDesync { what } => {
            format!("{}: {what}", head.trim())
        }
        BooleanError::ResultInvalid { .. } | BooleanError::Euler(_) => s.chars().take(300).collect(),
        _ => head.trim().to_string(),
    }
}

/// Folds every order (threaded), checking each built body against the
/// closed forms; returns a histogram of outcomes. "WRONG" entries are
/// silent wrong bodies.
fn survey(label: &str, members: &[AtRestBody<f64>], prisms: &[Prism], plate: bool) -> BTreeMap<String, usize> {
    let volume = union_volume(prisms, plate);
    let holds = |q: [f64; 3]| (plate && in_plate(q)) || prisms.iter().any(|p| p.holds(q));
    let probes = probes();
    let all = orders(members.len());
    let chunks: Vec<Vec<Vec<usize>>> = (0..4).map(|k| all.iter().skip(k).step_by(4).cloned().collect()).collect();
    let results: Vec<(Vec<usize>, String, Option<String>)> = std::thread::scope(|s| {
        let hs: Vec<_> = chunks
            .iter()
            .map(|chunk| {
                let probes = &probes;
                let holds = &holds;
                s.spawn(move || {
                    let mut out = Vec::new();
                    for order in chunk {
                        let r = match fold(members, order) {
                            Err((k, e)) => (order.clone(), format!("refuse@{k} {}", err_kind(&e)), None),
                            Ok(r) => {
                                let b = &r.body;
                                let counts = [b.faces().count(), b.edges().count(), b.vertices().count()];
                                let mut bad = Vec::new();
                                if validate_geometric(b, t()).is_err() {
                                    bad.push("tier3".to_string());
                                }
                                let v = topo::mass_properties(b, t()).unwrap().volume;
                                if (v - volume).abs() > 1e-7 {
                                    bad.push(format!("vol {v} vs {volume}"));
                                }
                                if let Err(e) = corners_disjoint(b) {
                                    bad.push(format!("corners[{}] fine[{:?}]", e.chars().take(80).collect::<String>(), corners_fine(b).err()));
                                }
                                let mut dis = 0;
                                let mut read = 0;
                                for &q in probes {
                                    if let Some(g) = inside_of(b, q) {
                                        read += 1;
                                        if g != holds(q) {
                                            dis += 1;
                                        }
                                    }
                                }
                                if dis > 0 {
                                    bad.push(format!("{dis} probes disagree"));
                                }
                                let tag = if bad.is_empty() {
                                    format!("build {counts:?}")
                                } else {
                                    format!("WRONG {counts:?} {}", bad.join(","))
                                };
                                let s = format!("{:?}", shape(b));
                                let _ = read;
                                (order.clone(), tag, Some(s))
                            }
                        };
                        out.push(r);
                    }
                    out
                })
            })
            .collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    let mut hist = BTreeMap::new();
    let mut shapes: BTreeMap<String, usize> = BTreeMap::new();
    for (order, tag, s) in &results {
        *hist.entry(tag.clone()).or_insert(0) += 1;
        if let Some(s) = s {
            *shapes.entry(s.clone()).or_insert(0) += 1;
        }
        if tag.starts_with("WRONG") {
            eprintln!("{label} {order:?}: {tag}");
        }
    }
    if shapes.len() > 1 {
        *hist.entry(format!("DISTINCT BODIES {}", shapes.len())).or_insert(0) += 1;
    }
    eprintln!("SURVEY {label}: {hist:?}");
    hist
}

fn with_plate(prisms: &[Prism]) -> Vec<AtRestBody<f64>> {
    let mut m = vec![plate_body()];
    m.extend(prisms.iter().map(Prism::body));
    m
}

#[test]
#[ignore = "probe"]
fn probe_k5() {
    let p: Vec<Prism> = (0..5)
        .map(|i| {
            let a = 72.0 * f64::from(i);
            Prism::on_line(a, a + 50.0, (0.5 - 0.03 * f64::from(i), 2.0 - 0.07 * f64::from(i)))
        })
        .collect();
    survey("k5 alone", &p.iter().map(Prism::body).collect::<Vec<_>>(), &p, false);
    survey("k5 plate", &with_plate(&p), &p, true);
}

#[test]
#[ignore = "probe"]
fn probe_staggers() {
    let sec = [(0.0, 50.0), (120.0, 170.0), (240.0, 290.0)];
    let zs: Vec<(&str, [(f64, f64); 3])> = vec![
        ("equal z", [(0.5, 2.0); 3]),
        ("equal tops", [(0.5, 2.0), (0.47, 2.0), (0.44, 2.0)]),
        ("equal bottoms", [(0.5, 2.0), (0.5, 1.7), (0.5, 1.81)]),
        ("nested", [(0.4, 2.0), (0.6, 1.5), (0.7, 1.2)]),
        ("bottom on plate top", [(1.0, 2.0), (0.47, 1.7), (0.44, 1.81)]),
        ("all bottoms on plate top", [(1.0, 2.0), (1.0, 1.7), (1.0, 1.81)]),
        ("top on plate top", [(0.5, 2.0), (0.3, 1.0), (0.44, 1.81)]),
        ("one top meets another's bottom", [(0.5, 1.5), (1.5, 2.0), (0.44, 1.81)]),
    ];
    for (name, z) in zs {
        let p: Vec<Prism> = (0..3).map(|i| Prism::on_line(sec[i].0, sec[i].1, z[i])).collect();
        survey(&format!("{name} alone"), &p.iter().map(Prism::body).collect::<Vec<_>>(), &p, false);
        survey(&format!("{name} plate"), &with_plate(&p), &p, true);
    }
}

/// The corner of prism 1 moved along `dir` by kε.
#[test]
#[ignore = "probe"]
fn probe_bands() {
    let eps = t().eps();
    eprintln!("eps = {eps}");
    for (dname, dir) in [("+x", (1.0, 0.0)), ("-x", (-1.0, 0.0)), ("+y", (0.0, 1.0)), ("diag", (0.6, -0.8))] {
        for k in [0.5, 1.0, 1.5, 2.0, 3.0, 5.0, 8.0, 10.0, 12.0, 20.0, 50.0, 100.0, 300.0, 1000.0, 3000.0, 1e4] {
            let mut p = three();
            p[1].apex.0 += dir.0 * k * eps;
            p[1].apex.1 += dir.1 * k * eps;
            survey(&format!("prism 1 apex {dname} {k}eps"), &with_plate(&p), &p, true);
        }
    }
}

/// D10: the 50° fixture with an arbitrary declaration at the last
/// step, every pair of faces touching the line, every class.
#[test]
#[ignore = "probe"]
fn probe_d10() {
    use topo::{BooleanCoincidence, ContactClass};
    let classes = [
        BooleanCoincidence::Contact(ContactClass::Rest),
        BooleanCoincidence::Contact(ContactClass::Tangent),
        BooleanCoincidence::Continuation,
        BooleanCoincidence::Seam,
    ];
    let prisms = three();
    let members = with_plate(&prisms);
    let touches = |b: &AtRestBody<f64>, f: topo::FaceKey| {
        let face = b.get_face(f).unwrap();
        common::meeting::cycles_of(b, face).into_iter().flatten().any(|he| {
            let p = b.half_edge_start_point(he).unwrap();
            (p.x - MEET[0]).hypot(p.y - MEET[1]) < 1e-6
        })
    };
    let all = orders(members.len());
    let hist: BTreeMap<String, usize> = std::thread::scope(|s| {
        let hs: Vec<_> = (0..4)
            .map(|c| {
                let all = &all;
                let members = &members;
                let touches = &touches;
                s.spawn(move || {
                    let mut hist: BTreeMap<String, usize> = BTreeMap::new();
                    for order in all.iter().skip(c).step_by(4) {
                        // declare at step `at_step` (1-based), plain elsewhere
                        for at_step in 1..order.len() {
                            let mut body = members[order[0]].clone();
                            let mut plain_body = None;
                            for (k, &i) in order.iter().enumerate().skip(1) {
                                if k == at_step {
                                    let a = body.clone();
                                    let b = &members[i];
                                    let rest_plain = |start: AtRestBody<f64>| {
                                        let mut x = start;
                                        let mut last = None;
                                        for &j in &order[k + 1..] {
                                            match union(&x, &members[j], t()) {
                                                Ok(BooleanResult::Body(r)) => {
                                                    x = r.body.clone();
                                                    last = Some(r.body);
                                                }
                                                Ok(BooleanResult::Empty) => return Err("empty".to_string()),
                                                Err(e) => return Err(err_kind(&e)),
                                            }
                                        }
                                        Ok(last.unwrap_or(x))
                                    };
                                    let plain = match union(&a, b, t()) {
                                        Ok(BooleanResult::Body(r)) => rest_plain(r.body).unwrap(),
                                        other => panic!("plain {order:?} step {k}: {other:?}"),
                                    };
                                    let ps = shape(&plain);
                                    plain_body = Some(ps.clone());
                                    let fa: Vec<_> = a.faces().map(|(f, _)| f).filter(|&f| touches(&a, f)).collect();
                                    let fb: Vec<_> = b.faces().map(|(f, _)| f).filter(|&f| touches(b, f)).collect();
                                    for &x in &fa {
                                        for &y in &fb {
                                            for class in classes {
                                                let decls = BooleanDeclarations {
                                                    coincident_faces: vec![FacePairDeclaration { a: x, b: y, class }],
                                                    ..BooleanDeclarations::none()
                                                };
                                                let tag = match union_with(&a, b, &decls, t()) {
                                                    Ok(BooleanResult::Body(r)) => match rest_plain(r.body) {
                                                        Ok(fin) => {
                                                            if shape(&fin) == ps {
                                                                format!("{class:?}: same body")
                                                            } else {
                                                                eprintln!("DIFF {order:?} step {k} {class:?}");
                                                                format!("{class:?}: DIFFERENT BODY")
                                                            }
                                                        }
                                                        Err(e) => format!("{class:?}: later step refuses {e}"),
                                                    },
                                                    Ok(BooleanResult::Empty) => format!("{class:?}: EMPTY"),
                                                    Err(e) => format!("{class:?}: refuse {}", err_kind(&e)),
                                                };
                                                *hist.entry(tag).or_insert(0) += 1;
                                            }
                                        }
                                    }
                                    break;
                                }
                                match union(&body, &members[i], t()) {
                                    Ok(BooleanResult::Body(r)) => body = r.body,
                                    other => panic!("{other:?}"),
                                }
                            }
                            let _ = plain_body;
                        }
                    }
                    hist
                })
            })
            .collect();
        let mut tot = BTreeMap::new();
        for h in hs {
            for (k, v) in h.join().unwrap() {
                *tot.entry(k).or_insert(0) += v;
            }
        }
        tot
    });
    for (k, v) in &hist {
        eprintln!("D10 {k}: {v}");
    }
    assert!(!hist.keys().any(|k| k.contains("DIFFERENT") || k.contains("EMPTY")));
}

/// D10, carried: every step carries the previous step's contact records
/// back in (vv/vf as `class`, ve, ee); the final body must be the
/// undeclared one, or the step refuses typed.
#[test]
#[ignore = "probe"]
fn probe_d10_carried() {
    use topo::{CarriedContacts, CarriedVf, CarriedVv, ContactClass};
    for (label, prisms) in [("three 50°", three()), ("four", four()), ("three off 0.1eps", three_off(0.1 * t().eps()))] {
        let members = with_plate(&prisms);
        let mut hist: BTreeMap<String, usize> = BTreeMap::new();
        for class in [ContactClass::Rest, ContactClass::Tangent] {
            for order in orders(members.len()) {
                let plain = shape(&fold(&members, &order).unwrap().body);
                let mut body = members[order[0]].clone();
                let mut carried = CarriedContacts::default();
                let mut tag = None;
                for &i in &order[1..] {
                    let decls = BooleanDeclarations { carried_a: carried.clone(), ..BooleanDeclarations::none() };
                    match union_with(&body, &members[i], &decls, t()) {
                        Ok(BooleanResult::Body(r)) => {
                            let c = &r.contacts;
                            carried = CarriedContacts {
                                vv: c.vv.iter().map(|&pair| CarriedVv { pair, class }).collect(),
                                vf: c.a_on_b.iter().chain(&c.b_on_a).map(|&rest| CarriedVf { rest, class }).collect(),
                                ve: c.ve.clone(),
                                ee: c.ee.clone(),
                            };
                            body = r.body;
                        }
                        Ok(BooleanResult::Empty) => {
                            tag = Some("EMPTY".to_string());
                            break;
                        }
                        Err(e) => {
                            tag = Some(format!("refuse {}", err_kind(&e)));
                            break;
                        }
                    }
                }
                let tag = tag.unwrap_or_else(|| {
                    if shape(&body) == plain { "same body".into() } else { "DIFFERENT BODY".into() }
                });
                *hist.entry(format!("{class:?}: {tag}")).or_insert(0) += 1;
            }
        }
        eprintln!("D10C {label}: {hist:?}");
        assert!(!hist.keys().any(|k| k.contains("DIFFERENT") || k.contains("EMPTY")));
    }
}

/// A prism over the triangle (apex, r at a0, r at a1) about MEET, z, then
/// turned by `delta` about the horizontal axis through `(MEET.xy, pivot_z)`
/// that tips +z toward bearing `toward` (degrees).
#[derive(Clone, Copy)]
struct Gen {
    r: f64,
    a0: f64,
    a1: f64,
    z: (f64, f64),
    toward: f64,
    delta: f64,
    pivot_z: f64,
}

impl Gen {
    fn foot(&self) -> [(f64, f64); 3] {
        let at = |a: f64| {
            let (s, c) = f64::to_radians(a).sin_cos();
            (self.r.mul_add(c, MEET[0]), self.r.mul_add(s, MEET[1]))
        };
        [(MEET[0], MEET[1]), at(self.a0), at(self.a1)]
    }
    fn pose(&self) -> common::meeting::Pose {
        let (s, c) = self.toward.to_radians().sin_cos();
        let axis = [-s, c, 0.0];
        // turn about the axis through the pivot: shift so pivot maps to itself
        let r = common::meeting::Pose::turn("g", axis, self.delta, [0.0, 0.0, 0.0]);
        let p = [MEET[0], MEET[1], self.pivot_z];
        let q = r.at(p);
        common::meeting::Pose::turn("g", axis, self.delta, [p[0] - q.x, p[1] - q.y, p[2] - q.z])
    }
    fn body(&self) -> AtRestBody<f64> {
        let pose = self.pose();
        let mut body = topo::Body::<f64>::new();
        common::prism_ops(&mut body, &self.foot(), self.z, |x, y, z| pose.at([x, y, z]), common::FaceGeometry::Certified, t());
        common::describe_as_intersections(&mut body, t());
        finished("a turned prism", body, t())
    }
    /// Inverse-pose membership.
    fn holds(&self, q: [f64; 3]) -> bool {
        let inv = Gen { delta: -self.delta, ..*self }.pose();
        let p = inv.at(q);
        let f = self.foot();
        let left = |(x0, y0): (f64, f64), (x1, y1): (f64, f64)| (x1 - x0) * (p.y - y0) - (y1 - y0) * (p.x - x0) > 0.0;
        self.z.0 < p.z && p.z < self.z.1 && (0..3).all(|i| left(f[i], f[(i + 1) % 3]))
    }
    fn vol(&self) -> f64 {
        let f = self.foot();
        area(&f) * (self.z.1 - self.z.0)
    }
}

fn survey_gen(label: &str, gens: &[Gen], plate: bool) -> BTreeMap<String, usize> {
    let mut members = Vec::new();
    if plate {
        members.push(plate_body());
    }
    members.extend(gens.iter().map(Gen::body));
    let probes = probes();
    let holds = |q: [f64; 3]| (plate && in_plate(q)) || gens.iter().any(|g| g.holds(q));
    let mut hist = BTreeMap::new();
    let mut shapes = std::collections::BTreeSet::new();
    for order in orders(members.len()) {
        let tag = match fold(&members, &order) {
            Err((k, e)) => format!("refuse@{k} {}", err_kind(&e)),
            Ok(r) => {
                let b = &r.body;
                let counts = [b.faces().count(), b.edges().count(), b.vertices().count()];
                let mut bad = Vec::new();
                if validate_geometric(b, t()).is_err() {
                    bad.push("tier3".to_string());
                }
                if corners_disjoint(b).is_err() {
                    bad.push("corners".into());
                }
                let dis = probes.iter().filter(|&&q| inside_of(b, q).is_some_and(|g| g != holds(q))).count();
                if dis > 0 {
                    bad.push(format!("{dis} probes disagree"));
                }
                let v = topo::mass_properties(b, t()).unwrap().volume;
                shapes.insert(format!("{:?}", shape(b)));
                if bad.is_empty() { format!("build {counts:?}") } else { format!("WRONG {counts:?} {} v={v}", bad.join(",")) }
            }
        };
        *hist.entry(tag).or_insert(0) += 1;
    }
    if shapes.len() > 1 {
        hist.insert(format!("DISTINCT BODIES {}", shapes.len()), 1);
    }
    eprintln!("GEN {label}: {hist:?}");
    hist
}

/// Non-transitive grouping: a tiny prism's edge (short arm) is within
/// the zero of both a regular prism's coincident edge and a tilted
/// prism's edge, which are decided apart from each other at their
/// longer arm.
#[test]
#[ignore = "probe"]
fn probe_nontransitive() {
    let eps = t().eps();
    for r_small in [0.003, 0.0005] {
        for toward in [25.0, 205.0, 265.0, 145.0] {
            // delta * r_small < eps (zero) while delta * 0.4 > 10 eps
            for kd in [0.3, 0.6, 0.9] {
                let delta = kd * eps / r_small;
                let p0 = Gen { r: 0.4, a0: 0.0, a1: 50.0, z: (0.5, 2.0), toward: 0.0, delta: 0.0, pivot_z: 0.5 };
                let p1 = Gen { r: r_small, a0: 120.0, a1: 170.0, z: (0.5, 1.7), toward: 0.0, delta: 0.0, pivot_z: 0.5 };
                let p2 = Gen { r: 0.4, a0: 240.0, a1: 290.0, z: (0.5, 1.81), toward, delta, pivot_z: 0.5 };
                let label = format!("r1={r_small} toward={toward} delta={delta:e} (d*0.4={:.1}eps, d*r1={:.2}eps)", delta * 0.4 / eps, delta * r_small / eps);
                survey_gen(&format!("{label} alone"), &[p0, p1, p2], false);
                // pivot below so the tilted edge passes through V
                let p2b = Gen { z: (0.3, 1.81), ..p2 };
                survey_gen(&format!("{label} through"), &[p0, p1, p2b], false);
            }
        }
    }
}

/// `corners_disjoint` with points grouped by vertex key rather than by
/// micron: distinct vertices a sliver apart are not compared.
fn corners_fine(body: &topo::Body<f64>) -> Result<(), String> {
    use std::f64::consts::TAU;
    for (fk, f) in body.faces() {
        let cycles = common::meeting::cycles_of(body, f);
        let pt = |he| body.half_edge_start_point(he).unwrap();
        let outer: Vec<_> = cycles[0].iter().map(|&he| pt(he)).collect();
        let (mut nx, mut ny, mut nz) = (0.0, 0.0, 0.0);
        for i in 0..outer.len() {
            let (a, b) = (outer[i], outer[(i + 1) % outer.len()]);
            nx += (a.y - b.y) * (a.z + b.z);
            ny += (a.z - b.z) * (a.x + b.x);
            nz += (a.x - b.x) * (a.y + b.y);
        }
        let n = geom_core::Vec3::new(nx, ny, nz).normalize();
        let seed = if n.x.abs() < 0.9 { geom_core::Vec3::new(1.0, 0.0, 0.0) } else { geom_core::Vec3::new(0.0, 1.0, 0.0) };
        let u = n.cross(seed).normalize();
        let v = n.cross(u);
        let angle = |d: geom_core::Vec3<f64>| d.dot(v).atan2(d.dot(u)).rem_euclid(TAU);
        let mut corners: BTreeMap<topo::VertexKey, Vec<(f64, f64)>> = BTreeMap::new();
        for cycle in &cycles {
            let m = cycle.len();
            for i in 0..m {
                let (prev, here, next) = (pt(cycle[(i + m - 1) % m]), pt(cycle[i]), pt(cycle[(i + 1) % m]));
                let from = angle(next - here);
                let sweep = (angle(prev - here) - from).rem_euclid(TAU);
                let vk = body.get_half_edge(cycle[i]).unwrap().start;
                corners.entry(vk).or_default().push((from, sweep));
            }
        }
        for (pk, cs) in corners {
            for (i, &(a, sa)) in cs.iter().enumerate() {
                for &(b, sb) in &cs[i + 1..] {
                    if (b - a).rem_euclid(TAU) < sa - 1e-9 || (a - b).rem_euclid(TAU) < sb - 1e-9 {
                        return Err(format!("{fk:?}: two corners at {pk:?} overlap"));
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
#[ignore = "probe"]
fn probe_bands_detail() {
    let eps = t().eps();
    for (dname, dir, ks) in [
        ("+x", (1.0, 0.0), vec![100.0, 300.0, 3000.0]),
        ("-x", (-1.0, 0.0), vec![50.0, 100.0]),
        ("+y", (0.0, 1.0), vec![50.0]),
        ("diag", (0.6, -0.8), vec![1.0, 1000.0]),
    ] {
        for k in ks {
            let mut p = three();
            p[1].apex.0 += dir.0 * k * eps;
            p[1].apex.1 += dir.1 * k * eps;
            survey(&format!("detail {dname} {k}eps"), &with_plate(&p), &p, true);
        }
    }
}

/// One prism tilted within the zero about a point on the line: it must
/// build as the on-line fixture does.
#[test]
#[ignore = "probe"]
fn probe_tilt_within_zero() {
    let eps = t().eps();
    for k in [0.05, 0.2] {
        for toward in [25.0, 85.0, 265.0] {
            let delta = k * eps;
            let p0 = Gen { r: 0.4, a0: 0.0, a1: 50.0, z: (0.5, 2.0), toward: 0.0, delta: 0.0, pivot_z: 0.5 };
            let p1 = Gen { r: 0.4, a0: 120.0, a1: 170.0, z: (0.47, 1.7), toward: 0.0, delta: 0.0, pivot_z: 0.5 };
            let p2 = Gen { r: 0.4, a0: 240.0, a1: 290.0, z: (0.44, 1.81), toward, delta, pivot_z: 1.0 };
            survey_gen(&format!("tilt {k}eps toward {toward} plate"), &[p0, p1, p2], true);
            survey_gen(&format!("tilt {k}eps toward {toward} alone"), &[p0, p1, p2], false);
        }
    }
}
