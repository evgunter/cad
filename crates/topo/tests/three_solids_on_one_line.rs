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
//! - the third crossing both against the other members joined, in
//!   every op and either operand slot, and the recourse its refusal
//!   names;
//! - one prism tilted about the line within the zero;
//! - a prism whose side face holds the line;
//! - the review's hunt seeds whose pierces meet at one point of the top;
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
//! and every order refuses it as one. The steps' own records carried
//! never serve another body (D10).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::finished;
use common::meeting::{MEET, PLATE, at, corners_disjoint, orders, shape};
use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, BooleanBody, BooleanError, BooleanErrorKind, BooleanResult, union,
    validate_geometric, validate_pseudomanifold,
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
const TWO_GERMS: BooleanErrorKind = BooleanErrorKind::CrossingsAlongAContactLine;

/// No order refuses.
fn none(_: &[usize]) -> Option<BooleanErrorKind> {
    None
}

/// The left fold of `members` by union in `order`, or the step that
/// refused and why. A kernel op called without `BooleanInputs` records
/// carries none of its operands' records, so the fold's contact record
/// is its last step's.
fn fold(
    members: &[AtRestBody<f64>],
    order: &[usize],
) -> Result<BooleanBody<f64>, (usize, Box<BooleanError>)> {
    let mut body = members[order[0]].clone();
    let mut last = None;
    for (k, &i) in order.iter().enumerate().skip(1) {
        match union(&body, &members[i], t()) {
            Ok(BooleanResult::Body(r)) => {
                body = r.body.clone();
                last = Some(r);
            }
            Ok(BooleanResult::Empty) => panic!("step {k} of {order:?} is empty"),
            Err(e) => return Err((k, Box::new(e))),
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

/// A Boolean op.
type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

/// The material an op makes of its operands' (A's, then B's).
type Material = fn(bool, bool) -> bool;

/// The three ops, each with its material.
const OPS: [(&str, Op, Material); 3] = [
    ("union", union, |a, b| a || b),
    ("intersection", topo::intersect, |a, b| a && b),
    ("subtraction", topo::subtract, |a, b| a && !b),
];

/// **A body sound against a closed form**: tier 3, and its material at
/// the probes against `holds`, at least 99% of them read.
fn sound(what: &str, body: &AtRestBody<f64>, holds: &dyn Fn([f64; 3]) -> bool) {
    assert_eq!(validate_geometric(body, t()), Ok(()), "{what}: tier 3");
    let probes = probes();
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
}

/// An operand of [`every_op`] in closed form: its body, its material and
/// its volume.
struct Closed<'a> {
    body: &'a AtRestBody<f64>,
    holds: &'a dyn Fn([f64; 3]) -> bool,
    volume: f64,
}

/// The cells of [`every_op`] that refuse `EdgeInFace`, by the cell's
/// name: each leaves an edge lying in a face whose material parts it,
/// and the parting is not built there.
///
/// - The line body less the 0° prism against the bisector, every order:
///   the prism's edge lies in the third's face along part of its length,
///   crossing the face's boundary at its corner on the plate's top, and
///   runs along the line body's own coincident edges there, whose
///   edge-edge records the division does not place (`Unplaced`;
///   `work/cleave/an-edge-running-past-a-face-along-coincident-edges-is-not-divided.md`).
///   At its midpoint, from the face's own direction round into it, the
///   material is solid to 30°, empty to 80°, solid to 180°, empty to
///   270°, solid to 300° and empty beyond.
/// - The line body towards 350° less the prisms over 0°–50° and
///   280°–330°: the prism's edge lies in the wedge's face, whose material
///   divides it, and an end touches an edge there (`DividedEndTouched`;
///   `work/cleave/the-boolean-parting-refuses-its-rarer-configurations.md`).
///
/// Main built each as one edge, its wedge claiming the empty sector
/// between the face's material and the prism's, which the result gate
/// refused `LineWedges` (their overlapping wedges).
const UNPARTED: &[&str] = &[
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [0, 1, 2], the line body first: true",
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [0, 2, 1], the line body first: true",
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [1, 0, 2], the line body first: true",
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [1, 2, 0], the line body first: true",
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [2, 0, 1], the line body first: true",
    "subtraction of the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body [2, 1, 0], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [0, 1, 2], the prism over [(1.5, 1.0), (1.569459271066772, 0.6060768987951167), (1.8464101615137753, 0.7999999999999998)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [0, 1, 2], the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [1, 0, 2], the prism over [(1.5, 1.0), (1.569459271066772, 0.6060768987951167), (1.8464101615137753, 0.7999999999999998)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [1, 0, 2], the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [1, 2, 0], the prism over [(1.5, 1.0), (1.569459271066772, 0.6060768987951167), (1.8464101615137753, 0.7999999999999998)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [1, 2, 0], the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [2, 1, 0], the prism over [(1.5, 1.0), (1.569459271066772, 0.6060768987951167), (1.8464101615137753, 0.7999999999999998)], the line body first: true",
    "subtraction of the wedge towards 350°, the line body [2, 1, 0], the prism over [(1.5, 1.0), (1.9, 1.0), (1.7571150438746157, 1.3064177772475911)], the line body first: true",
];

/// Whether `got` is the refusal [`UNPARTED`] names for the cell; a cell
/// it names that does not refuse so is a pin gone stale.
fn pinned(what: &str, got: &Result<BooleanResult<f64>, topo::BooleanError>) -> bool {
    let unparted = matches!(got, Err(topo::BooleanError::EdgeInFace { .. }));
    let pinned = UNPARTED.contains(&what);
    assert!(
        unparted || !pinned,
        "{what}: {got:?}, where it refuses `EdgeInFace`"
    );
    unparted && pinned
}

/// **Every op of `line` and `tool`, either operand first**: each builds
/// a body [`sound`] against the two closed forms, its volume theirs and
/// `common`'s, their common part's; or refuses the kind `refuses` names
/// for the op and whether `line` is the first operand.
fn every_op(
    what: &str,
    line: &Closed,
    tool: &Closed,
    common: f64,
    refuses: &dyn Fn(&str, bool) -> Option<BooleanErrorKind>,
) {
    for (name, op, material) in OPS {
        for line_first in [true, false] {
            let what = format!("{name} of {what}, the line body first: {line_first}");
            let (x, y) = if line_first {
                (line, tool)
            } else {
                (tool, line)
            };
            let got = op(x.body, y.body, t());
            if pinned(&what, &got) {
                continue;
            }
            let body = match (got, refuses(name, line_first)) {
                (Err(e), Some(want)) => {
                    assert_eq!(e.kind(), want, "{what}: {e:?}");
                    continue;
                }
                (Ok(BooleanResult::Body(r)), None) => r.body,
                (other, want) => panic!("{what}: {other:?}, where it refuses {want:?}"),
            };
            sound(&what, &body, &|q| material((x.holds)(q), (y.holds)(q)));
            let volume = match name {
                "union" => x.volume + y.volume - common,
                "intersection" => common,
                _ => x.volume - common,
            };
            let v = topo::test_support::pinned_volume("the measured body", &body, t()).volume;
            assert!(
                (v - volume).abs() < 1e-9,
                "{what}: volume {v}, closed form {volume}"
            );
        }
    }
}

/// Folds `prisms` (and the plate, member 0, when `plate`) by union in
/// every order and asserts each result (module docs).
///
/// Where a prism is tilted within the zero, the 3′ census may say it
/// cannot trace the touch at this tolerance, typed, in place of a
/// verdict.
///
/// The 3′ verdict reads the last step's contact record alone
/// ([`fold`]), so it refuses the contacts between earlier members that
/// the last one's do not cover, all on the line: `dropped[i]` of them
/// when member `i` folds last.
fn every_order(
    label: &str,
    prisms: &[Prism],
    plate: bool,
    counts: [usize; 3],
    dropped: &[usize],
    refuses: &dyn Fn(&[usize]) -> Option<BooleanErrorKind>,
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
            (Err((_, e)), Some(want)) if e.kind() == want => continue,
            (Err((k, e)), _) => panic!("{what}: step {k} refused: {e:?}"),
            (Ok(_), Some(want)) => panic!("{what}: built where it refuses {want:?}"),
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
                topo::ValidationError::UnrecordedContact { witness, .. }
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
            "{what}: tier 3′ refused by other than the records the uncarried fold drops: \
             {on_line:?}"
        );
        let v = topo::test_support::pinned_volume("the measured body", body, t()).volume;
        assert!(
            (v - volume).abs() < slack,
            "{what}: volume {v}, closed form {volume}"
        );
        assert_eq!(corners_disjoint(body, t()), Ok(()), "{what}: corners");
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
    // past its top; either then drops the edge-edge row of two of its
    // operand's edges along the line, which no step but the one that
    // joined them reads.
    let dropped = [6, 1, 2, 1];
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
        &[2, 2, 1],
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
        &[12, 2, 4, 2, 6],
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

/// **An edge crossing both wedges about a contact line refuses typed in
/// every op, and both arms of the recourse it names build**: the plate
/// and the two prisms touching along the line, folded in each order,
/// against the bridging prism (both wedges crossed), in either operand
/// slot, refuse [`TWO_GERMS`] by union, intersection and subtraction,
/// naming the line's vertex at the plate's top or the 120° prism's top
/// corner. Applying the intersection or subtraction to each piece
/// before the pieces are joined builds sound against the analytic
/// oracle (the union's recourse orders are the row above); so does
/// every op, order and slot with the bridge's corner moved 1e-2 off the
/// line along x or y, clear of the input tolerance.
#[test]
fn an_edge_crossing_both_wedges_refuses_typed_in_every_op_and_its_recourse_builds() {
    let prisms = [
        Prism::on_line(0.0, 50.0, (0.5, 2.0)),
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
        Prism::on_line(30.0, 140.0, (0.44, 1.81)),
    ];
    let mut members = vec![finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    )];
    members.extend(prisms.iter().map(Prism::body));
    let bridge = &members[3];
    let ends = [1.0, 1.7].map(|z| at(Point3::new(MEET[0], MEET[1], z)));
    let lines: Vec<_> = orders(3)
        .into_iter()
        .map(|order| (fold(&members, &order).unwrap().body, order))
        .collect();
    for (line, order) in &lines {
        for (name, op, _) in OPS {
            for line_is_a in [true, false] {
                let what = format!("{name} of the line from {order:?}, the line as A: {line_is_a}");
                let r = if line_is_a {
                    op(line, bridge, t())
                } else {
                    op(bridge, line, t())
                };
                let e = r.map(|_| ()).expect_err(&what);
                let BooleanError::CrossingsAlongAContactLine { a_vertex, b_vertex } = e else {
                    panic!("{what}: {e:?}");
                };
                let v = if line_is_a { a_vertex } else { b_vertex };
                let p = topo::readback::vertex_point(line, v)
                    .unwrap_or_else(|e| panic!("{what}: the line's vertex {v:?}: {e:?}"));
                assert!(ends.contains(&at(p)), "{what}: the line's vertex at {p:?}");
            }
        }
    }
    let in_line = |q| in_plate(q) || prisms[0].holds(q) || prisms[1].holds(q);
    let piece = |what: &str, body: Result<BooleanResult<f64>, BooleanError>| match body {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {:?}", other.map(|_| ())),
    };
    for (name, op, material) in &OPS[1..] {
        for (joined, other) in [([0, 1], 2), ([0, 2], 1)] {
            let what = format!("{name} of {joined:?} and of {other} with the bridge, joined");
            let base = fold(&members, &joined).unwrap().body;
            let x = piece(&what, op(&base, bridge, t()));
            let y = piece(&what, op(&members[other], bridge, t()));
            let body = piece(&what, union(&x, &y, t()));
            sound(&what, &body, &|q| material(in_line(q), prisms[2].holds(q)));
        }
    }
    let line_volume = union_volume(&prisms[..2], true);
    // The move leaves a sliver whose side reading is second order in
    // it (about d²), so d = 1e-2 reads about 1e-4, past
    // K·ε at the coarsest tolerance row.
    for (dx, dy) in [(1e-2, 0.0), (0.0, 1e-2)] {
        let mut moved = prisms[2];
        moved.foot[0].0 += dx;
        moved.foot[0].1 += dy;
        let off = moved.body();
        let solo = area(&moved.footprint()) * (moved.z.1 - moved.z.0);
        let common = line_volume + solo - union_volume(&[prisms[0], prisms[1], moved], true);
        let tool = Closed {
            body: &off,
            holds: &|q| moved.holds(q),
            volume: solo,
        };
        for (line_body, order) in &lines {
            let line = Closed {
                body: line_body,
                holds: &in_line,
                volume: line_volume,
            };
            every_op(
                &format!("the line from {order:?} and the bridge moved ({dx}, {dy})"),
                &line,
                &tool,
                common,
                &|_, _| None,
            );
        }
    }
}

/// **The line body less the bridge moved 1e-2 along x passes tier 3′
/// with its members' records carried, in every member order.** The
/// bridge's edge lies in the 0° prism's face along `y = 1` from the
/// plate's top to its own top corner and runs on into the plate below:
/// the material parts it there into two coincident edges, divided where
/// it crosses the face's bottom edge, the crossing one vertex
/// (`boolean::slit`). The bridge's own records are none.
#[test]
fn the_line_less_the_moved_bridge_passes_three_prime_with_its_records() {
    let op = |kind, x: &AtRestBody<f64>, y: &AtRestBody<f64>, records_a| {
        let inputs = topo::BooleanInputs {
            records_a,
            ..topo::BooleanInputs::default()
        };
        match topo::boolean_op_with(kind, x, y, &inputs, topo::SweepStrategy::Realized, t()) {
            Ok(BooleanResult::Body(r)) => r,
            other => panic!("{kind:?}: {:?}", other.map(|_| ())),
        }
    };
    let mut bridge = Prism::on_line(30.0, 140.0, (0.44, 1.81));
    bridge.foot[0].0 += 1e-2;
    let bridge = bridge.body();
    let members = [
        finished(
            "the plate",
            common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
            t(),
        ),
        Prism::on_line(0.0, 50.0, (0.5, 2.0)).body(),
        Prism::on_line(120.0, 170.0, (0.47, 1.7)).body(),
    ];
    for order in orders(3) {
        let mut line = members[order[0]].clone();
        let mut records = topo::OperandRecords::default();
        for &i in &order[1..] {
            let r = op(topo::BooleanOp::Union, &line, &members[i], records);
            records = r.contacts.carried();
            line = r.body;
        }
        let r = op(topo::BooleanOp::Subtract, &line, &bridge, records);
        let what = format!("the line from {order:?} less the moved bridge");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, t()).err(),
            None,
            "{what}: tier 3′"
        );
        // The bridge's line, x = 1.51 on y = 1: two edges from the face's
        // bottom edge (z = 1) to the bridge's top corner, one below, and
        // one vertex at the crossing.
        let (spans, vertices) = topo::test_support::along_line(
            &r.body,
            Point3::new(1.51, 1.0, 0.0),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
        );
        assert_eq!(
            spans,
            [(0.44, 1.0), (1.0, 1.81), (1.0, 1.81)],
            "{what}: the edges along the bridge's line"
        );
        assert_eq!(
            vertices,
            [0.44, 1.0, 1.81],
            "{what}: the vertices along the bridge's line"
        );
    }
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
    // Pinned red since PR 6053: the first case to run escalates at the
    // sector test at every ε, its margin just over the zero
    // (`work/splitdoor/main-red-a-prism-tilted-within-the-zero-escalates-at-the-sector-reach-since-arm-vs-reach.md`).
    test_utils::pinned_red::row(
        t().eps(),
        &test_utils::pinned_red::EVERY_ROW,
        "the prism tilted within the zero",
        &[
            "the 240° prism tilted 0.2ε towards 25°, and the plate, member order [3, 2, 1, 0]: \
             step 1 refused: Escalated { decision: Coincidence(Sectors, Moot), diag: \
             Indeterminate { margin: MarginDiag(Value(",
            ", None, ZeroWord(0))), terminal_sliver: false, band: Band { zero: ",
            " }, predicate: Some(\"bool_sector_within\") } }",
        ],
        a_prism_tilted_within_the_zero_builds_as_upright_row,
    );
}

