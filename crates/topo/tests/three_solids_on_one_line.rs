//! **Solids touching along one line build one body whatever order their
//! members fold in.**
//!
//! The plate `[0, 3] × [0, 2] × [0, 1]` with upright triangular prisms
//! over sectors of radius 0.4 about `(1.5, 1)`, z-ranges staggered. The
//! prisms touch pairwise along the vertical line through that point, so
//! once two of them are joined the line is held as two coincident edges
//! and the third meets an operand with two edges on one ray from its
//! vertex. The plate's top meets the line at [`MEET`], which every
//! footprint shares; when the prisms fold first, their coincident edges
//! pierce the top there once each.
//!
//! The rows, each in every member order:
//! - three prisms over 50° sectors, with the plate and alone;
//! - four prisms over 50° sectors;
//! - one prism's corner moved off the line, within the band (it touches
//!   the others' edges there) and outside it (a sliver apart).
//!
//! Each builds sound: its counts, tiers 3 and 3′, closed-form volume,
//! corners angularly disjoint, one vertex at [`MEET`] where the plate
//! is a member, the body of the first order compared by geometry, and
//! its material at probes about the line against the analytic union
//! of the members' closed forms.
//!
//! Three prisms over 60° sectors put one prism's lateral face in the
//! plane of another's, an undeclared flush continuation along the line,
//! and every order refuses it as one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::finished;
use common::meeting::{MEET, PLATE, at, corners_disjoint, orders, shape};
use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanBody, BooleanError, BooleanResult, union, validate_geometric,
    validate_pseudomanifold,
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

/// A fold's contact record is its last step's.
const DROPPED_RECORDS: &str = "work/wire/a-boolean-drops-its-operands-own-contact-records.md";

/// Whether a census witness `"(x, y, z)…"` stands within a micron of
/// the line.
fn on_the_line(witness: &str) -> bool {
    let Some(inside) = witness.strip_prefix('(').and_then(|w| w.split(')').next()) else {
        return false;
    };
    let n: Vec<f64> = inside.split(", ").filter_map(|x| x.parse().ok()).collect();
    matches!(n[..], [x, y, _] if (x - MEET[0]).hypot(y - MEET[1]) < 1e-6)
}

/// The refusal of an edge that crosses two wedges about a contact line.
const TWO_GERMS: &str = "an edge crosses two wedges about a contact line on one ray (unbuilt)";
const TWO_GERMS_ROW: &str = "work/tang/an-edge-crossing-two-wedges-about-a-contact-line-refuses.md";

