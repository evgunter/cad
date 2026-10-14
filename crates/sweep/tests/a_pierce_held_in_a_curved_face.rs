//! **A pierce point held inside a curved face of the piercing operand,
//! or a plane face holding one over a curved pierced face.**
//!
//! An operand whose solids touch along a line, one's edge lying in the
//! other's face, pierces a face of the other operand where that face
//! crosses its own. The section of the two passes the point, and the
//! face holding it takes a ring vertex there
//! (`topo::boolean::vtxfac::faces_holding_pierce`). The rows:
//!
//! - **A cylinder wall holds the point**: an upright rod with a square
//!   prism whose edge lies along one of the rod's rulings, joined, and a
//!   plate whose top that edge pierces inside the wall. Three poses of
//!   the ruling, both join orders, every op with the plate either way
//!   round, and every member order of the fold, carried.
//! - **A plane face holds it over a curved pierced face**: two prisms,
//!   one's edge lying in the other's side face, joined, and a lying
//!   cylinder whose top that edge pierces.
//! - **Both curved**: the rod and its square, joined, and the lying
//!   cylinder.
//! - **A wall whose trim cannot be read there**: the rod's wall pocketed
//!   beside the point, which the trim does not read with its ring. The
//!   point may lie in the face, so nothing is guessed: every pose
//!   refuses `HeldPierceUnbuilt` typed, its recourse the fold's other
//!   order, which builds.
//!
//! Each body that builds is sound: tier 3, tier 3′ on the carried fold,
//! its material at probes about the point against the members' closed
//! forms, and its volume against theirs (the lying cylinder's
//! intersections by quadrature over its height function, which is
//! exact to far below the tolerance).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, corners, finished, prism_at};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, BooleanError, BooleanErrorKind, BooleanResult};

/// The rod's radius and axis.
const R: f64 = 0.4;
const C: (f64, f64) = (1.5, 1.0);
/// The rod's height range.
const ROD_Z: (f64, f64) = (0.5, 2.0);
/// The square's height range, staggered off the rod's.
const SQ_Z: (f64, f64) = (0.44, 1.81);
/// The square's area: its diagonals are both `0.6`.
const SQ_AREA: f64 = 0.18;
/// The lying cylinder's radius; its axis runs along `x` at `y = 1`,
/// `z = 1 − RC`, so its top passes `(·, 1, 1)`.
const RC: f64 = 2.0;
/// The lying cylinder's span along `x`. Its end cap at `LYING_X.0`
/// crosses both prisms, so neither leaves a ring on its wall.
const LYING_X: (f64, f64) = (1.47, 3.0);
/// The plate.
const PLATE: [(f64, f64); 3] = [(0.0, 3.0), (0.0, 2.0), (0.0, 1.0)];
/// The ruling's bearings off the rod's seams (at `0` and `π`).
const TURNS: [f64; 3] = [0.5, 2.0, -1.0];

fn tol() -> Tol {
    Tol::witness()
}

fn extruded(
    plane: profile::SketchPlane<f64>,
    lp: profile::ProfileLoop<f64>,
    depth: f64,
) -> AtRestBody<f64> {
    let p = profile::Profile::new(plane, vec![lp])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &p,
        Extrusion::Distance {
            depth,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("the cylinder", body, tol())
}

/// The upright rod, through the extrude door: two wall faces, their
/// seam rulings at `±x` of its axis.
fn rod() -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(C.0, C.1), R, tol()).unwrap();
    let plane = profile::SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, ROD_Z.0)));
    extruded(plane, lp.into(), ROD_Z.1 - ROD_Z.0)
}

/// The cylinder lying along `x` over [`LYING_X`].
fn lying() -> AtRestBody<f64> {
    let lp = profile::circle(Point2::new(1.0, 1.0 - RC), RC, tol()).unwrap();
    let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_yz(Point3::new(
        LYING_X.0, 0.0, 0.0,
    )));
    extruded(plane, lp.into(), LYING_X.1 - LYING_X.0)
}

/// The square's corners, unturned and counterclockwise: its corner at
/// `(R, 0)` from the rod's axis, its other corners outside the rod.
const SQ: [(f64, f64); 4] = [(R, 0.0), (R + 0.3, -0.3), (R + 0.6, 0.0), (R + 0.3, 0.3)];