/// The row [`a_prism_tilted_within_the_zero_builds_as_upright`] runs, under its pin.
fn a_prism_tilted_within_the_zero_builds_as_upright_row() {
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
                &[6, 1, 2, 1],
                &none,
            );
            every_order(&label, &prisms, false, [15, 27, 18], &[2, 2, 1], &none);
        }
    }
}

/// **A prism whose side face holds the line builds in every member
/// order**: the plate, the prisms over 0°–50° and 120°–170°, and a
/// third over the triangle from 0.3 at 80° and 260° to 0.4 at 350°, its
/// side through the line in no other face's plane. Folded last onto the
/// plate, the third and the 120° prism, the 0° prism meets a contact
/// line whose ray holds the third's subdivision bisector, where the
/// plate's top parts the third's face, and its edge lies in that face's
/// half-turn: it is dropped there. Where the plate joins a body holding the third and
/// the 120° prism, the 120° prism's edge pierces the plate's top inside
/// the third's face, whose section passes the point, so the top's
/// section is one ring through the point twice. They pass tier 3′
/// against the last step's records whichever member folds last: the
/// step that folds the third last records the others' corners resting
/// on its face on the piece of it that holds them, and the plate or the
/// 0° prism folded last drops the 120° prism's edge in the third's face
/// and its corner at the edge's top end, which no step but the one that
/// joined them reads. In three orders a step reads the line's foot touching the top
/// beside a partner whose link runs along it (`vtxfac::partner_side`),
/// the second, which builds sound, and the fold builds after it.
#[test]
fn a_prism_whose_face_holds_the_line_builds_in_every_member_order() {
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
    // The orders whose pair beside a partner along the top is read, and
    // the step that reads it, sound.
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(prisms.iter().map(Prism::body));
    let probes = probes();
    for (order, step) in [([3, 2, 1, 0], 2), ([2, 3, 1, 0], 2), ([2, 0, 3, 1], 2)] {
        let what = format!("a face through the line, member order {order:?}");
        let r = fold(&members, &order[..=step])
            .unwrap_or_else(|(k, e)| panic!("{what}: step {k} refused: {e:?}"));
        assert_eq!(validate_geometric(&r.body, t()), Ok(()), "{what}: tier 3");
        let plate = order[..=step].contains(&0);
        let held: Vec<Prism> = order[..=step]
            .iter()
            .filter(|&&i| i > 0)
            .map(|&i| prisms[i - 1])
            .collect();
        let v = topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
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
        &[2, 2, 0, 0],
        &none,
    );
}