/// No order is unbuilt.
fn none(_: &[usize]) -> bool {
    false
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

/// Folds `prisms` (and the plate, member 0, when `plate`) by union in
/// every order and asserts each result (module docs).
///
/// The 3′ verdict reads the last step's contact record alone, so it
/// refuses the contacts between earlier members that the last one's do
/// not cover, all on the line ([`DROPPED_RECORDS`]): `dropped[i]` of
/// them when member `i` folds last.
fn every_order(
    label: &str,
    prisms: &[Prism],
    plate: bool,
    counts: [usize; 3],
    dropped: &[usize],
    unbuilt: &dyn Fn(&[usize]) -> bool,
) {
    let mut members = Vec::new();
    if plate {
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        members.push(finished("the plate", p, t()));
    }
    members.extend(prisms.iter().map(Prism::body));
    let volume = union_volume(prisms, plate);
    // A corner moved off the line within the zero builds as on it: the
    // volume reads within the move's sweep, its offset times the
    // prism's height (under 2).
    let moved: f64 = prisms
        .iter()
        .map(|p| (p.apex.0 - MEET[0]).hypot(p.apex.1 - MEET[1]))
        .sum();
    let slack = 2.0f64.mul_add(moved, 1e-9);
    let holds = |q: [f64; 3]| (plate && in_plate(q)) || prisms.iter().any(|p| p.holds(q));
    let probes = probes();
    let c = at(Point3::new(MEET[0], MEET[1], MEET[2]));
    let mut first = None;
    for order in orders(members.len()) {
        let what = format!("{label}, member order {order:?}");
        let r = match fold(&members, &order) {
            Err((_, BooleanError::ClassificationInvariant { what: why }))
                if unbuilt(&order) && why == TWO_GERMS =>
            {
                continue;
            }
            Err((k, e)) => panic!("{what}: step {k} refused: {e:?}"),
            Ok(_) if unbuilt(&order) => panic!("{what}: built ({TWO_GERMS_ROW} may be fixed)"),
            Ok(r) => r,
        };
        let body = &r.body;
        let got = [
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
        ];
        assert_eq!(got, counts, "{what}: faces, edges, vertices");
        assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
        let on_line: Vec<_> = match validate_pseudomanifold(body, &r.contacts, t()) {
            Ok(()) => Vec::new(),
            Err(es) => es
                .iter()
                .map(|e| match e {
                    topo::ValidationError::UndeclaredContact { witness, .. }
                        if on_the_line(witness) =>
                    {
                        witness.clone()
                    }
                    other => panic!("{what}: tier 3′ refused off the line: {other:?}"),
                })
                .collect(),
        };
        let last = order[order.len() - 1];
        assert_eq!(
            on_line.len(),
            dropped[last],
            "{what}: tier 3′ refused by other than the records {DROPPED_RECORDS} drops: \
             {on_line:?}"
        );
        let v = topo::mass_properties(body, t()).unwrap().volume;
        assert!(
            (v - volume).abs() < slack,
            "{what}: volume {v}, closed form {volume}"
        );
        assert_eq!(corners_disjoint(body), Ok(()), "{what}: corners");
        if plate {
            let at_c = body
                .vertices()
                .filter(|&(k, _)| at(topo::readback::vertex_point(body, k).unwrap()) == c)
                .count();
            assert_eq!(at_c, 1, "{what}: vertices at MEET");
        }
        let mut read = 0;
        for &q in &probes {
            if let Some(got) = inside_of(body, q) {
                assert_eq!(got, holds(q), "{what}: material at {q:?}");
                read += 1;
            }
        }
        assert!(
            read * 100 >= 99 * probes.len(),
            "{what}: {read} of {} probes read",
            probes.len()
        );
        let s = shape(body);
        match &first {
            None => first = Some(s),
            Some(f) => assert!(*f == s, "{what}: a different body from the first order"),
        }
    }
}

/// **Three prisms touching along one line build one body in every
/// member order**, with the plate and without it: the third meets the
/// first two's coincident edges along the ray they hold.
#[test]
fn three_prisms_touching_along_one_line_build_one_body_in_every_member_order() {
    // Folded last, the plate leaves the three prisms' contacts to the
    // operands' records, and the 1.7-high prism the other two's stretch
    // past its top.
    let dropped = [6, 0, 2, 0];
    every_order(
        "three prisms and the plate",
        &three(),
        true,
        [18, 39, 24],
        &dropped,
        &none,
    );
    every_order(
        "three prisms",
        &three(),
        false,
        [15, 27, 18],
        &[2, 2, 0],
        &none,
    );
}

/// **Four prisms touching along one line build one body in every member
/// order.**
#[test]
fn four_prisms_touching_along_one_line_build_one_body_in_every_member_order() {
    every_order(
        "four prisms and the plate",
        &four(),
        true,
        [22, 48, 29],
        &[12, 0, 2, 0, 6],
        &none,
    );
}

/// **A third prism crossing a wedge about the contact line builds in
/// every member order**: over 30°–60° or −60°–20°, it crosses the
/// first prism's wedge and touches the second's edge along the line.
/// Crossing both wedges (over 30°–140°), it crosses each edge of the
/// line: folded last, its edge meets the line's two coincident edges,
/// two crossings on one ray, and refuses typed ([`TWO_GERMS_ROW`]);
/// every other order builds.
#[test]
fn a_third_prism_crossing_the_wedges_about_the_line_builds_or_refuses_typed() {
    let with = |a0, a1| {
        vec![
            Prism::on_line(0.0, 50.0, (0.5, 2.0)),
            Prism::on_line(120.0, 170.0, (0.47, 1.7)),
            Prism::on_line(a0, a1, (0.44, 1.81)),
        ]
    };
    let dropped = [2, 0, 0, 0];
    every_order(
        "one wedge crossed at 30°",
        &with(30.0, 60.0),
        true,
        [20, 47, 30],
        &dropped,
        &none,
    );
    every_order(
        "one wedge crossed at 20°",
        &with(-60.0, 20.0),
        true,
        [20, 47, 30],
        &dropped,
        &none,
    );
    every_order(
        "both wedges crossed",
        &with(30.0, 140.0),
        true,
        [22, 55, 36],
        &[0; 4],
        &|order| order[3] == 3,
    );
}

/// **A prism's corner moved off the line builds or escalates typed in
/// every member order**, the move a multiple of ε (the band's zero is
/// ε, its escalation 10ε): within the zero it touches the others along
/// the line, and builds as they do; inside the band (3ε, 5ε) every
/// order escalates, in the boolean or at a carrier's certification;
/// at 100ε the slivers the move leaves read inside the band,
/// and each order escalates or builds a body whose census escalates
/// there; at 10⁴ε, and no less than the micron the corner and shape
/// reads round to, it stands apart, and builds.
#[test]
fn a_prism_moved_off_the_line_builds_or_escalates_typed_in_every_member_order() {
    let eps = t().eps();
    for k in [1e-3, 0.1] {
        every_order(
            &format!("three prisms and the plate, one moved {k}ε"),
            &three_off(k * eps),
            true,
            [18, 39, 24],
            &[6, 0, 2, 0],
            &none,
        );
    }
    every_order(
        "three prisms and the plate, one moved apart",
        &three_off((1e4 * eps).max(1e-5)),
        true,
        [18, 42, 27],
        &[2, 0, 2, 0],
        &none,
    );
    for k in [3.0, 5.0, 100.0] {
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        let mut members = vec![finished("the plate", p, t())];
        members.extend(three_off(k * eps).iter().map(Prism::body));
        for order in orders(members.len()) {
            let what = format!("one prism moved {k}ε, member order {order:?}");
            match fold(&members, &order) {
                Err((_, e)) if escalates(&e) => {}
                Err((s, e)) => panic!("{what}: step {s} refused {e:?}"),
                Ok(r) => {
                    assert!(k > 5.0, "{what}: built inside the band");
                    let es = validate_pseudomanifold(&r.body, &r.contacts, t()).unwrap_err();
                    assert!(
                        es.iter()
                            .any(|e| matches!(e, topo::ValidationError::CensusEscalated { .. })),
                        "{what}: built, and its census does not escalate: {es:?}"
                    );
                }
            }
        }
    }
}

/// Whether `e` is an in-band escalation: the boolean's own, or a
/// carrier's certification at an Euler door it calls.
fn escalates(e: &BooleanError) -> bool {
    matches!(
        e,
        BooleanError::Escalated { .. }
            | BooleanError::Euler(topo::EulerOpError::Certification {
                error: geom_brep::CertifyError::Escalated { .. },
            })
    )
}

/// **Three prisms over 60° sectors refuse their flush continuation in
/// every member order**: each lateral face lies in the plane of
/// another's, touching it along the line, and nothing declares it.
#[test]
fn three_prisms_flush_along_one_line_refuse_undeclared_in_every_member_order() {
    let prisms = [
        Prism::on_line(0.0, 60.0, (0.5, 2.0)),
        Prism::on_line(120.0, 180.0, (0.47, 1.7)),
        Prism::on_line(240.0, 300.0, (0.44, 1.81)),
    ];
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(prisms.iter().map(Prism::body));
    for order in orders(members.len()) {
        match fold(&members, &order) {
            Err((_, BooleanError::UndeclaredCoincidence { .. })) => {}
            Err((k, e)) => panic!("60° sectors, member order {order:?}: step {k} refused {e:?}"),
            Ok(_) => panic!("60° sectors, member order {order:?}: built"),
        }
    }
}
