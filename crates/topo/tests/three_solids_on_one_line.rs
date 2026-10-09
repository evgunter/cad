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
//! - a third prism crossing one wedge about the line, or both;
//! - one prism tilted about the line within the zero;
//! - a prism whose side face holds the line;
//! - the review's hunt seeds whose pierces the weld leaves apart;
//! - one prism's corner moved off the line, within the zero, inside the
//!   band and past it.
//!
//! Each order that builds is sound: its counts, tiers 3 and 3′,
//! closed-form volume, corners angularly disjoint, one vertex at
//! [`MEET`] where the plate is a member, the body of the first order
//! compared by geometry, and its material at probes about the line
//! against the analytic union of the members' closed forms. Each that
//! refuses refuses typed, as its row names.
//!
//! Three prisms over 60° sectors put one prism's lateral face in the
//! plane of another's, an undeclared flush continuation along the line,
//! and every order refuses it as one. A declaration at the line, or the
//! steps' own records carried, never serves another body (D10).
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

/// An upright prism over the triangle `foot` (counterclockwise), over
/// `z`; `moved` is how far a corner was moved off the line, and `tilt`
/// the turn `(bearing, angle)` that tips its +z towards the bearing
/// (degrees) by the angle (radians) about the point on the line at the
/// plate's top.
#[derive(Clone, Copy)]
struct Prism {
    foot: [(f64, f64); 3],
    z: (f64, f64),
    moved: f64,
    tilt: (f64, f64),
}

/// The point at the bearing `a` (degrees) `r` from [`MEET`].
fn bearing(a: f64, r: f64) -> (f64, f64) {
    let (s, c) = f64::to_radians(a).sin_cos();
    (r.mul_add(c, MEET[0]), r.mul_add(s, MEET[1]))
}

impl Prism {
    /// Over the sector from `a0` to `a1` (degrees) of radius 0.4 about
    /// [`MEET`].
    fn on_line(a0: f64, a1: f64, z: (f64, f64)) -> Self {
        Self {
            foot: [(MEET[0], MEET[1]), bearing(a0, 0.4), bearing(a1, 0.4)],
            z,
            moved: 0.0,
            tilt: (0.0, 0.0),
        }
    }

    /// The tilt as a rigid motion, by `sign` times its angle.
    fn pose(&self, sign: f64) -> common::meeting::Pose {
        let (s, c) = self.tilt.0.to_radians().sin_cos();
        let axis = [-s, c, 0.0];
        let turn = |t| common::meeting::Pose::turn("tilt", axis, sign * self.tilt.1, t);
        let q = turn([0.0; 3]).at(MEET);
        turn([MEET[0] - q.x, MEET[1] - q.y, MEET[2] - q.z])
    }

    fn footprint(&self) -> [(f64, f64); 3] {
        self.foot
    }

    fn body(&self) -> AtRestBody<f64> {
        let pose = self.pose(1.0);
        let mut body = topo::Body::<f64>::new();
        common::prism_ops(
            &mut body,
            &self.footprint(),
            self.z,
            |x, y, z| pose.at([x, y, z]),
            common::FaceGeometry::Certified,
            t(),
        );
        common::describe_as_intersections(&mut body, t());
        finished("a prism on the line", body, t())
    }