/// **A prism against a contact line whose ray holds a bisector builds
/// or refuses typed in every op**: the plate, the third and the 120°
/// prism of the row above, folded in every member order, then met in
/// either operand slot by an upright prism on the line, its edge along
/// the 120° prism's, both in the third's face, where at the plate's top
/// the ray holds that face's bisector. The 0° prism lies in the third's
/// half-turn about the line, and every body it makes is sound against
/// the members' closed forms. One over 40°–110° crosses the third's
/// face along the line, where the 120° prism's edge lies in it, its top
/// above that prism's corner or below it, and refuses `VertexReadTwice`
/// in every op, slot and order; the refusal's recourse builds (the row
/// below).
#[test]
fn a_prism_against_a_contact_line_on_a_bisector_builds_or_refuses_typed_in_every_op() {
    let face = Prism {
        foot: [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
        z: (0.44, 1.81),
        moved: 0.0,
        tilt: (0.0, 0.0),
    };
    let held = [Prism::on_line(120.0, 170.0, (0.47, 1.7)), face];
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(held.iter().map(Prism::body));
    let in_line = |q: [f64; 3]| in_plate(q) || held.iter().any(|p| p.holds(q));
    let line_volume = union_volume(&held, true);
    let lines: Vec<_> = orders(members.len())
        .into_iter()
        .map(|order| {
            let body = fold(&members, &order)
                .unwrap_or_else(|(k, e)| panic!("the line body {order:?}: step {k} refused: {e:?}"))
                .body;
            (order, body)
        })
        .collect();
    for (tool, crosses) in [
        (Prism::on_line(0.0, 50.0, (0.5, 2.0)), false),
        (Prism::on_line(40.0, 110.0, (0.5, 2.0)), true),
        (Prism::on_line(40.0, 110.0, (0.5, 1.6)), true),
    ] {
        let b = tool.body();
        let solo = area(&tool.footprint()) * (tool.z.1 - tool.z.0);
        let common = line_volume + solo - union_volume(&[held[0], held[1], tool], true);
        let tool_closed = Closed {
            body: &b,
            holds: &|q| tool.holds(q),
            volume: solo,
        };
        for (order, body) in &lines {
            let line = Closed {
                body,
                holds: &in_line,
                volume: line_volume,
            };
            let refuses = |_: &str, _| crosses.then_some(BooleanErrorKind::VertexReadTwice);
            let what = format!("the prism over {:?}, the line body {order:?}", tool.foot);
            every_op(&what, &line, &tool_closed, common, &refuses);
            if !crosses {
                edges_on_the_line_recorded(&what, body, &b);
            }
        }
    }
}

/// **The edges along the line of every body `x` and `y` make are
/// recorded where they touch**: every op, either operand first, builds,
/// and tier 3′ over the result's records refuses no overlap of two
/// edges, nor any record it reads stale or contradicted. The 0° prism's
/// edge runs along the line body's, which holds the third's face beside
/// it: in a subtraction both lie on the result, and each end of their
/// shared stretch reads the face with the line body's edge
/// (`recl::HeldFace`, `recl::Folded`).
fn edges_on_the_line_recorded(what: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>) {
    for (name, op, _) in OPS {
        for (first, a, b) in [(true, x, y), (false, y, x)] {
            let what = format!("{name} of {what}, the line body first: {first}");
            let got = op(a, b, t());
            if pinned(&what, &got) {
                continue;
            }
            let r = match got {
                Ok(BooleanResult::Body(r)) => r,
                Ok(BooleanResult::Empty) => continue,
                Err(e) => panic!("{what}: builds, got {e:?}"),
            };
            if let Err(errors) = validate_pseudomanifold(&r.body, &r.contacts, t()) {
                let wrong = errors.iter().find(|e| {
                    matches!(
                        e,
                        topo::ValidationError::UnrecordedContact {
                            contact: topo::CensusContact::EdgeEdgeOverlap { .. },
                            ..
                        } | topo::ValidationError::StaleContactDeclaration { .. }
                            | topo::ValidationError::ContactContradicted { .. }
                    )
                });
                assert!(wrong.is_none(), "{what}: {wrong:?} among {errors:?}");
            }
        }
    }
}

/// **A split of an edge lying in an earlier face reads the vertex it
/// mints against that face**: the per-PR set's poses of
/// [`a_contact_line_beside_a_face_holding_it_builds_in_every_op`] that
/// go wrong without that read. The wedge towards 350° holds the
/// line in its face, and the 120° prism's edge lies in it; each upright
/// prism, met first, splits that edge at a later face, and unions with
/// the line body into a body sound against the members' closed forms.
#[test]
fn a_split_of_an_edge_lying_in_an_earlier_face_reads_its_vertex_against_it() {
    let p120 = Prism::on_line(120.0, 170.0, (0.47, 1.7));
    let plate = finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    );
    let above = |p: &Prism| area(&p.footprint()) * (p.z.1 - PLATE[2].1);
    let wedge = Wedge {
        half: 45.0,
        toward: 350.0,
    };
    let members = [plate, p120.body(), wedge.body()];
    let in_line = |q| in_plate(q) || p120.holds(q) || wedge.holds(q);
    let line_volume = 6.0 + above(&p120) + wedge.volume() - wedge.common_with(&p120);
    for order in [[0, 1, 2], [1, 0, 2]] {
        let line = fold(&members, &order)
            .unwrap_or_else(|(k, e)| panic!("the line body {order:?}: step {k} refused: {e:?}"))
            .body;
        for tool in [
            Prism::on_line(0.0, 50.0, (0.5, 2.0)),
            Prism::on_line(40.0, 110.0, (0.5, 2.0)),
            Prism::on_line(180.0, 230.0, (0.5, 2.0)),
            Prism::on_line(280.0, 330.0, (0.5, 2.0)),
        ] {
            let what = format!("the line body {order:?} and the prism over {:?}", tool.foot);
            let body = match union(&tool.body(), &line, t()) {
                Ok(BooleanResult::Body(r)) => r.body,
                other => panic!("{what}: {other:?}"),
            };
            sound(&what, &body, &|q| tool.holds(q) || in_line(q));
            let solo = area(&tool.footprint()) * (tool.z.1 - tool.z.0);
            let common =
                area(&tool.footprint()) * (PLATE[2].1 - tool.z.0) + wedge.common_with(&tool);
            let v = topo::test_support::pinned_volume("the measured body", &body, t()).volume;
            let volume = line_volume + solo - common;
            assert!(
                (v - volume).abs() < 1e-9,
                "{what}: volume {v}, closed form {volume}"
            );
        }
    }
}