fn turned(turn: f64, (x, y): (f64, f64)) -> (f64, f64) {
    let (s, c) = turn.sin_cos();
    (C.0 + c * x - s * y, C.1 + s * x + c * y)
}

/// The square prism outside the rod whose edge is the rod's ruling at
/// bearing `turn`.
fn square_foot(turn: f64) -> Vec<(f64, f64)> {
    SQ.iter().map(|&p| turned(turn, p)).collect()
}

fn prism(foot: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
    finished(
        "a prism",
        prism_at(corners(foot), z.0, z.1 - z.0, tol()),
        tol(),
    )
}

/// The point at bearing `a` (degrees) `r` from the rod's axis.
fn bearing(a: f64, r: f64) -> (f64, f64) {
    let (s, c) = a.to_radians().sin_cos();
    (C.0 + r * c, C.1 + r * s)
}

/// Two prisms touching along the rod's axis line: one over the sector
/// 120°–170°, the other over a triangle whose side holds the line.
fn line_prisms() -> [(Vec<(f64, f64)>, (f64, f64)); 2] {
    [
        (
            vec![C, bearing(120.0, 0.4), bearing(170.0, 0.4)],
            (0.47, 1.7),
        ),
        (
            vec![bearing(80.0, 0.3), bearing(260.0, 0.3), bearing(350.0, 0.4)],
            (0.44, 1.81),
        ),
    ]
}

// The closed forms.

/// Inside the convex polygon `foot` (counterclockwise), strictly.
fn in_polygon(foot: &[(f64, f64)], (x, y): (f64, f64)) -> bool {
    (0..foot.len()).all(|i| {
        let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
        (b.0 - a.0) * (y - a.1) - (b.1 - a.1) * (x - a.0) > 0.0
    })
}

fn area(foot: &[(f64, f64)]) -> f64 {
    (0..foot.len())
        .map(|i| {
            let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
            a.0 * b.1 - b.0 * a.1
        })
        .sum::<f64>()
        / 2.0
}

/// The convex polygon's width along `x` at height `y`.
fn width(foot: &[(f64, f64)], y: f64) -> f64 {
    let xs: Vec<f64> = (0..foot.len())
        .filter_map(|i| {
            let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
            let (lo, hi) = (a.1.min(b.1), a.1.max(b.1));
            (lo <= y && y <= hi && a.1 != b.1).then(|| a.0 + (b.0 - a.0) * (y - a.1) / (b.1 - a.1))
        })
        .collect();
    let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if xs.is_empty() { 0.0 } else { hi - lo }
}

/// Composite Simpson's rule over `[a, b]`.
fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let n = 4000;
    let h = (b - a) / f64::from(n);
    let sum: f64 = (0..=n)
        .map(|i| {
            let w = if i == 0 || i == n {
                1.0
            } else if i % 2 == 1 {
                4.0
            } else {
                2.0
            };
            w * f(a + h * f64::from(i))
        })
        .sum();
    sum * h / 3.0
}

/// The lying cylinder's top over `y`.
fn roof(y: f64) -> f64 {
    1.0 - RC + (RC * RC - (y - 1.0).powi(2)).sqrt()
}

/// The part of the convex polygon `foot` (counterclockwise) at `x ≥ x0`.
fn clip_at(foot: &[(f64, f64)], x0: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..foot.len() {
        let (a, b) = (foot[i], foot[(i + 1) % foot.len()]);
        if a.0 >= x0 {
            out.push(a);
        }
        if (a.0 < x0) != (b.0 < x0) {
            let t = (x0 - a.0) / (b.0 - a.0);
            out.push((x0, a.1 + t * (b.1 - a.1)));
        }
    }
    out
}

/// The volume of the upright prism over `foot` from `z0` that lies in
/// the lying cylinder, whose top stays between `z0` and the prism's top
/// over `foot`: the integral of the width of `foot` within
/// [`LYING_X`] times the roof's height over `z0`, piecewise between
/// the clipped corners' heights.
fn under_roof(foot: &[(f64, f64)], z0: f64) -> f64 {
    let foot = clip_at(foot, LYING_X.0);
    let mut ys: Vec<f64> = foot.iter().map(|p| p.1).collect();
    ys.sort_by(f64::total_cmp);
    ys.windows(2)
        .map(|w| simpson(&|y| width(&foot, y) * (roof(y) - z0), w[0], w[1]))
        .sum()
}