    /// **The analytic oracle**: whether `q` is inside the prism, read
    /// off its closed form rather than any body.
    fn holds(&self, q: [f64; 3]) -> bool {
        let q = self.pose(-1.0).at(q);
        let q = [q.x, q.y, q.z];
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
    p[1].foot[0].0 += d;
    p[1].moved = d;
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

/// The refusal of an edge that crosses two wedges about a contact line
/// (`work/tang/an-edge-crossing-two-wedges-about-a-contact-line-refuses.md`).
const TWO_GERMS: &str = "an edge crosses two wedges about a contact line on one ray (unbuilt)";

/// No order refuses.
fn none(_: &[usize]) -> Option<&'static str> {
    None
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
/// Where a prism is tilted within the zero, the 3′ census may say it
/// cannot trace the touch at this tolerance, typed, in place of a
/// verdict.
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
    refuses: &dyn Fn(&[usize]) -> Option<&'static str>,
) {
    let mut members = Vec::new();
    if plate {
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        members.push(finished("the plate", p, t()));
    }
    members.extend(prisms.iter().map(Prism::body));
    let volume = union_volume(prisms, plate);
    // A corner moved off the line, or a prism tilted, within the zero
    // builds as on the line: the volume reads within the move's sweep,
    // a corner's offset times the prism's height (under 2), and a tilt's
    // angle times the prism's surface (under 2.5) times its farthest
    // point's distance from the pivot (under 1.6).
    let moved: f64 = prisms
        .iter()
        .map(|p| 2.0f64.mul_add(p.moved, 4.0 * p.tilt.1))
        .sum();
    let slack = moved + 1e-9;
    let tilted = prisms.iter().any(|p| p.tilt.1 != 0.0);
    let holds = |q: [f64; 3]| (plate && in_plate(q)) || prisms.iter().any(|p| p.holds(q));
    let probes = probes();
    let c = at(Point3::new(MEET[0], MEET[1], MEET[2]));
    let mut first = None;
    for order in orders(members.len()) {
        let what = format!("{label}, member order {order:?}");
        let r = match (fold(&members, &order), refuses(&order)) {
            (Err((_, e)), Some(want)) if format!("{e:?}").contains(want) => continue,
            (Err((k, e)), _) => panic!("{what}: step {k} refused: {e:?}"),
            (Ok(_), Some(want)) => panic!("{what}: built where it refuses {want}"),
            (Ok(r), None) => r,
        };
        let body = &r.body;
        let got = [
            body.faces().count(),
            body.edges().count(),
            body.vertices().count(),
        ];
        assert_eq!(got, counts, "{what}: faces, edges, vertices");
        assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
        let verdict = validate_pseudomanifold(body, &r.contacts, t()).err();
        let undecided = verdict.iter().flatten().any(|e| {
            matches!(
                e,
                topo::ValidationError::CensusUndecidable { .. }
                    | topo::ValidationError::CensusEscalated { .. }
            )
        });
        if undecided {
            // A tilt within the zero leaves the census faces too nearly
            // in line to trace; it says so, typed.
            assert!(tilted, "{what}: tier 3′ undecided untilted: {verdict:?}");
        }
        let on_line: Vec<_> = verdict
            .iter()
            .flatten()
            .filter(|_| !undecided)
            .map(|e| match e {
                topo::ValidationError::UndeclaredContact { witness, .. }
                    if on_the_line(witness) =>
                {
                    witness.clone()
                }
                other => panic!("{what}: tier 3′ refused off the line: {other:?}"),
            })
            .collect();
        let last = order[order.len() - 1];
        assert!(
            undecided || on_line.len() == dropped[last],
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
/// two crossings on one ray, and refuses typed ([`TWO_GERMS`]);
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
        &|order| (order[3] == 3).then_some(TWO_GERMS),
    );
}

/// **A prism tilted within the zero about the line builds as the
/// upright fixture does in every member order**: the 240° prism turned
/// by 0.05ε or 0.2ε (radians, so its edge leaves the line by at most
/// 0.2ε over its unit reach) towards 25°, 85° or 265° about the point on
/// the line at the plate's top, with the plate and alone. The census of
/// some prisms-alone orders cannot trace the touch at this tolerance and
/// says so, typed.
#[test]
fn a_prism_tilted_within_the_zero_builds_as_upright() {
    let eps = t().eps();
    for k in [0.05, 0.2] {
        for toward in [25.0, 85.0, 265.0] {
            let mut prisms = three();
            prisms[2].tilt = (toward, k * eps);
            let label = format!("the 240° prism tilted {k}ε towards {toward}°");
            every_order(
                &format!("{label}, and the plate"),
                &prisms,
                true,
                [18, 39, 24],
                &[6, 0, 2, 0],
                &none,
            );
            every_order(&label, &prisms, false, [15, 27, 18], &[2, 2, 0], &none);
        }
    }
}

/// A contact line whose ray holds a subdivision bisector of the other
/// solid.
const ON_A_BISECTOR: &str = "a contact line's ray holds a subdivision bisector (unbuilt)";

/// **A prism whose side face holds the line builds or refuses typed in
/// every member order**: the plate, the prisms over 0°–50° and
/// 120°–170°, and a third over the triangle from 0.3 at 80° and 260° to
/// 0.4 at 350°, its side through the line in no other face's plane.
/// Folded last onto the plate, the third and the 120° prism, the 0°
/// prism meets a contact line whose ray holds the third's subdivision
/// bisector, and refuses typed (unbuilt), in four orders. Eight refuse
/// `ResultInvalid { RingMeetsRing }` at the step that adds the plate to
/// a body holding the third and the 120° prism
/// (`work/tang/an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring.md`).
/// The other twelve build. In three of those orders a step reads the
/// line's foot touching the top beside a partner whose link runs along
/// it (`vtxfac::partner_side`): in [3, 2, 1, 0] and [2, 3, 1, 0] the
/// second, which builds sound, the fold refusing at the third; in
/// [2, 0, 3, 1] the third, which refuses at the bisector.
#[test]
fn a_prism_whose_face_holds_the_line_builds_or_refuses_typed() {
    let face = Prism {
        foot: [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
        z: (0.44, 1.81),
        moved: 0.0,
        tilt: (0.0, 0.0),
    };
    let prisms = [
        Prism::on_line(0.0, 50.0, (0.5, 2.0)),
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
        face,
    ];
    let refuses = |order: &[usize]| match order {
        [3, 0, 2, 1] | [0, 3, 2, 1] | [0, 2, 3, 1] | [2, 0, 3, 1] => Some(ON_A_BISECTOR),
        [3, 2, 1, 0]
        | [2, 3, 1, 0]
        | [2, 1, 3, 0]
        | [3, 1, 2, 0]
        | [1, 3, 2, 0]
        | [1, 2, 3, 0]
        | [3, 2, 0, 1]
        | [2, 3, 0, 1] => Some("RingMeetsRing"),
        _ => None,
    };
    // The orders whose pair beside a partner along the top is read, the
    // step each refuses at, and the body before it.
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(prisms.iter().map(Prism::body));
    let probes = probes();
    for (order, refused, want) in [
        ([3, 2, 1, 0], 3, "RingMeetsRing"),
        ([2, 3, 1, 0], 3, "RingMeetsRing"),
        ([2, 0, 3, 1], 3, ON_A_BISECTOR),
    ] {
        let step = refused - 1;
        let what = format!("a face through the line, member order {order:?}");
        match fold(&members, &order) {
            Err((k, e)) => assert!(
                k == refused && format!("{e:?}").contains(want),
                "{what}: refuses {want} at step {refused}, got step {k}: {e:?}"
            ),
            Ok(_) => panic!("{what}: built where it refuses {want}"),
        }
        let r = fold(&members, &order[..=step])
            .unwrap_or_else(|(k, e)| panic!("{what}: step {k} refused: {e:?}"));
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        let plate = order[..=step].contains(&0);
        let held: Vec<Prism> = order[..=step]
            .iter()
            .filter(|&&i| i > 0)
            .map(|&i| prisms[i - 1])
            .collect();
        let v = topo::mass_properties(&r.body, t()).unwrap().volume;
        let want_v = union_volume(&held, plate);
        assert!(
            (v - want_v).abs() < 1e-9,
            "{what}: step {step}, volume {v}, closed form {want_v}"
        );
        let holds = |q| (plate && in_plate(q)) || held.iter().any(|p| p.holds(q));
        let mut read = 0;
        for &q in &probes {
            if let Some(got) = inside_of(&r.body, q) {
                assert_eq!(got, holds(q), "{what}: step {step}, material at {q:?}");
                read += 1;
            }
        }
        assert!(
            read * 100 >= 99 * probes.len(),
            "{what}: step {step}, {read} probes read"
        );
    }
    every_order(
        "a face through the line",
        &prisms,
        true,
        [20, 47, 30],
        &[0, 0, 0, 2],
        &refuses,
    );
}

/// **A prism's corner moved off the line builds or escalates typed in
/// every member order**, the move a multiple of ε (the band's zero is
/// ε, its escalation 10ε): within the zero it touches the others along
/// the line, and builds as they do; inside the band (3ε, 5ε) every
/// order escalates, in the boolean or at a carrier's certification;
/// at 100ε the slivers the move leaves read inside the band,
/// and each order escalates or builds a body whose census escalates
/// there, sound by tier 3, volume and the oracle; at 10⁴ε, and no less than the micron the corner and shape
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
    let probes = probes();
    for k in [3.0, 5.0, 100.0] {
        let prisms = three_off(k * eps);
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        let mut members = vec![finished("the plate", p, t())];
        members.extend(prisms.iter().map(Prism::body));
        let volume = union_volume(&prisms, true);
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
                    assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
                    let v = topo::mass_properties(&r.body, t()).unwrap().volume;
                    assert!(
                        (v - volume).abs() < 2.0f64.mul_add(k * eps, 1e-9),
                        "{what}: volume {v}, closed form {volume}"
                    );
                    let holds = |q| in_plate(q) || prisms.iter().any(|p| p.holds(q));
                    let mut read = 0;
                    for &q in &probes {
                        if let Some(got) = inside_of(&r.body, q) {
                            assert_eq!(got, holds(q), "{what}: material at {q:?}");
                            read += 1;
                        }
                    }
                    assert!(
                        read * 100 >= 99 * probes.len(),
                        "{what}: {read} probes read"
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

/// The faces of `body` at a vertex on the line.
fn faces_on_the_line(body: &AtRestBody<f64>) -> Vec<topo::FaceKey> {
    let mut out = Vec::new();
    for (v, _) in body.vertices() {
        let p = topo::readback::vertex_point(body, v).unwrap();
        if (p.x - MEET[0]).hypot(p.y - MEET[1]) < 1e-9 {
            for f in body.faces_of_vertex(v).unwrap() {
                if !out.contains(&f) {
                    out.push(f);
                }
            }
        }
    }
    out
}

/// The left fold of `members` in `order` from `body`, already the fold
/// of its first `from` members, or the step that refused and why.
fn fold_from(
    members: &[AtRestBody<f64>],
    order: &[usize],
    from: usize,
    mut body: AtRestBody<f64>,
) -> Result<AtRestBody<f64>, (usize, BooleanError)> {
    for (k, &i) in order.iter().enumerate().skip(from) {
        match union(&body, &members[i], t()) {
            Ok(BooleanResult::Body(r)) => body = r.body,
            Ok(BooleanResult::Empty) => panic!("step {k} of {order:?} is empty"),
            Err(e) => return Err((k, e)),
        }
    }
    Ok(body)
}

/// **A declaration changes no verdict the contact line's route serves
/// (D10)**: every step of every member order of the 50° fixture, with
/// each coincidence class declared on each pair of faces at the line,
/// one pair at a time and the other steps undeclared, builds the
/// undeclared fold's body or refuses typed. Every one refuses at the
/// declaration door: no two faces there share a carrier, and a plane
/// pair takes neither `Tangent` nor `Seam`.
#[test]
fn a_declaration_at_the_contact_line_serves_the_undeclared_body_or_refuses() {
    use topo::{BooleanCoincidence, BooleanDeclarations, FacePairDeclaration, union_with};
    let classes = [
        BooleanCoincidence::REST,
        BooleanCoincidence::TANGENT,
        BooleanCoincidence::Continuation,
        BooleanCoincidence::Seam,
    ];
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(three().iter().map(Prism::body));
    let (mut served, mut refused) = (0, 0);
    for order in orders(members.len()) {
        let undeclared = shape(&fold(&members, &order).unwrap().body);
        let mut acc = members[order[0]].clone();
        for k in 1..order.len() {
            let what = format!("member order {order:?}, step {k}");
            let b = &members[order[k]];
            for fa in faces_on_the_line(&acc) {
                for fb in faces_on_the_line(b) {
                    for class in classes {
                        let decls = BooleanDeclarations {
                            coincident_faces: vec![FacePairDeclaration::new(fa, fb, class)],
                            ..BooleanDeclarations::none()
                        };
                        match union_with(&acc, b, &decls, t()) {
                            Ok(BooleanResult::Body(r)) => {
                                let rest = fold_from(&members, &order, k + 1, r.body)
                                    .unwrap_or_else(|(s, e)| {
                                        panic!(
                                            "{what}: {class:?} on {fa:?} × {fb:?}, step {s}: {e:?}"
                                        )
                                    });
                                assert!(
                                    shape(&rest) == undeclared,
                                    "{what}: {class:?} on {fa:?} × {fb:?} serves another body"
                                );
                                served += 1;
                            }
                            Ok(BooleanResult::Empty) => {
                                panic!("{what}: {class:?} on {fa:?} × {fb:?} serves empty")
                            }
                            Err(
                                BooleanError::ContactContradicted { .. }
                                | BooleanError::ContinuationContradicted { .. }
                                | BooleanError::UnsupportedDeclarationClass { .. },
                            ) => refused += 1,
                            Err(e) => {
                                panic!("{what}: {class:?} on {fa:?} × {fb:?} refused {e:?}")
                            }
                        }
                    }
                }
            }
            acc = fold_from(&members, &order[..=k], k, acc)
                .unwrap_or_else(|(s, e)| panic!("{what}: undeclared step {s}: {e:?}"));
        }
    }
    // Measured: the door refuses all 4320, so no declaration reaches the
    // contact line's route on this fixture.
    assert_eq!((served, refused), (0, 4320), "served, refused at the door");
}

/// **Carrying each step's own contact records serves the same body
/// (D10)**: every member order of the 50° fixture, of four prisms, and
/// of the 50° fixture with a corner moved 0.1ε, each step declaring the
/// records the step before it returned (its vertex pairs and rests as
/// `Rest`, then as `Tangent`; its vertex-edge and edge-edge contacts),
/// builds the undeclared fold's body.
#[test]
fn carrying_each_steps_records_serves_the_same_body() {
    use topo::{
        BooleanDeclarations, CarriedContacts, CarriedVf, CarriedVv, ContactClass, union_with,
    };
    let eps = t().eps();
    for (label, prisms) in [
        ("three", three()),
        ("four", four()),
        ("three, one moved 0.1ε", three_off(0.1 * eps)),
    ] {
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        let mut members = vec![finished("the plate", p, t())];
        members.extend(prisms.iter().map(Prism::body));
        for class in [ContactClass::Rest, ContactClass::Tangent] {
            for order in orders(members.len()) {
                let what = format!("{label}, {class:?}, member order {order:?}");
                let undeclared = shape(&fold(&members, &order).unwrap().body);
                let mut body = members[order[0]].clone();
                let mut carried = CarriedContacts::default();
                for (k, &i) in order.iter().enumerate().skip(1) {
                    let decls = BooleanDeclarations {
                        carried_a: carried.clone(),
                        ..BooleanDeclarations::none()
                    };
                    let r = match union_with(&body, &members[i], &decls, t()) {
                        Ok(BooleanResult::Body(r)) => r,
                        other => panic!("{what}: step {k}: {other:?}"),
                    };
                    let c = &r.contacts;
                    carried = CarriedContacts {
                        vv: c.vv.iter().map(|&pair| CarriedVv { pair, class }).collect(),
                        vf: c
                            .a_on_b
                            .iter()
                            .chain(&c.b_on_a)
                            .map(|&rest| CarriedVf { rest, class })
                            .collect(),
                        ve: c.ve.clone(),
                        ee: c.ee.clone(),
                    };
                    body = r.body;
                }
                assert!(shape(&body) == undeclared, "{what}: another body");
            }
        }
    }
}

/// The prisms of the review's random hunt at `seed` (overlapping
/// sectors): `k` prisms through the plate's top at [`MEET`], each over
/// a sector of its own radius from [`MEET`], half of them leant about
/// it; a pinned generator, so the fixture is the seed.
fn hunt(seed: u64, k: usize) -> Vec<Prism> {
    let mut s = seed
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    let mut rnd = move || {
        s = s
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    let _ = rnd();
    (0..k)
        .map(|_| {
            let w = 100.0f64.mul_add(rnd(), 25.0);
            let start = rnd() * 360.0;
            let lean = rnd() < 0.5;
            let toward = if rnd() < 0.7 {
                start + w / 2.0
            } else {
                rnd() * 360.0
            };
            let r = 0.3f64.mul_add(rnd(), 0.25);
            let z = (0.5f64.mul_add(rnd(), 0.3), 0.6f64.mul_add(rnd(), 1.4));
            let delta = if lean {
                0.3f64.mul_add(rnd(), 0.05)
            } else {
                0.0
            };
            Prism {
                foot: [(MEET[0], MEET[1]), bearing(start, r), bearing(start + w, r)],
                z,
                moved: 0.0,
                tilt: (toward, delta),
            }
        })
        .collect()
}

/// **Pierces a weld leaves apart build one body in every member order**:
/// the review's hunt seeds 506 (three prisms) and 507 (four), overlapping
/// sectors through the plate's top at [`MEET`], some upright on the line
/// and some leant about it. Folded first, their edges pierce the top at
/// [`MEET`], and a pierce the weld has joined meets another copy of a
/// pierce it already took, whose corners none of the joined vertex's
/// hold: the pair stays apart. Every order builds sound by tier 3,
/// corners, a probe oracle about [`MEET`], a Monte Carlo volume, and one
/// body.
#[test]
fn pierces_a_weld_leaves_apart_build_one_body_in_every_member_order() {
    for (seed, k) in [(506, 3), (507, 4)] {
        let prisms = hunt(seed, k);
        let mut members = vec![finished(
            "the plate",
            common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
            t(),
        )];
        members.extend(prisms.iter().map(Prism::body));
        let holds = |q: [f64; 3]| in_plate(q) || prisms.iter().any(|p| p.holds(q));
        let mut probes = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                for l in 0..9 {
                    let f = |n: i32, o: f64| 0.031f64.mul_add(f64::from(n) - 4.0, o);
                    probes.push([
                        MEET[0] + f(i, 0.0037),
                        MEET[1] + f(j, 0.0019),
                        MEET[2] + f(l, 0.0023),
                    ]);
                }
            }
        }
        // The prisms' volume above the top by Monte Carlo over a box
        // holding them, its error bounded at four standard deviations.
        let (volume, slack) = {
            let mut s = seed ^ 0x9e37_79b9_7f4a_7c15;
            let mut rnd = move || {
                s = s
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                (s >> 11) as f64 / (1u64 << 53) as f64
            };
            let n = 400_000;
            let lo: [f64; 3] = [0.6, 0.1, 1.0];
            let hi: [f64; 3] = [2.4, 1.9, 2.4];
            let hits = (0..n)
                .filter(|_| {
                    let q = [0, 1, 2].map(|i| (hi[i] - lo[i]).mul_add(rnd(), lo[i]));
                    prisms.iter().any(|p| p.holds(q))
                })
                .count();
            let cube = (0..3).map(|i| hi[i] - lo[i]).product::<f64>();
            let p = hits as f64 / f64::from(n);
            (
                cube.mul_add(p, 6.0),
                4.0 * cube * (p * (1.0 - p) / f64::from(n)).sqrt(),
            )
        };
        let mut first = None;
        for order in orders(members.len()) {
            let what = format!("hunt seed {seed}, member order {order:?}");
            let r = fold(&members, &order)
                .unwrap_or_else(|(s, e)| panic!("{what}: step {s} refused: {e:?}"));
            let body = &r.body;
            assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
            assert_eq!(corners_disjoint(body), Ok(()), "{what}: corners");
            let v = topo::mass_properties(body, t()).unwrap().volume;
            assert!(
                (v - volume).abs() < slack,
                "{what}: volume {v}, Monte Carlo {volume} ± {slack}"
            );
            let mut read = 0;
            for &q in &probes {
                if let Some(got) = inside_of(body, q) {
                    assert_eq!(got, holds(q), "{what}: material at {q:?}");
                    read += 1;
                }
            }
            assert!(
                read * 100 >= 99 * probes.len(),
                "{what}: {read} probes read"
            );
            let s = shape(body);
            match &first {
                None => first = Some(s),
                Some(f) => assert!(*f == s, "{what}: a different body from the first order"),
            }
        }
    }
}