/// **The recourse of a prism crossing a face beside a corner resting on
/// it builds in every op**: the line body of the row above is two
/// pieces, the plate with the third and the 120° prism, touching along
/// the line where the 120° prism's corner rests on the third's face. A
/// prism over 40°–110°, its top above that corner or below it, refuses
/// `VertexReadTwice` against the joined body in every op; applied to
/// each piece, the results then combined, as the refusal's recourse says,
/// each op builds a body sound against the members' closed forms: the
/// union in either grouping, the intersection of each piece joined, the
/// line less the prism as the plate's piece less it joined to the 120°
/// prism it misses, and the prism less the line piece by piece.
#[test]
fn the_recourse_of_a_prism_crossing_a_face_beside_a_resting_corner_builds() {
    let third = Prism {
        foot: [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
        z: (0.44, 1.81),
        moved: 0.0,
        tilt: (0.0, 0.0),
    };
    let p120 = Prism::on_line(120.0, 170.0, (0.47, 1.7));
    let plate = finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    );
    let built = |what: &str, r: Result<BooleanResult<f64>, BooleanError>| match r {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {other:?}"),
    };
    let base = built("the plate and the third", union(&plate, &third.body(), t()));
    let piece = p120.body();
    let in_line = |q| in_plate(q) || third.holds(q) || p120.holds(q);
    let line_volume = union_volume(&[third, p120], true);
    for top in [2.0, 1.6] {
        let tool = Prism::on_line(40.0, 110.0, (0.5, top));
        let b = tool.body();
        let solo = area(&tool.footprint()) * (tool.z.1 - tool.z.0);
        let all = union_volume(&[third, p120, tool], true);
        let common = line_volume + solo - all;
        let what = |route: &str| format!("the prism topped at {top}, {route}");
        // The 120° prism meets the tool only along the line.
        type Route<'a> = (&'a str, AtRestBody<f64>, f64, &'a dyn Fn([f64; 3]) -> bool);
        let routes: [Route; 5] = [
            (
                "the union, the plate's piece first",
                built(
                    &what("∪ 1"),
                    union(&built(&what("∪ 1a"), union(&b, &base, t())), &piece, t()),
                ),
                all,
                &|q| in_line(q) || tool.holds(q),
            ),
            (
                "the union, the 120° prism first",
                built(
                    &what("∪ 2"),
                    union(&built(&what("∪ 2a"), union(&b, &piece, t())), &base, t()),
                ),
                all,
                &|q| in_line(q) || tool.holds(q),
            ),
            (
                "the intersection, piece by piece",
                {
                    let x = built(&what("∩ a"), topo::intersect(&base, &b, t()));
                    match topo::intersect(&piece, &b, t()) {
                        Ok(BooleanResult::Empty) => x,
                        Ok(BooleanResult::Body(y)) => built(&what("∩"), union(&x, &y.body, t())),
                        Err(e) => panic!("{}: {e:?}", what("∩ b")),
                    }
                },
                common,
                &|q| in_line(q) && tool.holds(q),
            ),
            (
                "the line less the prism",
                built(
                    &what("∖"),
                    union(
                        &built(&what("∖ a"), topo::subtract(&base, &b, t())),
                        &piece,
                        t(),
                    ),
                ),
                line_volume - common,
                &|q| in_line(q) && !tool.holds(q),
            ),
            (
                "the prism less the line",
                built(
                    &what("∖′"),
                    topo::subtract(
                        &built(&what("∖′ a"), topo::subtract(&b, &base, t())),
                        &piece,
                        t(),
                    ),
                ),
                solo - common,
                &|q| tool.holds(q) && !in_line(q),
            ),
        ];
        for (route, body, volume, holds) in &routes {
            let what = what(route);
            sound(&what, body, holds);
            let v = topo::test_support::pinned_volume("the measured body", body, t()).volume;
            assert!(
                (v - volume).abs() < 1e-9,
                "{what}: volume {v}, closed form {volume}"
            );
        }
    }
}