fn in_rod(q: [f64; 3]) -> bool {
    (q[0] - C.0).powi(2) + (q[1] - C.1).powi(2) < R * R && ROD_Z.0 < q[2] && q[2] < ROD_Z.1
}

fn in_lying(q: [f64; 3]) -> bool {
    LYING_X.0 < q[0]
        && q[0] < LYING_X.1
        && (q[1] - 1.0).powi(2) + (q[2] - 1.0 + RC).powi(2) < RC * RC
}

fn in_plate(q: [f64; 3]) -> bool {
    (0..3).all(|i| PLATE[i].0 < q[i] && q[i] < PLATE[i].1)
}

fn in_prism(foot: &[(f64, f64)], z: (f64, f64), q: [f64; 3]) -> bool {
    z.0 < q[2] && q[2] < z.1 && in_polygon(foot, (q[0], q[1]))
}

/// Probes about the rod: a 9 × 9 × 14 grid within 0.95 of its axis
/// across, from `z = 0.06` to `1.98`, offset off the fixtures' planes
/// and rulings.
fn probes() -> Vec<[f64; 3]> {
    let mut out = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            for k in 0..14 {
                let across = |n: i32, d: f64| 0.235f64.mul_add(f64::from(n) - 4.0, d);
                out.push([
                    C.0 + across(i, 0.0137),
                    C.1 + across(j, 0.0071),
                    0.148f64.mul_add(f64::from(k), 0.0593),
                ]);
            }
        }
    }
    out
}

fn inside_of(body: &AtRestBody<f64>, q: [f64; 3]) -> Option<bool> {
    let band = geom_core::Band::linear(tol()).unwrap();
    match topo::point_in_solid(body, Point3::new(q[0], q[1], q[2]), band, tol()) {
        Ok(topo::SolidContainment::In) => Some(true),
        Ok(topo::SolidContainment::Out) => Some(false),
        Ok(topo::SolidContainment::OnBoundary)
        | Err(
            topo::PointInSolidError::Escalated { .. }
            | topo::PointInSolidError::VolumeUncertified
            | topo::PointInSolidError::Loop(topo::PointInLoopError::Escalated { .. }),
        ) => None,
        Err(e) => panic!("point in solid at {q:?}: {e:?}"),
    }
}

/// The winding number of `mesh` about `q`: the triangles' solid angles
/// over `4π` (Van Oosterom and Strackee), `1` inside a closed outward
/// mesh and `0` outside, away from its triangles.
fn winding(mesh: &mesh::Mesh, q: [f64; 3]) -> f64 {
    let q = Point3::new(q[0], q[1], q[2]);
    let mut omega = 0.0;
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            let [a, b, c] = tri.map(|i| mesh.positions[i as usize] - q);
            let (la, lb, lc) = (a.norm(), b.norm(), c.norm());
            let num = a.dot(b.cross(c));
            let den = la * lb * lc + a.dot(b) * lc + b.dot(c) * la + c.dot(a) * lb;
            omega += 2.0 * num.atan2(den);
        }
    }
    omega / (4.0 * PI)
}

/// **A body sound against a closed form**: tier 3, its volume, and its
/// material at the probes against `holds`. A probe the in-solid walk
/// leaves unread is read off the body's tessellation by its winding
/// number, where that is clear of a half; at least 99% are read.
fn sound(what: &str, body: &AtRestBody<f64>, holds: &dyn Fn([f64; 3]) -> bool, volume: f64) {
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    let v = topo::mass_properties(body, tol()).unwrap().volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "{what}: volume {v}, closed form {volume}"
    );
    let probes = probes();
    let mut mesh = None;
    let mut read = 0;
    for &q in &probes {
        let got = inside_of(body, q).or_else(|| {
            let mesh = mesh.get_or_insert_with(|| {
                mesh::tessellate(body, 1e-4, tol())
                    .unwrap_or_else(|e| panic!("{what}: the tessellation: {e:?}"))
            });
            let w = winding(mesh, q);
            ((w - 0.5).abs() > 0.4).then_some(w > 0.5)
        });
        if let Some(got) = got {
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

fn joined(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    match topo::union(a, b, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what}: {other:?}"),
    }
}

/// A Boolean op.
type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

/// The material an op makes of its operands' (A's, then B's).
type Material = fn(bool, bool) -> bool;

const OPS: [(&str, Op, Material); 3] = [
    ("union", topo::union, |a, b| a || b),
    ("intersection", topo::intersect, |a, b| a && b),
    ("subtraction", topo::subtract, |a, b| a && !b),
];

/// An operand in closed form: its body, its material and its volume.
struct Closed<'a> {
    body: &'a AtRestBody<f64>,
    holds: &'a dyn Fn([f64; 3]) -> bool,
    volume: f64,
}