/// A wedge whose triangular face lies in the vertical plane through the
/// line at 80° and 260°, its apex on the line at [`MEET`] and its face
/// opening upward by `half` (degrees) either side of the line, extruded
/// 0.3 towards the bearing `toward` (350° or 170°, across that plane).
#[derive(Clone, Copy)]
struct Wedge {
    half: f64,
    toward: f64,
}

impl Wedge {
    /// Its frame: the face's in-plane axis and the extrusion, a
    /// right-handed pair with +z.
    fn frame(&self) -> ([f64; 3], [f64; 3]) {
        let (s, c) = self.toward.to_radians().sin_cos();
        let w = [c, s, 0.0];
        // u × z = w.
        ([-s, c, 0.0], w)
    }

    fn foot(&self) -> [(f64, f64); 3] {
        let h = 0.4 * self.half.to_radians().tan();
        [(0.0, 0.0), (h, 0.4), (-h, 0.4)]
    }

    fn body(&self) -> AtRestBody<f64> {
        let (u, w) = self.frame();
        let mut body = topo::Body::<f64>::new();
        common::prism_ops(
            &mut body,
            &self.foot(),
            (0.0, 0.3),
            |x, y, z| {
                Point3::new(
                    x.mul_add(u[0], z.mul_add(w[0], MEET[0])),
                    x.mul_add(u[1], z.mul_add(w[1], MEET[1])),
                    MEET[2] + y,
                )
            },
            common::FaceGeometry::Certified,
            t(),
        );
        common::describe_as_intersections(&mut body, t());
        finished("a wedge on the line", body, t())
    }

    /// **The analytic oracle**, off its closed form.
    fn holds(&self, q: [f64; 3]) -> bool {
        let (u, w) = self.frame();
        let d = [q[0] - MEET[0], q[1] - MEET[1], q[2] - MEET[2]];
        let (x, y) = (d[0].mul_add(u[0], d[1] * u[1]), d[2]);
        let z = d[0].mul_add(w[0], d[1] * w[1]);
        let h = self.half.to_radians().tan();
        0.0 < z && z < 0.3 && y < 0.4 && x.abs() < y * h
    }

    fn volume(&self) -> f64 {
        area(&self.foot()) * 0.3
    }

    /// **Its common part with an upright prism, in closed form.** Its
    /// section at height `y` above the apex is the strip `0 < z < 0.3`
    /// of its frame cut to `|x| < y·tan(half)`, so the section's part in
    /// the prism's footprint is a fixed polygon cut to that band: its
    /// area is quadratic in `y` between the heights where the band's
    /// sides pass a vertex of that polygon, and Simpson's rule is exact
    /// on each such piece.
    fn common_with(&self, p: &Prism) -> f64 {
        let (u, w) = self.frame();
        let h = self.half.to_radians().tan();
        let world = |x: f64, z: f64| {
            (
                x.mul_add(u[0], z.mul_add(w[0], MEET[0])),
                x.mul_add(u[1], z.mul_add(w[1], MEET[1])),
            )
        };
        let local = |(x, y): (f64, f64)| {
            let d = (x - MEET[0], y - MEET[1]);
            (d.0.mul_add(u[0], d.1 * u[1]), d.0.mul_add(w[0], d.1 * w[1]))
        };
        let foot = p.footprint();
        let section = |y: f64| {
            let mut r = vec![
                world(-y * h, 0.0),
                world(y * h, 0.0),
                world(y * h, 0.3),
                world(-y * h, 0.3),
            ];
            if area(&r) < 0.0 {
                r.reverse();
            }
            for i in 0..3 {
                r = clip(&r, foot[i], foot[(i + 1) % 3]);
            }
            area(&r)
        };
        let (lo, hi) = ((p.z.0 - MEET[2]).max(0.0), (p.z.1 - MEET[2]).min(0.4));
        if lo >= hi {
            return 0.0;
        }
        // The polygon's vertices: the footprint's, and its edges'
        // crossings of the strip's two sides.
        let tri = foot.map(local);
        let mut cuts = vec![lo, hi];
        for i in 0..3 {
            let (a, b) = (tri[i], tri[(i + 1) % 3]);
            cuts.push(a.0.abs() / h);
            for z in [0.0, 0.3] {
                if (a.1 - z) * (b.1 - z) < 0.0 {
                    cuts.push(((z - a.1) / (b.1 - a.1)).mul_add(b.0 - a.0, a.0).abs() / h);
                }
            }
        }
        cuts.retain(|&y| lo <= y && y <= hi);
        cuts.sort_by(f64::total_cmp);
        cuts.windows(2)
            .map(|c| {
                let m = section(f64::midpoint(c[0], c[1]));
                (c[1] - c[0]) / 6.0 * 4.0f64.mul_add(m, section(c[0]) + section(c[1]))
            })
            .sum()
    }
}

/// **A contact line lying in a face of the other solid builds in every
/// op**: the plate and the prisms over 0°–50° and 120°–170°, folded in
/// every member order, hold the line's two edges at the plate's top in
/// one vertex; a [`Wedge`] there, its apex on the line, holds the ray
/// up the line inside its face, which no bisector parts there. Each op,
/// in either operand slot, is sound against the members' closed forms.
/// The wedge lies above the plate and the two prisms apart, so its
/// common part with the body is its parts in each prism.
#[test]
fn a_contact_line_in_a_face_of_the_other_builds_in_every_op() {
    let held = [
        Prism::on_line(0.0, 50.0, (0.5, 2.0)),
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
    ];
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(held.iter().map(Prism::body));
    let in_line = |q: [f64; 3]| in_plate(q) || held.iter().any(|p| p.holds(q));
    let line_volume = union_volume(&held, true);
    let lines: Vec<_> = orders(members.len())
        .into_iter()
        .map(|order| {
            let body = fold(&members, &order)
                .unwrap_or_else(|(k, e)| panic!("the line body {order:?}: step {k} refused: {e:?}"))
                .body;
            (order, body)
        })
        .collect();
    for wedge in [
        Wedge {
            half: 45.0,
            toward: 350.0,
        },
        Wedge {
            half: 70.0,
            toward: 170.0,
        },
    ] {
        let b = wedge.body();
        let tool = Closed {
            body: &b,
            holds: &|q| wedge.holds(q),
            volume: wedge.volume(),
        };
        let common = held.iter().map(|p| wedge.common_with(p)).sum();
        for (order, body) in &lines {
            let line = Closed {
                body,
                holds: &in_line,
                volume: line_volume,
            };
            every_op(
                &format!(
                    "the wedge of {}° towards {}°, the line body {order:?}",
                    wedge.half, wedge.toward
                ),
                &line,
                &tool,
                common,
                &|_, _| None,
            );
        }
    }
}