/// **Every op of `held` and `other`, either operand first**: each
/// builds a body [`sound`] against the two closed forms, `common` their
/// common part's volume.
fn every_op(what: &str, held: &Closed, other: &Closed, common: f64) {
    for (name, op, material) in OPS {
        for held_first in [true, false] {
            let what = format!("{name} of {what}, the held operand first: {held_first}");
            let (x, y) = if held_first {
                (held, other)
            } else {
                (other, held)
            };
            let body = match op(x.body, y.body, tol()) {
                Ok(BooleanResult::Body(r)) => r.body,
                other => panic!("{what}: {other:?}"),
            };
            let volume = match name {
                "union" => x.volume + y.volume - common,
                "intersection" => common,
                _ => x.volume - common,
            };
            sound(
                &what,
                &body,
                &|q| material((x.holds)(q), (y.holds)(q)),
                volume,
            );
        }
    }
}

/// **The fold of `members` by union in every member order, carried**:
/// each step's records serve the next, so tier 3′ reads every contact
/// the fold made. Each order builds a body sound against `holds` and
/// `volume`, and passes tier 3′.
fn every_order(
    what: &str,
    members: &[AtRestBody<f64>],
    holds: &dyn Fn([f64; 3]) -> bool,
    volume: f64,
) {
    for order in topo::test_support::meeting::orders(members.len()) {
        let what = format!("{what}, member order {order:?}");
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
                tol(),
            ) {
                Ok(BooleanResult::Body(r)) => r,
                other => panic!("{what}: step {k}: {other:?}"),
            };
            carried = r.contacts.carried();
            body = r.body.clone();
            last = Some(r);
        }
        let r = last.expect("at least two members");
        sound(&what, &r.body, holds, volume);
        assert_eq!(
            topo::validate_pseudomanifold(&r.body, &r.contacts, tol()).err(),
            None,
            "{what}: tier 3′"
        );
    }
}

/// **The cylinder wall holds the point**: the square's edge on the
/// rod's ruling pierces the plate's top inside the rod's wall. Every
/// op of the joined pair and the plate, the pair joined either way
/// round, either operand first; and every member order of the fold.
#[test]
fn a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order() {
    let plate = finished(
        "the plate",
        brick(PLATE[0], PLATE[1], PLATE[2], tol()),
        tol(),
    );
    let rod_volume = PI * R * R * (ROD_Z.1 - ROD_Z.0);
    let sq_volume = SQ_AREA * (SQ_Z.1 - SQ_Z.0);
    let common = PI * R * R * (PLATE[2].1 - ROD_Z.0) + SQ_AREA * (PLATE[2].1 - SQ_Z.0);
    for turn in TURNS {
        let foot = square_foot(turn);
        let (rod, sq) = (rod(), prism(&foot, SQ_Z));
        let in_pair = |q: [f64; 3]| in_rod(q) || in_prism(&foot, SQ_Z, q);
        for (order, a, b) in [("rod ∪ square", &rod, &sq), ("square ∪ rod", &sq, &rod)] {
            let pair = joined(&format!("turn {turn}: {order}"), a, b);
            every_op(
                &format!("turn {turn}: ({order}) and the plate"),
                &Closed {
                    body: &pair,
                    holds: &in_pair,
                    volume: rod_volume + sq_volume,
                },
                &Closed {
                    body: &plate,
                    holds: &in_plate,
                    volume: 6.0,
                },
                common,
            );
        }
        every_order(
            &format!("turn {turn}: the rod, the square and the plate"),
            &[rod, sq, plate.clone()],
            &|q| in_pair(q) || in_plate(q),
            6.0 + rod_volume + sq_volume - common,
        );
    }
}