/// **A contact line beside a face that holds it, no bisector there,
/// builds in every op**: the plate, the 120° prism and a [`Wedge`]
/// whose face holds the line from its apex at the plate's top, so that
/// vertex holds the 120° prism's edge and a sector of the wedge's face
/// the ray runs inside. Folded with the 120° prism last, the body
/// refuses `VertexReadTwice`: its corner rests on the wedge's face. In
/// every other member order it meets upright prisms on the line, none
/// meeting the 120° prism's wedge, by every op in either operand slot,
/// and is sound against the members' closed forms: the wedge lies above
/// the plate, so the common parts are its parts in each prism. Its
/// union, folded with the members' records carried, passes tier 3′
/// either way round.
#[test]
fn a_contact_line_beside_a_face_holding_it_builds_in_every_op() {
    // A union of `x` and `y` carrying their records.
    let carried_union =
        |x: &AtRestBody<f64>, y: &AtRestBody<f64>, records: [topo::OperandRecords; 2]| {
            let [records_a, records_b] = records;
            let inputs = topo::BooleanInputs {
                records_a,
                records_b,
            };
            match topo::boolean_op_with(
                topo::BooleanOp::Union,
                x,
                y,
                &inputs,
                topo::SweepStrategy::Realized,
                t(),
            ) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("a carried union: {other:?}"),
            }
        };
    let p120 = Prism::on_line(120.0, 170.0, (0.47, 1.7));
    let plate = finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    );
    let above = |p: &Prism| area(&p.footprint()) * (p.z.1 - PLATE[2].1);
    for wedge in [
        Wedge {
            half: 45.0,
            toward: 350.0,
        },
        Wedge {
            half: 70.0,
            toward: 170.0,
        },
    ] {
        let members = [plate.clone(), p120.body(), wedge.body()];
        let in_line = |q| in_plate(q) || p120.holds(q) || wedge.holds(q);
        let line_volume = 6.0 + above(&p120) + wedge.volume() - wedge.common_with(&p120);
        for order in orders(members.len()) {
            let what = format!(
                "the wedge towards {}°, the line body {order:?}",
                wedge.toward
            );
            let line_body = match fold(&members, &order) {
                Err((k, e)) if order[2] == 1 => {
                    assert_eq!(
                        e.kind(),
                        BooleanErrorKind::VertexReadTwice,
                        "{what}: step {k}"
                    );
                    continue;
                }
                Err((k, e)) => panic!("{what}: step {k} refused: {e:?}"),
                Ok(_) if order[2] == 1 => panic!("{what}: built where it refuses"),
                Ok(r) => r.body,
            };
            let mut carried_body = members[order[0]].clone();
            let mut records = topo::OperandRecords::default();
            for &i in &order[1..] {
                let r = carried_union(
                    &carried_body,
                    &members[i],
                    [records, topo::OperandRecords::default()],
                );
                records = r.contacts.carried();
                carried_body = r.body.clone();
            }
            let line = Closed {
                body: &line_body,
                holds: &in_line,
                volume: line_volume,
            };
            for tool in [
                Prism::on_line(0.0, 50.0, (0.5, 2.0)),
                Prism::on_line(40.0, 110.0, (0.5, 2.0)),
                Prism::on_line(180.0, 230.0, (0.5, 2.0)),
                Prism::on_line(280.0, 330.0, (0.5, 2.0)),
            ] {
                let what = format!("{what}, the prism over {:?}", tool.foot);
                let b = tool.body();
                for line_first in [true, false] {
                    let none = topo::OperandRecords::default;
                    let r = if line_first {
                        carried_union(&carried_body, &b, [records.clone(), none()])
                    } else {
                        carried_union(&b, &carried_body, [none(), records.clone()])
                    };
                    assert_eq!(
                        validate_pseudomanifold(&r.body, &r.contacts, t()).err(),
                        None,
                        "{what}, the line body first: {line_first}: the carried union at 3′"
                    );
                }
                let solo = area(&tool.footprint()) * (tool.z.1 - tool.z.0);
                let common =
                    area(&tool.footprint()) * (PLATE[2].1 - tool.z.0) + wedge.common_with(&tool);
                every_op(
                    &what,
                    &line,
                    &Closed {
                        body: &b,
                        holds: &|q| tool.holds(q),
                        volume: solo,
                    },
                    common,
                    &|_, _| None,
                );
            }
        }
    }
}

/// **The plate joins a body whose one lump's edge lies in the other's
/// face** as one body: the 120° prism touching the row above's third
/// prism along the line, its edge piercing the plate's top inside the
/// third's side face. The top's section is one ring through the point
/// twice, so tier 3 passes, with one vertex there and the closed-form
/// volume.
#[test]
fn the_plate_joins_an_edge_lying_in_a_face_as_one_body() {
    let prisms = [
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
        Prism {
            foot: [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
            z: (0.44, 1.81),
            moved: 0.0,
            tilt: (0.0, 0.0),
        },
    ];
    let body = match union(&prisms[0].body(), &prisms[1].body(), t()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("the two prisms: {other:?}"),
    };
    let plate = finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    );
    let r = match union(&body, &plate, t()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("body ∪ plate: {other:?}"),
    };
    assert_eq!(validate_geometric(&r, t()), Ok(()), "tier 3");
    let c = at(Point3::new(MEET[0], MEET[1], MEET[2]));
    let at_c = r
        .vertices()
        .filter(|&(k, _)| at(topo::readback::vertex_point(&r, k).unwrap()) == c)
        .count();
    assert_eq!(at_c, 1, "vertices at MEET");
    let v = topo::test_support::pinned_volume("the measured body", &r, t()).volume;
    let volume = union_volume(&prisms, true);
    assert!(
        (v - volume).abs() < 1e-9,
        "volume {v}, closed form {volume}"
    );
}

/// **The plate against a body whose one lump's edge lies in the other's
/// face builds in every op**: the third prism of the row above united
/// with the 120° prism, which touches its side face along the line,
/// and the plate, whose top that edge pierces inside that face. Each
/// op, either way round, is sound by tier 3, its closed-form volume and
/// its material at the probes against the members' closed forms.
#[test]
fn the_plate_against_an_edge_lying_in_a_face_builds_in_every_op() {
    let prisms = [
        Prism::on_line(120.0, 170.0, (0.47, 1.7)),
        Prism {
            foot: [bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
            z: (0.44, 1.81),
            moved: 0.0,
            tilt: (0.0, 0.0),
        },
    ];
    let body = match union(&prisms[0].body(), &prisms[1].body(), t()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("the two prisms: {other:?}"),
    };
    let plate = finished(
        "the plate",
        common::brick(PLATE[0], PLATE[1], PLATE[2], t()),
        t(),
    );
    // The prisms meet along the line alone, and each crosses the
    // plate's top.
    let solid: f64 = prisms
        .iter()
        .map(|p| area(&p.footprint()) * (p.z.1 - p.z.0))
        .sum();
    let common: f64 = prisms
        .iter()
        .map(|p| area(&p.footprint()) * (PLATE[2].1 - p.z.0))
        .sum();
    let in_prisms = |q: [f64; 3]| prisms.iter().any(|p| p.holds(q));
    every_op(
        "the two prisms and the plate",
        &Closed {
            body: &body,
            holds: &in_prisms,
            volume: solid,
        },
        &Closed {
            body: &plate,
            holds: &in_plate,
            volume: 6.0,
        },
        common,
        &|_, _| None,
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
            &[6, 1, 2, 1],
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
                    let v =
                        topo::test_support::pinned_volume("the measured body", &r.body, t()).volume;
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

/// **Three prisms over 60° sectors glue their flush continuation in
/// every member order**: each lateral face lies in the plane of
/// another's, touching it along the line, and its margins decide it
/// (D10).
#[test]
fn three_prisms_flush_along_one_line_build_undeclared_in_every_member_order() {
    let prisms = [
        Prism::on_line(0.0, 60.0, (0.5, 2.0)),
        Prism::on_line(120.0, 180.0, (0.47, 1.7)),
        Prism::on_line(240.0, 300.0, (0.44, 1.81)),
    ];
    let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
    let mut members = vec![finished("the plate", p, t())];
    members.extend(prisms.iter().map(Prism::body));
    for order in orders(members.len()) {
        if let Err((k, e)) = fold(&members, &order) {
            panic!("60° sectors, member order {order:?}: step {k} refused {e:?}");
        }
    }
}

/// **The carried fold serves the uncarried fold's body and passes 3′
/// (D10)**: every member order of the 50° fixture, of four prisms, and
/// of the 50° fixture with a corner moved 0.1ε and 0.5ε, each step carrying the
/// records the step before it returned. Where each step carries the
/// last one's records nothing on the line is dropped, so the result
/// passes 3′ with no finding: the `dropped` counts [`every_order`] pins
/// are the uncarried fold's alone. Red if the carry changes the body,
/// if a carried edge-edge row's lineage loses a touch, or if the census
/// asks for a row the carry never brings.
#[test]
fn the_carried_fold_passes_three_prime_in_every_member_order() {
    let eps = t().eps();
    for (label, prisms) in [
        ("three", three()),
        ("four", four()),
        ("three, one moved 0.1ε", three_off(0.1 * eps)),
        ("three, one moved 0.5ε", three_off(0.5 * eps)),
    ] {
        let p = common::brick(PLATE[0], PLATE[1], PLATE[2], t());
        let mut members = vec![finished("the plate", p, t())];
        members.extend(prisms.iter().map(Prism::body));
        for order in orders(members.len()) {
            let what = format!("{label}, member order {order:?}");
            let uncarried = shape(&fold(&members, &order).unwrap().body);
            let mut body = members[order[0]].clone();
            let mut carried = topo::OperandRecords::default();
            let mut last = None;
            for (k, &i) in order.iter().enumerate().skip(1) {
                let inputs = topo::BooleanInputs {
                    records_a: carried,
                    ..topo::BooleanInputs::default()
                };
                let r = match topo::boolean_op_with(
                    topo::BooleanOp::Union,
                    &body,
                    &members[i],
                    &inputs,
                    topo::SweepStrategy::Realized,
                    t(),
                ) {
                    Ok(BooleanResult::Body(r)) => r,
                    other => panic!("{what}: step {k}: {other:?}"),
                };
                carried = r.contacts.carried();
                body = r.body.clone();
                last = Some(r);
            }
            assert!(shape(&body) == uncarried, "{what}: another body");
            let r = last.expect("at least two members");
            assert_eq!(
                validate_pseudomanifold(&r.body, &r.contacts, t()).err(),
                None,
                "{what}: the carried fold at 3′ ({} e-e rows)",
                r.contacts.edge_edge().count()
            );
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

/// **Pierces of one point build one body in every member order**: the
/// review's hunt seeds 506 (three prisms) and 507 (four), overlapping
/// sectors through the plate's top at [`MEET`], some upright on the line
/// and some leant about it. Folded first, their edges pierce the top at
/// [`MEET`], and one ring serves them all (`vtxfac::mint_ring`). Every
/// order builds sound by tier 3,
/// corners, a probe oracle about [`MEET`], a Monte Carlo volume, and one
/// body.
#[test]
fn pierces_of_one_point_build_one_body_in_every_member_order() {
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
            assert_eq!(corners_disjoint(body, t()), Ok(()), "{what}: corners");
            let v = topo::test_support::pinned_volume("the measured body", body, t()).volume;
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