/// **A plane face holds the point over a curved pierced face**: two
/// prisms touching along the rod's axis line, one's side face holding
/// the other's edge, joined, and the lying cylinder, whose top that
/// edge pierces inside the side face.
#[test]
fn a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order() {
    let lying = lying();
    let prisms = line_prisms();
    let bodies: Vec<_> = prisms.iter().map(|(foot, z)| prism(foot, *z)).collect();
    let in_prisms = |q: [f64; 3]| prisms.iter().any(|(foot, z)| in_prism(foot, *z, q));
    let prisms_volume: f64 = prisms
        .iter()
        .map(|(foot, z)| area(foot) * (z.1 - z.0))
        .sum();
    let common: f64 = prisms.iter().map(|(foot, z)| under_roof(foot, z.0)).sum();
    let lying_volume = PI * RC * RC * (LYING_X.1 - LYING_X.0);
    for (order, a, b) in [
        ("sector ∪ triangle", &bodies[0], &bodies[1]),
        ("triangle ∪ sector", &bodies[1], &bodies[0]),
    ] {
        let pair = joined(order, a, b);
        every_op(
            &format!("({order}) and the lying cylinder"),
            &Closed {
                body: &pair,
                holds: &in_prisms,
                volume: prisms_volume,
            },
            &Closed {
                body: &lying,
                holds: &in_lying,
                volume: lying_volume,
            },
            common,
        );
    }
    every_order(
        "the two prisms and the lying cylinder",
        &[bodies[0].clone(), bodies[1].clone(), lying],
        &|q| in_prisms(q) || in_lying(q),
        lying_volume + prisms_volume - common,
    );
}

/// **A wall whose trim cannot be read at the point refuses typed**: the
/// joined rod and square pocketed beside the ruling, at bearing 2 rad
/// on the wall face the ruling lies in, so that face carries the
/// pocket's ring, which the cylinder's trim does not read. The point may
/// lie in the face, and nothing is guessed: the plate against the
/// pocketed pair refuses `HeldPierceUnbuilt`, not placed, either operand
/// first. Pocketing last builds.
#[test]
fn a_ringed_wall_holding_the_pierce_refuses_typed_and_pocketing_last_builds() {
    let plate = finished(
        "the plate",
        brick(PLATE[0], PLATE[1], PLATE[2], tol()),
        tol(),
    );
    let (px, py) = bearing(2.0f64.to_degrees(), R);
    let pocket = finished(
        "the pocket",
        brick(
            (px - 0.06, px + 0.06),
            (py - 0.06, py + 0.06),
            (1.3, 1.6),
            tol(),
        ),
        tol(),
    );
    let cut = |body: &AtRestBody<f64>, what: &str| match topo::subtract(body, &pocket, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("{what} ∖ the pocket: {other:?}"),
    };
    for turn in [0.5, 1.0] {
        let sq = prism(&square_foot(turn), SQ_Z);
        let pair = joined(&format!("turn {turn}: rod ∪ square"), &rod(), &sq);
        let pocketed = cut(&pair, &format!("turn {turn}: the pair"));
        for (what, a, b) in [
            ("the pocketed pair ∪ the plate", &pocketed, &plate),
            ("the plate ∪ the pocketed pair", &plate, &pocketed),
        ] {
            match topo::union(a, b, tol()) {
                Err(e @ BooleanError::HeldPierceUnbuilt { placed: false, .. }) => {
                    assert_eq!(e.kind(), BooleanErrorKind::HeldPierceUnbuilt);
                }
                other => panic!("turn {turn}: {what}: {other:?}"),
            }
        }
        let whole = joined(&format!("turn {turn}: the pair ∪ the plate"), &pair, &plate);
        let r = cut(&whole, &format!("turn {turn}: the pair and the plate"));
        assert_eq!(
            topo::validate_geometric(&r, tol()),
            Ok(()),
            "turn {turn}: pocketed last, at tier 3"
        );
    }
}
